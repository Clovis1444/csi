use eframe::egui::{Context, Ui, TopBottomPanel, CentralPanel, Layout, Align};

#[derive(Default, Debug)]
pub struct PageFrame{
    page_index: i32,
    page_count: i32,
}

#[derive(Default, Debug)]
pub struct PageFrameResponse {
    pub quit_clicked: bool,
    pub next_clicked: bool,
    pub prev_clicked: bool,
}

impl PageFrame {
    pub fn new(page_index: i32, page_count: i32) -> Self {
        Self { page_index: page_index, page_count: page_count }
    }

    pub fn show<R>(&self, ctx: &Context, add_contents: impl FnOnce(&mut Ui) -> R) -> PageFrameResponse {
        let mut response = PageFrameResponse::default();

        TopBottomPanel::top("Title Panel").show(ctx, |ui| {
            ui.heading("Page Title");
        });

        TopBottomPanel::bottom("Bottom Panel").show(ctx, |ui| {
            ui.columns_const(|[col_1, col_2, col_3]| {
                let col_1_layout = Layout::left_to_right(Align::Center);
                col_1.with_layout(col_1_layout, |ui| {
                    response.quit_clicked = ui.button("Quit").clicked();
                });

                col_2.centered_and_justified(|ui| {
                    ui.label(format!("{}/{}", self.page_index, self.page_count));
                });

                let col_3_layout = Layout::right_to_left(Align::Center);
                col_3.with_layout(col_3_layout, |ui| {
                    if ui.button("Next").clicked() { response.next_clicked = true; };
                    ui.add_space(10.0);
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
