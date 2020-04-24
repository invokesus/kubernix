var searchIndex={};
searchIndex["kubernix"] = {"doc":"kubernix","i":[[3,"Config","kubernix","The global configuration",null,null],[3,"Logger","","The main logging faccade",null,null],[3,"Kubernix","","The main entry point for the application",null,null],[11,"log_level","","The logging level of the application",0,[[["self"]],["levelfilter"]]],[11,"cidr","","The CIDR used for the cluster",0,[[["self"]],["ipv4network"]]],[11,"nodes","","The number of nodes to be registered",0,[[["self"]],["u8"]]],[11,"subcommand","","All available subcommands",0,[[["self"]],["option"]]],[11,"root","","Path where all the runtime data is stored",0,[[["self"]],["pathbuf"]]],[11,"overlay","","The Nix package overlay to be used",0,[[["self"]],["option"]]],[11,"packages","","Additional dependencies to be added to the environment",0,[[["self"]],["vec"]]],[11,"shell","","The shell executable to be used, defaults to $SHELL,…",0,[[["self"]],["option"]]],[11,"container_runtime","","The container runtime to be used for the nodes, irrelevant…",0,[[["self"]],["string"]]],[11,"no_shell","","Do not spawn an interactive shell after bootstrap",0,[[["self"]],["bool"]]],[11,"canonicalize_root","","Make the configs root path absolute",0,[[["self"]],["result"]]],[11,"to_file","","Write the current configuration to the internal set root…",0,[[["self"]],["result"]]],[11,"try_load_file","","Read the configuration from the internal set root path If…",0,[[["self"]],["result"]]],[11,"shell_ok","","Return the set shell as result type",0,[[["self"]],[["string"],["result",["string"]]]]],[11,"multi_node","","Returns true if multi node support is enabled",0,[[["self"]],["bool"]]],[11,"new","","Create a new logger",1,[[["levelfilter"]],["box"]]],[11,"error","","Log an error message",1,[[["str"]]]],[11,"start","","Start kubernix by consuming the provided configuration",2,[[["config"]],["result"]]],[11,"new_shell","","Spawn a new shell into the provided configuration…",2,[[["config"]],["result"]]],[11,"from","","",0,[[["t"]],["t"]]],[11,"into","","",0,[[],["u"]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"try_into","","",0,[[],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"type_id","","",0,[[["self"]],["typeid"]]],[11,"vzip","","",0,[[],["v"]]],[11,"from","","",1,[[["t"]],["t"]]],[11,"into","","",1,[[],["u"]]],[11,"try_from","","",1,[[["u"]],["result"]]],[11,"try_into","","",1,[[],["result"]]],[11,"borrow","","",1,[[["self"]],["t"]]],[11,"borrow_mut","","",1,[[["self"]],["t"]]],[11,"type_id","","",1,[[["self"]],["typeid"]]],[11,"vzip","","",1,[[],["v"]]],[11,"from","","",2,[[["t"]],["t"]]],[11,"into","","",2,[[],["u"]]],[11,"try_from","","",2,[[["u"]],["result"]]],[11,"try_into","","",2,[[],["result"]]],[11,"borrow","","",2,[[["self"]],["t"]]],[11,"borrow_mut","","",2,[[["self"]],["t"]]],[11,"type_id","","",2,[[["self"]],["typeid"]]],[11,"vzip","","",2,[[],["v"]]],[11,"drop","","",2,[[["self"]]]],[11,"default","","",0,[[],["self"]]],[11,"serialize","","",0,[[["self"],["__s"]],["result"]]],[11,"deserialize","","",0,[[["__d"]],["result"]]],[11,"enabled","","",1,[[["metadata"],["self"]],["bool"]]],[11,"log","","",1,[[["self"],["record"]]]],[11,"flush","","",1,[[["self"]]]],[11,"into_app","","",0,[[],["app"]]],[11,"augment_clap","","",0,[[["app"]],["app"]]],[11,"from_arg_matches","","",0,[[["argmatches"]],["self"]]]],"p":[[3,"Config"],[3,"Logger"],[3,"Kubernix"]]};
addSearchOptions(searchIndex);initSearch(searchIndex);