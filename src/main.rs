use std::{fs, io, thread};

use clap::Parser;
use tokio::runtime::Builder;

use crate::{commands::{Cli, Commands}, domains::{add_domain, delete_domain, list_domains}, rutas::{log_file, log_file_error}, service::{restart, start, status, stop}, ubuntu_srv::{install_service, set_enable_on_boot, uninstall_service}};

mod domains;
mod entry;
mod ubuntu_srv;
mod rutas;
mod commands;
mod service;
mod proc_info;
 

const BINARY : &'static [u8] = include_bytes!("./bin/ptech_dns_executor");

#[tokio::main]
async fn main() {

    let cli = Cli::parse();
    match cli.command {
        Commands::Start { detached } => {
            if detached {


                thread::spawn(|| {

                    let rt = Builder::new_current_thread()
                    .thread_name("runner_domain_hdlr")
                    .build().unwrap();
                    if let Err(e) = rt.block_on(start()) {
                        eprintln!("Error running detached service: {}", e);
                    }
                });
            } else if let Err(e) = start().await {
                eprintln!("Error starting service: {}", e);
            }
            print!("")
        }
        Commands::Install => {
            _ = install_service();
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
        _ => String::new()
    };

    let log_err = match fs::read_to_string(&error_log_path) {
        Ok(ctn) => ctn,
        _ => String::new()
    };

    Ok(vec![
        "Log OK : ".to_string(),
        log_ok,
        "Log Error: ".to_string(),
        log_err
        ])
}