// Hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod csi_app;

use std::process::ExitCode;

fn main() -> ExitCode {
    let mut settings = csi::settings::Settings::default();
    let mut app = csi_app::CsiApp::new(&mut settings);

    let f_path = "samples/csi_config.toml";
    match app.load_installer(f_path) {
        Ok(_) => {},
        Err(e) => {
            println!("Failed to load installer: {e}");
            return ExitCode::FAILURE;
        }
    }

    let result = match app.run_gui() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            println!("GUI exited with error: {e}");
            ExitCode::FAILURE
        },
    };

    return result;
}
