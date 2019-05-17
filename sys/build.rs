extern crate pkg_config;

use pkg_config::{Config, Error};
use std::env;
use std::io::prelude::*;
use std::io;
use std::process;

fn main() {
    if let Err(s) = find() {
        let _ = writeln!(io::stderr(), "{}", s);
        process::exit(1);
    }
}

fn find() -> Result<(), Error> {
    let package_name = "ostree-1";
    let shared_libs = ["ostree-1"];
    let version = if cfg!(feature = "v2019_2") {
        "2019.2"
    } else if cfg!(feature = "v2018_9") {
        "2018.9"
    } else if cfg!(feature = "v2018_7") {
        "2018.7"
    } else if cfg!(feature = "v2018_6") {
        "2018.6"
    } else if cfg!(feature = "v2018_5") {
        "2018.5"
    } else if cfg!(feature = "v2018_3") {
        "2018.3"
    } else if cfg!(feature = "v2018_2") {
        "2018.2"
    } else if cfg!(feature = "v2017_15") {
        "2017.15"
    } else if cfg!(feature = "v2017_13") {
        "2017.13"
    } else if cfg!(feature = "v2017_12") {
        "2017.12"
    } else if cfg!(feature = "v2017_11") {
        "2017.11"
    } else if cfg!(feature = "v2017_10") {
        "2017.10"
    } else if cfg!(feature = "v2017_9") {
        "2017.9"
    } else if cfg!(feature = "v2017_8") {
        "2017.8"
    } else if cfg!(feature = "v2017_7") {
        "2017.7"
    } else if cfg!(feature = "v2017_6") {
        "2017.6"
    } else if cfg!(feature = "v2017_4") {
        "2017.4"
    } else if cfg!(feature = "v2017_3") {
        "2017.3"
    } else if cfg!(feature = "v2017_2") {
        "2017.2"
    } else if cfg!(feature = "v2017_1") {
        "2017.1"
    } else if cfg!(feature = "v2016_14") {
        "2016.14"
    } else if cfg!(feature = "v2016_8") {
        "2016.8"
    } else if cfg!(feature = "v2016_7") {
        "2016.7"
    } else if cfg!(feature = "v2016_6") {
        "2016.6"
    } else if cfg!(feature = "v2016_5") {
        "2016.5"
    } else if cfg!(feature = "v2016_4") {
        "2016.4"
    } else if cfg!(feature = "v2015_7") {
        "2015.7"
    } else {
        "0.0"
    };

    if let Ok(lib_dir) = env::var("GTK_LIB_DIR") {
        for lib_ in shared_libs.iter() {
            println!("cargo:rustc-link-lib=dylib={}", lib_);
        }
        println!("cargo:rustc-link-search=native={}", lib_dir);
        return Ok(())
    }

    let target = env::var("TARGET").expect("TARGET environment variable doesn't exist");
    let hardcode_shared_libs = target.contains("windows");

    let mut config = Config::new();
    config.atleast_version(version);
    config.print_system_libs(false);
    if hardcode_shared_libs {
        config.cargo_metadata(false);
    }
    match config.probe(package_name) {
        Ok(library) => {
            if hardcode_shared_libs {
                for lib_ in shared_libs.iter() {
                    println!("cargo:rustc-link-lib=dylib={}", lib_);
                }
                for path in library.link_paths.iter() {
                    println!("cargo:rustc-link-search=native={}",
                             path.to_str().expect("library path doesn't exist"));
                }
            }
            Ok(())
        }
        Err(Error::EnvNoPkgConfig(_)) | Err(Error::Command { .. }) => {
            for lib_ in shared_libs.iter() {
                println!("cargo:rustc-link-lib=dylib={}", lib_);
            }
            Ok(())
        }
        Err(err) => Err(err),
    }
}

