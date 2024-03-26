//Initialisation des "crates" ou des librairies suplémentaires nécessaires.
use crate::{
    menus::menus::{
        options_menu,
        main_menu,
    },
    game::{
        game_logic::cls_scr::cls_title,
        options::{
            game_hint,
            game_size,
            game_tries,
        },
        main_game::game,
    },
};

pub fn main_menu_logic(
    mut first_cycle: bool, mut msg: String, mut settings:(u32, u32, u32, bool)
) -> (bool,bool,String,(u32, u32, u32, bool)) {

    //Quite le progam si le joueur veut plus jouer.
    if true == match main_menu(first_cycle, &msg) {
        //Quite le jeu.
        0 => true,
        //Joue au jeu.
        1 => {
            //Control si la fonction "game" rencontre une erreur et importe toutes les options de ce dernier.
            msg = game(settings);

        match msg.as_str() {
                "" => true,
                _  => {

                    //Controle si c'est la première partie du joueur et indique que ce n'est plus la première si c'est le cas.
                    first_cycle = false;
                    false
                },
            }
        },
        //Options du jeu.
        2 => {
            settings = options_menu_logic(settings);
            false
        },
        //Atrappe touts les autres inputs et indique qu'ils sont incorrect.
        _ => false,
    } {return (true, first_cycle, msg, settings)} else {return (false, first_cycle, msg, settings)}
}

pub fn options_menu_logic(mut settings:(u32, u32, u32, bool)) -> (u32, u32, u32, bool) {
    
    loop {
        let (mut option_size_max, option_size_min, mut option_tries, mut option_hint) = settings;
        //Affiche le menu des options avec leur configuration actuel.
        match options_menu(settings) {
            //Retourne au menu d'acueil.
            0 => {
                cls_title();
                break;
            },  
            //Option de la taille de la plage à chercher chaque manche.
            1 => {
                cls_title();
                option_size_max = game_size(option_size_max)
            },
            //Option du nombre de tentatives par manches.
            2 => {
                cls_title();
                option_tries = game_tries(option_tries)
            },
            //Option d'indice.
            3 => {
                cls_title();
                option_hint = game_hint(option_hint)
            }
            //Atrappe touts les autres inputs et indique qu'ils sont incorrect.
            _ => {
                cls_title();
                continue
            },
        };
        settings = (option_size_max, option_size_min, option_tries, option_hint);
    };
    settings
}
