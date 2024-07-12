use {
    super::{
        score_defaults::default_scores, score_edit::save_score_to_file, score_read::score_importer,
    },
    crate::{
        utilities::{file::open_file::load_existing_file, misc::errors::error_handling},
        ErrFormat, RuntimeFunctionBlob,
    },
    std::io::read_to_string,
};

pub fn score_file(
    RuntimeFunctionBlob {
        core_functions,
        comunication: _,
        settings,
    }: &RuntimeFunctionBlob,
) -> Result<Vec<String>, ErrFormat> {
    let error_code = 011;

    let score_raw: String;

    match load_existing_file(&core_functions.score_file_path, error_code) {
        Ok(scores_file) => {
            score_raw = match read_to_string(scores_file) {
                Ok(success) => success,
                Err(_) => return Err(error_handling(011)),
            };
            let high_scores = score_importer(&score_raw.as_str());
            for _ in settings.min_score..=settings.max_score {}
            println!("Scores loaded from file.");
            Ok(high_scores)
        }
        Err(_) => {
            match save_score_to_file(core_functions, &default_scores()) {
                Ok(_) => {
                    println!("Default score file created.");
                    return Ok(default_scores());
                }
                Err(error) => return Err(error),
            };
        }
    }
}
