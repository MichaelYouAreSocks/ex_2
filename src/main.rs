//Initialisation des "crates" ou des librairies suplémentaires nécessaires.
use devinette_numeros::{
    game::{
        game_logic::cls_scr::cls_title,
        options::default_settings,
    },
    menus::menu_logic::main_menu_logic,
    Settings,
};

//Logiciel mère.
fn main() {

    //Initialisation des vars, constantes et plages si applicable.
    let mut settings: Settings = default_settings();
    
    //Boucle contenant le program.
    while !settings.stop {

        //
        settings = main_menu_logic(settings);

        //Efface l'écran et affiche le titre du jeu.
        cls_title(); 
    };

    if settings.err_msg != "" {println!("{}",settings.err_msg)}
}

