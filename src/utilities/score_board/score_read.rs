use std::str::Lines;

pub fn score_importer(score_raw: &str) -> Vec<String> {
    let mut tmp: Vec<&str>;
    let mut result: String;
    let mut last_96_ranks: String;
    let mut high_score: Vec<String> = vec![];
    let mut score_as_lines: Lines = score_raw.lines();
    let score_line_count: usize = score_raw.lines().count();

    score_as_lines.nth(4);

    for lines_counted in 0..score_line_count {
        last_96_ranks = (lines_counted + 1).to_string() + "th";

        if let Some(line) = score_as_lines.next() {
            tmp = line.split('\t').collect();
            
            for column in 0..tmp.iter().count() {
                result = match tmp[column] {
                    "" | "1st" | "2nd" | "3rd" | "|" => continue,
                    msg if msg == last_96_ranks => continue,
                    _ => tmp[column],
                }
                .to_string();
                high_score.push(result);
            }
        }
    }
    high_score
}