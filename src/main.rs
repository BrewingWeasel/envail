extern crate yaml_rust;
use std::env;

use clap::{Parser, Subcommand};
use envail::{build, cd::envail_cd};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
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
        /// Path to shell to build for. Defaults to $SHELL
        #[arg(long)]
        shell: Option<String>,

        /// Path to config file to read from
        #[arg(short, long, default_value_t = String::from(".envail/config.yml"))]
        config: String,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Cd { dir, active_dirs } => {
            envail_cd(dir, active_dirs);
        }
        Commands::Build { shell, config } => {
            let shell = match shell {
                Some(v) => v,
                None => env::var("SHELL").unwrap(),
            };

            build(config, shell);
        }
    }
}
