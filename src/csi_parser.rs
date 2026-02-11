pub fn read_file(f_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let f_path = std::fs::canonicalize(f_path)?;

    return Ok(std::fs::read_to_string(&f_path)?);
}

pub fn read_file_as_strings(f_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let f_content = read_file(f_path)?;
    return Ok(f_content.lines().map(|s| s.to_string()).collect());
}
