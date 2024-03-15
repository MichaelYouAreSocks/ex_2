//Initialisation des "crates" ou des librairies suplémentaires nécessaires.
use std::cmp::Ordering;
use crate::game::game_logic::{
        cls_scr::cls_title,
        questions::numeric_input,
};
use rand::Rng;

//Fonction de jeu.
pub fn game(settings: (u32, u32, bool)) -> String {

    //Initialisation des vars, constantes et plages si applicable.
    let (max_range, max_tries, guess_hint) = settings;
    let min_range = 0; //Permet de changer la valeur minimum de la plage à chercher.
    let mut guess;
    let mut msg;
    let mut small_guess = min_range; //Indice min initial.
    let mut large_guess = max_range + 1; //Indice max initial.
    let secret_number = {
        rand::thread_rng().gen_range(min_range..=max_range) //Génère un nombre réel entier se trouvant entre "min_range et max_range".
    };

    //Boucle contenant le jeu.
    for tries in min_range..max_tries {

        //Affiche les indices si selectionné dans les options.
        msg = match guess_hint {
            //Concatène l'indice dans la var "msg". 
            true => {
                msg = format!("The number is between {} and {}.", small_guess, large_guess);
                msg
            },
            //S'assure que la var "msg" soit vide.
            false => {
                msg = "".to_string();
                msg
            },
        };

        //Définit la var "guess" en tant qu'alpha-numérique de la valeur indiqué par la fonction "numeric_input".
        guess = numeric_input(msg);
    
        //Control si la var "guess" est plus grande, plus petite ou equivalente à la var "secret_number".
        match guess.cmp(&secret_number) {
            //Affiche un message indiquant que le dernier numéro deviné est plus petit que celui cherché.
            Ordering::Less => {
                cls_title();
                println!("{guess} is too small! {} tries left", max_tries - tries); //Afiche que le numéro deviné est trop petit.
                if small_guess < guess {small_guess = guess}; //Stoque le numéro deviné s'il est plus proche de la cible que le précédent.
            }
            //Affiche un message indiquant que le dernier numéro deviné est plus grand que celui cherché.
            Ordering::Greater => {
                cls_title();
                println!("{guess} is too big! {} tries left", max_tries - tries); //Affiche que le numéro deviné est trop grand.
                if large_guess > guess {large_guess = guess}; //Stoque le numéro deviné s'il est plus proche de la cible que le précédent.
            }
            //Affiche un message indiquant que le joueur à gagnié et quel numéro était le bon.
            Ordering::Equal => {
                cls_title();
                return format!("You win! The correct number was: '{}'",secret_number); //Indique que le jeu n'a pas rencontré d'erreur.
            }
        };
    };
    cls_title();
    return format!("You loose! The secret number was {}", secret_number); //Indique au joueur qu'il a perdu et quel était le numéro cherché.
}