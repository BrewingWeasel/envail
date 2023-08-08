use std::{env, path::PathBuf};

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

    let child_of_active = is_child_of_active_dir(&dir, active_dirs);

    if !dir.starts_with(&cur_dir) && !child_of_active {
        if cur_dir.join(".envail").exists() {
            if !cur_dir.join(format!(".envail/build/{shell_name}")).exists() {
                println!("envail build;")
            }
            println!(
                "source {}/.envail/build/{shell_name}/leave;",
                cur_dir.display()
            );
            println!("_envail_delete_from_active {};", cur_dir.display())
        }
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
