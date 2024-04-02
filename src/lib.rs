pub mod game;
pub mod menus;
pub struct Settings {
    pub max_range: u32, // le plus grand numéro de la plâge à chercher.
    pub min_range: u32, //Le plus petit numéro de la plâge à chercher.
    pub max_tries: u32, //Quantité d'essais manqué avant la fin du jeu.
    pub guess_hint: bool, //Affiche ou pas un indice avec la plâge numérique restante à chercher.
    pub settings_count: u8, //Quantité de variables qui sont des options pour le joueur.

    pub first_cycle: bool, //"first_cycle" détecte si le joueur joue sa première partie.
    pub stop: bool, //Permet de quiter le jeu.

    pub msg: String, //Concatène les messages pour l'utilisateur.
    pub err_name: String, //Concatène les noms d'erreur.
    pub err_msg: String, //Concatène les messages d'erreur.
}