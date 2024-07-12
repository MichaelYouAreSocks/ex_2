use crate::{
    game::game,
    menus::{options::options_menu, scores::show_score_board},
    utilities::{
        misc::{cls_scr::cls_title, inputs::yes_no_else_input},
        start_menu::start_layout::start_menu_layout,
    },
    RuntimeFunctionBlob,
};

pub fn main_menu(
    mut runtime_blob: RuntimeFunctionBlob,
    high_scores: Vec<String>,
    mut wrong: bool,
    mut end_game_msg: String,
) -> (RuntimeFunctionBlob, Vec<String>, bool, String) {
    runtime_blob.comunication.msg = start_menu_layout(&runtime_blob, &high_scores);

    runtime_blob.comunication.user_in_alpha = yes_no_else_input(&runtime_blob.comunication, &wrong);

    match runtime_blob.comunication.user_in_alpha.as_str() {
        "n" | "N" | "q" | "Q" | "0" => {
            cls_title();
            if &runtime_blob.core_functions.first_cycle != &true {
                println!("Thanks for playing!");
            } else {
                println!("Hope you'll play soon!")
            };
            runtime_blob.core_functions.stop = true;
            return (runtime_blob, high_scores, wrong, end_game_msg);
        }
        "y" | "Y" | "" | "1" => {
            cls_title();
            runtime_blob = game(runtime_blob);
            runtime_blob.core_functions.first_cycle = match runtime_blob.comunication.msg.as_str() {
                "" => true,
                _ => {
                    end_game_msg = runtime_blob.comunication.msg.to_owned();
                    return (runtime_blob, high_scores, wrong, end_game_msg);
                }
            }
        }
        "o" | "O" | "s" | "S" | "2" => {
            cls_title();
            runtime_blob = options_menu(runtime_blob);
            runtime_blob.comunication.msg = end_game_msg.to_owned();
        }
        "b" | "B" | "r" | "R" | "3" => {
            cls_title();
            match show_score_board(&high_scores) {
                Ok(()) => {
                    runtime_blob.comunication.msg = end_game_msg.to_owned();
                }
                Err(error) => {
                    runtime_blob.core_functions.stop = true;
                    runtime_blob.core_functions.error_handler = error;
                    return (runtime_blob, high_scores, wrong, end_game_msg);
                }
            }
        }
        _ => {
            cls_title();
            wrong = true
        }
    }
    return (runtime_blob, high_scores, wrong, end_game_msg);
}
