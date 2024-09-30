use {
    crate::{utilities::misc::errors::error_handling, ErrFormat, Row},
    radsort::sort_by_key,
    std::collections::VecDeque,
};

//The "score_sorting" var will sort the score bord from largest search range, smallest max guess attempts,
//and smallest amount of guesses in order of priority.
pub fn score_sorting(score_vector: &Vec<String>) -> Result<Vec<String>, ErrFormat> {
    let error_code: u8 = 101;
    let mut rows: Vec<Row> = Vec::new();
    let mut sorted_scores: Vec<String> = Vec::new();
    let mut score_vector_deque: VecDeque<String> = VecDeque::from(score_vector.clone());

    //This loop converts the "score_vector" var into a format accepted by the "radsort" crate.
    let mut scores_by_row = loop {
        //Removes the second element of the "score_vector_deque" into "attempts_whole" for later usage.
        let attempts_whole: String = match score_vector_deque.remove(1) {
            Some(success) => success,
            None => break rows,
        };
        //Splits the "attempts_whole" var into the "attempts" vecdeque for later usage.
        let mut attempts: VecDeque<&str> = attempts_whole.split('/').collect();

        //Concatination of each score element into a "Row" struct and pushes it onto the "rows" vec.
        rows.push(Row {
            score: {
                if let Some(success) = score_vector_deque.pop_front() {
                    match success.parse::<u32>() {
                        Ok(success) => success as i64,
                        Err(_) => break rows,
                    }
                } else {
                    return Err(error_handling(error_code));
                }
            },
            number_attempts: {
                match attempts_pop_front(attempts.pop_front(), error_code) {
                    Ok(success) => success,
                    Err(error) => return Err(error),
                }
            },
            max_attempts: {
                match attempts_pop_front(attempts.pop_front(), error_code) {
                    Ok(success) => success,
                    Err(error) => return Err(error),
                }
            },
            name: {
                match score_vector_deque.pop_front() {
                    Some(success) => success,
                    None => break rows,
                }
            },
        })
    };

    //The "radsort" crate is used here as it uses an efficient algorithm for our application.
    //Radix sort operates in "O(n*w)" where "n" is the number of keys, and "w" is the key length.
    //The "rows.score" key is set as negative as to make the algorithm sort the elements in decreasing order.
    sort_by_key(&mut scores_by_row, |rows: &Row| {
        (-rows.score, rows.max_attempts, rows.number_attempts)
    });

    //Converison of the "scores_by_row" vec into a vecdeque for easier processing.
    let mut scores_by_row = VecDeque::from(scores_by_row);

    //Converts the scores back to the format used by the rest of the program.
    for _ in 0..score_vector.len() / 3 {
        let score_raw = match scores_by_row.pop_front() {
            Some(success) => success,
            None => break,
        };
        sorted_scores.push(score_raw.score.to_string());
        sorted_scores.push(format!(
            "{}/{}",
            score_raw.number_attempts, score_raw.max_attempts
        ));
        sorted_scores.push(score_raw.name);
    }

    return Ok(sorted_scores);
}

//This funcion is to concatinate identical code used to read the attempts values.
fn attempts_pop_front(attempts: Option<&str>, error_code: u8) -> Result<u32, ErrFormat> {
    match attempts {
        Some(success) => match success.parse::<u32>() {
            Ok(success) => Ok(success),
            Err(_) => return Err(error_handling(error_code)),
        },
        None => return Err(error_handling(error_code)),
    }
}
