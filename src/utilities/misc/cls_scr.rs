use console::Term;

pub fn cls_title() {
    let msg = format!(
        "{}{}",
        "Guess the number!\n",
        "-----------------"
    );
    let err_msg = format!("Couldn't clear terminal, and write game title.");
    clear_screen(msg, err_msg);
}

pub fn clear_screen(msg: String, err_msg: String) {
    match Term::stdout().clear_screen() {
        Ok(_) => println!("{msg}"),
        Err(_) => print!("{err_msg}"),
    };
}
