use {
    super::{settings_edit::save_setting_to_file, settings_read::settings_importer},
    crate::{
        utilities::{
            errors::error_handling, file::open_file::open_and_read_existing_file,
            settings::settings_defaults::default_settings,
        },
        ErrFormat, RuntimeFunctionBlob,
    },
    std::io::read_to_string,
};

pub fn settings_file() -> Result<RuntimeFunctionBlob, ErrFormat> {
    let settings_raw: String;

    let RuntimeFunctionBlob {
        settings,
        core_functions,
        comunication,
    } = default_settings();

    match open_and_read_existing_file(&core_functions.settings_file_path) {
        Ok(settings_file) => {
            settings_raw = match read_to_string(settings_file) {
                Ok(settings_raw) => settings_raw,
                Err(_) => return Err(error_handling(010)),
            };

            let runtime_blob: RuntimeFunctionBlob = RuntimeFunctionBlob {
                settings,
                core_functions,
                comunication,
            };

            let (
                imported_settings,
                RuntimeFunctionBlob {
                    settings,
                    core_functions,
                    comunication,
                },
            ) = settings_importer(settings_raw, runtime_blob);

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

            let runtime_blob: RuntimeFunctionBlob = RuntimeFunctionBlob {
                settings,
                core_functions,
                comunication,
            };

            Ok(runtime_blob)
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
