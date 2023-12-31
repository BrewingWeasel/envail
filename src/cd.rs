use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
};

use yaml_rust::YamlLoader;

use crate::{bash, fish, Shell};

#[derive(Eq, PartialEq, Hash)]
enum ActivatingFile {
    SingleFile(String),
    MultipleFiles(Vec<String>),
}

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
                if cur_path
                    .join(format!(".envail/build/{shell_name}/enter"))
                    .exists()
                {
                    println!(
                        "source {}/.envail/build/{shell_name}/enter;",
                        cur_path.display()
                    );
                }
            }
            if !cur_path.join(".envail/config.yml").exists() {
                for (name, (needed_file, specific_dirs)) in &global_vals {
                    if specific_dirs.contains(&cur_path.display().to_string()) {
                        run_enter(&cur_path, &shell_functions, shell_name, name)
                    }

                    match needed_file {
                        ActivatingFile::SingleFile(file) => {
                            if cur_path.join(file).exists() {
                                run_enter(&cur_path, &shell_functions, shell_name, name);
                            }
                        }
                        ActivatingFile::MultipleFiles(files) => {
                            for file in files {
                                if cur_path.join(file).exists() {
                                    run_enter(&cur_path, &shell_functions, shell_name, name);
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn run_enter(cur_path: &Path, shell_functions: &Box<dyn Shell>, shell_name: &str, name: &str) {
    shell_functions.add_to_active(&cur_path);
    if !cur_path
        .join(format!(".envail/build/{shell_name}/{name}"))
        .exists()
    {
        println!("envail build --config ~/.config/envail/{name}.yml --name {name};")
    }
    println!(
        "source {}/.envail/build/{shell_name}/{}enter;",
        cur_path.display(),
        name.to_owned() + "/"
    );
}

fn get_global_values() -> HashMap<String, (ActivatingFile, Vec<String>)> {
    let mut globals = HashMap::new();

    let config_dir: PathBuf = dirs::config_dir().unwrap();
    if let Ok(yaml) = fs::read_to_string(config_dir.join("envail/global.yml")) {
        let doc = &YamlLoader::load_from_str(&yaml).unwrap()[0];
        for (file_name, v) in doc.as_hash().unwrap() {
            let on_file = v["activate_on_file"].to_owned();

            let activater = if let Some(s) = on_file.as_str() {
                ActivatingFile::SingleFile(s.to_owned())
            } else {
                ActivatingFile::MultipleFiles(
                    on_file
                        .into_vec()
                        .unwrap()
                        .into_iter()
                        .map(|x| x.into_string().unwrap())
                        .collect(),
                )
            };

            let specific_dirs = if let Some(dir_names) = v["specific_dirs"].to_owned().into_vec() {
                dir_names
                    .into_iter()
                    .map(|x| x.into_string().unwrap())
                    .collect()
            } else {
                Vec::new()
            };

            globals.insert(
                file_name.as_str().unwrap().to_owned(),
                (activater, specific_dirs),
            );
        }
    }
    globals
}

fn exit_dir(cur_dir: &Path, shell_name: &str) {
    if !cur_dir.join(format!(".envail/build/{shell_name}")).exists() {
        println!("envail build;")
    }
    for i in fs::read_dir(cur_dir.join(format!(".envail/build/{shell_name}"))).unwrap() {
        let path = i.unwrap().path();
        if path.is_dir() {
            println!("source {};", path.join("leave").display());
        } else if path.ends_with("leave") {
            println!("source {};", path.display());
        }
    }
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
