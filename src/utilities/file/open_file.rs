use std::fs::{File, OpenOptions};

use crate::{utilities::misc::errors::error_handling, ErrFormat};

pub fn load_existing_file(path: &String, error_code: u8) -> Result<File, ErrFormat> {
    if let Ok(file) = OpenOptions::new().read(true).write(true).open(path) {
        Ok(file)
    } else {
        Err(error_handling(error_code))
    }
}
