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
    let mut runtime_blob: RuntimeFunctionBlob;
    let mut settings: Settings = Settings {
        max_range: 100,
        min_range: 1,
        max_tries: 7,
        guess_hint: true,
        settings_count: 4,
    };
    let core_functions: CoreFunctions = CoreFunctions {
        stop: false,
        first_cycle: true,
    };
    let mut comunication: Comunication = Comunication {
        user_in: String::new(),
        err_name: String::new(),
        err_msg: String::new(),
        msg: String::from(format!(
            "-This file contains the settings for the Number Guessing Game.\n{}\n{}\n\n{}\n{}\n\n{}\n{}\n\n{}\n{}",
    
            "-Up to what number do you want to guess?",
            "100",
            
            "-From what number do you want to guess?",
            "1",
            
            "-How many atempts do you want to guess the random number?",
            "7",
            
            "-Do you want hints while you play?",
            "true"
        ))
    };
    let read_err: ErrFormat = ErrFormat {
        name: String::from("Reading File"),
        msg: String::from(format!(
            "Settings file could not be read.\n{}\n{}",
            "If the file is being automatically removed by your anti-virus,",
            "please add an exception to it for the game to work."
        ))
    };
    let write_err: ErrFormat = ErrFormat { 
        name: String::from("Writing File"), 
        msg: String::from(format!(
            "Settings file couldn't be created or modified if it exists already.\n{}",
            "If the game isn't in a writable directory, please move it."
        ))
    };
    let mut tmp: (&str,&str);
    let settings_raw: String = String::new();
    
    //Écrit le contenu de la var "msg" dans le fichier "Settings.txt"
    if let Ok(_) = OpenOptions::new().read(true).write(true).create(true).open("Settings.txt") {
        //
        comunication.err_name = write_err.name;
        comunication.err_msg = write_err.msg;
        //
        if let Err(_) = write("Settings.txt",&comunication.msg) {

        };

        //
        if let Ok(settings_raw) = read_to_string("Settings.txt") {} else {
            comunication.err_name = read_err.name;
            comunication.err_msg = read_err.msg;
            
        };

        //
        let mut settings_raw = settings_raw.lines();

        //
        for line_checked in settings.settings_count..=1 {
            
            //Concatène le message d'erreur de la lecture d'un fichier d'options erroné.
            tmp = match loop_count {
                1 => ("Max_range","a number from 1 to 4'294'967'295"),
                2 => ("Min_range","a number from 0 to 4'294'967'294"),
                3 => ("Max_tries","a number from 1 to 4'294'967'295"),
                4 => ("Guess_hint","'true' or 'false'"),
                _ => ("","")
            };

            match loop_count {
                
            }
        };

    } else {

        //Écrit le contenu de la var "msg" dans le fichier "Settings.txt" 
        //et affiche le contenu de la var "write_err_msg" si une erreur est encontré.
        comunication.err_name = write_err.name;
        comunication.err_msg = write_err.msg;
    };

    runtime_blob = RuntimeFunctionBlob {
        settings,
        core_functions,
        comunication,
    };

    return runtime_blob;
}