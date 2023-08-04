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
}
