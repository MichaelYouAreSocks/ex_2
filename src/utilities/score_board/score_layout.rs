use crate::{utilities::misc::errors::error_handling, ErrFormat};

//Formats the scores to be shown ingame and sends it back to the source function.
pub fn score_game_layout(high_scores: &[String]) -> Result<String, ErrFormat> {
    let max_scores_shown: usize = 20;
    let mut score_layout: String;
    let mut score_rank: usize = 0;
    let mut layout: String = format!(
        "\n{}{}{}",
        "High Scores:\n", "------------\n", "Rank\tSearch\tAtempts\tName"
    );

    while score_rank < high_scores.len() / 3 && score_rank < max_scores_shown {
        let column_num: usize = score_rank * 3;
        let rank_plus_1: usize = score_rank + 1;
        score_layout = format!(
            "\n{}:\t{}{}{}{}{}{}",
            match score_rank {
                0 => format!("{}st ", rank_plus_1),
                1 => format!("{}nd ", rank_plus_1),
                2 => format!("{}rd ", rank_plus_1),
                3..=8 => format!("{}th ", rank_plus_1),
                9..=99 => format!("{}th", rank_plus_1),
                100..=4294967295 => format!("{}th", rank_plus_1),
                _ => return Err(error_handling(101)),
            },
            high_scores[column_num],
            "\t",
            high_scores[column_num + 1],
            "\t",
            high_scores[column_num + 2],
            ""
        );
        layout.push_str(score_layout.as_str());
        score_rank = rank_plus_1
    }

    Ok(layout)
}

//Formats the scores to be printed in a file and sends it back to the source function.
pub fn score_file_layout(high_scores: &[String]) -> Result<String, ErrFormat> {
    let mut score_rank: usize = 1;
    let amount_scores: usize = high_scores.len() / 3;
    let mut layout: String = format!(
        "{}\n{}\n{}\n{}\n{}",
        "This file contains the score board for :",
        "the Number Guessing Game.",
        "_________________________________________________",
        "\tRank\t|\tSize\t|\tAtempts\t|\tName\t|",
        "------------+-----------+-----------+-----------+"
    );

    while score_rank < amount_scores + 1 {
        layout += format!(
            "\n\t{}|",
            match score_rank {
                1 => "1st\t\t".to_string(),
                2 => "2nd\t\t".to_string(),
                3 => "3rd\t\t".to_string(),
                4..=9 => format!("{score_rank}th\t\t"),
                10..=99999 => format!("{score_rank}th\t"),
                100000..=999999 => format!("{score_rank}th"),
                1000000..=99999999 => format!("  {score_rank}th"),
                _ => return Err(error_handling(101)),
            }
            .as_str(),
        )
        .as_str();

        for row_num in (score_rank * 3 - 3)..=(score_rank * 3 - 1) {
            layout += format!("\t{}", high_scores[row_num].as_str()).as_str();
            layout += match high_scores[row_num].chars().count() {
                0..=3 => "\t\t|",
                4..=7 => "\t|",
                _ => "|",
            }
        }
        score_rank += 1;
    }

    Ok(layout)
}
