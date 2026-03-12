use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum InstallerPageType {
    Welcome,
    License,
    Components,
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InstallerPage {
    page_type: InstallerPageType,
    title: String,
    text: Option<String>,
    file_path: Option<String>,
    #[serde(default = "InstallerPage::default_prefer_file")]
    prefer_file: bool,
    vars: Option<Vec<Var>>,
    comps: Option<Vec<InstallComponent>>,
}

impl InstallerPage {
    pub fn page_type(&self) -> InstallerPageType { self.page_type }
    pub fn set_page_type(&mut self, page_type: InstallerPageType) { self.page_type = page_type }
    pub fn title(&self) -> &str { &self.title }
    pub fn set_title(&mut self, title: String) { self.title = title }
    pub fn raw_text(&self) -> Option<&str> { self.text.as_deref() }
    pub fn set_raw_text(&mut self, text: Option<String>) { self.text = text }
    pub fn file_path(&self) -> Option<&str> { self.text.as_deref() }
    pub fn set_file_path(&mut self, file_path: Option<String>) { self.file_path = file_path }
    pub fn prefer_file(&self) -> bool { self.prefer_file }
    pub fn set_prefer_file(&mut self, prefer_file: bool) { self.prefer_file = prefer_file }
    pub fn vars(&self) -> Option<&Vec<Var>> { self.vars.as_ref() }
    pub fn set_vars(&mut self, vars: Option<Vec<Var>>) { self.vars = vars }
    pub fn opts(&self) -> Option<&Vec<InstallComponent>> { self.comps.as_ref() }
    pub fn set_opts(&mut self, opts: Option<Vec<InstallComponent>>) { self.comps = opts }

    pub fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self.page_type {
            t @ (InstallerPageType::Welcome | InstallerPageType::License) => {
                match self.text() {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("InstallerPage::{t:?}: {e}").into()),
                }
            },
            t @ InstallerPageType::Components => {
                match self.comps.is_some() {
                    true => Ok(()),
                    false => Err(format!("InstallerPage::{t:?}: comps should be populated").into()),
                }
            },
            // Handle new PageTypes here
        }
    }
    pub fn is_valid(&self) -> bool {
        return self.validate().is_ok();
    }

    pub fn text(&self) -> Result<String, Box<dyn std::error::Error>> {
        let fallback_err = format!("Either 'text' or 'file_path' field should be defined and valid");

        let from_text = |err: String| {
            match self.text.clone() {
                Some(s) => Ok(s),
                None => Err(err.into()),
            }
        };
        let from_file_path = |err: String| {
            if let Some(path) = &self.file_path {
                match std::fs::read_to_string(path){
                    Ok(s) => { return Ok(s); },
                    Err(e) => {
                        return Err(format!("Failed to read file \"{path}\": {e}.").into());
                    }
                }
            }
            return Err(err.into());
        };

        let result_file = from_file_path(fallback_err.clone());
        let result_text = from_text(fallback_err);

        if self.prefer_file {
            if result_file.is_err() && result_text.is_err() {
                return result_file;
            } else {
                return result_file.or(result_text);
            }
        } else {
            if result_file.is_err() && result_text.is_err() {
                return result_text;
            } else {
                return result_text.or(result_file);
            }
        }
    }

    pub fn default_prefer_file() -> bool { true }
}
