use serde::{Deserialize, Serialize};

// TODO(clovis): integrate var_name in the future pages
#[derive(Deserialize, Serialize, Debug)]
pub enum InstallerPage {
    Custom(CustomPage),
    Welcome(WelcomePage),
    License(LicensePage),
}
impl InstallerPage {
    // TODO(clovis): add PageAction struct or trait
    pub fn has_action(&self) -> bool {
        match self {
            // InstallerPage::Welcome(_val) => { return true; }
            _ => { return false; }
        }
    }
    // Returns true if action was executed successfully
    pub fn exec_action(&self) -> bool {
        if !self.has_action() { return false; }

        return true;
    }
}

/////////////////////////////////////////////////////////////////////////////////
#[derive(Deserialize, Serialize, Debug)]
pub struct WelcomePage {
    title: String,
    desc: String,
}
impl Default for WelcomePage {
    fn default() -> Self {
        Self {
            title: String::from("Welcome title"),
            desc: String::from("License text")
        }
    }
}
impl WelcomePage {
    pub fn title(&self) -> &str { &self.title }
    pub fn set_title(&mut self, val: String) { self.title = val }
    pub fn desc(&self) -> &str { &self.desc }
    pub fn set_desc(&mut self, val: String) { self.desc = val }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LicensePage {
    title: String,
    text: Option<String>,
    file_path: Option<String>,
    #[serde(default = "LicensePage::default_prefer_file")]
    prefer_file: bool,
}
impl Default for LicensePage {
    fn default() -> Self {
        Self {
            title: String::from("License title"),
            text: Some(String::from("License text")),
            file_path: None,
            prefer_file: Self::default_prefer_file()
        }
    }
}
impl LicensePage{
    pub fn title(&self) -> &str { &self.title }
    pub fn set_title(&mut self, val: String) { self.title = val }
    pub fn prefer_file(&self) -> bool { self.prefer_file }
    pub fn set_prefer_file(&mut self, val: bool) { self.prefer_file = val }
    pub fn set_text(&mut self, val: Option<String>) { self.text = val }
    pub fn set_file_path(&mut self, val: Option<String>) { self.file_path = val }
    pub fn license_text(&self) -> Option<String> {
        let from_text = || { return self.text.clone(); };
        let from_file_path = || {
            if let Some(path) = &self.file_path {
                match std::fs::read_to_string(path){
                    Ok(s) => { return Some(s); },
                    Err(e) => {
                        println!("Failed to read license file {path}: {e}.");
                        return None;
                    }
                }
            }
            return None;
        };

        if self.prefer_file {
            return from_file_path().or_else(from_text);
        } else {
            return from_text().or_else(from_file_path);
        }
    }

    pub fn default_prefer_file() -> bool {return true;}

}

#[derive(Deserialize, Serialize, Debug)]
pub struct CustomPage {
    title: Option<String>,
    desc: Option<String>,
}
impl Default for CustomPage {
    fn default() -> Self {
        Self {
            title: None,
            desc: None,
        }
    }
}
impl CustomPage {
    pub fn title(&self) -> &Option<String> { &self.title }
    pub fn set_title(&mut self, val: Option<String>) { self.title = val }
    pub fn desc(&self) -> &Option<String> { &self.desc }
    pub fn set_desc(&mut self, val: Option<String>) { self.desc = val }
}
/////////////////////////////////////////////////////////////////////////////////
