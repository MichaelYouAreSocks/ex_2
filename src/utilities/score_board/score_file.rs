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
        mut core_functions,
        comunication,
        settings,
    }: RuntimeFunctionBlob,
) -> Result<RuntimeFunctionBlob, ErrFormat> {
    let error_code = 11;

    let score_raw: String;

    match load_existing_file(&core_functions.score_file_path, error_code) {
        Ok(scores_file) => {
            score_raw = match read_to_string(scores_file) {
                Ok(success) => success,
                Err(_) => return Err(error_handling(11)),
            };
            core_functions.high_score = score_importer(score_raw.as_str());
            match save_score_to_file(&core_functions) {
                Ok(success) => print!("{success}"),
                Err(error) => return Err(error),
            };
            for _ in settings.min_score..=settings.max_score {}
            println!("Scores loaded from file.");
            let runtime_blob = RuntimeFunctionBlob {
                core_functions,
                comunication,
                settings,
            };
            Ok(runtime_blob)
        }
        Err(_) => {
            core_functions.high_score = default_scores();
            match save_score_to_file(&core_functions) {
                Ok(_) => {
                    println!("Default score file created.");
                    let runtime_blob = RuntimeFunctionBlob {
                        core_functions,
                        comunication,
                        settings,
                    };
                    Ok(runtime_blob)
                }
                Err(error) => Err(error),
            }
        }
    }
}
