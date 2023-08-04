use std::{env, fs};

mod bash;
mod fish;

use yaml_rust::{Yaml, YamlLoader};

pub fn build(file: String, shell: String) {
    let yaml = fs::read_to_string(file).unwrap();
    let doc = &YamlLoader::load_from_str(&yaml).unwrap()[0];

    // use contains to allow different locations ie some distros put bash in different places
    let shell_functions: Box<dyn Shell> = if shell.contains("fish") {
        Box::new(fish::Fish {})
    } else if shell.contains("bash") || shell.contains("zsh") {
        // Everything done in the bash script should also work with zsh
        Box::new(bash::Bash {})
    } else {
        panic!("unsupported shell");
    };

    let mut enter_file = format!("#!{shell}\n");
    let mut out_file = format!("#!{shell}\n");

    if let Some(vars) = doc["vars"].as_hash() {
        for (k, v) in vars {
            let k = k.as_str().unwrap();
            match env::var(k) {
                Ok(old_v) => shell_functions.add_env_var(&mut out_file, k, old_v.as_str()),
                Err(_) => shell_functions.remove_env_var(&mut out_file, k),
            }
            shell_functions.add_env_var(&mut enter_file, k, v.as_str().unwrap());
        }
    }

    if let Some(aliases) = doc["aliases"].as_hash() {
        for (k, v) in aliases {
            let k = k.as_str().unwrap();
            shell_functions.remove_alias(&mut out_file, k);
            shell_functions.add_alias(&mut enter_file, k, v.as_str().unwrap());
        }
    }

    add_commands(&doc["on_enter"], &mut enter_file);
    add_commands(&doc["on_exit"], &mut out_file);

    // Some shells like zsh should be classified as bash, because they use the same script as bash
    let shell_name = shell_functions.get_name();
    _ = fs::create_dir_all(String::from(".envail/build/") + shell_name);
    fs::write(format!(".envail/build/{shell_name}/enter"), enter_file)
        .expect("Unable to write file");
    fs::write(format!(".envail/build/{shell_name}/leave"), out_file).expect("Unable to write file");
}

fn add_commands(doc: &Yaml, file: &mut String) {
    if let Some(commands) = doc.as_vec() {
        for command in commands {
            file.push_str(command.as_str().unwrap());
            file.push('\n');
        }
    }
}

pub trait Shell {
    fn add_env_var(&self, file: &mut String, k: &str, v: &str);
    fn remove_env_var(&self, file: &mut String, k: &str);
    fn add_alias(&self, file: &mut String, k: &str, v: &str);
    fn remove_alias(&self, file: &mut String, k: &str);
    fn get_name(&self) -> &str;
}
