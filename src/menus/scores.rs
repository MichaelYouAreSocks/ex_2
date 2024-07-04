use crate::{
    utilities::{
        cls_scr::cls_title, inputs::yes_no_else_input, score_board::score_layout::score_layout,
    },
    Comunication, ErrFormat,
};

pub fn show_score_board(high_scores: &Vec<String>) -> Result<(), ErrFormat> {
    let comunication = Comunication {
        msg: match score_layout(&high_scores) {
            Ok(success) => success,
            Err(error) => return Err(error),
        },
        user_in_alpha: String::new(),
        user_in_u32: 0,
    };

    let wrong: bool = false;

    match yes_no_else_input(&comunication, &wrong) {
        _ => {
            cls_title();
            return Ok(());
        }
    };
}
