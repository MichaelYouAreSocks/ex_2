use crate::{utilities::misc::cls_scr::cls_title, Comunication};
use std::io;

pub fn numeric_input(msg: &String) -> u32 {
    let mut wrong: bool = false;
    let mut user_in_alpha: String = String::new(); //

    loop {
        match wrong {
            false => {
                println!("{}", msg);
            }
            true => {
                cls_title();
                println!(
                    "{}\n'{}' isn't a valid input. Please try again.",
                    msg,
                    user_in_alpha.trim()
                );
                user_in_alpha = String::new();
            }
        };

        io::stdin()
            .read_line(&mut user_in_alpha)
            .expect("Failed to read line");

        if let Ok(user_in_u32) = user_in_alpha.trim().parse::<u32>() {
            return user_in_u32;
        } else if user_in_alpha.trim().parse::<String>().unwrap() == "" {
            return 0;
        } else {
            wrong = true
        }
    }
}

pub fn yes_no_else_input(comunication: &Comunication, wrong: &bool) -> String {
    let mut user_in_alpha: String = String::new();
    match wrong {
        false => println!("{}", comunication.msg),
        true => {
            println!(
                "{}\n'{}' isn't a valid input. Please try again.",
                comunication.msg, comunication.user_in_alpha
            );
        }
    };

    io::stdin()
        .read_line(&mut user_in_alpha)
        .expect("Failed to read line");

    user_in_alpha.trim().to_string()
}

pub fn name_input(mut comunication: Comunication) -> Comunication {
    let mut wrong: bool = false;
    //let mut user_in_alpha: String;
    comunication.msg = format!("{}{}", comunication.msg, "\nPlease enter a 3 letter name :\n\n");
    loop {
        cls_title();
        comunication.user_in_alpha = yes_no_else_input(&comunication, &wrong);
        wrong = match comunication.user_in_alpha.chars().count() {
            3 => false,
            _ => true,
        };
        if !wrong {return comunication}
    }
}
