use {
    super::open_file::open_and_read_existing_file,
    crate::{utilities::settings::defaults::default_settings, ErrFormat, RuntimeFunctionBlob},
    std::{
        fs::write,
        io::{read_to_string, Result},
        process::ExitCode,
        str::Lines,
    },
};

pub fn settings_file() -> Result<RuntimeFunctionBlob> {
    //Charge les paramêtres par défault en mémoir et les sépart en trois type qui sont : "Settings", "CoreFunctions", et "Comunication".
    let (mut settings, mut core_functions, mut comunication) = (
        default_settings().settings,
        default_settings().core_functions,
        default_settings().comunication,
    );
    let mut error_handler: ErrFormat = core_functions.error_handler;

    //Concatène le contenu de "Settings.txt" dans la var "comunication.msg".
    let default_options: String = format!(
        "-This file contains the settings for the Number Guessing Game.\n{}\n\n{}\n{}\n\n{}\n{}\n\n{}\n{}\n\n{}\n{}\n\n{}\n{}",

        "--------------------------------------------------------------",

        "-Up to what number do you want to guess?",
        &settings.max_range,

        "-From what number do you want to guess?",
        &settings.min_range,

        "-How many atempts do you want to guess the random number?",
        &settings.max_tries,

        "-How many atempts do you want to at least have?",
        &settings.min_tries,

        "-Do you want hints while you play?",
        &settings.guess_hint
    );

    //
    let read_err: ErrFormat = ErrFormat {
        code: ExitCode::from(1),
        name: format!("Reading File"),
        msg: format!(
            "Settings file could not be read.\n{}\n{}",
            "If the file is being automatically removed by your anti-virus,",
            "please add an exception to it for the game to work."
        ),
    };

    //
    let write_err: ErrFormat = ErrFormat {
        code: ExitCode::from(2),
        name: format!("Writing File"),
        msg: format!(
            "Settings file couldn't be created or modified if it was.\n{}",
            "If the game isn't in a writable directory, please move it."
        ),
    };

    let path: String = String::from("Settings.txt");
    let settings_raw: String;
    let settings_line_count: usize;
    let mut settings_as_lines: Lines;
    let mut imported_settings: u8 = 0;

    //Préparation du message d'erreurs de lecture
    error_handler = read_err;

    //Controle si un fichier "Settings.txt" existe déja et le créé s'il n'existe pas.
    match open_and_read_existing_file(&path) {
        Ok(settings_file) => {
            //Importe les option de jeu du fichier "Settings.txt"
            settings_raw = if let Ok(settings_raw) = read_to_string(settings_file) {
                //
                settings_raw
            } else {
                //
                //default_options
                "".to_string()
            };

            settings_as_lines = settings_raw.lines();

            settings_line_count = settings_raw.as_str().lines().count();

            //
            for _ in 0..settings_line_count {
                //
                let tmp = match settings_as_lines.next() {
                    Some(tmp) => tmp,
                    None => break,
                };

                //
                (error_handler.code, error_handler.name, error_handler.msg) = match imported_settings {
                    0 => {
                        //
                        if let Ok(tmp) = tmp.trim().parse::<u32>() {
                            settings.max_range = tmp
                        };
                        //
                        imported_settings = imported_settings + 1;
                        //
                        (
                            ExitCode::from(10),
                            String::from("Max_range"),
                            String::from("a number from 1 to 4'294'967'295"),
                        )
                    }
                    1 => {
                        //
                        if let Ok(tmp) = tmp.trim().parse::<u32>() {
                            settings.min_range = tmp
                        };
                        //
                        imported_settings = imported_settings + 1;
                        //
                        (
                            ExitCode::from(11),
                            String::from("Min_range"),
                            String::from("a number from 0 to 4'294'967'294"),
                        )
                    }
                    2 => {
                        //
                        if let Ok(tmp) = tmp.trim().parse::<u32>() {
                            settings.min_tries = tmp
                        };
                        //
                        imported_settings = imported_settings + 1;
                        //
                        (
                            ExitCode::from(12),
                            String::from("Min_tries"),
                            String::from("a number from 1 to 4'294'967'295")
                        )
                        
                    }
                    3 => {
                        //
                        if let Ok(tmp) = tmp.trim().parse::<u32>() {
                            settings.max_tries = tmp
                        };
                        //
                        imported_settings = imported_settings + 1;
                        //
                        (
                            ExitCode::from(13),
                            String::from("Max_tries"),
                            String::from("a number from 1 to 4'294'967'295"),
                        )
                    }
                    4 => {
                        //
                        if let Ok(tmp) = tmp.trim().parse::<bool>() {
                            runtime_blob.settings.guess_hint = tmp
                        };
                        //
                        imported_settings = imported_settings + 1;
                        //
                        (
                            ExitCode::from(14),
                            String::from("Guess_hint"),
                            String::from("'true' or 'false'"),
                        )
                    }
                    _ => (String::from(""), String::from("")),
                };

                //
                if imported_settings < runtime_blob.settings.settings_count {
                    println!("{} should be {}.", tmp_err_msg.name, tmp_err_msg.msg);
                };

                //
                if imported_settings == runtime_blob.settings.settings_count {
                    break;
                } else {
                    continue;
                };
            }
            Ok(runtime_blob)
        }
        Err(_) => {
            runtime_blob.comunication.err_name = write_err.name;
            runtime_blob.comunication.err_msg = write_err.msg;
            match write(&path, &default_options) {
                Ok(()) => Ok(runtime_blob),
                Err(_) => todo!(),
            }
        }
    }
}
