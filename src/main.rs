// #![allow(unreachable_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_mut)]

use dnf5daemon::package::get_packages;
use dnf5daemon::{daemon::DnfDaemon, package::DnfPackage};
// use env_logger;
use log::{debug, info};
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

fn get_scope(scope: Scope) -> String {
    match scope {
        Scope::All => "all".to_owned(),
        Scope::Available => "available".to_owned(),
        Scope::Installed => "installed".to_owned(),
    }
}

fn setup_logger(args: &Args) {
    if args.debug {
        env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    }
}

fn print_packages(packages: &Vec<DnfPackage>, scope: Scope) {
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Setup logging

    let args = Args::parse();
    setup_logger(&args);
    debug!("{:?}", args);
    if args.patterns.len() > 0 {
        info!("Starting");
        let dnf_daemon = DnfDaemon::new().await;
        dnf_daemon.base.read_all_repos().await.ok();
        let scope = get_scope(args.scope);
        let packages = get_packages(&dnf_daemon, &args.patterns, &scope).await;
        print_packages(&packages, args.scope);
        info!("Ending");
    }
    Ok(())
}
