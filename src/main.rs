extern crate globalenv;
extern crate yaml_rust;
use std::{env, fs};
use yaml_rust::{Yaml, YamlLoader};

fn main() {
    let yaml = fs::read_to_string(".envail/config.yml").unwrap();
    let doc = &YamlLoader::load_from_str(&yaml).unwrap()[0];

    let mut enter_file = String::from("#!/bin/fish\n");
    let mut out_file = String::from("#!/bin/fish\n");

    for (k, v) in doc["vars"].as_hash().unwrap() {
        let k = k.as_str().unwrap();
        match env::var(k) {
            Ok(old_v) => add_env_var(&mut out_file, k, old_v.as_str()),
            Err(_) => remove_env_var(&mut out_file, k),
        }
        add_env_var(&mut enter_file, k, v.as_str().unwrap());
    }

    add_commands(&doc["on_enter"], &mut enter_file);
    add_commands(&doc["on_exit"], &mut out_file);

    _ = fs::create_dir(".envail/build/");
    fs::write(".envail/build/enter", enter_file).expect("Unable to write file");
    fs::write(".envail/build/leave", out_file).expect("Unable to write file");
}

fn add_env_var(file: &mut String, k: &str, v: &str) {
    // file.push_str(&format!("set -g {} {}\n", k, v))
    file.push_str(&format!("export {}={}\n", k, v))
}

fn remove_env_var(file: &mut String, k: &str) {
    // file.push_str(&format!("set -e {}\n", k))
    file.push_str(&format!("unset {}\n", k))
}

fn add_commands(doc: &Yaml, file: &mut String) {
    for command in doc.as_vec().unwrap() {
        file.push_str(command.as_str().unwrap());
        file.push('\n');
    }
}
