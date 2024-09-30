use crate::Settings;

pub fn settings_file_layout(settings: &Settings) -> String {
    let layout: String = format!(
        "{}\n{}\n\n{}{}\n\n{}{}\n\n{}{}\n\n{}{}\n\n{}{}",
        "-This file contains the settings for the Number Guessing Game.",
        "--------------------------------------------------------------",
        "[Max_range] == ",
        settings.max_range,
        "[Min_range] == ",
        settings.min_range,
        "[Max_tries] == ",
        settings.max_tries,
        "[Min_tries] == ",
        settings.min_tries,
        "[Guess_hint] == ",
        settings.guess_hint
    );
    layout
}

pub fn settings_game_layout(settings: &Settings) -> String {
    format!(
        "Options:\n{}{}{}{}\n{}{}\n{}{}\n{}",
        "1 -> Size of search.\tMin: ",
        settings.min_range,
        "\tMax: ",
        settings.max_range,
        "2 -> Number of tries.\t: ",
        settings.max_tries,
        "3 -> Game hints.\t: ",
        settings.guess_hint,
        "0 -> Back to main menu."
    )
}
