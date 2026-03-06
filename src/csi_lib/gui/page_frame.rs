use eframe::egui::{self, Align, Button, CentralPanel, Context, Image, Label, Layout, TopBottomPanel, Ui, Vec2, RichText};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug)]
pub struct PageFrame{
    page_index: i32,
    page_count: i32,
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

impl PageFrame {
    const CTRL_BUTTON_SIZE:  Vec2 = Vec2::new(64.0, 32.0);
    const LANG_BUTTON_SIZE:  Vec2 = Vec2::new(32.0, 32.0);
    const THEME_BUTTON_SIZE: Vec2 = Vec2::new(32.0, 32.0);

    pub fn new(page_index: i32, page_count: i32, lang: Language) -> Self {
        let back_enabled = page_index > 1;
        let next_enabled = page_index < page_count;
        Self { page_index, page_count, lang, back_enabled, next_enabled }
    }

    // TODO(clovis): add zoom button?
    pub fn show<R>(&self, ctx: &Context, add_contents: impl FnOnce(&mut Ui) -> R) -> PageFrameResponse {
        let mut response = PageFrameResponse::default();

        TopBottomPanel::top("Title Panel").show(ctx, |ui| {
            ui.style_mut().spacing.item_spacing.x = 0.0;

            let lang_theme_spacing = Self::LANG_BUTTON_SIZE.y;
            let lang_label_spacing = 0.25 * Self::LANG_BUTTON_SIZE.y;

            let layout = Layout::right_to_left(Align::Center);
            ui.with_layout(layout, |ui| {
                // Theme button
                Self::make_theme_button(ui);

                ui.add_space(lang_theme_spacing);

                // Lang button
                Self::make_lang_button(ui, &mut response);

                ui.add_space(lang_label_spacing);

                // Lang label
                ui.label(self.lang.to_string());

                let title_width = f32::max(0.0, ui.available_width());

                // Title heading
                let title = Label::new(RichText::new("Page Title Text").heading());
                ui.add_sized(Vec2::new(title_width, ui.available_height()), title);
            });
        });

        TopBottomPanel::bottom("Bottom Panel").show(ctx, |ui| {
            ui.style_mut().spacing.item_spacing.x = 0.0;

            let back_next_spacing = Self::CTRL_BUTTON_SIZE.y;

            let layout = Layout::right_to_left(Align::Center);
            ui.with_layout(layout, |ui| {
                // Next button
                let next_b = Self::ctrl_button("Next");
                response.next_clicked = ui.add_enabled(self.next_enabled, next_b).clicked();

                ui.add_space(back_next_spacing);

                // Back button
                let back_b = Self::ctrl_button("Back");
                response.back_clicked = ui.add_enabled(self.back_enabled, back_b).clicked();

                let page_label_width = f32::max(0.0, ui.available_width() - Self::CTRL_BUTTON_SIZE.x);

                // Page label
                let page_label = Label::new(format!("{}/{}", self.page_index, self.page_count));
                ui.add_sized(Vec2::new(page_label_width, Self::CTRL_BUTTON_SIZE.y), page_label);

                // Quit button
                let quit_b = Self::ctrl_button("Quit");
                response.quit_clicked = ui.add(quit_b).clicked();
            });
        });

        CentralPanel::default().show(ctx , |ui| {
            ui.label("BEFORE USER");
            // USER STUFF BEGIN
            add_contents(ui);
            // USER STUFF END
            ui.label("AFTER USER");
        });

        return response;
    }

    fn ctrl_button(text: &str) -> Button<'_> {
        Button::new(text).min_size(Self::CTRL_BUTTON_SIZE)
    }
    fn make_lang_button(ui: &mut Ui, response: &mut PageFrameResponse) {
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

            let lang_icon = Image::new(lang_icon_src).shrink_to_fit().fit_to_exact_size(Self::LANG_BUTTON_SIZE);
            // Lang button
            ui.menu_image_button(lang_icon, |ui| {
                for l in Language::iter() {
                    if ui.button(l.to_string()).clicked() { response.lang = Some(l); }
                }
            });
        });
    }
    fn make_theme_button(ui: &mut Ui) {
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
            let theme_icon = Image::new(theme_icon_src).shrink_to_fit().fit_to_exact_size(Self::THEME_BUTTON_SIZE);

            let button = Button::image(theme_icon).min_size(Self::THEME_BUTTON_SIZE);

            if ui.add(button).clicked() { ui.ctx().set_theme(new_theme); }
        });
    }
}
