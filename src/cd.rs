use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
};

use yaml_rust::YamlLoader;

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

    let global_vals = get_global_values();

    let dir = match dir {
        None => env::var("HOME").unwrap(),
        Some(d) => d,
    };
    let dir = PathBuf::from(dir).canonicalize().unwrap();
    let cur_dir = env::current_dir().expect("unable to get current dir!");

    let child_of_active = is_child_of_active_dir(&dir, &active_dirs);

    // If a user is in a path further away from their active directory and they leave it, we still
    // want to run the scripts
    for old_active_dir in is_leaving_active_dir(&dir, active_dirs) {
        exit_dir(&PathBuf::from(old_active_dir), shell_name)
    }

    shell_functions.run_cd(&dir);

    if !child_of_active {
        let mut cur_dir_parts = cur_dir.into_iter().fuse();
        let mut new_dir_parts = dir.into_iter();
        let mut cur_path = PathBuf::new();
        while let Some(path) = new_dir_parts.next() {
            cur_path.push(path);
            if let Some(old_path) = cur_dir_parts.next() {
                if old_path == path {
                    continue;
                }
            }
            if cur_path.join(".envail").exists() {
                shell_functions.add_to_active(&cur_path);
                if !cur_path
                    .join(format!(".envail/build/{shell_name}"))
                    .exists()
                {
                    println!("envail build;")
                }
                println!(
                    "source {}/.envail/build/{shell_name}/enter;",
                    cur_path.display()
                );
            }
            if !cur_path.join(".envail/config.yml").exists() {
                for (name, needed_file) in &global_vals {
                    if cur_path.join(needed_file).exists() {
                        shell_functions.add_to_active(&cur_path);
                        if !cur_path
                            .join(format!(".envail/build/{shell_name}/{name}"))
                            .exists()
                        {
                            println!(
                                "envail build --config ~/.config/envail/{name}.yml --name {name};"
                            )
                        }
                        println!(
                            "source {}/.envail/build/{shell_name}/{}enter;",
                            cur_path.display(),
                            name.to_owned() + "/"
                        );
                    }
                }
            }
        }
    }
}

fn get_global_values() -> HashMap<String, String> {
    let mut globals = HashMap::new();

    let config_dir: PathBuf = dirs::config_dir().unwrap();
    if let Ok(yaml) = fs::read_to_string(config_dir.join("envail/global.yml")) {
        let doc = &YamlLoader::load_from_str(&yaml).unwrap()[0];
        for (file_name, v) in doc.as_hash().unwrap() {
            let on_file = &v["activate_on_file"];
            globals.insert(
                file_name.as_str().unwrap().to_owned(),
                on_file.as_str().unwrap().to_owned(),
            );
        }
    }
    globals
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
