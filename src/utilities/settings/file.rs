use {
    crate::{utilities::settings::defaults::default_settings, ErrFormat, RuntimeFunctionBlob},
    std::{fs::{write, File, OpenOptions},io::{read_to_string, Result},str::Lines}
};

pub fn settings_file() -> Result<RuntimeFunctionBlob> {
    //Charge les paramêtres par défault en mémoir et les sépart en trois type qui sont : "Settings", "CoreFunctions", et "Comunication".
    let mut runtime_blob = default_settings();

    //Concatène le contenu de "Settings.txt" dans la var "comunication.msg".
    let default_options: String = format!(
        "-This file contains the settings for the Number Guessing Game.\n{}\n\n{}\n{}\n\n{}\n{}\n\n{}\n{}\n\n{}\n{}",

        "--------------------------------------------------------------",

        "-Up to what number do you want to guess?",
        &runtime_blob.settings.max_range,

        "-From what number do you want to guess?",
        &runtime_blob.settings.min_range,

        "-How many atempts do you want to guess the random number?",
        &runtime_blob.settings.max_tries,

        "-Do you want hints while you play?",
        &runtime_blob.settings.guess_hint
    );

    //
    let read_err: ErrFormat = ErrFormat {
        name: format!("Reading File"),
        msg: format!(
            "Settings file could not be read.\n{}\n{}",
            "If the file is being automatically removed by your anti-virus,",
            "please add an exception to it for the game to work."
        ),
    };

    //
    let write_err: ErrFormat = ErrFormat {
        name: format!("Writing File"),
        msg: format!(
            "Settings file couldn't be created or modified if it was.\n{}",
            "If the game isn't in a writable directory, please move it."
        ),
    };

    //
    let mut tmp_err_msg: ErrFormat = ErrFormat {
        name: String::new(),
        msg: String::new(),
    };

    let path: String = String::from("Settings.txt");
    let settings_raw: String;
    let settings_line_count: usize;
    let mut settings_as_lines: Lines;
    let mut imported_settings: u8 = 0;

    //Préparation des messages d'erreurs de lecture
    runtime_blob.comunication.err_name = read_err.name.to_string();
    runtime_blob.comunication.err_msg = read_err.msg.to_string();

    //Controle si un fichier "Settings.txt" existe déja et le créé s'il n'existe pas.
    match open_and_read_existing_file(&path) {
        Ok(settings_file) => {
            //Importe les option de jeu du fichier "Settings.txt"
            settings_raw = match read_to_string(settings_file) {
                Ok(settings_raw) => {
                    settings_line_count = count_string_lines(settings_raw.as_str());
                    //
                    settings_raw
                }
                Err(_) => {
                    todo!()
                }
            };

            settings_as_lines = settings_raw.lines();

            //
            for _ in 0..settings_line_count {
                //
                let tmp = match settings_as_lines.next() {Some(tmp) => tmp,None => break};

                //
                (tmp_err_msg.name, tmp_err_msg.msg) = match imported_settings {
                    0 => {
                        //
                        if let Ok(tmp) = tmp.trim().parse::<u32>() {runtime_blob.settings.max_range = tmp};
                        //
                        imported_settings = imported_settings + 1;
                        //
                        (String::from("Max_range"),String::from("a number from 1 to 4'294'967'295"))
                    }
                    1 => {
                        //
                        if let Ok(tmp) = tmp.trim().parse::<u32>() {runtime_blob.settings.min_range = tmp};
                        //
                        imported_settings = imported_settings + 1;
                        //
                        (String::from("Min_range"),String::from("a number from 0 to 4'294'967'294"))
                    }
                    2 => {
                        //
                        if let Ok(tmp) = tmp.trim().parse::<u32>() {runtime_blob.settings.max_tries = tmp};
                        //
                        imported_settings = imported_settings + 1;
                        //
                        (String::from("Max_tries"),String::from("a number from 1 to 4'294'967'295"),)
                    }
                    3 => {
                        //
                        if let Ok(tmp) = tmp.trim().parse::<bool>() {runtime_blob.settings.guess_hint = tmp};
                        //
                        imported_settings = imported_settings + 1;
                        //
                        (String::from("Guess_hint"),String::from("'true' or 'false'"))
                    }
                    _ => (String::from(""), String::from("")),
                };

                //
                if imported_settings < runtime_blob.settings.settings_count {
                    println!("{} should be {}.", tmp_err_msg.name, tmp_err_msg.msg);
                };

                //
                if imported_settings == runtime_blob.settings.settings_count {
                    break;
                } else {
                    continue;
                };
            }
            Ok(runtime_blob)
        }
        Err(_) => {
            runtime_blob.comunication.err_name = write_err.name;
            runtime_blob.comunication.err_msg = write_err.msg;
            match write_default_settings_to_file(&default_options, &path) {Ok(()) => Ok(runtime_blob),Err(_) => todo!()}
        }
    }
}

fn write_default_settings_to_file(contents: &String, path: &String) -> Result<()> {
    //
    write(path, contents)
}

fn open_and_read_existing_file(path: &String) -> Result<File> {
    let settings_file = OpenOptions::new().read(true).write(true).open(path);
    settings_file
}

fn count_string_lines(s: &str) -> usize {
    s.lines().count()
}
