use crate::{utilities::errors::error_handling, ErrFormat};

pub fn score_layout(high_scores: &Vec<String>) -> Result<String, ErrFormat> {
    let mut score_layout: String;
    let mut score_rank: usize = 0;
    let rank_column: usize = 0;
    let mut layout: String = format!(
        "{}{}{}",
        "This file contains the high-score board for the Number Guessing Game.\n",
        "--------------------------------------------------------------\n\n",
        "\tRank\tSize\tAtempts\tName\n"
    );
    while score_rank < 10 {
        let score_rank_plus_1 = score_rank + 1;
        score_layout = format!(
            "\n{}:\t{}{}{}{}{}{}",
            match score_rank {
                0 => format!("\t{}st ", score_rank_plus_1),
                1 => format!("\t{}nd ", score_rank_plus_1),
                2 => format!("\t{}rd ", score_rank_plus_1),
                3..=8 => format!("\t{}th ", score_rank_plus_1),
                9..=99 => format!("\t{}th", score_rank_plus_1),
                _ => return Err(error_handling(31)),
            },
            high_scores[score_rank * 3].to_string(),
            match tabulation_counter(high_scores[score_rank * 3].to_string().chars().count()) {
                Ok(success) => success,
                Err(error) => return Err(error),
            },
            high_scores[score_rank * 3 + 1].to_string(),
            match tabulation_counter(high_scores[score_rank + 1].to_string().chars().count()) {
                Ok(success) => success,
                Err(error) => return Err(error),
            },
            high_scores[rank_column * 3 + 2].to_string(),
            ""
        );
        layout.push_str(score_layout.as_str());
        score_rank = score_rank_plus_1
    }
    Ok(layout)
}

fn tabulation_counter(size: usize) -> Result<String, ErrFormat> {
    return Ok(match size / 4 {
        0 => "",
        1 => "\t",
        2 => "\t\t",
        3 => "\t\t\t",
        _ => return Err(error_handling(31)),
    }
    .to_string());
}

pub fn score_layout2(high_scores: &Vec<String>) -> Result<String, ErrFormat> {
    let mut score_layout: String;
    let mut score_rank: usize = 0;
    let mut layout = format!(
        "{}{}{}",
        "-This file contains the high-score board for the Number Guessing Game.",
        "----------------------------------------------------------------------",
        "\tRank\tSize\tAtempts\tName",
    );
    while score_rank <= 9 {
        //let score_rank_plus_1 = score_rank + 1;
        score_layout = format!(
            "\t{}st\t{}\t{}\t{}",
            score_rank,
            high_scores[score_rank].to_string().chars().count(),
            match tabulation_counter(high_scores[score_rank].to_string().chars().count()) {
                Ok(success) => success,
                Err(error) => return Err(error),
            },
            match tabulation_counter(high_scores[score_rank + 1].to_string().chars().count()) {
                Ok(success) => success,
                Err(error) => return Err(error),
            },
            //high_scores[score_rank + 1].to_string(),
        );
        layout.push_str(score_layout.as_str());
        score_rank = score_rank + 1;
    }
    layout.push_str(&format!(
        "\t\t{}\t{}\t{}\t{}",
        "10",
        high_scores[9].to_string().chars().count(),
        match tabulation_counter(high_scores[9].to_string().chars().count()) {
            Ok(success) => success,
            Err(error) => return Err(error),
        },
        match tabulation_counter(high_scores[10].to_string().chars().count()) {
            Ok(success) => success,
            Err(error) => return Err(error),
        },
        //high_scores[10].to_string(),
    ));
    Ok(layout)
}
