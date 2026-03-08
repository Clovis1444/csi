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
        let installer = csi::parser::installer_from_file(f_path)?;

        if let Err(e) = installer.validate() {
            return Err(e);
        }

        self.installer = Some(installer);

        return Ok(());
    }
    #[allow(dead_code)]
    pub fn unload_installer(&mut self) { self.installer = None; }

    pub fn run_gui(&self) -> Result<(), Box<dyn std::error::Error>> {
        let result = match self.installer.clone() {
            Some(i) => csi::gui::InstallerGui::run(i)?,
            None => { return Err("Installer is not loaded".into()); },
        };

        return Ok(result);
    }
}
