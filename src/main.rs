//Initialisation des "crates" ou des librairies suplémentaires nécessaires.
use devinette_numeros::{
    utilities::{
        cls_scr::cls_title,
        default_game_settings::default_settings,
    }, 
    menus::menu_logic::main_menu_logic,
    RuntimeFunctionBlob,
};

//Logiciel mère.
fn main() {

    //Initialisation des vars, constantes et plages si applicable.
    let mut runtime_blob: RuntimeFunctionBlob = default_settings();
    
    //Boucle contenant le program.
    while !runtime_blob.core_functions.stop {

        //
        runtime_blob = main_menu_logic(runtime_blob);

        //Efface l'écran et affiche le titre du jeu.
        cls_title(); 
    };

    if runtime_blob.comunication.err_msg != "" {println!("{}",runtime_blob.comunication.err_msg)}
}

