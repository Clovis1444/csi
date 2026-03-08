mod installer_page;
mod installer_action;

pub use installer_page::*;
pub use installer_action::*;

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Installer {
    general: InstallerGeneral,
    #[serde(alias = "page")]
    pages: Vec<InstallerPage>,
    // Default values for variables
    #[serde(default)]
    vars: HashMap<String, String>,
    #[serde(alias = "action", default)]
    actions: Vec<InstallerAction>,
}
impl Installer {
    pub fn pages(&self) -> &Vec<InstallerPage> { &self.pages }

    pub fn pages_count(&self) -> i32 { self.pages.len() as i32 }

    pub fn is_valid(&self) -> bool {
        return self.validate(false).is_ok();
    }
    pub fn validate(&self, log: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut used_vars: HashSet<String> = HashSet::new();
        // TODO(clovis): do the same thing with pages
        // TODO v2 (clovis): If we know what vars will be used,
        // then there is no need to force user to define vars.
        // Just define it after Installer creation
        //
        // Populate used_vars
        for action in &self.actions {
            for var in action.vars() {
                used_vars.insert(var);
            }
        }

        let mut unused_vars: HashSet<String> = HashSet::new();
        // Populate unused_vars
        for var in self.vars.keys() {
            if !used_vars.contains(var) { unused_vars.insert(var.clone()); }
        }

        if log {
            println!("[WARNING] Variables {:?} declared but not used.", unused_vars);
        }

        let mut undeclared_vars: HashSet<String> = used_vars.clone();
        // Populate undeclared_vars
        undeclared_vars.retain(|var| { !self.vars.contains_key(var) });

        // Return Error if there are any undeclared variables
        if !undeclared_vars.is_empty() {
            let err_str = format!("Error: variables {:?} used but not declared!", undeclared_vars);
            return Err(err_str.into());
        }

        // Return Error if one of the pages is not valid
        for page in &self.pages {
            page.validate()?
        }

        return Ok(());
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
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
