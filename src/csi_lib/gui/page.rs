use eframe::egui::{Ui, ScrollArea, Frame, Label};

use crate::core::InstallerPage;
use crate::settings::Settings;

pub trait GuiPage {
    fn gui_page(&self, ui: &mut Ui, settings: &Settings, license_accepted: bool) -> Result<PageResponse, Box<dyn std::error::Error>>;
}

impl GuiPage for InstallerPage {
    fn gui_page(&self, ui: &mut Ui, settings: &Settings, license_accepted: bool) -> Result<PageResponse, Box<dyn std::error::Error>> {
        match self.page_type() {
            crate::core::InstallerPageType::Custom => todo!(),
            crate::core::InstallerPageType::Welcome => Err("Not yet Implemented!".into()),
            crate::core::InstallerPageType::License => Ok(create_license_page(ui, settings, self.text()?, license_accepted)),
        }
    }
}

#[derive(Debug)]
pub struct PageResponse {
    pub allow_next: bool,
    pub license_accepted: Option<bool>,
    // TODO(clovis): add returned vars here
}
impl Default for PageResponse {
    fn default() -> Self {
        Self {
            allow_next: true,
            license_accepted: None,
        }
    }
}

fn create_license_page(ui: &mut Ui, settings: &Settings, text: String, license_accepted: bool) -> PageResponse {
    let mut response = PageResponse::default();

    ui.vertical_centered(|ui| {
        Frame::group(ui.style()).show(ui, |ui| {
            let sa_height = ui.available_height() - settings.gui.ctrl_button_size.y;
            ScrollArea::vertical().max_height(sa_height).show(ui, |ui| {
                let text_label = Label::new(text);

                ui.add(text_label);
            });
        });

        let mut checkbox_buf = license_accepted;
        let checkbox = eframe::egui::Checkbox::new(&mut checkbox_buf, "Accept");
        ui.add_sized(settings.gui.ctrl_button_size, checkbox);

        response.license_accepted = Some(checkbox_buf);
        response.allow_next = checkbox_buf;
    });

    return response;
}
