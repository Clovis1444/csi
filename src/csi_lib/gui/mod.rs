mod page_frame;

use eframe::egui::{self, Style, Theme, Vec2};

pub struct InstallerGui {
    page_index: i32,
    page_count: i32,
    lang: page_frame::Language,
}

impl InstallerGui {
    pub fn run() -> Result<(), eframe::Error> {
        // Set window properties here
        let native_options = eframe::NativeOptions {
            centered: true,
            viewport: egui::ViewportBuilder {
                title: Some("CSI".to_string()),
                app_id: Some("csi".to_string()),
                // icon: Some(""),
                inner_size: Some(Vec2 { x: 800.0, y: 600.0 }),
                min_inner_size: Some(Vec2 { x: 400.0, y: 400.0 }),
                decorations: Some(false),
                ..Default::default()
            },
            ..Default::default()
        };

        return eframe::run_native(
            "CSI",
            native_options,
            Box::new(|cc| Ok(Box::new(InstallerGui::new(cc)))),
        );
    }

    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Set style
        let style = Style{
            // Change style here
            ..Default::default()
        };
        cc.egui_ctx.set_style(style);

        // Set theme
        if let Some(theme) = cc.egui_ctx.system_theme() {
            cc.egui_ctx.set_theme(theme);
        }
        else{
            cc.egui_ctx.set_theme(Theme::Dark);
        }

        egui_extras::install_image_loaders(&cc.egui_ctx);

        Self { page_index: 7, page_count: 13, lang: page_frame::Language::English }
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
        let pf = page_frame::PageFrame::new(
            self.page_index,
            self.page_count,
            self.lang
        );
        let response = pf.show(ctx, |ui| {
            ui.label("THIS IS USER LABEL!");
            if ui.button("Click me!").clicked() {
                println!("Clicked from user!");
            }
        });

        if let Some(l) = response.lang { self.lang = l; }
        if response.next_clicked { self.next_page(); }
        if response.back_clicked { self.prev_page(); }
        if response.quit_clicked { ctx.send_viewport_cmd(egui::ViewportCommand::Close); }
    }
}
