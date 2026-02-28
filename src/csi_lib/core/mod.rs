mod installer_page;
mod installer_action;
pub use installer_page::*;
pub use installer_action::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Installer {
    pub general: InstallerGeneral,
    #[serde(alias = "page")]
    pub pages: Vec<InstallerPage>,
    // Default values for variables
    vars: Option<HashMap<String, String>>,
    #[serde(alias = "action")]
    actions: Option<Vec<InstallerAction>>,
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
    source_dir: String,
}
impl Default for InstallerGeneral {
    fn default() -> Self {
        let source_dir = std::env::current_dir().expect("Current dir should be returned");
        let source_dir = source_dir.to_str().expect("PathBuf should be converted to &str");

        Self {
            program_name: String::from("Program Name"),
            program_desc: String::from("Program Description"),
            source_dir: String::from(source_dir),
        }
    }
}
impl InstallerGeneral {
    pub fn program_name(&self) -> &str { &self.program_name }
    pub fn set_program_name(&mut self, val: String) { self.program_name = val }
    pub fn program_desc(&self) -> &str { &self.program_desc }
    pub fn set_program_desc(&mut self, val: String) { self.program_desc = val }
    pub fn source_dir(&self) -> &str { &self.source_dir }
    pub fn set_source_dir(&mut self, val: String) { self.source_dir = val }
}
