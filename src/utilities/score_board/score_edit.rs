use crate::{
    utilities::{errors::error_handling, score_board::score_layout::score_layout},
    RuntimeFunctionBlob,
};
use std::fs::write;

pub fn save_score_to_file(
    mut runtime_blob: RuntimeFunctionBlob,
    high_scores: &Vec<String>,
) -> RuntimeFunctionBlob {
    match write(
        &runtime_blob.core_functions.score_file_path,
        match score_layout(high_scores) {
            Ok(success) => success,
            Err(_) => {
                runtime_blob.core_functions.error_handler = error_handling(020);
                runtime_blob.core_functions.stop = true;
                return runtime_blob;
            }
        },
    ) {
        Ok(_) => runtime_blob,
        Err(_) => {
            runtime_blob.core_functions.error_handler = error_handling(020);
            runtime_blob.core_functions.stop = true;
            return runtime_blob;
        }
    }
}
