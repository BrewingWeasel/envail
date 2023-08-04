extern crate yaml_rust;
use std::env;

use clap::Parser;
use envail::build;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to shell to build for. Defaults to $SHELL
    #[arg(long)]
    shell: Option<String>,

    /// Path to config file to read from
    #[arg(short, long, default_value_t = String::from(".envail/config.yml"))]
    config: String,
}

fn main() {
    let args = Args::parse();

    let shell = match args.shell {
        Some(v) => v,
        None => env::var("SHELL").unwrap(),
    };

    build(args.config, shell)
}
