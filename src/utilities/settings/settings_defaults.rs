use crate::{Comunication, CoreFunctions, ErrFormat, RuntimeFunctionBlob, Settings};

pub fn default_settings() -> RuntimeFunctionBlob {
    let settings: Settings = Settings {
        max_range: 100,
        min_range: 1,
        max_tries: 7,
        min_tries: 1,
        guess_hint: true,
        max_score: 10,
        min_score: 0,
        settings_count: 5,
    };

    let core_functions: CoreFunctions = CoreFunctions {
        high_score: vec![String::new()],
        first_cycle: true,
        stop: false,
        error_handler: ErrFormat {
            code: 0,
            name: String::new(),
            msg: String::new(),
        },
        settings_file_path: String::from("settings.cfg"),
        score_file_path: String::from("scores.txt"),
    };

    let comunication: Comunication = Comunication {
        user_in_alpha: String::new(),
        user_in_u32: 0,
        msg: String::new(),
    };

    let runtime_blob: RuntimeFunctionBlob = RuntimeFunctionBlob {
        settings,
        core_functions,
        comunication,
    };

    runtime_blob
}
