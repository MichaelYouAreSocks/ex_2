//Initialisation des "crates" ou des librairies suplémentaires nécessaires.
use crate::{game::game_logic::cls_scr::cls_title,Comunication};
use std::io;

//Transforme une valeur alpha-numérique en numérique sans négatif.
pub fn numeric_input(mut comunication: Comunication) -> Comunication {

    //Initialisation des vars, constantes et plages si applicable.
    let mut wrong: bool = false; //Permet d'afficher un message si le dernier input du joueur est faux.

    loop {
        //Affiche le context de l'input. Et affiche l'input s'il n'est pas correct.
        match wrong {
            //Affiche le context de la question.
            false => {
                println!("{}", comunication.msg);
                wrong
            },
            //Affiche que la var "input" n'était pas correct en plus du context de la question.
            true => {
                cls_title();
                println!(
                    "{}\n'{}' isn't a valid input. Please try again.", 
                    comunication.msg, 
                    comunication.user_in.trim()
                );
                comunication.user_in = String::new(); //Vide le contenu de la var "input"
                wrong = false;
                wrong
            },
        };

        //Permet à l'utilisateur d'indiquer une valeur au program.
        io::stdin()
            .read_line(&mut comunication.user_in)
            .expect("Failed to read line");

        //Transformation d'une var alpha-num. en numérique sans négatif si c'est possible.
        if let Ok(user_in) = comunication.user_in.trim().parse() {
            comunication.user_in = user_in;
            return comunication
        } else {
            wrong = true;
            wrong
        };
    };
}

//Permet d'avoir une multitude de choix de réponces à une question.
pub fn yes_no_else_input(mut comunication: Comunication,wrong: bool) -> Comunication {
    //Affiche le context de la question.
    match wrong {
        false => println!("{}", comunication.msg),
        true => { //Affiche que la var "input" n'était pas correct en plus du context de la question.
            cls_title();
            println!("{}\n'{}' isn't a valid input. Please try again.", comunication.msg, comunication.user_in);
            comunication.user_in = String::new() //Purge la var "input".
        },
    };

    //Permet à l'utilisateur d'indiquer une valeur au program.
    io::stdin()
        .read_line(&mut comunication.user_in)
        .expect("Failed to read line");

        comunication.user_in = comunication.user_in.trim().to_string();

    return comunication
}