//Initialisation des "crates" ou des librairies suplémentaires nécessaires.
use console::Term;

//Efface le terminal et affiche le nom du program.
pub fn cls_title() {
    //Efface le terminal.
    Term::stdout().clear_screen().expect("Couldn't clear screen.");
    //Affiche le nom du program.
    println!("Guess the number!\n_________________");
}