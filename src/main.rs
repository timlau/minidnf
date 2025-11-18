#![allow(unused)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_mut)]

use dnf5daemon::{daemon::DnfDaemon, package::DnfPackage};
// use env_logger;
use futures_util::{self, StreamExt};
use log::debug;
use std::collections::HashMap;
use std::error::Error;

use clap::{Parser, ValueEnum};
use env_logger::Env;
use termion::color;

/// Simple program to test the dnf5 dbus app
#[derive(Parser, Debug)]
#[command(arg_required_else_help = true)]
#[command(version, about, long_about = None)]
struct Args {
    /// packages to search for
    // #[arg(short, long)]
    patterns: Vec<String>,

    /// Package scope
    #[arg(long, value_enum, default_value = "all")]
    scope: Scope,

    /// Enable debug logging
    #[arg(long, short)]
    debug: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lowercase")]
enum Scope {
    All,
    Installed,
    Available,
}

fn setup_logger(args: &Args) {
    if args.debug {
        env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    }
}

fn print_packages(packages: &[DnfPackage], scope: Scope) {
    if scope == Scope::All || scope == Scope::Installed {
        println!("\nInstalled Packages:{}", color::Fg(color::LightGreen));
        for pkg in packages.iter().filter(|pkg| pkg.is_installed) {
            let na = format!("{}.{}", pkg.name, pkg.arch);
            println!("{0:<50} {1:<20} {2:<15}", na, pkg.evr, pkg.repo_id);
        }
    };
    if scope == Scope::All || scope == Scope::Available {
        println!("\n{}Available Packages:", color::Fg(color::Reset));
        for pkg in packages.iter().filter(|pkg| !pkg.is_installed) {
            let na = format!("{}.{}", pkg.name, pkg.arch);
            println!("{0:<50} {1:<20} {2:<15}", na, pkg.evr, pkg.repo_id);
        }
    }
}

async fn download_progress_signal(dnf_daemon: &DnfDaemon) -> Result<(), zbus::Error> {
    let mut download_progress = dnf_daemon.base.receive_download_progress().await?;
    while let Some(signal) = download_progress.next().await {
        let args = signal.args()?;
        print!("\rSignal: download_progress : {:?}", args);
    }
    Ok::<(), zbus::Error>(())
}

async fn download_add_new_signal(dnf_daemon: &DnfDaemon) -> Result<(), zbus::Error> {
    let mut download_add_new = dnf_daemon.base.receive_download_add_new().await?;
    while let Some(signal) = download_add_new.next().await {
        let args = signal.args()?;
        println!("Signal: download_add_new : {:?}", args);
    }
    Ok::<(), zbus::Error>(())
}

async fn do_install(dnf_daemon: &DnfDaemon, pkgs: &Vec<String>) {
    let options: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>> = HashMap::new();
    println!(" --> Installing packages {:?}", pkgs);
    dnf_daemon.rpm.install(pkgs, options.clone()).await.ok();
    if let Ok(rc) = dnf_daemon.goal.resolve(options.clone()).await {
        println!("resolve : {:?}", rc);
        let (_txmbrs, result) = rc;
        if result == 0 {
            // everything is Ok, do transaction
            let rc = dnf_daemon.goal.do_transaction(options.clone()).await.ok();
            println!("do_transaction : {:?}", rc);
        } else {
            println!("resolve failed with code : {}", result)
        }
    };
}

async fn do_remove(dnf_daemon: &DnfDaemon, pkgs: &Vec<String>) {
    let options: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>> = HashMap::new();
    println!(" --> Removing packages : {:?}", &pkgs);
    dnf_daemon.rpm.remove(pkgs, options.clone()).await.ok();
    if let Ok(rc) = dnf_daemon.goal.resolve(options.clone()).await {
        let (txmbrs, result) = rc;
        for (_, action, _, _, tx_pkgs) in txmbrs {
            println!("Action : {action}");

            for (key, value) in tx_pkgs {
                if key == "full_nevra" {
                    println!("{:?}", value);
                }
            }
        }
        if result == 0 {
            // everything is Ok, do transaction
            let rc = dnf_daemon.goal.do_transaction(options.clone()).await.ok();
            println!("do_transaction : {:?}", rc);
        } else {
            println!("resolve failed with code : {}", result)
        }
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Setup logging

    let args = Args::parse();
    setup_logger(&args);
    debug!("{:?}", args);
    if !args.patterns.is_empty() {
        if let Ok(dnf_daemon) = DnfDaemon::new().await {
            futures_util::try_join!(
                async { download_add_new_signal(&dnf_daemon).await },
                async { download_progress_signal(&dnf_daemon).await },
                async {
                    dnf_daemon.base.read_all_repos().await.ok();
                    // std::process::exit(0);
                    //let pkgs = ["0ad"];
                    let pkgs = args.patterns;

                    do_install(&dnf_daemon, &pkgs).await;
                    dnf_daemon.base.reset().await.ok();
                    do_remove(&dnf_daemon, &pkgs).await;
                    println!("\nMain job has completed, use Ctrl-C to Quit");
                    Ok::<(), zbus::Error>(())
                }
            )?;
        } else {
            println!("Can't make connection to dnf5daemon-server");
        };
    }
    Ok(())
}
