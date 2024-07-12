use {
    crate::{
        utilities::{misc::errors::error_handling, score_board::score_layout::score_file_layout},
        CoreFunctions, ErrFormat,
    },
    std::fs::write,
};

pub fn save_score_to_file(
    core_functions: &CoreFunctions,
    high_scores: &Vec<String>,
) -> Result<String, ErrFormat> {
    match write(
        &core_functions.score_file_path,
        match score_file_layout(high_scores) {
            Ok(success) => success,
            Err(error) => return Err(error),
        },
    ) {
        Ok(_) => Ok(format!("Default high scores were loaded.")),
        Err(_) => Err(error_handling(021)),
    }
}
