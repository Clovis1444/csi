use eframe::egui::{self, Align, Button, CentralPanel, Context, Image, Layout, TopBottomPanel, Ui, Vec2};
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

    pub fn show<R>(&self, ctx: &Context, add_contents: impl FnOnce(&mut Ui) -> R) -> PageFrameResponse {
        let mut response = PageFrameResponse::default();

        TopBottomPanel::top("Title Panel").show(ctx, |ui| {
            ui.columns_const(|[col_1, col_2]| {
                let col_1_layout = Layout::left_to_right(Align::Center);
                col_1.with_layout(col_1_layout, |ui| {
                    // Title heading
                    ui.heading("Page Title");
                });

                let col_2_layout = Layout::right_to_left(Align::Center);
                col_2.with_layout(col_2_layout, |ui| {
                    // Theme button
                    Self::make_theme_button(ui);

                    ui.add_space(Self::LANG_BUTTON_SIZE.x * 0.5);

                    // Lang button
                    Self::make_lang_button(ui, &mut response);
                    // Lang label
                    ui.label(self.lang.to_string());
                });
            });
        });

        TopBottomPanel::bottom("Bottom Panel").show(ctx, |ui| {
            let original_item_spacing = ui.style().spacing.item_spacing;
            ui.style_mut().spacing.item_spacing = Vec2::default();

            // TODO(clovis): fix negative desired size when zooming in

            let available_width = ui.available_width();
            let total_buttons_width = 3.0 * Self::CTRL_BUTTON_SIZE.x;
            let back_next_spacing = Self::CTRL_BUTTON_SIZE.y;
            let remaining_space = available_width - total_buttons_width - back_next_spacing;

            let layout = Layout::left_to_right(Align::Center);
            ui.with_layout(layout, |ui| {
                // Quit button
                let quit_b = Self::ctrl_button("Quit");
                response.quit_clicked = ui.add(quit_b).clicked();

                // Page number label
                let label = egui::Label::new(format!("{}/{}", self.page_index, self.page_count));
                ui.add_sized(Vec2::new(remaining_space, ui.available_height()), label);

                // Back button
                let back_b = Self::ctrl_button("Back");
                response.back_clicked = ui.add_enabled(self.back_enabled, back_b).clicked();

                ui.add_space(back_next_spacing);

                // Next button
                let next_b = Self::ctrl_button("Next");
                response.next_clicked = ui.add_enabled(self.next_enabled, next_b).clicked();
            });
            ui.style_mut().spacing.item_spacing = original_item_spacing;
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
                if ui.button(l.to_string()).clicked() {response.lang = Some(l);}
            }
        });
    }
    fn make_theme_button(ui: &mut Ui) {
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
    }
}
