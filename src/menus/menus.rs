//Initialisation des "crates" ou des librairies suplémentaires nécessaires.
use crate::{
    game::game_logic::{
        cls_scr::cls_title,
        questions::{
            numeric_input, yes_no_else_input
        }
    }, 
    Comunication, 
    CoreFunctions,
    RuntimeFunctionBlob,
};

pub fn main_menu(mut runtime_blob:RuntimeFunctionBlob) -> RuntimeFunctionBlob {

    //Initialisation des vars, constantes et plages si applicable.
    let mut wrong: bool = false; //Définit la var "wrong".
    let mut core_functions: CoreFunctions = runtime_blob.core_functions;
    let mut comunication: Comunication = runtime_blob.comunication;

    loop {
        //Affiche un message différant dépendant de si le joueur joue plus d'une fois.
        comunication.msg = {
            //Affiche un message spécifique dépendant de si le joueur joue sa première partie de la session.
            format!(
                "1 -> Play{}!\n2 -> Options\n0 -> Quit{}", 
                if &core_functions.first_cycle == &true {""} else {" again"}, 
                match comunication.msg.as_str() {
                    "" => "".to_string(),
                    _ => format!("\n{}", &comunication.msg)
                }
            )
        };

        comunication = yes_no_else_input(comunication,wrong);

        //Laisse l'utilisateur choisir s'il veux jouer.
        match comunication.user_in.as_str() {
            //Quite le jeu.
            "n" | "N" | "q" | "Q" | "0" => {
                cls_title();
                //Affiche un message dépandant de si le joueur à joué au minimum une partie avant de quiter le jeu.
                    if &core_functions.first_cycle != &true {
                    println!("Thanks for playing!");
                } else {
                    println!("Hope you'll play soon!")
                };
                runtime_blob.comunication.user_in = "0".to_string();
                return runtime_blob
            },
            //Joue au jeu.
            "y" | "Y" | "" | "1" => {
                cls_title();
                runtime_blob.comunication.user_in = "1".to_string();
                return runtime_blob
            },
            //Affiche les oprions.
            "o" | "O" | "s" | "S" | "2" => {
                cls_title();
                runtime_blob.comunication.user_in = "2".to_string();
                return runtime_blob
            },
            //Atrappe tout les autres inputs et indique qu'ils sont incorrect.
            _ => {
                cls_title();
                wrong = true
            },
        }
    };
}

//Menu d'options de jeu.
pub fn options_menu(mut runtime_blob: RuntimeFunctionBlob) -> RuntimeFunctionBlob {
    let RuntimeFunctionBlob {
        mut settings, 
        mut core_functions, 
        mut comunication,
    } = runtime_blob;
    
    loop {
        //Concatène le menu des option.
        comunication.msg = format!(
            "Options:\n{}{}{}{}\n{}{}\n{}{}\n{}",
            "1 -> Size of search.\tMin: ", settings.min_range, "\tMax: ", settings.max_range,
            "2 -> Number of tries.\t: ", settings.max_tries,
            "3 -> Game hints.\t: ", settings.guess_hint,
            "0 -> Back to main menu."
        );

        cls_title();
        comunication = numeric_input(comunication); //Permet de choisir quelle option le joueur aimerait changer.
        
        //
        match comunication.user_in
        .trim()
        .parse::<u8>()
        .expect(format!("{}",comunication.user_in)
                        .as_str()) {
            //Retourne au menu d'acueil.
            0 => return runtime_blob,
            //Option de la taille de la plage à chercher chaque manche.
            1 => return runtime_blob,
            //Option du nombre de tentatives par manches.
            2 => return runtime_blob,
            //Option d'indice.
            3 => return runtime_blob,
            //Atrappe touts les autres inputs et indique qu'ils sont incorrect.
            _ => {
                cls_title();
                println!("Couldn't load setting n°{}.",comunication.user_in);
            },
        };
    };
}
