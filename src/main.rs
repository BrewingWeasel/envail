extern crate dirs;
extern crate yaml_rust;
use std::env;

use clap::{Parser, Subcommand};
use envail::{build, cd::envail_cd};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to shell to build for. Defaults to $SHELL
    #[arg(long)]
    shell: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// The cd command that envail aliases cd to. Generally you should just use cd
    Cd {
        dir: Option<String>,
        active_dirs: Option<Vec<String>>,
    },
    Build {
        /// Path to config file to read from
        #[arg(short, long, default_value_t = String::from(".envail/config.yml"))]
        config: String,

        #[arg(long)]
        name: Option<String>,
    },
}

fn main() {
    let args = Args::parse();
    let shell = match args.shell {
        Some(v) => v,
        None => env::var("SHELL").unwrap(),
    };

    match args.command {
        Commands::Cd { dir, active_dirs } => {
            envail_cd(dir, active_dirs, shell);
        }
        Commands::Build { config, name } => {
            build(config, shell, name);
        }
    }
}
