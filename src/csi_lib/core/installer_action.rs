use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct InstallerAction {
    #[serde(alias = "type")]
    action_type: InstallerActionType,
    #[serde(alias = "source")]
    var_input: String,
    #[serde(alias = "cond", default)]
    var_conds: Vec<String>,
    output_path: String,
}
impl InstallerAction {
    pub fn vars(&self) -> Vec<String> {
        let mut var_list: Vec<String> = self.var_conds.clone();
        var_list.push(self.var_input.clone());

        return var_list;
    }

    pub fn is_valid(&self) -> bool {
        let result = match self.action_type {
            InstallerActionType::Copy | InstallerActionType::Download
            => { true },
        };

        return result;
    }

    pub fn action_type(&self) -> &InstallerActionType { &self.action_type }
    pub fn v_input(&self) -> &str { &self.var_input }
    pub fn output_path(&self) -> &str { &self.output_path }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum InstallerActionType {
    Copy,
    Download,
}
impl ToString for InstallerActionType {
    fn to_string(&self) -> String {
        match self {
            InstallerActionType::Copy => String::from("Copy"),
            InstallerActionType::Download => String::from("Download"),
        }
    }
}
