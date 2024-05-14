//Initialisation des "crates" ou des librairies suplémentaires nécessaires.
use devinette_numeros::{
    menus::start::main_menu,
    utilities::{cls_scr::cls_title, settings::file::settings_file}
};

//Logiciel mère.
fn main() {
    //
    cls_title();

    //
    let runtime_blob = settings_file();

    //Initialisation des vars, constantes et plages si applicable.
    match runtime_blob {
        Ok(mut runtime_blob) => {
            //Boucle contenant le program.
            while !runtime_blob.core_functions.stop {
                //
                cls_title();

                //
                runtime_blob = main_menu(runtime_blob)
            }
        }
        Err(_) => {
            let runtime_blob = runtime_blob.unwrap();

            //
            println!(
                "Error : \n{}\n\n{}\n{}\n\n{}\n",
                runtime_blob.comunication.err_name,
                "Probable cause : ",
                runtime_blob.comunication.err_msg,
                "Details : ",
                //Error
            )
        }
    }
}
