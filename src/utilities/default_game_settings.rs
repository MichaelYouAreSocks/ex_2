use std::fs::{read_to_string,write,OpenOptions};
use crate::{
    RuntimeFunctionBlob,
    Settings,
    CoreFunctions,
    Comunication,
    ErrFormat,
};

//Génère le fichier d'options s'il n'existe pas et le lit.
{
    //
    let mut settings: Settings = Settings {
        max_range: 100,
        min_range: 1,
        max_tries: 7,
        guess_hint: true,
        settings_count: 4,
    };

    //
    let mut core_functions: CoreFunctions = CoreFunctions {
        stop: false,
        first_cycle: true,
    };

    //
    let mut comunication: Comunication = Comunication {
        user_in_alpha: String::new(),
        user_in_u32: 0,
        err_name: String::new(),
        err_msg: String::new(),
        msg: String::new(),
    };

    //Initialisation des vars, constantes et plages si applicable.
    let mut runtime_blob: RuntimeFunctionBlob = RuntimeFunctionBlob {
        settings: Box::new(settings),
        core_functions: Box::new(core_functions),
        comunication: Box::new(comunication),
    };

    //
    runtime_blob.comunication.msg = format!(
        "-This file contains the settings for the Number Guessing Game.\n{}\n\n{}\n{}\n\n{}\n{}\n\n{}\n{}\n\n{}\n{}",

        "--------------------------------------------------------------",

        "-Up to what number do you want to guess?",
        &runtime_blob.settings.max_range,

        "-From what number do you want to guess?",
        &runtime_blob.settings.min_range,

        "-How many atempts do you want to guess the random number?",
        &runtime_blob.settings.max_tries,

        "-Do you want hints while you play?",
        &runtime_blob.settings.guess_hint
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
    let mut settings_raw: std::str::Lines;
    let imported_settings: u8 = 1;

    //Controle si un fichier "Settings.txt" existe déja et le créé s'il n'existe pas.
    if let Ok(_) = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("Settings.txt")
    {
        //Préparation des messages d'erreurs d'écriture
        runtime_blob.comunication.err_name = write_err.name.to_string();
        runtime_blob.comunication.err_msg = write_err.msg.to_string();

        //Importe les option de jeu du fichier "Settings.txt"
        settings_raw = if let Ok(settings_raw) = read_to_string("Settings.txt") {
            let settings_raw = settings_raw.lines();
            settings_raw
        } else {
            runtime_blob.comunication.err_name = read_err.name.to_string();
            runtime_blob.comunication.err_msg = format!("{}", read_err.msg);
            runtime_blob.core_functions.stop = true;
            return;
        };

        //
        while imported_settings <= 4 {
            //Concatène le message d'erreur de la lecture d'un fichier d'options erroné.
            tmp_err = match imported_settings {
                1 => ("Max_range", "a number from 1 to 4'294'967'295"),
                2 => ("Min_range", "a number from 0 to 4'294'967'294"),
                3 => ("Max_tries", "a number from 1 to 4'294'967'295"),
                4 => ("Guess_hint", "'true' or 'false'"),
                _ => ("", ""),
            };
            (tmp_err_msg.name, tmp_err_msg.msg) = (tmp_err.0.to_string(), tmp_err.1.to_string());

            match imported_settings {
                1 => {
                    runtime_blob.settings.max_range = settings_raw
                        .next()
                        .unwrap()
                        .parse::<u32>()
                        .expect(format!("{} should be {}.", tmp_err.0, tmp_err.1).as_str());
                }
                2 => {
                    runtime_blob.settings.min_range = settings_raw
                        .next()
                        .unwrap()
                        .parse::<u32>()
                        .expect(format!("{} should be {}.", tmp_err.0, tmp_err.1).as_str());
                }
                3 => {
                    runtime_blob.settings.max_tries = settings_raw
                        .next()
                        .unwrap()
                        .parse::<u32>()
                        .expect(format!("{} should be {}.", tmp_err.0, tmp_err.1).as_str());
                }
                _ => {}
            }

            //
            if imported_settings == 4 {
                match settings_raw.next().unwrap().parse::<bool>().unwrap() {
                    true => {
                        runtime_blob.settings.guess_hint = true;
                    }
                    false => {
                        runtime_blob.settings.guess_hint = false;
                    }
                    _ => {
                        println!("{} should be {}.", tmp_err.0, tmp_err.1);
                        runtime_blob.core_functions.stop = true;
                        break;
                    }
                };
            };
        }
        runtime_blob.comunication.err_name = write_err.name.to_string();
        runtime_blob.comunication.err_msg = write_err.msg.to_string();

        //
        write("Settings.txt", &runtime_blob.comunication.msg).expect(
            format!(
                "Error : {}\n{}\n{}",
                &runtime_blob.comunication.err_name,
                "Probable cause : ",
                &runtime_blob.comunication.err_msg,
            )
            .as_str(),
        );
    } else {
        //Écrit le contenu de la var "msg" dans le fichier "Settings.txt"
        //et affiche le contenu de la var "write_err_msg" si une erreur est encontré.
        runtime_blob.comunication.err_name = write_err.name.to_string();
        runtime_blob.comunication.err_msg = write_err.msg.to_string();
    };
}
