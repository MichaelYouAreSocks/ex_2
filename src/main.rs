//Initialisation des "crates" ou des librairies suplémentaires nécessaires.
use devinette_numeros::{RuntimeFunctionBlob, ErrFormat, menus::start::main_menu, utilities::{cls_scr::cls_title, settings::file::file::settings_file}};

//Logiciel mère.
fn main() {
    //
    cls_title();

    let settings_loading_check: Result<RuntimeFunctionBlob, ErrFormat> = settings_file();

    //Initialisation des vars, constantes et plages si applicable.
    if match settings_loading_check.clone() {
        Ok(mut runtime_blob) => {
            //Boucle contenant le program.
            while !&runtime_blob.core_functions.stop {

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
            settings_loading_check.unwrap()
        }
    }.core_functions.error_handler.code != 0 {
        
    };
}
