use {
    crate::{utilities::misc::errors::error_handling, RuntimeFunctionBlob},
    std::{num::ParseIntError, str::Lines},
};

pub fn settings_importer(
    settings_raw: String,
    mut runtime_blob: RuntimeFunctionBlob,
) -> (u8, RuntimeFunctionBlob) {
    let RuntimeFunctionBlob {
        mut settings,
        mut core_functions,
        comunication,
    } = runtime_blob;

    let mut lines_searched: u8 = 0;
    let mut imported_settings: u8 = 0;
    let mut settings_name_value: Vec<&str>;
    let mut settings_as_lines: Lines = settings_raw.lines();
    let settings_line_count: u8 = settings_raw.lines().count() as u8;

    while imported_settings < settings.settings_count {
        core_functions.error_handler = error_handling(210 + imported_settings);

        settings_name_value = match settings_as_lines.next() {
            Some(tmp) => tmp.trim().split(" == ").collect::<Vec<_>>(),
            None => break,
        };

        match settings_name_value.remove(0) {
            "[Max_range]" => {
                if let Ok(tmp) = settings_value_reader(settings_name_value) {
                    settings.max_range = tmp;
                    imported_settings += 1;
                }
            }
            "[Min_range]" => {
                if let Ok(tmp) = settings_value_reader(settings_name_value) {
                    settings.min_range = tmp;
                    imported_settings += 1;
                }
            }
            "[Max_tries]" => {
                if let Ok(tmp) = settings_value_reader(settings_name_value) {
                    settings.max_tries = tmp;
                    imported_settings += 1;
                }
            }
            "[Min_tries]" => {
                if let Ok(tmp) = settings_value_reader(settings_name_value) {
                    settings.min_tries = tmp;
                    imported_settings += 1;
                }
            }
            "[Guess_hint]" => {
                if let Ok(tmp) = settings_name_value.remove(0).trim().parse::<bool>() {
                    settings.guess_hint = tmp;

                    imported_settings += 1;
                }
            }
            _ => core_functions.error_handler = error_handling(214),
        };
        lines_searched += 1;
        if lines_searched >= settings_line_count {
            break;
        }
    }

    runtime_blob = RuntimeFunctionBlob {
        settings,
        core_functions,
        comunication,
    };

    (imported_settings, runtime_blob)
}

fn settings_value_reader(mut settings_name_value: Vec<&str>) -> Result<u32, ParseIntError> {
    settings_name_value.remove(0).trim().parse::<u32>()
}
