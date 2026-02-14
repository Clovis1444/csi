use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Installer {
    pub general: InstallerGeneral,
    pub pages: Vec<InstallerPage>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InstallerGeneral {
    pub program_name: String,
    pub program_desc: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InstallerPage {
    pub page_type: InstallerPageType,
    pub title: String,
    pub desc: String,
}
impl InstallerPage {}

#[derive(Deserialize, Serialize, Debug)]
pub enum InstallerPageType {
    Welcome,
    License,
}
// impl From<&str> for InstallerPageType {
//     fn from(s: &str) -> Self {
//         match s.to_lowercase().as_str() {
//             "welcome" => InstallerPageType::Welcome,
//             "license" => InstallerPageType::License,
//             invalid_val => {
//                 panic!(
//                     "Can't convert {invalid_val} to {}.",
//                     std::any::type_name::<InstallerPageType>()
//                 )
//             }
//         }
//     }
// }
// impl std::fmt::Display for InstallerPageType {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             InstallerPageType::Welcome => write!(f, "Welcome"),
//             InstallerPageType::License => write!(f, "License"),
//         }
//     }
// }
