//Initialisation des "crates" ou des librairies suplémentaires nécessaires.
use std::cmp::Ordering;
use crate::{
    game::game_logic::{
        cls_scr::cls_title,
        questions::numeric_input,
    },
    RuntimeFunctionBlob,
};
use rand::Rng;

//Fonction de jeu.
pub fn game(mut runtime_blob: RuntimeFunctionBlob) -> RuntimeFunctionBlob {
    let RuntimeFunctionBlob {
        mut settings, 
        mut core_functions, 
        mut comunication,
    } = runtime_blob;

    //Initialisation des vars, constantes et plages si applicable.
    let mut user_in: u32; //
    let mut small_guess: u32 = settings.min_range - 1; //Indice min initial.
    let mut large_guess: u32 = settings.max_range + 1; //Indice max initial.
    let secret_number: u32 = {
        rand::thread_rng().gen_range(settings.min_range..=settings.max_range)
    }; //Génère un nombre réel entier se trouvant entre "min_range et max_range".

    //Boucle contenant le jeu.
    for tries in settings.min_range - 1..settings.max_tries {

        //Affiche les indices si selectionné dans les options.
        match settings.guess_hint {
            //Concatène l'indice dans la var "msg". 
            true => {
                comunication.msg = format!("The number is between {} and {}.", small_guess, large_guess)
            },
            //S'assure que la var "msg" soit vide.
            false => {
                comunication.msg = "".to_string()
            },
        };

        //Définit la var "guess" en tant qu'alpha-numérique de la valeur indiqué par la fonction "numeric_input".
        comunication = numeric_input(comunication);
        comunication.user_in = comunication.user_in.parse::<>().unwrap();
    
        //Control si la var "guess" est plus grande, plus petite ou equivalente à la var "secret_number".
        match user_in.cmp(&secret_number) {
            //Affiche un message indiquant que le dernier numéro deviné est plus petit que celui cherché.
            Ordering::Less => {
                cls_title();
                println!(
                    "{} is too small! {} {} left", 
                    comunication.user_in, 
                    settings.max_tries - tries - 1, 
                    if tries == 1 {"trie"} else {"tries"}
                ); //Afiche que le numéro deviné est trop petit.

                if small_guess < user_in {
                    small_guess = user_in
                }; //Stoque le numéro deviné s'il est plus proche de la cible que le précédent.
            }
            //Affiche un message indiquant que le dernier numéro deviné est plus grand que celui cherché.
            Ordering::Greater => {
                cls_title();
                println!(
                    "{} is too big! {} {} left", 
                    comunication.user_in, 
                    settings.max_tries - tries - 1, 
                    if tries == 1 {"trie"} else {"tries"}
                ); //Affiche que le numéro deviné est trop grand.

                if large_guess > user_in {
                    large_guess = user_in
                }; //Stoque le numéro deviné s'il est plus proche de la cible que le précédent.
            }
            //Affiche un message indiquant que le joueur à gagnié et quel numéro était le bon.
            Ordering::Equal => {
                cls_title();
                comunication.msg = format!(
                    "You win! The correct number was: '{}'",
                    secret_number
                );//Indique que le jeu n'a pas rencontré d'erreur.
                runtime_blob = RuntimeFunctionBlob {settings,core_functions,comunication};
                return runtime_blob;
            }
        };
    };
    cls_title();
    comunication.msg = format!(
        "You loose! The secret number was {}",
        secret_number
    );//Indique au joueur qu'il a perdu et quel était le numéro cherché.
    runtime_blob = RuntimeFunctionBlob {settings,core_functions,comunication};
    runtime_blob
}