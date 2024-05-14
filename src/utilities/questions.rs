//Initialisation des "crates" ou des librairies suplémentaires nécessaires.
use crate::{utilities::cls_scr::cls_title, Comunication};
use std::io;

//Transforme une valeur alpha-numérique en numérique sans négatif.
pub fn numeric_input(msg: &String) -> u32 {
    //Initialisation des vars, constantes et plages si applicable.
    let mut wrong: bool = false; //Permet d'afficher un message si le dernier input du joueur est faux.
    let mut user_in_alpha: String = String::new(); //

    loop {
        //Affiche le context de l'input. Et affiche l'input s'il n'est pas correct.
        match wrong {
            //Affiche le context de la question.
            false => {
                println!("{}", msg);
            }
            //Affiche que la var "input" n'était pas correct en plus du context de la question.
            true => {
                cls_title();
                println!(
                    "{}\n'{}' isn't a valid input. Please try again.",
                    msg,
                    user_in_alpha.trim()
                );
                user_in_alpha = String::new();
            }
        };

        //Permet à l'utilisateur d'indiquer une valeur au program.
        io::stdin()
            .read_line(&mut user_in_alpha)
            .expect("Failed to read line");

        //Transformation d'une var alpha-num. en numérique sans négatif si c'est possible.
        if let Ok(user_in_u32) = user_in_alpha.trim().parse::<u32>() {
            //
            return user_in_u32;

        //
        } else if "".to_string() == user_in_alpha.trim().parse::<String>().unwrap() {
            return 0;
        } else {
            //
            wrong = true
        }
    }
}

//Permet d'avoir une multitude de choix de réponces à une question.
pub fn yes_no_else_input(comunication: &Comunication, wrong: &bool) -> String {
    //
    let mut user_in_alpha: String = String::new();

    //Affiche le context de la question.
    match wrong {
        false => println!("{}", comunication.msg),
        true => {
            //Affiche que la var "input" n'était pas correct en plus du context de la question.
            cls_title();
            println!(
                "{}\n'{}' isn't a valid input. Please try again.",
                comunication.msg, comunication.user_in_alpha
            );
        }
    };

    //Permet à l'utilisateur d'indiquer une valeur au program.
    io::stdin()
        .read_line(&mut user_in_alpha)
        .expect("Failed to read line");

    //
    user_in_alpha.trim().to_string()
}
