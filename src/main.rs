//Initialisation des "crates" ou des librairies suplémentaires nécessaires.
use devinette_numeros::{
    menus::menu_logic::main_menu_logic,
    utilities::{
        cls_scr::cls_title, 
        settings::file::settings_file,
    },
    CoreFunctions
};

//Logiciel mère.
fn main() {

    //Initialisation des vars, constantes et plages si applicable.
    if let Ok(mut runtime_blob) = settings_file() {

        let mut core_function: &CoreFunctions = &runtime_blob.core_functions;
    
        //Boucle contenant le program.
        while runtime_blob.core_functions.stop == false {
    
            //
            cls_title();
    
            //
            runtime_blob = main_menu_logic(runtime_blob);
        };

        //
        if runtime_blob.comunication.err_name != "" {
            println!("Error:\t{}\n{}",runtime_blob.comunication.err_name,runtime_blob.comunication.err_msg)
        }
    };
}

