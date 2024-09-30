pub mod game;
pub mod menus;
pub mod utilities;

//Struct contenant les paramêtres quand le programe tourne.
#[derive(Clone)]
pub struct Settings {
    pub max_range: u32,     //le plus grand numéro de la plâge à chercher.
    pub min_range: u32,     //Le plus petit numéro de la plâge à chercher.
    pub max_tries: u32,     //Quantité d'essais manqué avant la fin du jeu.
    pub min_tries: u32,     //Quantité d'éssais minimum autorisé par le jeu.
    pub guess_hint: bool,   //Affiche un indice avec la plâge numérique restante à chercher.
    pub max_score: u8,      //La quantité maximal de scores à afficher dans le menu et le ficher.
    pub min_score: u8,      //La quantité minimum de scores à afficher dans le menu et le ficher.
    pub settings_count: u8, //Quantité de variables qui sont des options pour le joueur.
}

//Struct contenant les donnés d'erreurs.
#[derive(Clone, Debug)]
pub struct ErrFormat {
    pub code: u8,     //Numéro du code d'erreur.
    pub name: String, //Nom du code d'erreure.
    pub msg: String,  //Message pour aider à diagnostiquer l'erreur.
}

//Struct contenant les valeurs nécessaire au fonctionnement du jeu.
#[derive(Clone)]
pub struct CoreFunctions {
    pub high_score: Vec<String>, //Concatène les meilleurs scores des dernières parties.
    pub first_cycle: bool,       //Détecte si le joueur joue sa première partie.
    pub stop: bool,              //Permet de quiter le jeu.
    pub error_handler: ErrFormat, //Concatène tout ce qui se rapport à une erreure.
    pub settings_file_path: String, //Contien l'emplacement par défaut du fichier de paramêtres.
    pub score_file_path: String, //Contien l'emplacement par défaut du fichier de scores.
}

//Stuct contenant les divers messages addressés au joueur et les inputs de ce dernier.
#[derive(Clone)]
pub struct Comunication {
    pub msg: String,           //Concatène les messages pour l'utilisateur.
    pub user_in_alpha: String, //Concatène les inputs alphabêtiques de l'utilisateur.
    pub user_in_u32: u32,      //Concatène les inputs numériques de l'utilisateur.
}

//Struct concatènant les Structs nécessaire au programe de tourner.
#[derive(Clone)]
pub struct RuntimeFunctionBlob {
    pub settings: Settings,            //Concatène la struct Settings.
    pub core_functions: CoreFunctions, //Concatène la struct CoreFunctions.
    pub comunication: Comunication,    //Concatène la struct Comunication.
}

//Struct contenant un différant format de donnés que celui utilisé actuellement.
#[derive(Clone)]
pub struct Row {
    pub score: i64,           //Contien le score actuel.
    pub number_attempts: u32, //Contien le nombre de tentatives faites.
    pub max_attempts: u32,    //Contien le nombre de tentatives tôtal.
    pub name: String,         //Contien le nom du joueur.
}
