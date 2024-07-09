use {
    super::{
        score_defaults::default_scores, score_edit::save_score_to_file, score_read::score_importer,
    },
    crate::{
        utilities::{errors::error_handling, file::open_file::open_and_read_existing_file},
        ErrFormat, RuntimeFunctionBlob,
    },
    std::io::read_to_string,
};

pub fn score_file(runtime_blob: &RuntimeFunctionBlob) -> Result<Vec<String>, ErrFormat> {
    let RuntimeFunctionBlob {
        core_functions,
        comunication: _,
        settings,
    } = &runtime_blob;

    let score_raw: String;

    match open_and_read_existing_file(&core_functions.score_file_path) {
        Ok(scores_file) => {
            score_raw = match read_to_string(scores_file) {
                Ok(success) => success,
                Err(_) => return Err(error_handling(011)),
            };
            let high_scores = match score_importer(&score_raw.as_str()) {
                Ok(success) => success,
                Err(error) => return Err(error),
            };
            for _ in settings.min_score..=settings.max_score {}
            Ok(high_scores)
        }
        Err(_) => {
            match save_score_to_file(runtime_blob, &default_scores()) {
                Ok(_) => {
                    println!("Default high scores were loaded.");
                    return Ok(default_scores());
                }
                Err(error) => return Err(error),
            };
        }
    }
}
