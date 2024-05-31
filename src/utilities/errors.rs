use crate::ErrFormat;

pub fn read_err() -> ErrFormat {
    //
    let read_err: ErrFormat = ErrFormat {
        code: 001,
        name: format!("Reading File"),
        msg: format!(
            "Settings file could not be read.\n{}\n{}",
            "If the file is being automatically removed by your anti-virus,",
            "please add an exception to it for the game to work."
        ),
    };
    read_err
}

pub fn write_err() -> ErrFormat {
    //
    let write_err: ErrFormat = ErrFormat {
        code: 002,
        name: format!("Writing File"),
        msg: format!(
            "Settings file couldn't be created or modified.\n{}",
            "If the game isn't in a writable directory, please move it."
        ),
    };
    write_err
}