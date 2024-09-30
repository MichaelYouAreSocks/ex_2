use crate::{
    utilities::{
        misc::{cls_scr::cls_title, inputs::yes_no_else_input},
        score_board::{score_file::score_file, score_layout::score_game_layout},
    },
    Comunication, ErrFormat, RuntimeFunctionBlob,
};

pub fn show_score_board(runtime_blob: RuntimeFunctionBlob) -> Result<Vec<String>, ErrFormat> {
    let runtime_blob = match score_file(runtime_blob) {
        Ok(scores) => scores,
        Err(error) => return Err(error),
    };

    let comunication = Comunication {
        msg: match score_game_layout(&runtime_blob.core_functions.high_score) {
            Ok(success) => success,
            Err(error) => return Err(error),
        },
        user_in_alpha: String::new(),
        user_in_u32: 0,
    };
    let wrong: bool = false;
    yes_no_else_input(&comunication, &wrong);
    {
        cls_title();
        Ok(runtime_blob.core_functions.high_score)
    }
}
