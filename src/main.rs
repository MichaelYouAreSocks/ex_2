//Initialisation des "crates" ou des librairies suplémentaires nécessaires.
use ex_2::{
    game::game_logic::cls_scr::cls_title,
    menus::menu_logic::main_menu_logic,
};

//Logiciel mère.
fn main() {

    //Initialisation des vars, constantes et plages si applicable.
    let mut first_cycle = true; //"first_cycle" détecte si le joueur joue sa première partie.
    let (
        mut option_size_max, 
        mut option_size_min, 
        mut option_tries, 
        mut option_hint
    ) = default_settings(); //
    let mut settings; //Concatène les réglages du jeu.
    let mut stop; //Permet de quiter le jeu.
    let mut msg = "".to_string(); //Var permettant la concaténation des messages pour l'utilisateur.

    cls_title();

    //Boucle contenant le program.
    loop {
        settings = (option_size_max, option_size_min, option_tries, option_hint);

        (stop,
            first_cycle,
            msg,
            (
                option_size_max, 
                option_size_min, 
                option_tries, 
                option_hint
            )
        ) = main_menu_logic(first_cycle,msg,settings);

        if stop == true {break} else {continue};
    };
}

