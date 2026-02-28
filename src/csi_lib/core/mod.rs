mod installer_page;
mod installer_action;
pub use installer_page::*;
pub use installer_action::*;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Installer {
    pub general: InstallerGeneral,
    #[serde(alias = "page")]
    pub pages: Vec<InstallerPage>,
}
impl Installer {
    pub fn is_valid(&self) -> bool {
        todo!()
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InstallerGeneral {
    program_name: String,
    program_desc: String,
    source_dir: std::path::PathBuf,
}
impl Default for InstallerGeneral {
    fn default() -> Self {
        Self {
            program_name: String::from("Program Name"),
            program_desc: String::from("Program Description"),
            source_dir: std::env::current_dir().expect("Current dir should return"),
        }
    }
}
impl InstallerGeneral {
    pub fn program_name(&self) -> &str { &self.program_name }
    pub fn set_program_name(&mut self, val: String) { self.program_name = val }
    pub fn program_desc(&self) -> &str { &self.program_desc }
    pub fn set_program_desc(&mut self, val: String) { self.program_desc = val }
}
