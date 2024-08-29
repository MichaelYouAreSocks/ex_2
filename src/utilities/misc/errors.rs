use crate::ErrFormat;

pub fn error_handling(error_code: u8) -> ErrFormat {
    match error_code / 10 {
        1 => ErrFormat {
            code: error_code,
            name: "Reading File".to_string(),
            msg: format!(
                "The '{} file' could not be read.\n{}\n{}",
                match error_code {
                    10 => "Settings",
                    11 => "Score",
                    _ => todo!(),
                },
                "If the file is being automativally removed by your anti-virus,",
                "pleade add an exception to it for the game to work."
            ),
        },
        2 => ErrFormat {
            code: error_code,
            name: "Writing File".to_string(),
            msg: format!(
                "The '{} file' couldn't be created or modified.\n{}",
                match error_code {
                    20 => "Settings",
                    21 => "Score",
                    _ => todo!(),
                },
                "If the game isn't in a writable directory, please move it."
            ),
        },
        10 => ErrFormat {
            code: error_code,
            name: "Invalid file structure".to_string(),
            msg: format!(
                "The '{}' file has an unsuported layout.\n{}",
                match error_code {
                    100 => "Settings",
                    101 => "Score",
                    _ => todo!(),
                },
                "Please rename the file and regenerate the default one by relaunching the game."
            ),
        },
        21 => match error_code {
            210 => ErrFormat {
                code: error_code,
                name: "Max_range".to_string(),
                msg: "a number from 1 to 4'294'967'295".to_string(),
            },
            211 => ErrFormat {
                code: error_code,
                name: "Min_range".to_string(),
                msg: "a number from 0 to 4'294'967'294".to_string(),
            },
            212 => ErrFormat {
                code: error_code,
                name: "Max_tries".to_string(),
                msg: "a number from 1 to 4'294'967'295".to_string(),
            },
            213 => ErrFormat {
                code: error_code,
                name: "Min_tries".to_string(),
                msg: "a number from 1 to 4'294'967'295".to_string(),
            },
            _ => error_handling(255),
        },
        _ => ErrFormat {
            code: 255,
            name: "New Error!".to_string(),
            msg: format!(
                "This error has never been seen before!\n{}\n{}\n{}\n{}",
                "If you see this message,",
                "Congrats! You managed to break the game in a way I didn't forsee as possible.",
                "Please open an issue on github with a screenshot of this message,",
                "and a bref description of what you were doing to know what to do next."
            ),
        },
    }
}

pub fn err_print(error: &ErrFormat) {
    println!(
        "\nError code : \n{}\n\nError :\n{}\n\nProbable cause : \n{}",
        error.code, error.name, error.msg,
    );
}
