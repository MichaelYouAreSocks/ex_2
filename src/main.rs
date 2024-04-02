use ex_2::Settings;

//Initialisation des "crates" ou des librairies suplémentaires nécessaires.
use crate::{
    game::game_logic::{
        cls_scr::cls_title,
        options::default_settings,
    },
    menus::menu_logic::main_menu_logic,
    Settings,
};

//Logiciel mère.
fn main() {
    //Initialisation des vars, constantes et plages si applicable.
    let mut settings: Settings = default_settings();

    //Efface l'écran et affiche le titre du jeu.
    cls_title(); 
    
    //Boucle contenant le program.
    while settings.stop { settings = main_menu_logic(settings) };
}

