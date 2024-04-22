//Initialisation des "crates" ou des librairies suplémentaires nécessaires.
use crate::{
    game::main_game::game,
    utilities::{
        cls_scr::cls_title,
        game_settings::{
            game_hint,
            game_size,
            game_tries,
        }
    }, 
    menus::menus::{
        main_menu,
        options_menu,
    },
    RuntimeFunctionBlob,
};

pub fn main_menu_logic() {
    //Quite le progam si le joueur veut plus jouer.
    runtime_blob = main_menu(runtime_blob);

    match runtime_blob.comunication.user_in_u32 {
        //Quite le jeu.
        0 => runtime_blob.core_functions.stop = true,
        //Joue au jeu.
        1 => {
            //Lance le jeu et transmet toutes les variables utiles à ce dernier.
            runtime_blob = game(runtime_blob);
            //Controle si c'est la première partie du joueur et indique que ce n'est plus la première si c'est le cas.
            runtime_blob.core_functions.first_cycle = match runtime_blob.comunication.msg.as_str() {
                "" => {true},
                _  => {false},
            };
        },
        //Options du jeu.
        2 => runtime_blob = options_menu_logic(runtime_blob),
        //Atrappe touts les autres inputs et indique qu'ils sont incorrect.
        _ => runtime_blob.core_functions.stop = false,
    };
    runtime_blob
}

fn options_menu_logic() {
    loop {
        //Affiche le menu des options avec leur configuration actuel.
        runtime_blob = options_menu(runtime_blob);
        match runtime_blob.comunication.user_in_u32 {
            //Retourne au menu d'acueil.
            0 => {
                cls_title();
                break;
            },  
            //Option de la taille de la plage à chercher chaque manche.
            1 => {
                cls_title();
                runtime_blob = game_size(runtime_blob)
            },
            //Option du nombre de tentatives par manches.
            2 => {
                cls_title();
                runtime_blob = game_tries(runtime_blob)
            },
            //Option d'indice.
            3 => {
                cls_title();
                runtime_blob = game_hint(runtime_blob)
            }
            //Atrappe touts les autres inputs et indique qu'ils sont incorrect.
            _ => {
                cls_title();
                continue
            },
        };
    };
    runtime_blob
}
