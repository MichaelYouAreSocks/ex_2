use crate::{utilities::errors::error_handling, ErrFormat};

pub fn score_layout(high_scores: &Vec<String>) -> Result<String, ErrFormat> {
    let mut score_layout: String;
    let mut score_rank: usize = 0;
    let mut layout: String = format!(
        "{}\n{}\n\n{}",
        "-This file contains the high-score board for the Number Guessing Game.",
        "--------------------------------------------------------------",
        "\tRank\tSize\tAtempts\tName"
    );
    while score_rank <= 10 {
        let score_rank_plus_1 = score_rank + 1;
        score_layout = format!(
            "\n{}:\t{}{}{}{}{}",
            match score_rank {
                0 => format!("{}st\t", score_rank_plus_1),
                1 => format!("{}nd\t", score_rank_plus_1),
                2 => format!("{}rd\t", score_rank_plus_1),
                3..=9 => format!("{}th\t", score_rank_plus_1),
                10 => format!("{}th", score_rank_plus_1),
                _ => return Err(error_handling(31)),
            },
            high_scores[score_rank].to_string(),
            match tabulation_counter(high_scores[score_rank].to_string().chars().count()) {
                Ok(success) => success,
                Err(error) => return Err(error),
            },
            match tabulation_counter(high_scores[score_rank + 1].to_string().chars().count()) {
                Ok(success) => success,
                Err(error) => return Err(error),
            },
            high_scores[score_rank + 2].to_string(),
            ""
        );
        layout.push_str(score_layout.as_str());
        score_rank = score_rank + 1
    }
    Ok(layout)
}

fn tabulation_counter(size: usize) -> Result<String, ErrFormat> {
    return Ok(match size / 4 {
        0 => "",
        1 => "\t",
        2 => "\t\t",
        3 => "\t\t\t",
        4 => "\t\t\t\t",
        5 => "\t\t\t\t\t",
        6 => "\t\t\t\t\t\t",
        7 => "\t\t\t\t\t\t\t",
        8 => "\t\t\t\t\t\t\t\t",
        9 => "\t\t\t\t\t\t\t\t\t",
        10 => "\t\t\t\t\t\t\t\t\t\t",
        _ => return Err(error_handling(31)),
    }
    .to_string());
}
