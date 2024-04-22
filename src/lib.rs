pub mod game;
pub mod menus;
pub mod utilities;

struct Settings {
    pub max_range: u32, // le plus grand numéro de la plâge à chercher.
    pub min_range: u32, //Le plus petit numéro de la plâge à chercher.
    pub max_tries: u32, //Quantité d'essais manqué avant la fin du jeu.
    pub guess_hint: bool, //Affiche ou pas un indice avec la plâge numérique restante à chercher.
    pub settings_count: u8, //Quantité de variables qui sont des options pour le joueur.
}
struct CoreFunctions {
    pub first_cycle: bool, //"first_cycle" détecte si le joueur joue sa première partie.
    pub stop: bool, //Permet de quiter le jeu.
}
struct Comunication {
    pub msg: String, //Concatène les messages pour l'utilisateur.
    pub user_in_alpha: String, //Concatène les inputs de l'utilisateur.
    pub user_in_u32: u32, // 
    pub err_name: String, //Concatène les noms d'erreur.
    pub err_msg: String, //Concatène les messages d'erreur.
}
pub struct ErrFormat {
    pub name: String,
    pub msg: String,
}
// Concatène les "stucts" pour faciliter l'échange d'information entre les différentes fonctions. 
pub struct RuntimeFunctionBlob {
    pub settings: Box<Settings>, //
    pub core_functions: Box<CoreFunctions>, //
    pub comunication: Box<Comunication>, //
}
