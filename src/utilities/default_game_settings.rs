use std::fs::{
    read_to_string, 
    write, 
    OpenOptions,
};
use crate::{
    RuntimeFunctionBlob,
    Settings,
    CoreFunctions,
    Comunication,
    ErrFormat,
};

//Génère le fichier d'options s'il n'existe pas et le lit.
pub fn default_settings() -> RuntimeFunctionBlob {

    //Initialisation des vars, constantes et plages si applicable.
    let runtime_blob: RuntimeFunctionBlob;
    let mut settings: Settings = Settings {
        max_range: 100,
        min_range: 1,
        max_tries: 7,
        guess_hint: true,
        settings_count: 4,
    };
    let mut core_functions: CoreFunctions = CoreFunctions {
        stop: false,
        first_cycle: true,
    };
    let mut comunication: Comunication = Comunication {
        user_in_alpha: String::new(),
        user_in_u32: 0,
        err_name: String::new(),
        err_msg: String::new(),
        msg: String::from(format!(
            "-This file contains the settings for the Number Guessing Game.\n{}\n{}\n\n{}\n{}\n\n{}\n{}\n\n{}\n{}",
    
            "-Up to what number do you want to guess?",
            settings.max_range,
            
            "-From what number do you want to guess?",
            settings.min_range,
            
            "-How many atempts do you want to guess the random number?",
            settings.max_tries,
            
            "-Do you want hints while you play?",
            settings.guess_hint
        ))
    };
    let read_err: ErrFormat = ErrFormat {
        name: "Reading File".into(),
        msg: format!(
            "Settings file could not be read.\n{}\n{}",
            "If the file is being automatically removed by your anti-virus,",
            "please add an exception to it for the game to work."
        ).into()
    };
    let write_err: ErrFormat = ErrFormat { 
        name: "Writing File".into(),
        msg: format!(
            "Settings file couldn't be created or modified if it exists already.\n{}",
            "If the game isn't in a writable directory, please move it."
        ).into()
    };
    let mut tmp_err_msg: ErrFormat = ErrFormat {
        name: String::new().into(),
        msg: String::new().into(),
    };
    let mut tmp_err: (&str,&str);
    let settings_raw;
    
    //Controle si un fichier "Settings.txt" existe déja et le créé sinon.
    if let Ok(_) = OpenOptions::new().read(true).write(true).create(true).open("Settings.txt") {
        
        //Préparation des messages d'erreurs d'écriture 
        comunication.err_name = write_err.name.to_string();
        comunication.err_msg = write_err.msg.to_string();

        //Importe les option de jeu du fichier "Settings.txt"
        settings_raw = match read_to_string("Settings.txt") {
            Ok(settings_raw) => {
                settings_raw.lines()
            },
            Err(_) => {
                comunication.err_name = read_err.name.to_string();
                comunication.err_msg = format!("{}",read_err.msg);
                core_functions.stop = true;
                runtime_blob = RuntimeFunctionBlob {
                    settings,
                    core_functions,
                    comunication,
                };
                return runtime_blob;
            },
        };

        //
        for imported_settings in 1..=4 {

            settings_raw.filter(!|-|);
            
            //Concatène le message d'erreur de la lecture d'un fichier d'options erroné.
            tmp_err = match imported_settings {
                1 => {
                    settings.max_range = settings_raw;
                    ("Max_range","a number from 1 to 4'294'967'295")
                },
                2 => ("Min_range","a number from 0 to 4'294'967'294"),
                3 => ("Max_tries","a number from 1 to 4'294'967'295"),
                4 => ("Guess_hint","'true' or 'false'"),
                _ => ("","")
            };
            (tmp_err_msg.name,tmp_err_msg.msg) = (tmp_err.0.into(),tmp_err.1.into());

            //
            
        };
        
        //
        if let Err(_) = write("Settings.txt",&comunication.msg) {

        };

    } else {

        //Écrit le contenu de la var "msg" dans le fichier "Settings.txt" 
        //et affiche le contenu de la var "write_err_msg" si une erreur est encontré.
        comunication.err_name = write_err.name.to_string();
        comunication.err_msg = write_err.msg.to_string();
    };

    runtime_blob = RuntimeFunctionBlob {
        settings,
        core_functions,
        comunication,
    };

    runtime_blob
}