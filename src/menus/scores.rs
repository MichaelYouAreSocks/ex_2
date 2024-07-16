use {
    crate::{
        utilities::{
            misc::{cls_scr::cls_title, inputs::yes_no_else_input},
            score_board::{score_file::score_file, score_layout::score_game_layout},
        },
        Comunication, ErrFormat, RuntimeFunctionBlob,
    },
    std::u32::MIN,
};

pub fn show_score_board(runtime_blob: &RuntimeFunctionBlob) -> Result<Vec<String>, ErrFormat> {
    let high_scores = match score_file(runtime_blob) {
        Ok(scores) => scores,
        Err(error) => return Err(error)
    };
    
    let comunication = Comunication {
        msg: match score_game_layout(&high_scores) {
            Ok(success) => success,
            Err(error) => return Err(error),
        },
        user_in_alpha: String::new(),
        user_in_u32: MIN,
    };
    let wrong: bool = false;
    match yes_no_else_input(&comunication, &wrong) {
        _ => {
            cls_title();
            return Ok(high_scores);
        }
    };
}
