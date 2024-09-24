use {
    std::collections::VecDeque,
    radsort::sort_by_key,
    crate::{utilities::misc::errors::error_handling, Column, ErrFormat}
};

pub fn score_sorting(score_vector: Vec<String>) -> Result<Vec<String>,ErrFormat> {
    //let error_code = 101;
    let score_columns_count: usize = score_vector.iter().count()/3; 
    let mut attempts: Vec<String> = Vec::new();
    let mut columns: Vec<Column> = Vec::new();
    let mut sorted_scores: Vec<String> = Vec::new();
    let mut score_vector: VecDeque<String> =  VecDeque::from(score_vector);

    let mut scores_by_columns = loop {

        attempts.push(match score_vector.remove(1) {
            Some(success) => success,
            None => break columns
        });
        let mut attempts = VecDeque::from(attempts.clone());
        
        columns.push(Column {
            score: {
                match attempts_pop_front(score_vector){
                    Ok(success) => {
                        score_vector = success.0;
                        success.1
                    },
                    Err(_) => break columns
                }
            }, 
            number_attempts: {
                match attempts_pop_front(attempts) {
                    Ok(success) => {
                        attempts = success.0; 
                        success.1
                    },
                    Err(_) => break columns
                }
            },
            max_attempts: {
                match attempts_pop_front(attempts) {
                    Ok(success) => success.1,
                    Err(_) => break columns
                }
            },
            name: {
                match score_vector.pop_front() {
                    Some(success) => success,
                    None => break columns
                }
            }
        })
    };

    sort_by_key(&mut scores_by_columns, |column: &Column| (
        column.score, 
        column.number_attempts, 
        column.max_attempts
    ));

    let mut scores_by_columns = VecDeque::from(scores_by_columns);

    for _ in 0.. score_columns_count {
        let score_rowe = match scores_by_columns.pop_front() {
            Some(success) => success,
            None => break
        };
        sorted_scores.push(score_rowe.score.to_string());
        sorted_scores.push(score_rowe.number_attempts.to_string());
        sorted_scores.push(score_rowe.max_attempts.to_string());
        sorted_scores.push(score_rowe.name);
    }

    return Ok(sorted_scores);
}

fn attempts_pop_front(mut attempts: VecDeque<String>) -> Result<(VecDeque<String>,u32),ErrFormat> {
    let error_code: u8 = 101;
    match attempts.pop_front(){
    Some(success) => {
        match success.parse::<u32>() {
            Ok(success) => Ok((attempts,success)),
            Err(_) => return Err(error_handling(error_code))
        }
    },
    None => return Err(error_handling(error_code))
}}