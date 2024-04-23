use std::{fs::{write,File,OpenOptions}, io::{prelude::*, BufReader,Result}};
use crate::{ErrFormat, RuntimeFunctionBlob, utilities::settings::defaults::default_settings};

pub fn settings_file() -> Result<Box<RuntimeFunctionBlob>> {
    
    //Charge les paramêtres par défault en mémoir et les sépart en trois type qui sont : "Settings", "CoreFunctions", et "Comunication".
    let runtime_blob = default_settings();
    let mut settings: Box<crate::Settings> = runtime_blob.settings;
    let mut core_functions: Box<crate::CoreFunctions> = runtime_blob.core_functions;
    let mut comunication: Box<crate::Comunication> = runtime_blob.comunication;
    
    //Concatène le contenu de "Settings.txt" dans la var "comunication.msg".
    comunication.msg = format!(
        "-This file contains the settings for the Number Guessing Game.\n{}\n\n{}\n{}\n\n{}\n{}\n\n{}\n{}\n\n{}\n{}",

        "--------------------------------------------------------------",

        "-Up to what number do you want to guess?",
        &settings.max_range,

        "-From what number do you want to guess?",
        &settings.min_range,

        "-How many atempts do you want to guess the random number?",
        &settings.max_tries,

        "-Do you want hints while you play?",
        &settings.guess_hint
    );

    //
    let read_err: ErrFormat = ErrFormat {
        name: format!("Reading File"),
        msg: format!(
            "Settings file could not be read.\n{}\n{}",
            "If the file is being automatically removed by your anti-virus,",
            "please add an exception to it for the game to work."
        ),
    };

    //
    let write_err: ErrFormat = ErrFormat {
        name: format!("Writing File"),
        msg: format!(
            "Settings file couldn't be created or modified if it exists already.\n{}",
            "If the game isn't in a writable directory, please move it."
        ),
    };

    //
    let mut tmp_err_msg: ErrFormat = ErrFormat {
        name: String::new(),
        msg: String::new(),
    };

    let mut tmp_err: (&str, &str);
    let imported_settings: u8 = 0;

    //Controle si un fichier "Settings.txt" existe déja et le créé s'il n'existe pas.
    if let Ok(settings_raw) = OpenOptions::new().read(true).write(true).create(true).open("Settings.txt") {

        //Préparation des messages d'erreurs d'écriture
        comunication.err_name = write_err.name.to_string();
        comunication.err_msg = write_err.msg.to_string();

        //Importe les option de jeu du fichier "Settings.txt"
        let settings_raw: BufReader<File> = BufReader::new(settings_raw);

        //
        for lines in settings_raw.lines() {
            
            //Concatène le message d'erreur de la lecture d'un fichier d'options erroné.
            tmp_err = match imported_settings {
                0 => ("Max_range", "a number from 1 to 4'294'967'295"),
                1 => ("Min_range", "a number from 0 to 4'294'967'294"),
                2 => ("Max_tries", "a number from 1 to 4'294'967'295"),
                3 => ("Guess_hint", "'true' or 'false'"),
                _ => ("", ""),
            };
            (tmp_err_msg.name, tmp_err_msg.msg) = (tmp_err.0.to_string(), tmp_err.1.to_string());

            match imported_settings {
                0 => settings.max_range = lines.unwrap().parse::<u32>().expect(format!("{} should be {}.", tmp_err.0, tmp_err.1).as_str()),
                1 => settings.min_range = lines.unwrap().parse::<u32>().expect(format!("{} should be {}.", tmp_err.0, tmp_err.1).as_str()),
                2 => settings.max_tries = lines.unwrap().parse::<u32>().expect(format!("{} should be {}.", tmp_err.0, tmp_err.1).as_str()),
                3 => {
                    if let Ok(game_hint) = lines.unwrap().parse::<bool>() {settings.guess_hint = game_hint;
                    } else {
                        println!("{} should be {}.", tmp_err.0, tmp_err.1);
                        core_functions.stop = true;
                        break;
                    }
                }
                _ => continue
            }

            //
            if imported_settings == settings.settings_count {break} else {continue};
        };
        
        comunication.err_name = write_err.name.to_string();
        comunication.err_msg = write_err.msg.to_string();

        //
        write("Settings.txt", &comunication.msg).expect(
            format!(
                "Error : {}\n{}\n{}",
                &comunication.err_name,
                "Probable cause : ",
                &comunication.err_msg,
            )
            .as_str(),
        );
    } else {
        comunication.err_name = read_err.name;
        comunication.err_msg = read_err.msg;
        core_functions.stop = true;
        //Écrit le contenu de la var "msg" dans le fichier "Settings.txt"
        //et affiche le contenu de la var "write_err_msg" si une erreur est encontré.
        //runtime_blob.comunication.err_name = write_err.name.to_string();
        //runtime_blob.comunication.err_msg = write_err.msg.to_string();
    };
    
    let runtime_blob = RuntimeFunctionBlob {
        settings: settings,
        core_functions: core_functions,
        comunication: comunication
    };
    return Result::Ok(Box::new(runtime_blob));
}