//Initialisation des "crates" ou des librairies suplémentaires nécessaires.
use crate::game::game_logic::{
    cls_scr::cls_title,
    questions::{
        numeric_input,
        yes_no_else_input,
    }
};

//Demande la taille de la plage numérique à chercher souhaité.
pub fn game_size(option_size: u32) -> u32 {
    let max_range = numeric_input(format!("Input the largest number you want.\nCurrent:\t{option_size}"));
    cls_title();
    max_range
}

//Demande le nombre de tentatives que le joueur aimerait avoir.
pub fn game_tries(option_tries: u32) -> u32 {
    let max_tries = numeric_input(format!("How many attempts do you want?\nCurrent:\t{option_tries}"));
    cls_title();
    max_tries
}

//Demande si le joueur souhaite avoir des indices quand il joue.
pub fn game_hint(option_hint: bool) -> bool {

    //Initialisation des vars, constantes et plages si applicable.
    let mut wrong = false; //Définit la var "wrong".
    let mut input = String::new(); //Définit la var "input".

    return loop {
        let option_hint = format!("Do you want to get hints while playing? (Y/N)\nCurrent:\t{option_hint}");
        
        //Affiche la var "option_hint" et demande si le joueur veux la changer.
        input = yes_no_else_input(option_hint, input, wrong);
            
        match input.as_str() {
            //Retourne à la liste des options et indique que le joueur ne veux pas d'indices.
            "n" | "N" | "0" | "false" | "False" | "f" | "F" => {
                cls_title();
                break false
            },
            //Retourne à la liste des options et indique que le joueur veux des indices.
            "y" | "Y" | "1" | "true" | "True" | "t" | "T" | "" => {
                cls_title();
                break true
            },
            //Atrappe touts les autres inputs et indique qu'ils sont incorrect.
            _ => {
                cls_title();
                wrong = true;
            },
        };
    };

}