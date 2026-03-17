use std::collections::HashMap;

use eframe::egui::{Checkbox, Frame, Label, ScrollArea, Ui};

use crate::core::{InstallerPage, InstallerPageText, InstallerAction, InstallComponent};
use crate::settings::Settings;

pub trait GuiPage {
    // TODO(clovis): too many params. Needs refactoring
    fn gui_page(&self, ui: &mut Ui, settings: &Settings, last_res: Option<&PageResponse>, actions: Option<&Vec<InstallerAction>>, vars: &HashMap<String, String> ) -> Result<PageResponse, Box<dyn std::error::Error>>;
}

impl GuiPage for InstallerPage {
    fn gui_page(&self, ui: &mut Ui, settings: &Settings, last_res: Option<&PageResponse>, actions: Option<&Vec<InstallerAction>>, vars: &HashMap<String, String> ) -> Result<PageResponse, Box<dyn std::error::Error>> {
        let res = last_res.unwrap_or(&PageResponse::Empty);

        match self {
            InstallerPage::Welcome(v) =>
                Ok(create_welcome_page(ui, settings, v.get_text()?)),
            InstallerPage::License(v) =>
                Ok(create_license_page(ui, settings, v.get_text()?, res.license_or_default())),
            InstallerPage::Components(v) =>
                Ok(create_components_page(ui, settings, v.get_text().ok(), v.components(), res.components_or_default())),
            InstallerPage::Preinstall =>
                Ok(create_preinstall_page(ui, actions, vars)),
            #[allow(unreachable_patterns)]
            _ => Err("Not yet Implemented!".into()),
        }
    }
}

#[derive(Clone, Debug)]
pub enum PageResponse {
    Empty,
    License(LicenseResponse),
    Components(ComponentsResponse),
}
impl PageResponse {
    pub fn allow_next(&self) -> bool {
        match self {
            PageResponse::Empty | PageResponse::Components(_) => true,
            PageResponse::License(r) => r.allow_next,
        }
    }

    pub fn license_or_default(&self) -> LicenseResponse {
        if let Self::License(v) = self { v.clone() }
        else { LicenseResponse::default() }
    }
    pub fn components_or_default(&self) -> ComponentsResponse {
        if let Self::Components(v) = self { v.clone() }
        else { ComponentsResponse::default() }
    }
}

#[derive(Clone, Debug, Default)]
pub struct LicenseResponse {
    pub allow_next: bool,
    pub license_accepted: bool,
}
#[derive(Clone, Debug, Default)]
pub struct ComponentsResponse {
    pub vars: HashMap<String, String>,
}

////////////////////////////////////////////////////////////////////////////////
fn create_welcome_page(ui: &mut Ui, _settings: &Settings, text: String) -> PageResponse {
    ScrollArea::vertical().show(ui, |ui| {
        let label = Label::new(text);
        ui.add_sized(ui.available_size(), label);
    });

    return PageResponse::Empty;
}

// Accepts last response
fn create_license_page(ui: &mut Ui, settings: &Settings, text: String, mut res: LicenseResponse) -> PageResponse {
    ui.vertical_centered(|ui| {
        Frame::group(ui.style()).show(ui, |ui| {
            let sa_height = ui.available_height() - settings.gui.ctrl_button_size.y - ui.style().spacing.item_spacing.y;
            ScrollArea::vertical().max_height(sa_height).show(ui, |ui| {
                let text_label = Label::new(text);

                // TODO(clovis): sizing label width causes strange displaying of long lines
                ui.add_sized(ui.available_size(), text_label);
            });
        });

        let checkbox = Checkbox::new(&mut res.license_accepted, "Accept");
        ui.add_sized(settings.gui.ctrl_button_size, checkbox);

        res.allow_next = res.license_accepted;
    });

    return PageResponse::License(res);
}

// Accepts last response
fn create_components_page(ui: &mut Ui, settings: &Settings, text: Option<String>, comps: &Vec<InstallComponent>, mut res: ComponentsResponse) -> PageResponse {
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
                    for (key, val) in &res.vars {
                        if let Ok(v) = val.parse::<bool>() { cb_bufs.insert(key.to_string(), v); }
                    }

                    for comp in comps {
                        if !cb_bufs.contains_key(&comp.var) { cb_bufs.insert(comp.var.clone(), comp.checked); }
                        let mut cb_buf = cb_bufs.get_mut(&comp.var).expect("should be in HashMap");

                        let cb = Checkbox::new(&mut cb_buf, comp.name.clone());

                        if ui.add_enabled(comp.enabled, cb).contains_pointer() { hovered_component = Some(comp); }
                    }

                    let string_map: HashMap<String, String> = cb_bufs.into_iter().map(|(k, v)| (k, v.to_string()) ).collect();
                    res.vars = string_map;

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

    return PageResponse::Components(res);
}

fn create_preinstall_page(ui: &mut Ui, actions: Option<&Vec<InstallerAction>>, vars: &HashMap<String, String>) -> PageResponse {
    ui.columns_const(|[c1, c2]|{
        c1.vertical(|ui| {
            ui.heading("Actions:");
            match actions {
                Some(v) => {
                    ui.label(format!("{:#?}", v));
                },
                None => {
                    ui.label("No actions provided");
                },
            }
        });

        c2.vertical(|ui| {
            ui.heading("Vars:");
            ui.label(format!("{:#?}", vars));
        });
    });

    ui.add_space(ui.available_height());

    return PageResponse::Empty;
}
////////////////////////////////////////////////////////////////////////////////
