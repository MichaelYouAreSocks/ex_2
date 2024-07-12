use crate::{utilities::misc::errors::error_handling, ErrFormat};

pub fn score_game_layout(high_scores: &Vec<String>) -> Result<String, ErrFormat> {
    let mut score_layout: String;
    let mut score_rank: usize = 0;
    let mut layout: String = format!(
        "\n{}{}{}",
        "High Scores:\n",
        "------------\n",
        "Rank\tSearch\tAtempts\tName"
    );
    while score_rank < high_scores.iter().count() / 3 {
        let column_num = score_rank * 3;
        let rank_plus_1 = score_rank + 1;
        score_layout = format!(
            "\n{}:\t{}{}{}{}{}{}",
            match score_rank {
                0 => format!("{}st ", rank_plus_1),
                1 => format!("{}nd ", rank_plus_1),
                2 => format!("{}rd ", rank_plus_1),
                3..=8 => format!("{}th ", rank_plus_1),
                9..=99 => format!("{}th", rank_plus_1),
                _ => return Err(error_handling(31)),
            },
            high_scores[column_num].to_string(),
            "\t",
            high_scores[column_num + 1].to_string(),
            "\t",
            high_scores[column_num + 2].to_string(),
            ""
        );
        layout.push_str(score_layout.as_str());
        score_rank = rank_plus_1
    }
    Ok(layout)
}

pub fn score_file_layout(high_scores: &Vec<String>) -> Result<String, ErrFormat> {
    let mut score_layout: String;
    let mut score_rank: usize = 0;
    let mut layout: String = format!(
        "{}\n{}\n{}\n{}\n{}",
        "This file contains the score board for :",
        "the Number Guessing Game.",
        "_____________________________________________",
        "\tRank\t|\tSize\t|\tAtempts\t|\tName",
        "------------+-----------+-----------+--------"
    );
    while score_rank < high_scores.iter().count() / 3 {
        let column_num = score_rank * 3;
        score_layout = format!(
            "\n{}{}{}{}",
            match ranking(score_rank) {
                Ok(success) => success,
                Err(error) => return Err(error)
            },
            score_spacing(high_scores, column_num),
            score_spacing(high_scores, column_num + 1),
            score_spacing(high_scores, column_num + 2),
        );
        layout.push_str(score_layout.as_str());
        score_rank = score_rank + 1
    }
    Ok(layout)
}

fn ranking(score_rank: usize) -> Result<String, ErrFormat> {
    let score_rank = score_rank + 1;
    Ok(
        match score_rank {
            1 => format!("\t1st\t\t"),
            2 => format!("\t2nd\t\t"),
            3 => format!("\t3rd\t\t"),
            4..=9 => format!("\t{score_rank}th\t\t"),
            10..=99999 => format!("\t{score_rank}th\t"),
            100000..=999999 => format!("\t{score_rank}th"),
            1000000..=99999999 => format!("  {score_rank}th"),
            _ => return Err(error_handling(31)),
        }
    )
}

fn score_spacing(high_scores: &Vec<String>, column_num: usize) -> String {
    let mut msg: String = String::from("\t");
    msg.push_str(high_scores[column_num].as_str());
    msg.push_str(
        format!(
            "{}", 
            match high_scores[column_num].chars().count() {
                0..=3 => "\t\t",
                4..=7 => "\t",
                _ => ""

            }
        ).as_str()
    );

    return msg;
}
