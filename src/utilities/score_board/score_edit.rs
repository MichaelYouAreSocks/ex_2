use {
    crate::{
        utilities::{errors::error_handling, score_board::score_layout::score_layout},
        ErrFormat, RuntimeFunctionBlob,
    },
    std::fs::write,
};

pub fn save_score_to_file(
    runtime_blob: &RuntimeFunctionBlob,
    high_scores: &Vec<String>,
) -> Result<String, ErrFormat> {
    match write(
        &runtime_blob.core_functions.score_file_path,
        match score_layout(high_scores) {
            Ok(success) => success,
            Err(_) => return Err(error_handling(021)),
        },
    ) {
        Ok(_) => Ok(format!("Default high scores were loaded.")),
        Err(_) => Err(error_handling(021)),
    }
}
