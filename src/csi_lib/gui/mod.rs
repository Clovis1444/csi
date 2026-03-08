mod page_frame;

use eframe::egui::{self, Style, Theme, Vec2};
use crate::core::{Installer, InstallerPage};

pub struct InstallerGui {
    installer: Installer,
    // Note: 0 based indexing
    page_index: i32,
    lang: page_frame::Language,
}

impl InstallerGui {
    pub fn run(installer: Installer) -> Result<(), eframe::Error> {
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
            Box::new(|cc| Ok(Box::new(InstallerGui::new(cc, installer)))),
        );
    }

    fn new(cc: &eframe::CreationContext<'_>, installer: Installer) -> Self {
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

        Self {
            installer: installer,
            page_index: 0,
            lang: page_frame::Language::English,
        }
    }

    fn next_page(&mut self) {
        self.page_index = std::cmp::max(0, std::cmp::min(self.pages_count() - 1, self.page_index + 1));
    }
    fn prev_page(&mut self) {
        self.page_index = std::cmp::max(0, self.page_index - 1);
    }

    fn pages_count(&self) -> i32 { self.installer.pages_count() }
    fn page(&self) -> Option<&InstallerPage> { self.installer.pages().get(self.page_index as usize) }
}

impl eframe::App for InstallerGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // TODO(clovis): refactor InstallerPage
        let title = if let Some(page) = self.page() {
            page.title()
        } else {
          "Empty Page"
        };

        let pf = page_frame::PageFrame::new(
            title,
            self.page_index + 1,
            self.pages_count(),
            self.lang
        );
        let response = pf.show(ctx, |ui| {
            let page_text = if let Some(page) = self.page() {
                page.text().unwrap_or(String::from("Empty Page. No text here."))
            } else {
                String::from("Empty Page. No text here.")
            };

            let label = egui::Label::new(page_text);
            ui.add_sized(ui.available_size(), label);
        });

        if let Some(l) = response.lang { self.lang = l; }
        if response.next_clicked { self.next_page(); }
        if response.back_clicked { self.prev_page(); }
        if response.quit_clicked { ctx.send_viewport_cmd(egui::ViewportCommand::Close); }
    }
}
