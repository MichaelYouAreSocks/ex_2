//Initialisation des "crates" ou des librairies suplémentaires nécessaires.
use crate::{game::game_logic::cls_scr::cls_title, Settings};
use std::io;

//Transforme une valeur alpha-numérique en numérique sans négatif.
pub fn numeric_input(mut settings: Settings) -> Settings {

    //Initialisation des vars, constantes et plages si applicable.
    let mut wrong: bool = false; //Permet d'afficher un message si le dernier input du joueur est faux.

    loop {
        //Affiche le context de l'input. Et affiche l'input s'il n'est pas correct.
        match wrong {
            //Affiche le context de la question.
            false => {
                println!("{}", settings.msg);
                wrong
            },
            //Affiche que la var "input" n'était pas correct en plus du context de la question.
            true => {
                cls_title();
                println!(
                    "{}\n'{}' isn't a valid input. Please try again.", 
                    settings.msg, 
                    settings.user_in.trim()
                );
                settings.user_in = String::new(); //Vide le contenu de la var "input"
                wrong = false;
                wrong
            },
        };

        //Permet à l'utilisateur d'indiquer une valeur au program.
        io::stdin()
            .read_line(&mut settings.user_in)
            .expect("Failed to read line");

        //Transformation d'une var alpha-num. en numérique sans négatif si c'est possible.
        if let Ok(user_in) = settings.user_in.trim().parse() {
            settings.user_in = user_in;
            return settings
        } else {
            wrong = true;
            wrong
        };
    };
}

//Permet d'avoir une multitude de choix de réponces à une question.
pub fn yes_no_else_input(mut settings: Settings,wrong: bool) -> Settings {
    //Affiche le context de la question.
    match wrong {
        false => println!("{}", settings.msg),
        true => { //Affiche que la var "input" n'était pas correct en plus du context de la question.
            cls_title();
            println!("{}\n'{}' isn't a valid input. Please try again.", settings.msg, settings.user_in);
            settings.user_in = String::new() //Purge la var "input".
        },
    };

    //Permet à l'utilisateur d'indiquer une valeur au program.
    io::stdin()
        .read_line(&mut settings.user_in)
        .expect("Failed to read line");

    settings.user_in = settings.user_in.trim().to_string();

    return settings
}