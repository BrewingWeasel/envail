use std::{env, path::PathBuf};

pub fn envail_cd(dir: Option<String>) {
    let dir = match dir {
        None => env::var("HOME").unwrap(),
        Some(d) => d,
    };
    let dir = PathBuf::from(dir).canonicalize().unwrap();
    let cur_dir = env::current_dir().expect("unable to get current dir!");
    if !dir.starts_with(&cur_dir) {
        if cur_dir.join(".envail").exists() {
            if !cur_dir.join(".envail/build/fish").exists() {
                println!("cargo run -- build")
            }
            println!("source {}/.envail/build/fish/leave", cur_dir.display());
        }
    }
    println!("cd {}", dir.display());
    if dir.join(".envail").exists() {
        if !dir.join(".envail/build/fish").exists() {
            println!("cargo run -- build")
        }
        println!(
            "source {}/.envail/build/fish/enter",
            dir.as_os_str().to_str().unwrap()
        );
    }
}
