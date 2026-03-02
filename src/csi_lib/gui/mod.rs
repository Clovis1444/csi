mod page_frame;

use eframe::egui;

pub fn hello_egui() {
    // Set window properties here
    let native_options = eframe::NativeOptions {
        centered: true,
        viewport: egui::ViewportBuilder {
            title: Some("CSI".to_string()),
            app_id: Some("csi".to_string()),
            inner_size: Some(egui::Vec2 { x: 800.0, y: 600.0 }),
            decorations: Some(false),
            ..Default::default()
        },
        ..Default::default()
    };

    let _r = eframe::run_native(
        "CSI",
        native_options,
        Box::new(|cc| Ok(Box::new(InstallerGui::new(cc)))),
    );
}

#[derive(Default)]
struct InstallerGui {
    page_index: i32,
    page_count: i32,
}

impl InstallerGui {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self {page_index: 7, page_count: 13}
    }

    fn next_page(&mut self) {
        self.page_index = std::cmp::min(self.page_count, self.page_index + 1);
    }
    fn prev_page(&mut self) {
        self.page_index = std::cmp::max(1, self.page_index - 1);
    }
}

impl eframe::App for InstallerGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let response = page_frame::PageFrame::new(self.page_index, self.page_count).show(ctx, |ui| {
            ui.label("THIS IS USER LABEL!");
            if ui.button("Click me!").clicked() {
                println!("Clicked from user!");
            }
        });

        if response.next_clicked { self.next_page(); }
        if response.prev_clicked { self.prev_page(); }
        if response.quit_clicked { ctx.send_viewport_cmd(egui::ViewportCommand::Close); }
    }
}
