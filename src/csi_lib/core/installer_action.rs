use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum InstallerAction {
    None,
    Copy(CopyInstallerAction),
    Download(DownloadInstallerAction),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CopyInstallerAction {
    input: String,
    output: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DownloadInstallerAction {
    input: String,
    output: String,
}
