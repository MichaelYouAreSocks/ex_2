use crate::RuntimeFunctionBlob;

pub fn start_menu_layout(runtime_blob: &RuntimeFunctionBlob, high_scores: &Vec<String>) -> String {
    //Affiche un message spécifique dépendant de si le joueur joue sa première partie de la session.
    format!(
        "1 -> Play{}\t\t\t{}{}\n2 -> Options\t\t\t{}{}\n3 -> Score Board\t\t{}{}\n0 -> Quit{}\t\t\t{}{}",
        if runtime_blob.core_functions.first_cycle == true {
            "!"
        } else {
            " again?"
        },
        "1st : ",
        high_scores[0],
        "2nd : ",
        high_scores[1],
        "3rd : ",
        high_scores[2],
        match runtime_blob.comunication.msg.as_str() {
            "" => "".to_string(),
            _ => format!("\n{}", &runtime_blob.comunication.msg),
        },
        "4th : ",
        high_scores[4],
    )
}