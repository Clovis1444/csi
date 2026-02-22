use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Installer {
    pub general: InstallerGeneral,
    #[serde(alias = "page")]
    pub pages: Vec<InstallerPage>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InstallerGeneral {
    program_name: String,
    program_desc: String,
}
impl Default for InstallerGeneral {
    fn default() -> Self {
        Self {
            program_name: String::from("Program Name"),
            program_desc: String::from("Program Description")
        }
    }
}
impl InstallerGeneral {
    pub fn program_name(&self) -> &str { &self.program_name }
    pub fn set_program_name(&mut self, val: String) { self.program_name = val }
    pub fn program_desc(&self) -> &str { &self.program_desc }
    pub fn set_program_desc(&mut self, val: String) { self.program_desc = val }

}

#[derive(Deserialize, Serialize, Debug)]
pub enum InstallerPage {
    Welcome(WelcomePage),
    License(LicensePage),
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

/////////////////////////////////////////////////////////////////////////////////
