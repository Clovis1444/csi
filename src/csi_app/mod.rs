use csi::settings::Settings;
use csi::core::Installer;

#[derive(Debug)]
pub struct CsiApp<'a> {
    installer: Option<Installer>,
    settings: &'a mut Settings,
}

impl<'a> CsiApp<'a> {
    pub fn new(settings: &'a mut Settings) -> Self {
        Self {
            installer: None,
            settings: settings,
        }
    }

    pub fn load_installer(&mut self, f_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let installer = Installer::from_file(f_path, self.settings.log)?;

        self.installer = Some(installer);

        return Ok(());
    }
    #[allow(dead_code)]
    pub fn unload_installer(&mut self) { self.installer = None; }

    pub fn run_gui(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let result = match &mut self.installer {
            Some(i) => csi::gui::InstallerGui::run(self.settings, i)?,
            None => { return Err("Installer is not loaded".into()); },
        };

        return Ok(result);
    }
}
