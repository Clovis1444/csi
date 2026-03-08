use eframe::egui::Vec2;

#[derive(Debug)]
pub struct Settings {
    pub gui: Gui,
    pub log: bool,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            gui: Gui::default(),
            log: true,
        }
    }
}

#[derive(Debug)]
pub struct Gui {
    pub ctrl_button_size: Vec2,
    pub lang_button_size: Vec2,
    pub theme_button_size: Vec2,
}
impl Default for Gui {
    fn default() -> Self {
        Self {
            ctrl_button_size:  Vec2::new(64.0, 32.0),
            lang_button_size:  Vec2::new(32.0, 32.0),
            theme_button_size: Vec2::new(32.0, 32.0),
        }
    }
}
