mod page_frame;
mod page;

use std::collections::HashMap;
use eframe::egui::{self, Style, Theme, Vec2};

use crate::core::{Installer, InstallerPage};
use crate::settings::Settings;

use page::GuiPage;

pub struct InstallerGui<'a> {
    installer: &'a mut Installer,
    settings: &'a Settings,
    // Note: 0 based indexing
    page_index: i32,
    lang: page_frame::Language,
    allow_next: bool,
    p_responses: HashMap<i32, page::PageResponse>,
}

impl<'a> InstallerGui<'a> {
    pub fn run(settings: &'a Settings, installer: &'a mut Installer) -> Result<(), eframe::Error> {
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
            Box::new(|cc| Ok(Box::new(InstallerGui::new(cc, settings, installer)))),
        );
    }

    fn new(cc: &eframe::CreationContext<'_>, settings: &'a Settings, installer: &'a mut Installer) -> Self {
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
            settings: settings,
            page_index: 0,
            lang: page_frame::Language::English,
            allow_next: false,
            p_responses: HashMap::new(),
        }
    }

    fn handle_responses(&mut self,
        ctx: &egui::Context,
        pf_res: page_frame::PageFrameResponse,
        p_res: Option<page::PageResponse>,
    ) {
        // Page response handling
        match p_res {
            Some(res) => {
                self.allow_next = res.allow_next;
                self.p_responses.insert(self.page_index, res);
            },
            None => { self.allow_next = true; },
        }

        // PageFrame response handling
        if let Some(l) = pf_res.lang { self.lang = l; }
        if pf_res.next_clicked { self.next_page(); }
        if pf_res.back_clicked { self.prev_page(); }
        if pf_res.quit_clicked { ctx.send_viewport_cmd(egui::ViewportCommand::Close); }
    }

    fn next_page(&mut self) {
        let old_i = self.page_index;
        self.page_index = std::cmp::max(0, std::cmp::min(self.pages_count() - 1, self.page_index + 1));

        if old_i != self.page_index { self.allow_next = false; }
    }
    fn prev_page(&mut self) {
        let old_i = self.page_index;
        self.page_index = std::cmp::max(0, self.page_index - 1);

        if old_i != self.page_index { self.allow_next = false; }
    }

    fn pages_count(&self) -> i32 { self.installer.pages_count() }
    fn page(&self) -> Option<&InstallerPage> { self.installer.pages().get(self.page_index as usize) }
    fn last_page_res(&self) -> Option<&page::PageResponse> { self.p_responses.get(&self.page_index) }
}

impl<'a> eframe::App for InstallerGui<'a> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let title = if let Some(page) = self.page() {
            page.title()
        } else {
          "Empty Page"
        };

        let pf = page_frame::PageFrame::new(
            self.settings,
            title,
            self.page_index + 1,
            self.pages_count(),
            self.lang,
            self.allow_next,
        );

        let mut p_res: Option<page::PageResponse> = None;

        let pf_res = pf.show(ctx, |ui| {
            if let Some(page) = self.page() {
                match page.gui_page(ui, self.settings, self.last_page_res()) {
                    Ok(res) => { p_res = Some(res); },
                    Err(e) => { ui.label(e.to_string()); },
                }
            } else {
                ui.label("Empty Page. No text here.");
            };
        });

        self.handle_responses(ctx, pf_res, p_res);
    }
}
