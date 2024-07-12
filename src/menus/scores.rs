use {
    crate::{
        utilities::{
            misc::{cls_scr::cls_title, inputs::yes_no_else_input},
            score_board::score_layout::score_game_layout,
        },
        Comunication, ErrFormat,
    },
    std::u32::MIN,
};

pub fn show_score_board(high_scores: &Vec<String>) -> Result<(), ErrFormat> {
    cls_title();
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
            return Ok(());
        }
    };
}
