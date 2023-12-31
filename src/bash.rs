use std::path::Path;

use crate::Shell;
pub struct Bash {}

impl Shell for Bash {
    fn add_env_var(&self, file: &mut String, k: &str, v: &str) {
        file.push_str(&format!("export {}=\"{}\"\n", k, v))
    }
    fn remove_env_var(&self, file: &mut String, k: &str) {
        file.push_str(&format!("unset {}\n", k))
    }
    fn add_alias(&self, file: &mut String, k: &str, v: &str) {
        file.push_str(&format!("alias {k}=\"{v}\"\n"))
    }
    fn remove_alias(&self, file: &mut String, k: &str) {
        file.push_str(&format!("unalias {k}\n"))
    }
    fn get_name(&self) -> &str {
        "bash"
    }
    fn escape_alias(&self) -> &str {
        "\\"
    }
    fn run_cd(&self, dir: &Path) {
        println!("\\cd {};", dir.display());
    }
    fn add_to_active(&self, dir: &Path) {
        println!("envail_active_dirs+=({});", dir.display());
    }
}
