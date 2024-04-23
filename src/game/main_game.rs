//Initialisation des "crates" ou des librairies suplémentaires nécessaires.
use std::cmp::Ordering;
use crate::{
    utilities::{
        cls_scr::cls_title,
        questions::numeric_input,
    },
    RuntimeFunctionBlob,
};
use rand::Rng;

//Fonction de jeu.
pub fn game(mut runtime_blob: Box<RuntimeFunctionBlob>) -> Box<RuntimeFunctionBlob> {

    //Initialisation des vars, constantes et plages si applicable.
    let mut small_guess: u32 = runtime_blob.settings.min_range - 1; //Indice min initial.
    let mut large_guess: u32 = runtime_blob.settings.max_range + 1; //Indice max initial.
    let secret_number: u32 = {
        rand::thread_rng().gen_range(runtime_blob.settings.min_range..=runtime_blob.settings.max_range)
    }; //Génère un nombre réel entier se trouvant entre "min_range et max_range".

    //Boucle contenant le jeu.
    for tries in runtime_blob.settings.min_range - 1..runtime_blob.settings.max_tries {

        //Affiche les indices si selectionné dans les options.
        match runtime_blob.settings.guess_hint {
            //Concatène l'indice dans la var "msg". 
            true => {
                runtime_blob.comunication.msg = format!("The number is between {} and {}.", small_guess, large_guess)
            },
            //S'assure que la var "msg" soit vide.
            false => {
                runtime_blob.comunication.msg = "".to_string()
            },
        };

        //Définit la var "comunication.user_in" en tant qu'alpha-numérique de la valeur indiqué par la fonction "numeric_input".
        runtime_blob.comunication.user_in_u32 = numeric_input(&runtime_blob.comunication);
    
        //Control si la var "comunication.user_in_32" est plus grande, plus petite ou equivalente à la var "secret_number".
        match runtime_blob.comunication.user_in_u32.cmp(&secret_number) {
            //Affiche un message indiquant que le dernier numéro deviné est plus petit que celui cherché.
            Ordering::Less => {
                cls_title();
                println!(
                    "{} is too small! {} {} left", 
                    runtime_blob.comunication.user_in_u32, 
                    runtime_blob.settings.max_tries - tries - 1, 
                    if tries == 1 {"trie"} else {"tries"}
                ); //Afiche que le numéro deviné est trop petit.

                if small_guess < runtime_blob.comunication.user_in_u32 {
                    small_guess = runtime_blob.comunication.user_in_u32
                }; //Stoque le numéro deviné s'il est plus proche de la cible que le précédent.
            }
            //Affiche un message indiquant que le dernier numéro deviné est plus grand que celui cherché.
            Ordering::Greater => {
                cls_title();
                println!(
                    "{} is too big! {} {} left", 
                    runtime_blob.comunication.user_in_u32, 
                    runtime_blob.settings.max_tries - tries - 1, 
                    if tries == 1 {"trie"} else {"tries"}
                ); //Affiche que le numéro deviné est trop grand.

                if large_guess > runtime_blob.comunication.user_in_u32 {
                    large_guess = runtime_blob.comunication.user_in_u32
                }; //Stoque le numéro deviné s'il est plus proche de la cible que le précédent.
            }
            //Affiche un message indiquant que le joueur à gagnié et quel numéro était le bon.
            Ordering::Equal => {
                cls_title();
                runtime_blob.comunication.msg = format!(
                    "You win! The correct number was: '{}'",
                    secret_number
                );
            }
        };
    };
    cls_title();
    runtime_blob.comunication.msg = format!(
        "You loose! The secret number was {}",
        secret_number
    );//Indique au joueur qu'il a perdu et quel était le numéro cherché.
    //runtime_blob = RuntimeFunctionBlob {settings,core_functions,comunication};
    runtime_blob
}