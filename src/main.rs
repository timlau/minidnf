// #![allow(unused)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_mut)]

mod args;
mod signals;
mod utils;

use args::{Args, Commands};
use clap::Parser;
use dnf5daemon::{DnfDaemon, package::get_packages};
use futures_util::{self};
use log::debug;
use signals::{signal_download_add_new, signal_download_progress};
use std::collections::HashMap;
use std::error::Error;
use utils::print_packages;
use utils::{setup_logger, show_transaction};

async fn do_install(dnf_daemon: &DnfDaemon, pkgs: &Vec<String>) -> Result<(), zbus::Error> {
    let options: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>> = HashMap::new();
    println!(" --> Installing packages {:?}", pkgs);
    dnf_daemon.rpm.install(pkgs, options.clone()).await.ok();
    if let Ok(rc) = dnf_daemon.goal.resolve(options.clone()).await {
        //println!("resolve : {:?}", rc);
        let txmbrs = rc.0;
        let result = rc.1;
        show_transaction(&txmbrs);
        if result == 0 {
            // everything is Ok, do transaction
            let rc = dnf_daemon.goal.do_transaction(options.clone()).await.ok();
            println!("do_transaction : {:?}", rc);
        } else {
            println!("resolve failed with code : {}", result)
        }
    };
    Ok::<(), zbus::Error>(())
}

async fn do_remove(dnf_daemon: &DnfDaemon, pkgs: &Vec<String>) -> Result<(), zbus::Error> {
    let options: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>> = HashMap::new();
    println!(" --> Removing packages : {:?}", &pkgs);
    dnf_daemon.rpm.remove(pkgs, options.clone()).await.ok();
    if let Ok(rc) = dnf_daemon.goal.resolve(options.clone()).await {
        let txmbrs = rc.0;
        let result = rc.1;
        show_transaction(&txmbrs);
        if result == 0 {
            // everything is Ok, do transaction
            let rc = dnf_daemon.goal.do_transaction(options.clone()).await.ok();
            println!("do_transaction : {:?}", rc);
        } else {
            println!("resolve failed with code : {}", result)
        }
    };
    Ok::<(), zbus::Error>(())
}

async fn main_action(args: &Args, dnf_daemon: &DnfDaemon) -> Result<(), zbus::Error> {
    dnf_daemon.base.read_all_repos().await.ok();
    match &args.command {
        Some(Commands::Install { pkgs }) => {
            do_install(dnf_daemon, pkgs).await?;
        }
        Some(Commands::Remove { pkgs }) => {
            do_remove(dnf_daemon, pkgs).await?;
        }
        Some(Commands::List { pkgs, scope }) => {
            let scp = scope.to_string();
            if let Ok(packages) = get_packages(&dnf_daemon, pkgs, &scp).await {
                print_packages(&packages, *scope);
            };
        }
        _ => {
            println!("No command provided, use --help for more information.");
        }
    }
    println!("Main action completed. Press Ctrl+C to exit.");
    Ok::<(), zbus::Error>(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Setup logging

    let args = Args::parse();
    setup_logger(&args);
    debug!("{:?}", args);
    if let Ok(dnf_daemon) = DnfDaemon::new().await {
        futures_util::try_join!(
            // --  listen for signals
            async { signal_download_add_new(&dnf_daemon).await },
            async { signal_download_progress(&dnf_daemon).await },
            // -- Main action
            async { main_action(&args, &dnf_daemon).await }
        )?;
    } else {
        println!("Can't make connection to dnf5daemon-server");
    }
    Ok(())
}
