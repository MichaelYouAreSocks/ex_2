//Initialisation des "crates" ou des librairies suplémentaires nécessaires.
use crate::game::game_logic::{
    cls_scr::cls_title,
    questions::{
        yes_no_else_input,
        numeric_input,
    }
};

pub fn main_menu(first_cycle: bool, msg: &String) -> u8 {

    //Initialisation des vars, constantes et plages si applicable.
    let mut input = String::new();
    let mut wrong = false; //Définit la var "wrong".

    loop {
        //Affiche un message différant dépendant de si le joueur joue plus d'une fois.
        let msg = {
            //Affiche un message spécifique dépendant de si le joueur joue sa première partie de la session.
            format!(
                "1 -> Play{}!\n2 -> Options\n0 -> Quit{}", 
                if first_cycle == true {""} else {" again"}, 
                if msg == "" {"".to_string()} else {format!("\n{}", msg)}
            )
        };

        input = yes_no_else_input(msg, input, wrong);

        //Laisse l'utilisateur choisir s'il veux jouer.
        match input.as_str() {
            //Quite le jeu.
            "n" | "N" | "q" | "Q" | "0" => {
                cls_title();
                //Affiche un message dépandant de si le joueur à joué au minimum une partie avant de quiter le jeu.
                    if first_cycle != true {
                    println!("Thanks for playing!");
                } else {
                    println!("Hope you'll play soon!")
                };
                return 0;
            },
            //Joue au jeu.
            "y" | "Y" | "" | "1" => {
                cls_title();
                return 1;
            },
            //Affiche les oprions.
            "o" | "O" | "s" | "S" | "2" => {
                cls_title();
                return 2;
            },
            //Atrappe tout les autres inputs et indique qu'ils sont incorrect.
            _ => {
                cls_title();
                if wrong != true {wrong = true};
            },
        };
    };
}

//Menu d'options de jeu.
pub fn options_menu(settings:(u32, u32, bool)) -> u32 {

    //Initialisation des vars, constantes et plages si applicable.
    let (option_size, option_tries, option_hint) = settings;
    let mut msg;
    let mut input;

    loop {
        //Concatène le menu des option.
        msg = format!(
            "Options:\n{}{}\n{}{}\n{}{}\n{}",
            "1 -> Size of search.\t: ", option_size,
            "2 -> Number of tries.\t: ", option_tries,
            "3 -> Game hints.\t: ", option_hint,
            "0 -> Back to main menu."
        ).trim().to_string();

        cls_title();
        //Permet de choisir quelle option le joueur aimerait changer.
        input = numeric_input(msg);
        //
        match input {
            //Retourne au menu d'acueil.
            0 => return 0,
            //Option de la taille de la plage à chercher chaque manche.
            1 => return 1,
            //Option du nombre de tentatives par manches.
            2 => return 2,
            //Option d'indice.
            3 => return 3,
            //Atrappe touts les autres inputs et indique qu'ils sont incorrect.
            _ => {
                cls_title();
                println!("Couldn't load setting n°{}.",input);
            },
        };
    };
}
