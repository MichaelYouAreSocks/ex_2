//Initialisation des "crates" ou des librairies suplémentaires nécessaires.
use crate::{
    utilities::{
        cls_scr::cls_title,
        questions::{
            numeric_input, yes_no_else_input
        }
    }, Comunication, RuntimeFunctionBlob 
};

pub fn main_menu(mut runtime_blob:RuntimeFunctionBlob) -> RuntimeFunctionBlob {

    //Initialisation des vars, constantes et plages si applicable.
    let mut wrong: bool = false; //Définit la var "wrong".
    let RuntimeFunctionBlob {
        settings,
        core_functions, 
        mut comunication,
    } = runtime_blob;

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
        match comunication.user_in_alpha.as_str() {
            //Quite le jeu.
            "n" | "N" | "q" | "Q" | "0" => {
                cls_title();
                //Affiche un message dépandant de si le joueur à joué au minimum une partie avant de quiter le jeu.
                    if &core_functions.first_cycle != &true {
                    println!("Thanks for playing!");
                } else {
                    println!("Hope you'll play soon!")
                };
                comunication.user_in_alpha = String::new();
                comunication.user_in_u32 = 0;
                runtime_blob = RuntimeFunctionBlob {settings,core_functions,comunication};
                return runtime_blob
            },
            //Joue au jeu.
            "y" | "Y" | "" | "1" => {
                cls_title();
                comunication.user_in_alpha = String::new();
                comunication.user_in_u32 = 1;
                runtime_blob = RuntimeFunctionBlob {settings,core_functions,comunication};
                return runtime_blob
            },
            //Affiche les oprions.
            "o" | "O" | "s" | "S" | "2" => {
                cls_title();
                comunication.user_in_alpha = String::new();
                comunication.user_in_u32 = 2;
                runtime_blob = RuntimeFunctionBlob {settings,core_functions,comunication};
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
        settings, 
        core_functions, 
        comunication: _,
    } = runtime_blob;

    let mut comunication = Comunication {
        msg: String::new(),
        user_in_alpha: String::new(),
        user_in_u32: 0,
        err_msg: "".into(),
        err_name: "".into(),
    };
    
    loop {
        //Concatène le menu des option.
        comunication.msg = format!(
            "Options:\n{}{}{}{}\n{}{}\n{}{}\n{}",
            "1 -> Size of search.\tMin: ", settings.min_range, "\tMax: ", settings.max_range,
            "2 -> Number of tries.\t: ", settings.max_tries,
            "3 -> Game hints.\t: ", settings.guess_hint,
            "0 -> Back to main menu."
        );
        
        //Permet de choisir quelle option le joueur aimerait changer.
        comunication = numeric_input(comunication);
        match comunication.user_in_u32 {
            //Retourne au menu d'acueil.
            0 => {
                cls_title();
                comunication.user_in_alpha = String::new();
                comunication.user_in_u32 = 0;
                runtime_blob = RuntimeFunctionBlob {settings,core_functions,comunication};
                return runtime_blob
            },
            //Option de la taille de la plage à chercher chaque manche.
            1 => {
                cls_title();
                comunication.user_in_alpha = String::new();
                comunication.user_in_u32 = 1;
                runtime_blob = RuntimeFunctionBlob {settings,core_functions,comunication};
                return runtime_blob
            },
            //Option du nombre de tentatives par manches.
            2 => {
                cls_title();
                comunication.user_in_alpha = String::new();
                comunication.user_in_u32 = 2;
                runtime_blob = RuntimeFunctionBlob {settings,core_functions,comunication};
                return runtime_blob
            },
            //Option d'indice.
            3 => {
                cls_title();
                comunication.user_in_alpha = String::new();
                comunication.user_in_u32 = 3;
                runtime_blob = RuntimeFunctionBlob {settings,core_functions,comunication};
                return runtime_blob
            },
            //Atrappe touts les autres inputs et indique qu'ils sont incorrect.
            _ => {
                cls_title();
                println!("Couldn't load setting n°{}.",comunication.user_in_u32);
            },
        };
    };
}
