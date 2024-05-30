use std::fs::{File, OpenOptions};

pub fn open_and_read_existing_file(path: &String) -> Result<File, ()> {
    if let Ok(settings_file) = OpenOptions::new().read(true).write(true).open(path) {
        Ok(settings_file)
    } else {
        Err(())
    }
}
