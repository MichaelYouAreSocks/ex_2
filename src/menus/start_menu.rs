use crate::{
    game::game,
    menus::{options::options_menu, scores::show_score_board},
    utilities::{
        misc::{cls_scr::cls_title, inputs::yes_no_else_input},
        score_board::score_edit::save_score_to_file,
        start_menu::start_layout::start_menu_layout,
    },
    RuntimeFunctionBlob,
};

pub fn main_menu(
    mut runtime_blob: RuntimeFunctionBlob,
    mut wrong: bool,
    mut end_game_msg: String,
) -> (RuntimeFunctionBlob, bool, String) {
    runtime_blob.comunication.msg = start_menu_layout(&runtime_blob);

    runtime_blob.comunication.user_in_alpha = yes_no_else_input(&runtime_blob.comunication, &wrong);

    match runtime_blob.comunication.user_in_alpha.as_str() {
        "n" | "N" | "q" | "Q" | "0" => {
            cls_title();
            if !runtime_blob.core_functions.first_cycle {
                println!("Thanks for playing!");
            } else {
                println!("Hope you'll play soon!")
            };
            runtime_blob.core_functions.stop = true;
            return (runtime_blob, wrong, end_game_msg);
        }
        "y" | "Y" | "" | "1" => {
            cls_title();
            runtime_blob = game(runtime_blob);
            match save_score_to_file(&runtime_blob.core_functions) {
                Ok(success) => println!("{}", success),
                Err(error) => {
                    runtime_blob.core_functions.stop = true;
                    runtime_blob.core_functions.error_handler = error;
                    return (runtime_blob, wrong, end_game_msg);
                }
            };
            runtime_blob.core_functions.first_cycle = match runtime_blob.comunication.msg.as_str() {
                "" => true,
                _ => {
                    runtime_blob.comunication.msg.clone_into(&mut end_game_msg);
                    return (runtime_blob, wrong, end_game_msg);
                }
            }
        }
        "o" | "O" | "s" | "S" | "2" => {
            cls_title();
            runtime_blob = options_menu(runtime_blob);
            end_game_msg.clone_into(&mut runtime_blob.comunication.msg);
        }
        "b" | "B" | "r" | "R" | "3" => {
            cls_title();
            match show_score_board(runtime_blob.clone()) {
                Ok(scores) => {
                    runtime_blob.core_functions.high_score = scores;
                    end_game_msg.clone_into(&mut runtime_blob.comunication.msg);
                }
                Err(error) => {
                    runtime_blob.core_functions.stop = true;
                    runtime_blob.core_functions.error_handler = error;
                    return (runtime_blob, wrong, end_game_msg);
                }
            }
        }
        _ => {
            cls_title();
            wrong = true
        }
    }
    (runtime_blob, wrong, end_game_msg)
}
