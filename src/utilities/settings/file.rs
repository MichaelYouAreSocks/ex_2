use {
    super::{settings_layout::settings_layout, read::settings_importer},
    crate::{utilities::{errors::error_handling, file::open::open_and_read_existing_file, settings::defaults::default_settings}, ErrFormat, RuntimeFunctionBlob},
    std::{fs::write, io::read_to_string},
};

pub fn settings_file() ->  Result<RuntimeFunctionBlob, ErrFormat> {
    //Charge les paramêtres par défault en mémoir et les sépart en trois vars.
    
    let settings_raw: String;

    let RuntimeFunctionBlob {
        settings, core_functions, comunication
    } = default_settings();

    //Controle si un fichier "Settings.txt" existe déja et le créé s'il n'existe pas.
    match open_and_read_existing_file(&core_functions.settings_file_path) {
        Ok(settings_file) => {
            //Importe les option de jeu du fichier "Settings.txt"
            settings_raw = match read_to_string(settings_file) {
                Ok(tmp) => tmp,
                Err(_) => return Err(error_handling(010))
            };

            let runtime_blob: RuntimeFunctionBlob = RuntimeFunctionBlob{
                settings,
                core_functions,
                comunication,
            };

            let (imported_settings, RuntimeFunctionBlob {
                settings, 
                core_functions, 
                comunication
            }) = settings_importer(settings_raw, runtime_blob);

            //
            if imported_settings < settings.settings_count {
                println!("{} should be {}.\n{}", 
                core_functions.error_handler.name, 
                core_functions.error_handler.msg, 
                "Defaults were used instead");
            };

            let runtime_blob: RuntimeFunctionBlob = RuntimeFunctionBlob{
                settings,
                core_functions,
                comunication,
            };

            println!("Settings loaded from file.");

            Ok(runtime_blob)
        }
        Err(_) => {
            if let Ok(_) = write(&core_functions.settings_file_path, settings_layout(&settings)){
                let runtime_blob: RuntimeFunctionBlob = RuntimeFunctionBlob{
                    settings,
                    core_functions,
                    comunication: comunication,
                };
                println!("Default settings file created and loaded.");
                Ok(runtime_blob)
            } else {
                return Err(error_handling(010))
            }
        }
    }
}