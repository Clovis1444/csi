use eframe::egui::{self, Align, CentralPanel, Context, Layout, TopBottomPanel, Ui, Image};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Default, Debug)]
pub struct PageFrame{
    page_index: i32,
    page_count: i32,
    lang: Language,
}

#[derive(Default, Debug)]
pub struct PageFrameResponse {
    pub quit_clicked: bool,
    pub next_clicked: bool,
    pub prev_clicked: bool,
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
    pub fn new(page_index: i32, page_count: i32, lang: Language) -> Self {
        Self { page_index, page_count, lang }
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
                    let lang_icon_src = egui::include_image!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/lang_icon32.png"));
                    let lang_icon = Image::new(lang_icon_src).fit_to_original_size(1.0);

                    // Lang button
                    ui.menu_image_button(lang_icon, |ui| {
                        for l in Language::iter() {
                            if ui.button(l.to_string()).clicked() {response.lang = Some(l);}
                        }
                    });

                    // Lang label
                    ui.label(self.lang.to_string());
                });
            });
        });

        TopBottomPanel::bottom("Bottom Panel").show(ctx, |ui| {
            ui.columns_const(|[col_1, col_2, col_3]| {
                let col_1_layout = Layout::left_to_right(Align::Center);
                col_1.with_layout(col_1_layout, |ui| {
                    // Quit button
                    response.quit_clicked = ui.button("Quit").clicked();
                });

                col_2.centered_and_justified(|ui| {
                    // Page number label
                    ui.label(format!("{}/{}", self.page_index, self.page_count));
                });

                let col_3_layout = Layout::right_to_left(Align::Center);
                col_3.with_layout(col_3_layout, |ui| {
                    // Next button
                    if ui.button("Next").clicked() { response.next_clicked = true; };
                    ui.add_space(10.0);
                    // Prev button
                    if ui.button("Prev").clicked() { response.prev_clicked = true; };
                });
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
}
