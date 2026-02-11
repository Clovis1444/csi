// Hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod csi_parser;

fn main() {
    let f_path = "../samples/test.txt";

    match csi_parser::read_file_as_strings(f_path) {
        Ok(v) => {
            let content: String = v
                .iter()
                .enumerate()
                .map(|(i, val)| format!("{i} {val}\n"))
                .collect();

            println!("{f_path} content:\n{content}")
        }
        Err(e) => {
            println!("Failed to read {f_path}: {e}.")
        }
    };
}
