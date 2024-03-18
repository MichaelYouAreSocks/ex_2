//Initialisation des "crates" ou des librairies suplémentaires nécessaires.
use ex_2::{
    game::game_logic::cls_scr::cls_title,
    menus::menu_logic::main_menu_logic,
};

//Logiciel mère.
fn main() {
    //Initialisation des vars, constantes et plages si applicable.
    let mut first_cycle = true; //"first_cycle" détecte si le joueur joue sa première partie.
    let mut option_size = 100;  //"option_size" permet de choisir la taille de la plage à chercher chaque manche.
    let mut option_tries = 10;   //"option_tries" permet de choisir le nombre de tentatives par manche.
    let mut option_hint = false; //"oprion_hint" permet de choisir si l'on veut des indices ou pas.
    let mut settings = (option_size, option_tries, option_hint); //Concatène les réglages du jeu.
    let mut stop; //Permet de quiter le jeu.
    let mut msg = "".to_string(); //Var permettant la concaténation des messages pour l'utilisateur.

    cls_title();

    //Boucle contenant le program.
    loop {
        (stop,first_cycle,msg,(option_size, option_tries, option_hint)) = main_menu_logic(first_cycle,msg,settings);

        settings = (option_size, option_tries, option_hint);

        if stop == true {break} else {continue};
    };
}
