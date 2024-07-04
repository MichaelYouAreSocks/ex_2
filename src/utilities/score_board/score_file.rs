use std::io::read_to_string;

use crate::{
    utilities::{
        errors::error_handling,
        file::open_file::open_and_read_existing_file,
    },
    ErrFormat, RuntimeFunctionBlob,
};

use super::{
    score_defaults::default_scores, score_read::score_importer,
};

pub fn score_file(runtime_blob: &RuntimeFunctionBlob) -> Result<Vec<String>, ErrFormat> {
    let RuntimeFunctionBlob {
        core_functions,
        comunication: _,
        settings,
    } = &runtime_blob;

    let score_raw: String;
    let mut high_scores: Vec<String> = default_scores();

    match open_and_read_existing_file(&core_functions.score_file_path) {
        Ok(scores_file) => {
            score_raw = match read_to_string(scores_file) {
                Ok(score_raw) => score_raw,
                Err(_) => return Err(error_handling(021)),
            };
            high_scores = match score_importer(&score_raw, runtime_blob) {
                Ok(success) => success,
                Err(error) => return Err(error),
            };
            for _ in settings.min_score..=settings.max_score {}
            Ok(high_scores)
        }

        Err(_) => {
            return Err(error_handling(011));
        }
    }
}
