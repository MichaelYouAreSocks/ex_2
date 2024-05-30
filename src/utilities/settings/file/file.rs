use {
    super::open_file::open_and_read_existing_file,
    crate::{utilities::settings::defaults::default_settings, ErrFormat, RuntimeFunctionBlob},
    std::{fs::write, io::read_to_string, str::Lines},
};

pub fn settings_file() ->  Result<RuntimeFunctionBlob, ErrFormat> {
    //Charge les paramêtres par défault en mémoir et les sépart en trois type qui sont : "Settings", "CoreFunctions", et "Comunication".
    let RuntimeFunctionBlob {
        mut settings, 
        mut core_functions, 
        comunication
    } = default_settings();

    //
    let path = String::from("Settings.txt");

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
        code: 001,
        name: format!("Reading File"),
        msg: format!(
            "Settings file could not be read.\n{}\n{}",
            "If the file is being automatically removed by your anti-virus,",
            "please add an exception to it for the game to work."
        ),
    };

    //
    let write_err: ErrFormat = ErrFormat {
        code: 002,
        name: format!("Writing File"),
        msg: format!(
            "Settings file couldn't be created or modified.\n{}",
            "If the game isn't in a writable directory, please move it."
        ),
    };
    
    let settings_raw: String;
    let settings_line_count: usize;
    let mut settings_as_lines: Lines;
    let mut imported_settings: u8 = 0;

    //Controle si un fichier "Settings.txt" existe déja et le créé s'il n'existe pas.
    match open_and_read_existing_file(&path) {
        Ok(settings_file) => {
            //Importe les option de jeu du fichier "Settings.txt"
            settings_raw = match read_to_string(settings_file) {
                Ok(tmp) => tmp,
                Err(_) => return Err(read_err)
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
                core_functions.error_handler = match imported_settings {
                    0 => {
                        //
                        if let Ok(tmp) = tmp.trim().parse::<u32>() {
                            settings.max_range = tmp
                        };
                        //
                        imported_settings = imported_settings + 1;
                        //
                        ErrFormat{
                            code: 010,
                            name: String::from("Max_range"),
                            msg: String::from("a number from 1 to 4'294'967'295"),
                        }

                    }
                    1 => {
                        //
                        if let Ok(tmp) = tmp.trim().parse::<u32>() {
                            settings.min_range = tmp
                        };
                        //
                        imported_settings = imported_settings + 1;
                        //
                        ErrFormat{
                            code: 011,
                            name: String::from("Min_range"),
                            msg: String::from("a number from 0 to 4'294'967'294"),
                        }
                    }
                    2 => {
                        //
                        if let Ok(tmp) = tmp.trim().parse::<u32>() {
                            settings.min_tries = tmp
                        };
                        //
                        imported_settings = imported_settings + 1;
                        //
                        ErrFormat {
                            code: 012,
                            name: String::from("Min_tries"),
                            msg: String::from("a number from 1 to 4'294'967'295")
                        }
                        
                    }
                    3 => {
                        //
                        if let Ok(tmp) = tmp.trim().parse::<u32>() {
                            settings.max_tries = tmp
                        };
                        //
                        imported_settings = imported_settings + 1;
                        //
                        ErrFormat {
                            code: 013,
                            name: String::from("Max_tries"),
                            msg: String::from("a number from 1 to 4'294'967'295"),
                        }
                    }
                    4 => {
                        //
                        if let Ok(tmp) = tmp.trim().parse::<bool>() {
                            settings.guess_hint = tmp
                        };
                        //
                        imported_settings = imported_settings + 1;
                        //
                        ErrFormat {
                            code: 014,
                            name: String::from("Guess_hint"),
                            msg: String::from("'true' or 'false'"),
                        }
                    }
                    _ => {
                        core_functions.error_handler
                    }
                };

                //
                if imported_settings == settings.settings_count {
                    break;
                } else {
                    continue;
                };
            };

            //
            if imported_settings < settings.settings_count {
                println!("{} should be {}.", core_functions.error_handler.name, core_functions.error_handler.msg);
            };

            core_functions.error_handler = write_err;

            let runtime_blob: RuntimeFunctionBlob = RuntimeFunctionBlob{
                settings,
                core_functions,
                comunication,
            };

            Ok(runtime_blob)
        }
        Err(_) => {

            let runtime_blob: RuntimeFunctionBlob = RuntimeFunctionBlob{
                settings,
                core_functions,
                comunication: comunication,
            };
            if let Ok(_) = write(path, &default_options){
                Ok(runtime_blob)
            } else {
                return Err(write_err)
            }
        }
    }
}
