
pub fn score_board(high_score: Vec<String>, other_scores: Vec<String>) -> String {
    let layout: String = format!(
        "{}\n{}\n\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n",
        "-This file contains the high-score board for the Number Guessing Game.",

        "--------------------------------------------------------------",

        "1st\t:\t",
        settings.max_range,

        "-From what number do you want to guess? [Min_range]",
        settings.min_range,

        "-How many atempts do you want to guess the random number? [Max_tries]",
        settings.max_tries,

        "-How many atempts do you want to at least have? [Min_tries]",
        settings.min_tries,

        "-Do you want hints while you play? [Guess_hint]",
        settings.guess_hint
    );
    return layout
}