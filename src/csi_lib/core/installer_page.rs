use serde::{Deserialize, Serialize};

use std::error::Error;

use crate::utils;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
pub enum InstallerPage {
    Welcome(WelcomePage),
    License(LicensePage),
    Components(ComponentsPage),
    // Create new pages here
    #[serde(skip)]
    Preinstall,
    #[serde(skip)]
    Installation,
    #[serde(skip)]
    Postinstall,
}
impl InstallerPage {
    pub fn title(&self) -> &str {
        match self {
            InstallerPage::Welcome(v) => &v.title,
            InstallerPage::License(v) => &v.title,
            InstallerPage::Components(v) => &v.title,
            InstallerPage::Preinstall => "Preinstall overview",
            InstallerPage::Installation => "Installing...",
            InstallerPage::Postinstall => "Installation finished",
        }
    }

    // TODO(clovis): What about validation? Do we really need it?
    pub fn validate(&self) -> Result<(), Box<dyn Error>>{
        Ok(())
    }
    pub fn is_valid(&self) -> bool { self.validate().is_ok() }
}

pub trait InstallerPageText {
    fn raw_text(&self) -> Option<&str>;
    fn file_path(&self) -> Option<&str>;
    fn prefer_file(&self) -> Option<bool>;
    fn default_prefer_file(&self) -> bool { true }
    fn get_text(&self) -> Result<String, Box<dyn Error>> {
        if self.get_prefer_file() {
            match self.get_file_text() {
                Ok(v) => { return Ok(v); },
                Err(e) => {
                    if let Some(text) = self.raw_text() { return Ok(text.to_string()); }
                    else { return Err(e); }
                },
            }
        } else {
            match self.raw_text() {
                Some(v) => { return Ok(v.to_string()) },
                None => {
                    if let Ok(t) = self.get_file_text() { return Ok(t); }
                    else { return Err("No `text` provided".into()); }
                },
            }
        }
    }
    fn get_file_text(&self) -> Result<String, Box<dyn Error>> {
        let path = self.file_path().ok_or("No `file_path` provided")?;
        return utils::read_file(path);
    }
    fn get_prefer_file(&self) -> bool {
        self.prefer_file().unwrap_or(self.default_prefer_file())
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Var {
    pub key: String,
    pub def: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct InstallComponent {
    pub name: String,
    pub desc: String,
    // TODO(clovis): handle same var names
    pub var: String,
    #[serde(default = "InstallComponent::default_checked")]
    pub checked: bool,
    #[serde(default = "InstallComponent::default_enabled")]
    pub enabled: bool,
}
impl InstallComponent {
    pub fn default_checked() -> bool { true }
    pub fn default_enabled() -> bool { true }
}

////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WelcomePage {
    title: String,

    text: Option<String>,
    file_path: Option<String>,
    prefer_file: Option<bool>,
}
impl InstallerPageText for WelcomePage {
    fn raw_text(&self) -> Option<&str> { self.text.as_deref() }
    fn file_path(&self) -> Option<&str> { self.file_path.as_deref() }
    fn prefer_file(&self) -> Option<bool> { self.prefer_file }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LicensePage {
    title: String,

    text: Option<String>,
    file_path: Option<String>,
    prefer_file: Option<bool>,
}
impl InstallerPageText for LicensePage {
    fn raw_text(&self) -> Option<&str> { self.text.as_deref() }
    fn file_path(&self) -> Option<&str> { self.file_path.as_deref() }
    fn prefer_file(&self) -> Option<bool> { self.prefer_file }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ComponentsPage {
    title: String,

    text: Option<String>,
    file_path: Option<String>,
    prefer_file: Option<bool>,

    components: Vec<InstallComponent>,
}
impl ComponentsPage {
    pub fn components(&self) -> &Vec<InstallComponent> { &self.components }
}
impl InstallerPageText for ComponentsPage {
    fn raw_text(&self) -> Option<&str> { self.text.as_deref() }
    fn file_path(&self) -> Option<&str> { self.file_path.as_deref() }
    fn prefer_file(&self) -> Option<bool> { self.prefer_file }
}
////////////////////////////////////////////////////////////////////////////////
