use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum InstallerPageType {
    Custom,
    Welcome,
    License,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InstallerPage {
    page_type: InstallerPageType,
    title: String,
    text: Option<String>,
    file_path: Option<String>,
    #[serde(default = "InstallerPage::default_prefer_file")]
    prefer_file: bool,
    vars: Option<HashMap<String, String>>,
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

    pub fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self.page_type {
            InstallerPageType::Custom => todo!(),
            InstallerPageType::Welcome => Ok(()),
            t @ InstallerPageType::License => {
                if self.text() == None {
                    Err(format!("Invalid or missing 'text' or 'file_path' field in page with type '{:?}'", t).into())
                } else {
                    Ok(())
                }
            },
            // Handle new PageTypes here
        }
    }
    pub fn is_valid(&self) -> bool {
        return self.validate().is_ok();
    }

    pub fn text(&self) -> Option<String> {
        let from_text = || { return self.text.clone(); };
        let from_file_path = || {
            if let Some(path) = &self.file_path {
                match std::fs::read_to_string(path){
                    Ok(s) => { return Some(s); },
                    Err(e) => {
                        println!("Failed to read file {path}: {e}.");
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

    pub fn default_prefer_file() -> bool { true }
}
// impl InstallerPage {
//     // TODO(clovis): add PageAction struct or trait
//     pub fn has_action(&self) -> bool {
//         match self {
//             // InstallerPage::Welcome(_val) => { return true; }
//             _ => { return false; }
//         }
//     }
//     // Returns true if action was executed successfully
//     pub fn exec_action(&self) -> bool {
//         if !self.has_action() { return false; }

//         return true;
//     }
// }

// /////////////////////////////////////////////////////////////////////////////////
// #[derive(Deserialize, Serialize, Clone, Debug)]
// pub struct WelcomePage {
//     title: String,
//     desc: String,
// }
// impl Default for WelcomePage {
//     fn default() -> Self {
//         Self {
//             title: String::from("Welcome title"),
//             desc: String::from("License text")
//         }
//     }
// }
// impl WelcomePage {
//     pub fn title(&self) -> &str { &self.title }
//     pub fn set_title(&mut self, val: String) { self.title = val }
//     pub fn desc(&self) -> &str { &self.desc }
//     pub fn set_desc(&mut self, val: String) { self.desc = val }
// }

// #[derive(Deserialize, Serialize, Clone, Debug)]
// pub struct LicensePage {
//     title: String,
//     text: Option<String>,
//     file_path: Option<String>,
//     #[serde(default = "LicensePage::default_prefer_file")]
//     prefer_file: bool,
// }
// impl Default for LicensePage {
//     fn default() -> Self {
//         Self {
//             title: String::from("License title"),
//             text: Some(String::from("License text")),
//             file_path: None,
//             prefer_file: Self::default_prefer_file()
//         }
//     }
// }
// impl LicensePage{
//     pub fn title(&self) -> &str { &self.title }
//     pub fn set_title(&mut self, val: String) { self.title = val }
//     pub fn prefer_file(&self) -> bool { self.prefer_file }
//     pub fn set_prefer_file(&mut self, val: bool) { self.prefer_file = val }
//     pub fn set_text(&mut self, val: Option<String>) { self.text = val }
//     pub fn set_file_path(&mut self, val: Option<String>) { self.file_path = val }
//     pub fn license_text(&self) -> Option<String> {
//         let from_text = || { return self.text.clone(); };
//         let from_file_path = || {
//             if let Some(path) = &self.file_path {
//                 match std::fs::read_to_string(path){
//                     Ok(s) => { return Some(s); },
//                     Err(e) => {
//                         println!("Failed to read license file {path}: {e}.");
//                         return None;
//                     }
//                 }
//             }
//             return None;
//         };

//         if self.prefer_file {
//             return from_file_path().or_else(from_text);
//         } else {
//             return from_text().or_else(from_file_path);
//         }
//     }

//     pub fn default_prefer_file() -> bool {return true;}

// }

// #[derive(Deserialize, Serialize, Clone, Debug)]
// pub struct CustomPage {
//     title: Option<String>,
//     desc: Option<String>,
// }
// impl Default for CustomPage {
//     fn default() -> Self {
//         Self {
//             title: None,
//             desc: None,
//         }
//     }
// }
// impl CustomPage {
//     pub fn title(&self) -> &Option<String> { &self.title }
//     pub fn set_title(&mut self, val: Option<String>) { self.title = val }
//     pub fn desc(&self) -> &Option<String> { &self.desc }
//     pub fn set_desc(&mut self, val: Option<String>) { self.desc = val }
// }
// /////////////////////////////////////////////////////////////////////////////////
