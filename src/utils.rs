#![allow(dead_code)]

use crate::args::{Args, Scope};
use dnf5daemon::package::DnfPackage;
use env_logger::Env;
use std::collections::HashMap;
use zbus::zvariant::OwnedValue;

use termion::color;

pub fn print_packages(packages: &[DnfPackage], scope: Scope) {
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

pub fn show_transaction(
    txmbrs: &Vec<(
        String,
        String,
        String,
        HashMap<String, OwnedValue>,
        HashMap<String, OwnedValue>,
    )>,
) {
    // (object_type, action, reason, {transaction_item_attributes}, {object})
    for (_, action, reason, _, tx_pkg) in txmbrs {
        let sub_reason = String::try_from(tx_pkg.get("reason").unwrap().to_owned()).unwrap();
        let full_nevra = String::try_from(tx_pkg.get("full_nevra").unwrap().to_owned()).unwrap();
        if sub_reason == "None" || sub_reason == *reason {
            println!(" {} {} for {} ", action, full_nevra, reason);
        } else {
            println!(
                " {} {} for {} ({}) ",
                action, full_nevra, reason, sub_reason
            );
        }
    }
}

pub fn setup_logger(args: &Args) {
    if args.debug {
        env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    }
}
