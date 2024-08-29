use {
    crate::{
        utilities::misc::{
            cls_scr::cls_title,
            inputs::{name_input, numeric_input},
        },
        RuntimeFunctionBlob,
    },
    rand::Rng,
    std::cmp::Ordering,
};

pub fn game(mut runtime_blob: RuntimeFunctionBlob) -> RuntimeFunctionBlob {
    let mut small_guess: u32 = runtime_blob.settings.min_range - 1;
    let mut large_guess: u32 = runtime_blob.settings.max_range + 1;
    let secret_number: u32 = rand::thread_rng()
        .gen_range(runtime_blob.settings.min_range..=runtime_blob.settings.max_range);

    for tries in runtime_blob.settings.min_range - 1..runtime_blob.settings.max_tries {
        match runtime_blob.settings.guess_hint {
            true => {
                runtime_blob.comunication.msg =
                    format!("The number is between {} and {}.", small_guess, large_guess)
            }
            false => runtime_blob.comunication.msg = "".to_string(),
        };

        runtime_blob.comunication.user_in_u32 = numeric_input(&runtime_blob.comunication.msg);

        match &runtime_blob.comunication.user_in_u32.cmp(&secret_number) {
            Ordering::Less => {
                cls_title();
                println!(
                    "{} is too small! {} {} left",
                    runtime_blob.comunication.user_in_u32,
                    runtime_blob.settings.max_tries - tries - 1,
                    if tries == 1 { "trie" } else { "tries" }
                );

                if small_guess < runtime_blob.comunication.user_in_u32 {
                    small_guess = runtime_blob.comunication.user_in_u32
                };
            }
            Ordering::Greater => {
                cls_title();
                println!(
                    "{} is too big! {} {} left",
                    runtime_blob.comunication.user_in_u32,
                    runtime_blob.settings.max_tries - tries - 1,
                    if tries == 1 { "trie" } else { "tries" }
                );
                if large_guess > runtime_blob.comunication.user_in_u32 {
                    large_guess = runtime_blob.comunication.user_in_u32
                };
            }
            Ordering::Equal => {
                let RuntimeFunctionBlob {
                    mut comunication,
                    settings,
                    mut core_functions,
                } = runtime_blob;
                comunication = name_input(comunication);
                cls_title();
                core_functions
                    .high_score
                    .push(format!("{}", settings.max_range));
                core_functions
                    .high_score
                    .push(format!("{}/{}", tries, settings.max_tries));
                core_functions
                    .high_score
                    .push(comunication.user_in_alpha.to_string());
                comunication.msg = format!("You win! The secret number was {}", secret_number);
                return RuntimeFunctionBlob {
                    comunication,
                    core_functions,
                    settings,
                };
            }
        };
    }
    cls_title();
    runtime_blob.comunication.msg = format!("You loose! The secret number was {}", secret_number);
    runtime_blob
}
