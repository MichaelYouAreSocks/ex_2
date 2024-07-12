use crate::{
    utilities::misc::{
        cls_scr::cls_title,
        inputs::{numeric_input, yes_no_else_input},
    },
    RuntimeFunctionBlob,
};

pub fn game_size(mut runtime_blob: RuntimeFunctionBlob) -> RuntimeFunctionBlob {
    runtime_blob.comunication.msg = format!(
        "Input the largest number you want.\nCurrent:\t{}",
        runtime_blob.settings.max_range
    );
    let tmp: u32 = numeric_input(&runtime_blob.comunication.msg);
    if tmp >= runtime_blob.settings.min_range {
        runtime_blob.settings.max_range = tmp;
    };
    cls_title();
    runtime_blob
}

pub fn game_tries(mut runtime_blob: RuntimeFunctionBlob) -> RuntimeFunctionBlob {
    runtime_blob.comunication.msg = format!(
        "How many attempts do you want?\nCurrent:\t{}",
        runtime_blob.settings.max_tries
    );
    let tmp = numeric_input(&runtime_blob.comunication.msg);
    if tmp >= runtime_blob.settings.min_tries {
        runtime_blob.settings.max_tries = tmp;
    };
    cls_title();
    runtime_blob
}

pub fn game_hint(mut runtime_blob: RuntimeFunctionBlob) -> RuntimeFunctionBlob {
    let mut wrong: bool = false;

    runtime_blob.settings.guess_hint = 'guess_hint_menu_loop: loop {
        runtime_blob.comunication.msg = format!(
            "Do you want to get hints while playing? (Y/N)\nCurrent:\t{}",
            runtime_blob.settings.guess_hint
        );

        runtime_blob.comunication.user_in_alpha =
            yes_no_else_input(&runtime_blob.comunication, &wrong);

        match runtime_blob.comunication.user_in_alpha.as_str() {
            "n" | "N" | "0" | "false" | "False" | "f" | "F" => {
                cls_title();
                break 'guess_hint_menu_loop false;
            }
            "y" | "Y" | "1" | "true" | "True" | "t" | "T" => {
                cls_title();
                break 'guess_hint_menu_loop true;
            }
            "" => {
                cls_title();
                runtime_blob.comunication.msg = String::new();
                break 'guess_hint_menu_loop runtime_blob.settings.guess_hint;
            }
            _ => {
                cls_title();
                wrong = true;
            }
        }
    };
    runtime_blob
}
