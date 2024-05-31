use crate::{utilities::errors::write_err, RuntimeFunctionBlob};
use super::default_layout::default_settings_layout;
use std::fs::write;

pub fn save_setting_to_file(mut runtime_blob: RuntimeFunctionBlob) -> RuntimeFunctionBlob {
    match write(
        &runtime_blob.core_functions.settings_file_path, 
        default_settings_layout(&runtime_blob.settings)
    ) {
        Ok(_) => {
            runtime_blob
        },
        Err(_) => {
            runtime_blob.core_functions.error_handler = write_err();
            runtime_blob.core_functions.stop = true;
            runtime_blob
        }
    }
}