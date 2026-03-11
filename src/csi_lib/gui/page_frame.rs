use eframe::egui::{self, Align, Button, CentralPanel, Context, Image, Label, Layout, TopBottomPanel, Ui, Vec2, RichText, Frame};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::settings::Settings;

#[derive(Debug)]
pub struct PageFrame<'a> {
    settings: &'a Settings,
    title: String,
    page_index: i32,
    pages_count: i32,
    lang: Language,
    back_enabled: bool,
    next_enabled: bool,
}

#[derive(Default, Debug)]
pub struct PageFrameResponse {
    pub quit_clicked: bool,
    pub next_clicked: bool,
    pub back_clicked: bool,
    pub lang: Option<Language>,
}

#[derive(Default, Debug, Clone, Copy, EnumIter, strum_macros::Display)]
pub enum Language {
    #[default]
    English,
    #[strum(to_string = "Русский")]
    Russian,
}

impl<'a> PageFrame<'a> {
    pub fn new(settings: &'a Settings,title: &str, page_index: i32, page_count: i32, lang: Language, allow_next: bool) -> Self {
        let back_enabled = page_index > 1;
        let next_enabled = page_index < page_count && allow_next;
        Self {
            settings: settings,
            title: String::from(title),
            page_index: page_index,
            pages_count: page_count,
            lang: lang,
            back_enabled: back_enabled,
            next_enabled: next_enabled,
        }
    }

    // TODO(clovis): add zoom button?
    pub fn show<R>(&self, ctx: &Context, add_contents: impl FnOnce(&mut Ui) -> R) -> PageFrameResponse {
        let mut response = PageFrameResponse::default();

        TopBottomPanel::top("Title Panel").show(ctx, |ui| {
            ui.style_mut().spacing.item_spacing.x = 0.0;

            let lang_theme_spacing = self.settings.gui.lang_button_size.y;
            let lang_label_spacing = 0.25 * self.settings.gui.lang_button_size.y;

            let layout = Layout::right_to_left(Align::Center);
            ui.with_layout(layout, |ui| {
                // Theme button
                self.make_theme_button(ui);

                ui.add_space(lang_theme_spacing);

                // Lang button
                self.make_lang_button(ui, &mut response);

                ui.add_space(lang_label_spacing);

                // Lang label
                ui.label(self.lang.to_string());

                let title_width = f32::max(0.0, ui.available_width());

                // Title heading
                let title = Label::new(RichText::new(self.title.clone()).heading());
                ui.add_sized(Vec2::new(title_width, ui.available_height()), title);
            });
        });

        TopBottomPanel::bottom("Bottom Panel").show(ctx, |ui| {
            ui.style_mut().spacing.item_spacing.x = 0.0;

            let back_next_spacing = self.settings.gui.ctrl_button_size.y;

            let layout = Layout::right_to_left(Align::Center);
            ui.with_layout(layout, |ui| {
                // Next button
                let next_b = self.ctrl_button("Next");
                response.next_clicked = ui.add_enabled(self.next_enabled, next_b).clicked();

                ui.add_space(back_next_spacing);

                // Back button
                let back_b = self.ctrl_button("Back");
                response.back_clicked = ui.add_enabled(self.back_enabled, back_b).clicked();

                let page_label_width = f32::max(0.0, ui.available_width() - self.settings.gui.ctrl_button_size.x);

                // Page label
                let page_label = Label::new(format!("{}/{}", self.page_index, self.pages_count));
                ui.add_sized(Vec2::new(page_label_width, self.settings.gui.ctrl_button_size.y), page_label);

                // Quit button
                let quit_b = self.ctrl_button("Quit");
                response.quit_clicked = ui.add(quit_b).clicked();
            });
        });

        CentralPanel::default().show(ctx , |ui| {
            // USER STUFF BEGINS
            Frame::group(ui.style()).show(ui, |ui| {
                ui.scope(|ui| {
                    add_contents(ui);
                });
            });
            // USER STUFF ENDS
        });

        return response;
    }

    fn ctrl_button(&self, text: &str) -> Button<'_> {
        Button::new(text).min_size(self.settings.gui.ctrl_button_size)
    }
    fn make_lang_button(&self, ui: &mut Ui, response: &mut PageFrameResponse) {
        ui.scope(|ui| {
            ui.style_mut().spacing.button_padding = Vec2::default();

            let lang_icon_src;
            // TODO(clovis): factor this out into some AssetManager?
            match ui.ctx().theme() {
                egui::Theme::Dark => {
                    lang_icon_src = egui::include_image!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/lang64-dark.png"));
                },
                egui::Theme::Light => {
                    lang_icon_src = egui::include_image!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/lang64-light.png"));
                },
            };

            let lang_icon = Image::new(lang_icon_src).shrink_to_fit().fit_to_exact_size(self.settings.gui.lang_button_size);
            // Lang button
            ui.menu_image_button(lang_icon, |ui| {
                for l in Language::iter() {
                    if ui.button(l.to_string()).clicked() { response.lang = Some(l); }
                }
            });
        });
    }
    fn make_theme_button(&self, ui: &mut Ui) {
        ui.scope(|ui| {
            ui.style_mut().spacing.button_padding = Vec2::default();

            let theme_icon_src;
            let new_theme;
            match ui.ctx().theme() {
                egui::Theme::Dark =>  {
                    theme_icon_src = egui::include_image!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/theme64-dark.png"));
                    new_theme = egui::Theme::Light;
                },
                egui::Theme::Light => {
                    theme_icon_src = egui::include_image!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/theme64-light.png"));
                    new_theme = egui::Theme::Dark;
                }
            }
            let theme_icon = Image::new(theme_icon_src).shrink_to_fit().fit_to_exact_size(self.settings.gui.theme_button_size);

            let button = Button::image(theme_icon).min_size(self.settings.gui.theme_button_size);

            if ui.add(button).clicked() { ui.ctx().set_theme(new_theme); }
        });
    }
}
