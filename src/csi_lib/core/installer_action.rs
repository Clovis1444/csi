use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum InstallerAction {
    None,
    Copy(CopyInstallerAction),
    Download(DownloadInstallerAction),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CopyInstallerAction {
    var_name: String,
    output_path: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DownloadInstallerAction {
    var_name: String,
    output_path: String,
}
