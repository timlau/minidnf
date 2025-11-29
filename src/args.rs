use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(arg_required_else_help = true)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// packages to search for

    /// Enable debug logging
    #[arg(long, short)]
    pub debug: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    /// Install packages
    Install {
        /// packages to install
        pkgs: Vec<String>,
    },
    /// Remove packages
    Remove {
        /// packages to remove
        pkgs: Vec<String>,
    },
    List {
        /// List packages
        pkgs: Vec<String>,
        /// Package scope
        #[arg(long, value_enum, default_value = "all")]
        scope: Scope,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lowercase")]
pub enum Scope {
    All,
    Installed,
    Available,
}

impl Scope {
    pub fn to_string(&self) -> String {
        match self {
            Scope::All => "all".to_string(),
            Scope::Installed => "installed".to_string(),
            Scope::Available => "available".to_string(),
        }
    }
}
