use std::path::PathBuf;
use std::error::Error;

pub fn path_from_str(path: &str) -> Result<PathBuf, Box<dyn Error>> {
    let path = shellexpand::full(path)?;
    let path_buf = PathBuf::from(path.as_ref()).canonicalize()?;

    return Ok(path_buf);
}

pub fn read_file(f_path: &str) -> Result<String, Box<dyn Error>> {
    let f_path = match path_from_str(f_path){
        Ok(v) => v,
        Err(e) => { return Err(format!("{:?}: {}", f_path, e).into()); },
    };
    match std::fs::read_to_string(&f_path) {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("{:?}: {}", f_path, e).into()),
    }
}
