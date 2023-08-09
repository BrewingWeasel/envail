use std::path::Path;

use crate::Shell;
pub struct Fish {}

impl Shell for Fish {
    fn add_env_var(&self, file: &mut String, k: &str, v: &str) {
        file.push_str(&format!("set -g {} \"{}\"\n", k, v))
    }
    fn remove_env_var(&self, file: &mut String, k: &str) {
        file.push_str(&format!("set -e {}\n", k))
    }
    fn add_alias(&self, file: &mut String, k: &str, v: &str) {
        file.push_str(&format!("alias {k} \"{v}\"\n"))
    }
    fn remove_alias(&self, file: &mut String, k: &str) {
        file.push_str(&format!("functions -e {k}\n"))
    }
    fn get_name(&self) -> &str {
        "fish"
    }
    fn escape_alias(&self) -> &str {
        "command "
    }
    fn run_cd(&self, dir: &Path) {
        println!("builtin cd {};", dir.display());
    }
    fn add_to_active(&self, dir: &Path) {
        println!("set -a envail_active_dirs {};", dir.display());
    }
}
