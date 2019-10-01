use crate::Config;
use failure::{bail, format_err, Fallible};
use ipnetwork::IpNetwork;
use log::{debug, info};
use serde_json::{json, to_string_pretty};
use std::{
    fs::{self, create_dir_all},
    net::Ipv4Addr,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

#[derive(Default)]
pub struct Pki {
    pub admin: Pair,
    pub apiserver: Pair,
    pub ca: Pair,
    pub controller_manager: Pair,
    pub kubelet: Pair,
    pub proxy: Pair,
    pub scheduler: Pair,
    pub service_account: Pair,
}

#[derive(Clone, Default)]
pub struct Pair {
    cert: PathBuf,
    key: PathBuf,
}

impl Pair {
    pub fn new(dir: &Path, name: &str) -> Pair {
        let cert = dir.join(format!("{}.pem", name));
        let key = dir.join(format!("{}-key.pem", name));
        Pair { cert, key }
    }

    pub fn cert(&self) -> &Path {
        &self.cert
    }

    pub fn key(&self) -> &Path {
        &self.key
    }
}

#[derive(Clone)]
struct PkiConfig {
    ca: Pair,
    ca_config: PathBuf,
    dir: PathBuf,
    hostnames: String,
}

impl Pki {
    pub fn new(config: &Config, ip: &str, hostname: &str) -> Fallible<Pki> {
        info!("Generating certificates");

        // Create the target dir
        let pki_dir = &config.root.join(&config.pki.dir);
        create_dir_all(pki_dir)?;

        // Find out the first Service IP
        let service_addr = match config.kube.service_cidr {
            IpNetwork::V4(n) => n.nth(1).ok_or_else(|| {
                format_err!(
                    "Unable to retrieve first IP from service CIDR: {}",
                    config.kube.service_cidr
                )
            })?,
            _ => Ipv4Addr::LOCALHOST,
        };

        // Set the hostnames
        let hostnames = &[
            ip,
            &service_addr.to_string(),
            &Ipv4Addr::LOCALHOST.to_string(),
            hostname,
            "kubernetes",
            "kubernetes.default",
            "kubernetes.default.svc",
            "kubernetes.default.svc.cluster",
            "kubernetes.svc.cluster.local",
        ];

        let ca = Self::setup_ca(pki_dir)?;
        let pki_config = PkiConfig {
            dir: pki_dir.to_owned(),
            ca: ca.clone(),
            ca_config: Self::write_ca_config(pki_dir)?,
            hostnames: hostnames.join(","),
        };

        Ok(Pki {
            admin: Self::setup_admin(&pki_config)?,
            apiserver: Self::setup_apiserver(&pki_config)?,
            controller_manager: Self::setup_controller_manager(&pki_config)?,
            kubelet: Self::setup_kubelet(&pki_config, hostname)?,
            proxy: Self::setup_proxy(&pki_config)?,
            scheduler: Self::setup_scheduler(&pki_config)?,
            service_account: Self::setup_service_account(&pki_config)?,
            ca,
        })
    }

    fn setup_ca(dir: &Path) -> Fallible<Pair> {
        const NAME: &str = "ca";
        debug!("Creating CA certificates");
        const CN: &str = "Kubernetes";
        let csr = dir.join("ca-csr.json");
        Self::write_csr(CN, CN, &csr)?;

        let mut cfssl = Command::new("cfssl")
            .arg("gencert")
            .arg("-initca")
            .arg(csr)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?;

        let pipe = cfssl
            .stdout
            .take()
            .ok_or_else(|| format_err!("unable to get stdout"))?;
        let output = Command::new("cfssljson")
            .arg("-bare")
            .arg(dir.join(NAME))
            .stdin(pipe)
            .output()?;
        if !output.status.success() {
            debug!("cfssl/json stdout: {}", String::from_utf8(output.stdout)?);
            debug!("cfssl/json stderr: {}", String::from_utf8(output.stderr)?);
            bail!("CA certificate generation failed");
        }
        debug!("CA certificates created");
        Ok(Pair::new(dir, NAME))
    }

    fn setup_kubelet(pki_config: &PkiConfig, hostname: &str) -> Fallible<Pair> {
        let name = format!("system:node:{}", hostname);
        let csr_file = pki_config.dir.join("node-csr.json");
        Self::write_csr(&name, "system:nodes", &csr_file)?;
        Ok(Self::generate(pki_config, hostname, &csr_file)?)
    }

    fn setup_admin(pki_config: &PkiConfig) -> Fallible<Pair> {
        const NAME: &str = "admin";
        let csr_file = pki_config.dir.join("admin-csr.json");
        Self::write_csr(NAME, "system:masters", &csr_file)?;
        Ok(Self::generate(pki_config, NAME, &csr_file)?)
    }

    fn setup_controller_manager(pki_config: &PkiConfig) -> Fallible<Pair> {
        const NAME: &str = "kube-controller-manager";
        const CN: &str = "system:kube-controller-manager";
        let csr_file = pki_config.dir.join("kube-controller-manager-csr.json");
        Self::write_csr(CN, CN, &csr_file)?;
        Ok(Self::generate(pki_config, NAME, &csr_file)?)
    }

    fn setup_proxy(pki_config: &PkiConfig) -> Fallible<Pair> {
        const NAME: &str = "kube-proxy";
        let csr_file = pki_config.dir.join("admin-csr.json");
        Self::write_csr("system:kube-proxy", "system:node-proxier", &csr_file)?;
        Ok(Self::generate(pki_config, NAME, &csr_file)?)
    }

    fn setup_scheduler(pki_config: &PkiConfig) -> Fallible<Pair> {
        const NAME: &str = "kube-scheduler";
        let csr_file = pki_config.dir.join("kube-scheduler-csr.json");
        const CN: &str = "system:kube-scheduler";
        Self::write_csr(CN, CN, &csr_file)?;
        Ok(Self::generate(pki_config, NAME, &csr_file)?)
    }

    fn setup_apiserver(pki_config: &PkiConfig) -> Fallible<Pair> {
        const NAME: &str = "kubernetes";
        let csr_file = pki_config.dir.join("kubernetes-csr.json");
        Self::write_csr(NAME, NAME, &csr_file)?;
        Ok(Self::generate(pki_config, NAME, &csr_file)?)
    }

    fn setup_service_account(pki_config: &PkiConfig) -> Fallible<Pair> {
        const NAME: &str = "service-account";
        let csr_file = pki_config.dir.join("service-account-csr.json");
        Self::write_csr("service-accounts", "Kubernetes", &csr_file)?;
        Ok(Self::generate(pki_config, NAME, &csr_file)?)
    }

    fn generate(pki_config: &PkiConfig, name: &str, csr: &Path) -> Fallible<Pair> {
        debug!("Creating certificate for {}", name);

        let mut cfssl = Command::new("cfssl")
            .arg("gencert")
            .arg(format!("-ca={}", pki_config.ca.cert().display()))
            .arg(format!("-ca-key={}", pki_config.ca.key().display()))
            .arg(format!("-config={}", pki_config.ca_config.display()))
            .arg("-profile=kubernetes")
            .arg(format!("-hostname={}", pki_config.hostnames))
            .arg(csr)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?;

        let pipe = cfssl
            .stdout
            .take()
            .ok_or_else(|| format_err!("unable to get stdout"))?;
        let output = Command::new("cfssljson")
            .arg("-bare")
            .arg(pki_config.dir.join(name))
            .stdin(pipe)
            .output()?;
        if !output.status.success() {
            debug!("cfssl/json stdout: {}", String::from_utf8(output.stdout)?);
            debug!("cfssl/json stderr: {}", String::from_utf8(output.stderr)?);
            bail!("cfssl command failed");
        }
        debug!("Certificate created for {}", name);

        Ok(Pair::new(&pki_config.dir, name))
    }

    fn write_csr(cn: &str, o: &str, dest: &Path) -> Fallible<()> {
        let csr = json!({
          "CN": cn,
          "key": {
            "algo": "rsa",
            "size": 2048
          },
          "names": [
            {
              "C": "US",
              "L": "Portland",
              "O": o,
              "OU": "Kubernetes",
              "ST": "Oregon"
            }
          ]
        });
        fs::write(dest, to_string_pretty(&csr)?)?;
        Ok(())
    }

    fn write_ca_config(dir: &Path) -> Fallible<PathBuf> {
        let cfg = json!({
          "signing": {
            "default": {
              "expiry": "8760h"
            },
            "profiles": {
              "kubernetes": {
              "usages": [
                "signing",
                "key encipherment",
                "server auth",
                "client auth"
              ],
              "expiry": "8760h"
              }
            }
          }
        });
        let dest = dir.join("ca-config.json");
        fs::write(&dest, to_string_pretty(&cfg)?)?;
        Ok(dest)
    }
}
