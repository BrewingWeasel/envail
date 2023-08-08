use std::{env, path::PathBuf};

pub fn envail_cd(dir: Option<String>, active_dirs: Option<Vec<String>>) {
    let dir = match dir {
        None => env::var("HOME").unwrap(),
        Some(d) => d,
    };
    let dir = PathBuf::from(dir).canonicalize().unwrap();
    let cur_dir = env::current_dir().expect("unable to get current dir!");

    let child_of_active = is_child_of_active_dir(&dir, active_dirs);

    if !dir.starts_with(&cur_dir) && !child_of_active {
        if cur_dir.join(".envail").exists() {
            if !cur_dir.join(".envail/build/fish").exists() {
                println!("envail build;")
            }
            println!("source {}/.envail/build/fish/leave;", cur_dir.display());
            println!("_envail_delete_from_active {};", dir.display())
        }
    }

    println!("builtin cd {};", dir.display());

    if !child_of_active && dir.join(".envail").exists() {
        println!("set -a envail_active_dirs {};", dir.display());
        if !dir.join(".envail/build/fish").exists() {
            println!("envail build;")
        }
        println!("source .envail/build/fish/enter;");
    }
}

fn is_child_of_active_dir(dir: &PathBuf, active_dirs: Option<Vec<String>>) -> bool {
    match active_dirs {
        Some(dirs) => {
            for i in dirs {
                if dir.starts_with(i) {
                    return true;
                }
            }
        }
        None => {}
    }
    false
}
