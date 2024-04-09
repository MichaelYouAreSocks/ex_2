//Initialisation des "crates" ou des librairies suplémentaires nécessaires.
use crate::{
    Settings,
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
};

pub fn main_menu_logic(mut settings: Settings) -> Settings {
    settings = main_menu(settings);
    //Quite le progam si le joueur veut plus jouer.
    match settings.user_in 
    .parse::<u32>()
    .unwrap() {
        //Quite le jeu.
        0 => settings.stop = true,
        //Joue au jeu.
        1 => {
            //Control si la fonction "game" rencontre une erreur et importe toutes les options de ce dernier.
            settings = game(settings);
            //Controle si c'est la première partie du joueur et indique que ce n'est plus la première si c'est le cas.
            settings.first_cycle = match settings.msg.as_str() {
                "" => {true},
                _  => {false},
            };
        },
        //Options du jeu.
        2 => settings = options_menu_logic(settings),
        //Atrappe touts les autres inputs et indique qu'ils sont incorrect.
        _ => settings.stop = false,
    };
    settings
}

fn options_menu_logic(mut settings: Settings) -> Settings {
    
    loop {
        settings = options_menu(settings);
        //Affiche le menu des options avec leur configuration actuel.
        match settings.user_in.parse::<u8>().unwrap() {
            //Retourne au menu d'acueil.
            0 => {
                cls_title();
                break;
            },  
            //Option de la taille de la plage à chercher chaque manche.
            1 => {
                cls_title();
                settings = game_size(settings)
            },
            //Option du nombre de tentatives par manches.
            2 => {
                cls_title();
                settings = game_tries(settings)
            },
            //Option d'indice.
            3 => {
                cls_title();
                settings = game_hint(settings)
            }
            //Atrappe touts les autres inputs et indique qu'ils sont incorrect.
            _ => {
                cls_title();
                continue
            },
        };
    };
    settings
}
