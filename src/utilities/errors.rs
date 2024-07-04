use crate::ErrFormat;

pub fn error_handling(error_code: u8) -> ErrFormat {
    match error_code / 10 {
        1 => ErrFormat {
            code: error_code,
            name: format!("Reading File"),
            msg: format!(
                "The '{} file' could not be read.\n{}\n{}",
                match error_code {
                    010 => "Settings",
                    011 => "Score",
                    _ => todo!(),
                },
                "If the file is being automativally removed by your anti-virus,",
                "pleade add an exception to it for the game to work."
            ),
        },
        2 => ErrFormat {
            code: error_code,
            name: format!("Writing File"),
            msg: format!(
                "The '{} file' couldn't be created or modified.\n{}",
                match error_code {
                    020 => "Settings",
                    021 => "Score",
                    _ => todo!(),
                },
                "If the game isn't in a writable directory, please move it."
            ),
        },
        10 => ErrFormat {
            code: error_code,
            name: format!("Invalid file structure"),
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
        _ => ErrFormat {
            code: 255,
            name: format!("New Error!"),
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
        "\nError code : \n{}\n\n{}\n{}\n\n{}\n{}",
        error.code, "Error :", error.name, "Probable cause : ", error.msg,
    );
}
