use std::{fs, io, thread};

use clap::Parser;

use crate::{
    commands::{Cli, Commands},
    domains::{add_domain, delete_domain, list_domains},
    rutas::{log_file, log_file_error},
    service::{restart, start, start_detached, status, stop},
    ubuntu_srv::{install_service, set_enable_on_boot, uninstall_service},
};

mod commands;
mod domains;
mod entry;
mod proc_info;
mod rutas;
mod service;
mod ubuntu_srv;

const BINARY: &'static [u8] = include_bytes!("./bin/executor");

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Start { detached } => {
            if detached {
                thread::spawn(|| {
                    if let Err(e) = start_detached() {
                        eprintln!("Error running detached service: {}", e);
                    }
                });
            } else if let Err(e) = start().await {
                eprintln!("Error starting service: {}", e);
            }
            print!("")
        }
        Commands::Install => {
            if let Err(f) = install_service() {
                println!("Failed: {:?}", f)
            }
        }
        Commands::Uninstall => {
            _ = uninstall_service();
        }
        Commands::EnableOnBoot { activate } => {
            _ = set_enable_on_boot(activate);
        }
        Commands::Stop => {
            _ = stop().await;
        }
        Commands::Status => {
            _ = status().await;
        }
        Commands::Restart => {
            _ = restart().await;
        }
        Commands::AddDomain {
            name,
            token,
            activated,
            txt,
        } => {
            add_domain(&name, &token, activated, txt);
        }
        Commands::DeleteDomain { name } => {
            delete_domain(&name);
        }
        Commands::ListDomain => {
            list_domains();
        }
        Commands::ViewLog => {
            let l = read_log_errors();
            match l {
                Ok(d) => {
                    for el in d {
                        println!("{}", el)
                    }
                }
                _ => {
                    println!("Failed retrive logs")
                }
            }
        }
    }
}

fn read_log_errors() -> io::Result<Vec<String>> {
    let log_path = log_file();
    let error_log_path = log_file_error();

    let log_ok = match fs::read_to_string(&log_path) {
        Ok(ctn) => ctn,
        _ => String::new(),
    };

    let log_err = match fs::read_to_string(&error_log_path) {
        Ok(ctn) => ctn,
        _ => String::new(),
    };

    Ok(vec![
        "Log OK : ".to_string(),
        log_ok,
        "Log Error: ".to_string(),
        log_err,
    ])
}
