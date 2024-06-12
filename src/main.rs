//Initialisation des "crates" ou des librairies suplémentaires nécessaires.
use number_guessing_game::{menus::start::main_menu, utilities::{cls_scr::cls_title, errors::read_err, score_board::{self, file::read_score_file}, settings::file::file::settings_file}, ErrFormat, RuntimeFunctionBlob};

//Logiciel mère.
fn main() {
    //
    cls_title();

    let settings_loading_check: Result<RuntimeFunctionBlob, ErrFormat> = settings_file();

    //Initialisation des vars, constantes et plages si applicable.
    if match settings_loading_check {
        Ok(mut runtime_blob) => {

            
            let score_loading_check:Result<Vec<String>, ErrFormat> = read_score_file(&runtime_blob);

            //
            let scores = match &score_loading_check {
                Ok(tmp) => tmp,
                Err(error_handler) => {
                    
                    println!(
                        "Error code : \n{}\n\n{}\n{}\n\n{}\n{}",
                        error_handler.code,
                        "Error :",
                        error_handler.name,
                        "Probable cause : ",
                        error_handler.msg,
                    );
                    &score_loading_check.unwrap()
                }
            };



            //Boucle contenant le program.
            while !&runtime_blob.core_functions.stop {

                //
                runtime_blob = main_menu(runtime_blob, &scores)
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
