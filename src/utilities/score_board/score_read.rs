use std::str::Lines;

use crate::ErrFormat;

pub fn score_importer(score_raw: &str) -> Result<Vec<String>, ErrFormat> {
    let mut tmp: Vec<&str>;
    let mut high_score: Vec<String> = vec![];
    let mut result: String;
    let mut last_96_ranks: String;
    let mut score_as_lines: Lines = score_raw.lines();
    let score_line_count: usize = score_raw.lines().count();
    score_as_lines.nth(2);

    for lines_counted in 0..score_line_count {
        if let Some(line) = score_as_lines.next() {
            tmp = line.split('\t').collect();
            for column in 0..tmp.iter().count() {
                last_96_ranks = (lines_counted + 1).to_string() + "th :";
                result = match tmp[column] {
                    "" | "1st :" | "2nd :" | "3rd :" => continue,
                    msg if msg == last_96_ranks => continue,
                    _ => tmp[column],
                }
                .to_string();
                high_score.push(result);
            }
        }
    }
    Ok(high_score)
}
