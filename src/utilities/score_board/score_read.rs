use crate::{ErrFormat, RuntimeFunctionBlob};

use super::score_defaults::default_scores;

pub fn score_importer(
    score_raw: &String,
    runtime_blob: &RuntimeFunctionBlob,
) -> Result<Vec<String>, ErrFormat> {
    let RuntimeFunctionBlob {
        comunication: _,
        core_functions,
        settings: _,
    } = runtime_blob;

    let mut high_scores: Vec<String> = default_scores();

    let mut scores_as_lines = score_raw.lines();

    for _ in 0..9 {
        //
        let imported_score = match scores_as_lines.next() {
            Some(tmp) => tmp,
            None => break,
        };
        high_scores.push(imported_score.to_string());
    }

    Ok(high_scores)
}
