use csi::settings::Settings;
use csi::core::Installer;

pub struct CsiApp {
    installer: Option<Installer>,
    settings: Settings,
}

impl CsiApp {
    pub fn new(settings: Settings) -> Self {
        Self {
            installer: None,
            settings: settings,
        }
    }

    pub fn load_installer(&mut self, f_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.installer = Some(csi::parser::installer_from_file(f_path)?);

        return Ok(());
    }
    #[allow(dead_code)]
    pub fn unload_installer(&mut self) { self.installer = None; }

    pub fn run_gui(&self) -> Result<(), Box<dyn std::error::Error>> {
        return Ok(csi::gui::InstallerGui::run()?);
    }
}
