extern crate globalenv;
extern crate yaml_rust;
use std::{env, fs};
use yaml_rust::{Yaml, YamlLoader};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to shell to build for. Defaults to $SHELL
    #[arg(long)]
    shell: Option<String>,

    /// Path to config file to read from
    #[arg(short, long, default_value_t = String::from(".envail/config.yml"))]
    config: String,
}

fn main() {
    let args = Args::parse();
    let yaml = fs::read_to_string(args.config).unwrap();
    let doc = &YamlLoader::load_from_str(&yaml).unwrap()[0];

    let shell = match args.shell {
        Some(v) => v,
        None => env::var("SHELL").unwrap(),
    };

    let shell_functions: Box<dyn Shell> = if shell.contains("fish") {
        Box::new(Fish {})
    } else if shell.contains("bash") {
        Box::new(Bash {})
    } else {
        panic!("unsupported shell");
    };

    let mut enter_file = format!("#!{shell}\n");
    let mut out_file = format!("#!{shell}\n");

    for (k, v) in doc["vars"].as_hash().unwrap() {
        let k = k.as_str().unwrap();
        match env::var(k) {
            Ok(old_v) => shell_functions.add_env_var(&mut out_file, k, old_v.as_str()),
            Err(_) => shell_functions.remove_env_var(&mut out_file, k),
        }
        shell_functions.add_env_var(&mut enter_file, k, v.as_str().unwrap());
    }

    for (k, v) in doc["aliases"].as_hash().unwrap() {
        let k = k.as_str().unwrap();
        shell_functions.remove_alias(&mut out_file, k);
        shell_functions.add_alias(&mut enter_file, k, v.as_str().unwrap());
    }

    add_commands(&doc["on_enter"], &mut enter_file);
    add_commands(&doc["on_exit"], &mut out_file);

    _ = fs::create_dir(".envail/build/");
    fs::write(".envail/build/enter", enter_file).expect("Unable to write file");
    fs::write(".envail/build/leave", out_file).expect("Unable to write file");
}

trait Shell {
    fn add_env_var(&self, file: &mut String, k: &str, v: &str);
    fn remove_env_var(&self, file: &mut String, k: &str);
    fn add_alias(&self, file: &mut String, k: &str, v: &str);
    fn remove_alias(&self, file: &mut String, k: &str);
}

struct Fish {}
struct Bash {}

impl Shell for Fish {
    fn add_env_var(&self, file: &mut String, k: &str, v: &str) {
        file.push_str(&format!("set -g {} {}\n", k, v))
    }
    fn remove_env_var(&self, file: &mut String, k: &str) {
        file.push_str(&format!("set -e {}\n", k))
    }
    fn add_alias(&self, file: &mut String, k: &str, v: &str) {
        file.push_str(&format!("alias {k} \"{v}\"\n"))
    }
    fn remove_alias(&self, file: &mut String, k: &str) {
        file.push_str(&format!("functions -e {k}\n"))
    }
}

impl Shell for Bash {
    fn add_env_var(&self, file: &mut String, k: &str, v: &str) {
        file.push_str(&format!("export {}={}\n", k, v))
    }
    fn remove_env_var(&self, file: &mut String, k: &str) {
        file.push_str(&format!("unset {}\n", k))
    }
    fn add_alias(&self, file: &mut String, k: &str, v: &str) {
        file.push_str(&format!("alias {k}=\"{v}\"\n"))
    }
    fn remove_alias(&self, file: &mut String, k: &str) {
        file.push_str(&format!("unalias {k}\n"))
    }
}

fn add_commands(doc: &Yaml, file: &mut String) {
    for command in doc.as_vec().unwrap() {
        file.push_str(command.as_str().unwrap());
        file.push('\n');
    }
}
