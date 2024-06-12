use std::io::read_to_string;

use crate::{utilities::{errors::read_err, settings::file::open::open_and_read_existing_file}, ErrFormat, RuntimeFunctionBlob};

pub fn read_score_file(runtime_blob: &RuntimeFunctionBlob) -> Result<Vec<String>, ErrFormat> {
    
    let scores_raw: String;

    let mut top_scores: Vec<String> = vec![String::new(); 10];
    match open_and_read_existing_file(&runtime_blob.core_functions.score_board_path) {
        Ok(scores_file) => {
            scores_raw = match read_to_string(scores_file) {
                Ok(tmp) => tmp,
                Err(_) => return Err(read_err())
            }
        },
            
        Err(_) => return Err(read_err()),
    };

    let mut scores_as_lines = scores_raw.lines();

    for _ in 0..9 {
        //
        let imported_score = match scores_as_lines.next() {
            Some(tmp) => tmp,
            None => break,
        };
        top_scores.push(imported_score.to_string()) ;
    };

    Ok(top_scores)
}