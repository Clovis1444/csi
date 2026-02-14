use super::core;

pub fn installer_from_file(f_path: &str) -> Result<core::Installer, Box<dyn std::error::Error>> {
    let f_content: String = read_file(f_path)?;
    // let f_toml = f_content.parse::<toml::Table>().unwrap();

    let installer = toml::from_str::<core::Installer>(&f_content)?;

    return Ok(installer);
}

fn read_file(f_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let f_path = std::fs::canonicalize(f_path)?;
    return Ok(std::fs::read_to_string(&f_path)?);
}
