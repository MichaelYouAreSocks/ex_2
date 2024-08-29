use number_guessing_game::{
    menus::start_menu::main_menu,
    utilities::{
        misc::{cls_scr::cls_title, errors::err_print},
        score_board::score_file::score_file,
        settings::settings_file::settings_file,
    },
};
fn main() {
    let mut wrong: bool = false;
    let mut msg: String = String::new();
    cls_title();
    match settings_file() {
        Ok(mut runtime_blob) => {
            runtime_blob = match score_file(runtime_blob) {
                Ok(success) => success.to_owned(),
                Err(error) => {
                    err_print(&error);
                    return;
                }
            };

            while !&runtime_blob.core_functions.stop {
                (runtime_blob, wrong, msg) = main_menu(runtime_blob, wrong, msg);
            }
        }
        Err(error) => {
            err_print(&error);
        }
    };
}
