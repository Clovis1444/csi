use eframe::egui::{Ui, ScrollArea, Frame, Label};

use crate::core::InstallerPage;
use crate::settings::Settings;

pub trait GuiPage {
    fn gui_page(&self, ui: &mut Ui, settings: &Settings, last_res: Option<&PageResponse>) -> Result<PageResponse, Box<dyn std::error::Error>>;
}

impl GuiPage for InstallerPage {
    fn gui_page(&self, ui: &mut Ui, settings: &Settings, last_res: Option<&PageResponse>) -> Result<PageResponse, Box<dyn std::error::Error>> {
        match self.page_type() {
            // crate::core::InstallerPageType::Welcome => Ok(create_welcome_page(ui, settings, self.text()?)),
            crate::core::InstallerPageType::License => Ok(create_license_page(ui, settings, self.text()?, last_res)),
            _ => Err("Not yet Implemented!".into()),
        }
    }
}

#[derive(Clone, Debug)]
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

fn create_license_page(ui: &mut Ui, settings: &Settings, text: String, last_res: Option<&PageResponse>) -> PageResponse {
    let mut response = last_res.cloned().unwrap_or_default();

    ui.vertical_centered(|ui| {
        Frame::group(ui.style()).show(ui, |ui| {
            let sa_height = ui.available_height() - settings.gui.ctrl_button_size.y - ui.style().spacing.item_spacing.y;
            ScrollArea::vertical().max_height(sa_height).show(ui, |ui| {
                let text_label = Label::new(text);

                // TODO(clovis): sizing label width causes strange displaying of long lines
                ui.add_sized(ui.available_size(), text_label);
            });
        });

        let mut checkbox_buf = response.license_accepted.unwrap_or(false);
        let checkbox = eframe::egui::Checkbox::new(&mut checkbox_buf, "Accept");
        ui.add_sized(settings.gui.ctrl_button_size, checkbox);

        response.license_accepted = Some(checkbox_buf);
        response.allow_next = checkbox_buf;
    });

    return response;
}

// fn create_welcome_page(ui: &mut Ui, _settings: &Settings, text: String) -> PageResponse {
//     let label = Label::new(text);
//     ui.add_sized(ui.available_size(), label);

//     return PageResponse::default();
// }
