use std::path::PathBuf;
use std::error::Error;

pub fn path_from_str(path: &str) -> Result<PathBuf, Box<dyn Error>> {
    let path = shellexpand::full(path)?;
    let path_buf = PathBuf::from(path.as_ref()).canonicalize()?;

    return Ok(path_buf);
}

// pub fn is_file_valid(path: &str) -> bool {
//     let path_buf = match path_from_str(path){
//         Ok(val) => { val },
//         Err(_) => { return false; },
//     };

//     return path_buf.is_file();
// }

// pub fn is_dir_valid(path: &str) -> bool {
//     let path_buf = match path_from_str(path){
//         Ok(val) => { val },
//         Err(_) => { return false; },
//     };

//     return path_buf.is_dir();
// }
