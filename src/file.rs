use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

pub fn read(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(why) => Err(why),
    }
}

