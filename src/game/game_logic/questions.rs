//Initialisation des "crates" ou des librairies suplémentaires nécessaires.
use crate::game::game_logic::cls_scr::cls_title;
use std::io;

//Transforme une valeur alpha-numérique en numérique sans négatif.
pub fn numeric_input(msg: String,) -> u32 {

    //Initialisation des vars, constantes et plages si applicable.
    let mut input = String::new(); //Compare ce que le joueur input avec ce qu'attends le jeu.
    let mut wrong = false; //Permet d'afficher un message si le dernier input du joueur est faux.

    loop {
        //Affiche le context de l'input. Et affiche l'input s'il n'est pas correct.
        match wrong {
            //Affiche le context de la question.
            false => {
                println!("{}", msg);
                wrong
            },
            //Affiche que la var "input" n'était pas correct en plus du context de la question.
            true => {
                cls_title();
                println!("{}\n'{}' isn't a valid input. Please try again.", msg, input.trim());
                input = String::new(); //Vide le contenu de la var "input"
                wrong = false;
                wrong
            },
        };

        //Permet à l'utilisateur d'indiquer une valeur au program.
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        //Transformation d'une var alpha-num. en numérique sans négatif si c'est possible.
        if let Ok(input) = input.trim().parse() {
            return input
        } else {
            wrong = true;
            wrong
        };
    };
}

//Permet d'avoir une multitude de choix de réponces à une question.
pub fn yes_no_else_input(msg: String, mut input:String, wrong: bool) -> String {
    //Affiche le context de la question.
    match wrong {
        false => {
            //Affiche le context de la question.
            println!("{}", msg);
        },
        //Affiche que la var "input" n'était pas correct en plus du context de la question.
        true => {
            cls_title();
            println!("{}\n'{}' isn't a valid input. Please try again.", msg, input);
            input = String::new() //Purge la var "input".
        },
    };

    //Permet à l'utilisateur d'indiquer une valeur au program.
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    return input.trim().to_string();
}