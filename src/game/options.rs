//Initialisation des "crates" ou des librairies suplémentaires nécessaires.
use crate::game::game_logic::{
    cls_scr::cls_title,
    questions::{
        numeric_input,
        yes_no_else_input,
    }
};
use std::{
    fs::{
        read_to_string,
        write,
        OpenOptions,
    },
    u32,
};

//Génère le fichier d'options s'il n'existe pas et le lit.
pub fn default_settings() -> (u32, u32, u32, bool) {

    //Initialisation des vars, constantes et plages si applicable.
    let mut max_range = 100;
    let mut min_range = 1;
    let mut max_tries = 10;
    let mut guess_hint = true;
    let mut amount_settings = 4;
    let mut loop_count;
    let mut settings;
    let mut err_name;
    let mut err_msg;

    let msg = format!(
        "//This file contains the settings for the Number Guessing Game.\n\n{}\n\n{}\n\n{}\n\n{}",

        "//Up to what number do you want to guess?\n100",
        
        "//From what number do you want to guess?\n1",
        
        "//How many atempts do you want to guess the random number?\n10",
        
        "//Do you want hints in your game?\ntrue"
    );

    let read_err_msg = format!(
        "Settings file could not be read.\n{}\n{}",
        "If the file is being automatically removed by your anti-virus,",
        "please add an exception to it for this game."
    );

    let write_err_msg = format!(
        "Settings file already exists or couldn't be created.\n{}",
        "If the game isn't in a writable directory, please move it."
    );

    if let Ok(settings_raw) = read_to_string("Settings.txt") {

        let mut settings_raw = settings_raw.lines();

        for loop_count in amount_settings..=1 {
            
            //Concatène le message d'erreur de la lecture d'un fichier d'options déja présent.
            let (err_name,err_msg) = match loop_count {
                4 => ("max_range","a number from 1 to 4'294'967'295"),
                3 => ("min_range","a number from 0 to 4'294'967'294"),
                2 => ("max_tries","a number from 1 to 4'294'967'295"),
                1 => ("guess_hint","'true' or 'false'"),
            };

            let settings = settings_raw
            .next_back()
            .unwrap()
            .parse()
            .expect(
                format!(
                    "'{}' should be {}.",err_name,err_msg
                )
                .as_str()
            );
            for _ in 1..=2 {settings_raw.next_back();};

            let settings = settings_raw
                .next()
                .unwrap();

            let Ok(settings) = settings
            .parse()
            .expect(
                format!(
                    "'{}' should be {}.",
                    err_name,
                    err_msg,
                )
                .as_str()
            );
            let Err(settings) = settings
            .parse()
            .expect(
                format!(
                    "'{}' should be {}.",
                    err_name,
                    err_msg,
                )
                .as_str()
            );
        };

    } else {

        //Écrit le contenu de la var "msg" dans le fichier "Settings.txt" 
        //et affiche le contenu de la var "write_err_msg" si une erreur est encontré.
        if let Ok(settings_raw) = OpenOptions::new()
        .write(true)
        .create(true)
        .open("Settings.txt") {
            write("Settings.txt", msg)
            .expect("Settings.txt");
        } else {
            println!("{}",read_err_msg);
            };

    };

    return (max_range, min_range, max_tries, guess_hint);
}

//Demande la taille de la plage numérique à chercher souhaité.
pub fn game_size(option_size: u32) -> u32 {
    let max_range = numeric_input(format!("Input the largest number you want.\nCurrent:\t{option_size}"));
    cls_title();
    max_range
}

//Demande le nombre de tentatives que le joueur aimerait avoir.
pub fn game_tries(option_tries: u32) -> u32 {
    let max_tries = numeric_input(format!("How many attempts do you want?\nCurrent:\t{option_tries}"));
    cls_title();
    max_tries
}

//Demande si le joueur souhaite avoir des indices quand il joue.
pub fn game_hint(option_hint: bool) -> bool {

    //Initialisation des vars, constantes et plages si applicable.
    let mut wrong = false; //Définit la var "wrong".
    let mut input = String::new(); //Définit la var "input".

    return loop {
        let option_hint = format!("Do you want to get hints while playing? (Y/N)\nCurrent:\t{option_hint}");
        
        //Affiche la var "option_hint" et demande si le joueur veux la changer.
        input = yes_no_else_input(option_hint, input, wrong);
            
        match input.as_str() {
            //Retourne à la liste des options et indique que le joueur ne veux pas d'indices.
            "n" | "N" | "0" | "false" | "False" | "f" | "F" => {
                cls_title();
                break false
            },
            //Retourne à la liste des options et indique que le joueur veux des indices.
            "y" | "Y" | "1" | "true" | "True" | "t" | "T" => {
                cls_title();
                break true
            },
            //Atrappe touts les autres inputs et indique qu'ils sont incorrect.
            _ => {
                cls_title();
                wrong = true;
            },
        };
    };

}