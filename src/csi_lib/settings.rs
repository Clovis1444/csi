#[derive(Debug)]
pub struct Settings {
    pub log: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            log: true,
        }
    }
}
