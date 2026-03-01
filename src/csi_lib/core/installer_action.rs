use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct InstallerAction {
    #[serde(alias = "type")]
    action_type: InstallerActionType,
    #[serde(alias = "source")]
    var_source: String,
    #[serde(alias = "cond", default)]
    var_conds: Vec<String>,
    output_path: String,
}
impl InstallerAction {
    pub fn vars(&self) -> Vec<String> {
        let mut var_list: Vec<String> = self.var_conds.clone();
        var_list.push(self.var_source.clone());

        return var_list;
    }

    pub fn is_valid(&self) -> bool {
        let result = match self.action_type {
            InstallerActionType::Copy | InstallerActionType::Download
            => { true },
        };

        return result;
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum InstallerActionType {
    Copy,
    Download,
}
