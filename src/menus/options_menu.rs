use crate::{
    utilities::{
        misc::{cls_scr::cls_title, inputs::numeric_input},
        settings::{
            in_game_settings::{game_hint, game_size, game_tries},
            settings_edit::save_setting_to_file,
            settings_layout::settings_game_layout,
        },
    },
    RuntimeFunctionBlob,
};

pub fn options_menu(mut runtime_blob: RuntimeFunctionBlob) -> RuntimeFunctionBlob {
    'menu: loop {
        let msg: String = settings_game_layout(&runtime_blob.settings);

        match numeric_input(&msg) {
            0 => {
                cls_title();
                break 'menu;
            }
            1 => {
                cls_title();
                runtime_blob = save_setting_to_file(game_size(runtime_blob));
            }
            2 => {
                cls_title();
                runtime_blob = save_setting_to_file(game_tries(runtime_blob));
            }
            3 => {
                cls_title();
                runtime_blob = save_setting_to_file(game_hint(runtime_blob));
            }
            _ => {
                cls_title();
                println!(
                    "Couldn't load setting n°{}.",
                    runtime_blob.comunication.user_in_u32
                );
            }
        };
    }
    runtime_blob
}
