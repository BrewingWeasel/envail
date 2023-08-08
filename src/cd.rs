use std::{
    env,
    path::{Path, PathBuf},
};

use crate::{bash, fish, Shell};

pub fn envail_cd(dir: Option<String>, active_dirs: Option<Vec<String>>, shell: String) {
    let shell_functions: Box<dyn Shell> = if shell.contains("fish") {
        Box::new(fish::Fish {})
    } else if shell.contains("bash") || shell.contains("zsh") {
        // Everything done in the bash script should also work with zsh
        Box::new(bash::Bash {})
    } else {
        panic!("unsupported shell");
    };

    let shell_name = shell_functions.get_name();

    let dir = match dir {
        None => env::var("HOME").unwrap(),
        Some(d) => d,
    };
    let dir = PathBuf::from(dir).canonicalize().unwrap();
    let cur_dir = env::current_dir().expect("unable to get current dir!");

    let child_of_active = is_child_of_active_dir(&dir, &active_dirs);

    if !dir.starts_with(&cur_dir) && !child_of_active && cur_dir.join(".envail").exists() {
        exit_dir(&cur_dir, shell_name)
    }

    // If a user is in a path further away from their active directory and they leave it, we still
    // want to run the scripts
    for old_active_dir in is_leaving_active_dir(&dir, active_dirs) {
        exit_dir(&PathBuf::from(old_active_dir), shell_name)
    }

    shell_functions.run_cd(&dir);

    if !child_of_active && dir.join(".envail").exists() {
        shell_functions.add_to_active(&dir);
        if !dir.join(format!(".envail/build/{shell_name}")).exists() {
            println!("envail build;")
        }
        println!("source .envail/build/{shell_name}/enter;");
    }
}

fn exit_dir(cur_dir: &Path, shell_name: &str) {
    if !cur_dir.join(format!(".envail/build/{shell_name}")).exists() {
        println!("envail build;")
    }
    println!(
        "source {}/.envail/build/{shell_name}/leave;",
        cur_dir.display()
    );
    println!("_envail_delete_from_active {};", cur_dir.display())
}

fn is_child_of_active_dir(dir: &PathBuf, active_dirs: &Option<Vec<String>>) -> bool {
    if let Some(dirs) = active_dirs {
        for i in dirs {
            if dir.starts_with(i) {
                return true;
            }
        }
    }
    false
}

fn is_leaving_active_dir(dir: &PathBuf, active_dirs: Option<Vec<String>>) -> Vec<String> {
    let mut dirs_left = vec![];
    if let Some(dirs) = active_dirs {
        for i in dirs {
            if !dir.starts_with(&i) {
                dirs_left.push(i)
            }
        }
    }
    dirs_left
}
