//Initialisation des "crates" ou des librairies suplémentaires nécessaires.
use devinette_numeros::{menus::start::main_menu, utilities::{cls_scr::cls_title, settings::file::file::settings_file}};

//Logiciel mère.
fn main() {
    //
    cls_title();

    let runtime_blob

    //Initialisation des vars, constantes et plages si applicable.
    let runtime_blob = match settings_file() {
        Ok(mut runtime_blob) => {
            //Boucle contenant le program.
            while !runtime_blob.core_functions.stop {
                //
                cls_title();

                //
                runtime_blob = main_menu(runtime_blob)
            }
            runtime_blob
        }
        Err(error_handler) => {
            
            println!(
                "Error code : \n{}\n\n{}\n{}\n\n{}\n{}",
                error_handler.code,
                "Error :",
                error_handler.name,
                "Probable cause : ",
                error_handler.msg,
            );
        }
    };
    if runtime_blob.core_functions.error_handler.code != 0 {

    }
}
