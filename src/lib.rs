use std::{env, fs};

pub mod bash;
pub mod fish;

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

    add_aliases(doc, &shell_functions, &mut enter_file, &mut out_file);

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

fn add_aliases(
    doc: &Yaml,
    shell_functions: &Box<dyn Shell>,
    enter_file: &mut String,
    out_file: &mut String,
) {
    if let Some(aliases) = doc["aliases"].as_hash() {
        for (k, v) in aliases {
            let k = k.as_str().unwrap();
            shell_functions.add_alias(enter_file, k, v.as_str().unwrap());
            shell_functions.remove_alias(out_file, k);
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::{bash::Bash, fish::Fish};
    use yaml_rust::YamlLoader;
    const ALIASES_NORMAL: &str = "aliases:
            alias1: command1
            alias2: command2
            alias3: command2
        ";
    const ALIASES_EMPTY: &str = "aliases:
    ";

    #[test]
    fn aliases_fish_normal() {
        let mut enter_string = String::new();
        let mut exit_string = String::new();
        let shell: Box<dyn Shell> = Box::new(Fish {});

        add_aliases(
            &YamlLoader::load_from_str(ALIASES_NORMAL).unwrap()[0],
            &shell,
            &mut enter_string,
            &mut exit_string,
        );
        assert_eq!(
            enter_string,
            "alias alias1 \"command1\"
alias alias2 \"command2\"
alias alias3 \"command2\"
"
        );
        assert_eq!(
            exit_string,
            "functions -e alias1
functions -e alias2
functions -e alias3
"
        );
    }
    #[test]
    fn aliases_fish_empty() {
        let mut enter_string = String::new();
        let mut exit_string = String::new();
        let shell: Box<dyn Shell> = Box::new(Fish {});

        add_aliases(
            &YamlLoader::load_from_str(ALIASES_EMPTY).unwrap()[0],
            &shell,
            &mut enter_string,
            &mut exit_string,
        );
        assert_eq!(enter_string, "")
    }

    #[test]
    fn aliases_bash_normal() {
        let mut enter_string = String::new();
        let mut exit_string = String::new();
        let shell: Box<dyn Shell> = Box::new(Bash {});

        add_aliases(
            &YamlLoader::load_from_str(ALIASES_NORMAL).unwrap()[0],
            &shell,
            &mut enter_string,
            &mut exit_string,
        );
        assert_eq!(
            enter_string,
            "alias alias1=\"command1\"
alias alias2=\"command2\"
alias alias3=\"command2\"
"
        );
        assert_eq!(
            exit_string,
            "unalias alias1
unalias alias2
unalias alias3
"
        );
    }

    #[test]
    fn aliases_bash_empty() {
        let mut enter_string = String::new();
        let mut exit_string = String::new();
        let shell: Box<dyn Shell> = Box::new(Bash {});

        add_aliases(
            &YamlLoader::load_from_str(ALIASES_EMPTY).unwrap()[0],
            &shell,
            &mut enter_string,
            &mut exit_string,
        );
        assert_eq!(enter_string, "")
    }
}
