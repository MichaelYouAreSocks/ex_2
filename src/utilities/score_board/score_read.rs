use {
    crate::{utilities::errors::error_handling, ErrFormat},
    std::str::Lines,
};

pub fn score_importer(score_raw: String) -> Result<Vec<String>, ErrFormat> {
    let mut high_score: Vec<String> = Vec::new();
    let mut imported_scores: usize = 0;
    let mut score_as_lines: Lines = score_raw.lines();
    let score_line_count: u8 = score_raw.lines().count() as u8;

    for lines_searched in 0..=score_line_count - 1 {
        let individual_score: &str = match score_as_lines.next() {
            Some(tmp) => tmp,
            None => break,
        };
        match imported_scores {
            0..=100 => {
                high_score[imported_scores].push_str(individual_score);
                imported_scores = imported_scores + 1;

                if lines_searched > score_line_count {
                    return Err(error_handling(101));
                } else {
                    error_handling(101)
                }
            }
            _ => error_handling(101),
        };
    }

    Ok(high_score)
}
