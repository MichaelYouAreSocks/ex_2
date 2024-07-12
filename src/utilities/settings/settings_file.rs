use {
    super::{settings_edit::save_setting_to_file, settings_read::settings_importer},
    crate::{
        utilities::{
            file::open_file::load_existing_file, misc::errors::error_handling,
            settings::settings_defaults::default_settings,
        },
        ErrFormat, RuntimeFunctionBlob,
    },
    std::io::read_to_string,
};

pub fn settings_file() -> Result<RuntimeFunctionBlob, ErrFormat> {
    let RuntimeFunctionBlob {
        settings,
        core_functions,
        comunication,
    } = default_settings();
    let error_code = 010;

    let settings_raw: String;

    match load_existing_file(&core_functions.settings_file_path, error_code) {
        Ok(settings_file) => {
            settings_raw = match read_to_string(settings_file) {
                Ok(settings_raw) => settings_raw,
                Err(_) => return Err(error_handling(010)),
            };

            let (
                imported_settings,
                RuntimeFunctionBlob {
                    settings,
                    core_functions,
                    comunication,
                },
            ) = settings_importer(
                settings_raw,
                RuntimeFunctionBlob {
                    settings,
                    core_functions,
                    comunication,
                },
            );

            if imported_settings < settings.settings_count {
                println!(
                    "{} should be {}.\n{}",
                    core_functions.error_handler.name,
                    core_functions.error_handler.msg,
                    "Defaults were used instead"
                );
            } else {
                println!("Settings loaded from file.");
            };

            Ok(RuntimeFunctionBlob {
                settings,
                core_functions,
                comunication,
            })
        }
        Err(_) => {
            let mut runtime_blob: RuntimeFunctionBlob = RuntimeFunctionBlob {
                settings,
                core_functions,
                comunication,
            };
            runtime_blob = save_setting_to_file(runtime_blob);
            Ok(runtime_blob)
        }
    }
}
