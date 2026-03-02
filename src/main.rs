// Hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::ExitCode;

fn main() -> ExitCode {
    // let f_path = "samples/csi_config.toml";
    // let installer: csi::core::Installer = match csi::parser::installer_from_file(f_path) {
    //     Ok(v) => v,
    //     Err(e) => {
    //         println!("Failed to create installer: {e}");
    //         return ExitCode::FAILURE;
    //     }
    // };
    // println!("{installer:#?}");

    csi::gui::hello_egui();

    return ExitCode::SUCCESS;
}
