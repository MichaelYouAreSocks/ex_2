//Initialisation des "crates" ou des librairies suplémentaires nécessaires.
use std::cmp::Ordering;
use crate::{
    game::game_logic::{
        cls_scr::cls_title,
        questions::numeric_input,
    },
    Settings,
};
use rand::Rng;

//Fonction de jeu.
pub fn game(mut settings: Settings) -> Settings {

    //Initialisation des vars, constantes et plages si applicable.
    let mut guess; //Déclare la var "guess".
    let mut small_guess = settings.min_range - 1; //Indice min initial.
    let mut large_guess = settings.max_range + 1; //Indice max initial.
    let secret_number = {
        rand::thread_rng().gen_range(settings.min_range..=settings.max_range)
    }; //Génère un nombre réel entier se trouvant entre "min_range et max_range".

    //Boucle contenant le jeu.
    for tries in settings.min_range - 1..settings.max_tries {

        //Affiche les indices si selectionné dans les options.
        match settings.guess_hint {
            //Concatène l'indice dans la var "msg". 
            true => {
                settings.msg = format!("The number is between {} and {}.", small_guess, large_guess)
            },
            //S'assure que la var "msg" soit vide.
            false => {
                settings.msg = "".to_string()
            },
        };

        //Définit la var "guess" en tant qu'alpha-numérique de la valeur indiqué par la fonction "numeric_input".
        guess = numeric_input(&settings.msg);
    
        //Control si la var "guess" est plus grande, plus petite ou equivalente à la var "secret_number".
        match guess.cmp(&secret_number) {
            //Affiche un message indiquant que le dernier numéro deviné est plus petit que celui cherché.
            Ordering::Less => {
                cls_title();
                println!(
                    "{} is too small! {} {} left", 
                    guess, 
                    settings.max_tries - tries - 1, 
                    if tries == 1 {"trie"} else {"tries"}
                ); //Afiche que le numéro deviné est trop petit.

                if small_guess < guess {
                    small_guess = guess
                }; //Stoque le numéro deviné s'il est plus proche de la cible que le précédent.
            }
            //Affiche un message indiquant que le dernier numéro deviné est plus grand que celui cherché.
            Ordering::Greater => {
                cls_title();
                println!(
                    "{} is too big! {} {} left", 
                    guess, 
                    settings.max_tries - tries - 1, 
                    if tries == 1 {"trie"} else {"tries"}
                ); //Affiche que le numéro deviné est trop grand.

                if large_guess > guess {
                    large_guess = guess
                }; //Stoque le numéro deviné s'il est plus proche de la cible que le précédent.
            }
            //Affiche un message indiquant que le joueur à gagnié et quel numéro était le bon.
            Ordering::Equal => {
                cls_title();
                settings.msg = format!(
                    "You win! The correct number was: '{}'",
                    secret_number
                );//Indique que le jeu n'a pas rencontré d'erreur.
                return settings
            }
        };
    };
    cls_title();
    settings.msg = format!(
        "You loose! The secret number was {}",
        secret_number
    );//Indique au joueur qu'il a perdu et quel était le numéro cherché.
    settings
}