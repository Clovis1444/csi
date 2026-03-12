use std::collections::HashMap;

use eframe::egui::{Checkbox, Frame, Label, ScrollArea, Ui};

use crate::core::{InstallerPage, InstallerPageType, InstallComponent};
use crate::settings::Settings;

pub trait GuiPage {
    fn gui_page(&self, ui: &mut Ui, settings: &Settings, last_res: Option<&PageResponse>) -> Result<PageResponse, Box<dyn std::error::Error>>;
}

impl GuiPage for InstallerPage {
    fn gui_page(&self, ui: &mut Ui, settings: &Settings, last_res: Option<&PageResponse>) -> Result<PageResponse, Box<dyn std::error::Error>> {
        match self.page_type() {
            InstallerPageType::Welcome => Ok(create_welcome_page(ui, settings, self.text()?)),
            InstallerPageType::License => Ok(create_license_page(ui, settings, self.text()?, last_res)),
            InstallerPageType::Components => {
                Ok(create_components_page(ui, settings, self.text().ok(), self.opts().ok_or("No opts found")?, last_res))
            },
            #[allow(unreachable_patterns)]
            _ => Err("Not yet Implemented!".into()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PageResponse {
    pub allow_next: bool,
    pub license_accepted: Option<bool>,
    pub vars: Option<HashMap<String, String>>,
}
impl Default for PageResponse {
    fn default() -> Self {
        Self {
            allow_next: true,
            license_accepted: None,
            vars: None,
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
        let checkbox = Checkbox::new(&mut checkbox_buf, "Accept");
        ui.add_sized(settings.gui.ctrl_button_size, checkbox);

        response.license_accepted = Some(checkbox_buf);
        response.allow_next = checkbox_buf;
    });

    return response;
}

fn create_welcome_page(ui: &mut Ui, _settings: &Settings, text: String) -> PageResponse {
    let label = Label::new(text);
    ui.add_sized(ui.available_size(), label);

    return PageResponse::default();
}

fn create_components_page(ui: &mut Ui, settings: &Settings, text: Option<String>, comps: &Vec<InstallComponent>, last_res: Option<&PageResponse>) -> PageResponse {
    let mut response = last_res.cloned().unwrap_or_default();

    ui.vertical_centered(|ui| {
        if let Some(t) = text {
            ui.add_space(0.5 * settings.gui.ctrl_button_size.y);

            let label = Label::new(t);
            ui.add(label);

            ui.add_space(0.5 * settings.gui.ctrl_button_size.y);
        }

        ui.columns_const(|[col1, col2]| {
            let mut hovered_component: Option<&InstallComponent> = None;

            // CheckBox column
            col1.group(|ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    ui.heading("Components:");

                    let mut cb_bufs: HashMap<String, bool> = HashMap::new();
                    if let Some(vars) = &response.vars {
                        for (key, val) in vars {
                            if let Ok(v) = val.parse::<bool>() { cb_bufs.insert(key.to_string(), v); }
                        }
                    }

                    for comp in comps {
                        if !cb_bufs.contains_key(&comp.var) { cb_bufs.insert(comp.var.clone(), comp.checked); }
                        let mut cb_buf = cb_bufs.get_mut(&comp.var).expect("should be in HashMap");

                        let cb = Checkbox::new(&mut cb_buf, comp.name.clone());

                        if ui.add_enabled(comp.enabled, cb).contains_pointer() { hovered_component = Some(comp); }
                    }

                    let string_map: HashMap<String, String> = cb_bufs.into_iter().map(|(k, v)| (k, v.to_string()) ).collect();
                    response.vars = Some(string_map);

                    ui.add_space(ui.available_height());
                });
            });

            // Desc column
            col2.group(|ui| {
                // Note: assign id or this scrool area will use the same id as previous one
                ScrollArea::vertical().id_salt("desc_scroll")
                    .max_width(f32::INFINITY)
                    .max_height(f32::INFINITY)
                    .show(ui, |ui| {
                    if let Some(comp) = hovered_component {
                        ui.heading(format!("{}:", comp.name));
                        ui.label(&comp.desc);
                    } else {
                        ui.heading("Component description:");
                        ui.label("Hover over a component to learn more about it.");
                    }

                    ui.add_space(ui.available_height());
                });
            });
        });

    });

    return response;
}
