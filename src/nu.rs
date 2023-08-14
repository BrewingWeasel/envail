use std::path::Path;

use crate::Shell;
pub struct Nu {}

impl Shell for Nu {
    fn add_env_var(&self, file: &mut String, k: &str, v: &str) {
        file.push_str(&format!("$env.{} = \"{}\"\n", k, v))
    }
    fn remove_env_var(&self, file: &mut String, k: &str) {
        file.push_str(&format!("hide-env {}\n", k))
    }
    fn add_alias(&self, file: &mut String, k: &str, v: &str) {
        file.push_str(&format!("alias {k} = {v}\n"))
    }
    fn remove_alias(&self, file: &mut String, k: &str) {
        // No native way to remove alias so we have to hack it
        file.push_str(&format!("alias {k} = ^{k}\n"))
    }
    fn get_name(&self) -> &str {
        "nu"
    }
    fn escape_alias(&self) -> &str {
        "^"
    }
    fn run_cd(&self, dir: &Path) {
        println!("cd {};", dir.display());
    }
    fn add_to_active(&self, dir: &Path) {
        println!(
            "$env.envail_active_dirs = ($env.envail_active_dirs | append \"{}\");",
            dir.display()
        );
    }
}
