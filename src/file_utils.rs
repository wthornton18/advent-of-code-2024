use std::io::Read;
use std::{fs::File, path::Path};

pub fn read_to_string<P: AsRef<Path>>(path: P) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
