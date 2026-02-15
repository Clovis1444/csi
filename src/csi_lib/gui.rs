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
        Box::new(|cc| Ok(Box::new(CSIEguiApp::new(cc)))),
    );
}

#[derive(Default)]
struct CSIEguiApp {
    b_click_count: u64,
}

impl CSIEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for CSIEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                // Title
                ui.heading("Hello from egui!");

                ui.add_space(20.0);
                ui.group(|ui| {
                    // Desc
                    ui.label(
                        r#"This is the description of this window.
                It contains some text.
                It is multiline.
                It is implemented as label.
                Total line count of this label is 5."#,
                    );
                });
                ui.add_space(30.0);
            });

            ui.columns(2, |columns| {
                columns[0].with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    let b_name = if self.b_click_count != 0 {
                        format!("Clicked {} times!", self.b_click_count)
                    } else {
                        "Click me!".to_string()
                    };
                    if ui.button(b_name).clicked() {
                        self.b_click_count += 1;
                    };
                });

                columns[1].with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                    if ui.button("Clear count").clicked() {
                        self.b_click_count = 0;
                    };
                });
            });
        });
    }
}
