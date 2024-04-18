//Initialisation des "crates" ou des librairies suplémentaires nécessaires.
use crate::{
    game::{
        game_logic::cls_scr::cls_title, 
        main_game::game, 
        options::{
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

pub fn main_menu_logic(mut runtime_blob: RuntimeFunctionBlob) -> RuntimeFunctionBlob {
    let RuntimeFunctionBlob {
        mut settings, 
        mut core_functions, 
        mut comunication,
    } = runtime_blob;
    
    runtime_blob = main_menu(runtime_blob);
    //Quite le progam si le joueur veut plus jouer.
    match comunication.user_in 
    .parse::<u32>()
    .unwrap() {
        //Quite le jeu.
        0 => core_functions.stop = true,
        //Joue au jeu.
        1 => {
            //Control si la fonction "game" rencontre une erreur et importe toutes les options de ce dernier.
            runtime_blob = game(runtime_blob);
            //Controle si c'est la première partie du joueur et indique que ce n'est plus la première si c'est le cas.
            core_functions.first_cycle = match comunication.msg.as_str() {
                "" => {true},
                _  => {false},
            };
        },
        //Options du jeu.
        2 => runtime_blob = options_menu_logic(runtime_blob),
        //Atrappe touts les autres inputs et indique qu'ils sont incorrect.
        _ => core_functions.stop = false,
    };
    runtime_blob = RuntimeFunctionBlob {settings,core_functions,comunication};
    runtime_blob
}

fn options_menu_logic(mut runtime_blob: RuntimeFunctionBlob) -> RuntimeFunctionBlob {
    let RuntimeFunctionBlob {
        mut settings, 
        mut core_functions, 
        mut comunication,
    } = runtime_blob;

    loop {
        runtime_blob = options_menu(runtime_blob);
        //Affiche le menu des options avec leur configuration actuel.
        match comunication.user_in.parse::<u8>().unwrap() {
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
