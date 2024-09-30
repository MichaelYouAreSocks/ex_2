use crate::RuntimeFunctionBlob;

pub fn start_menu_layout(runtime_blob: &RuntimeFunctionBlob) -> String {
    format!(
        "{}{}{}{}\t{}\t{}\n{}{}{}{}\t{}\t{}\n{}{}{}{}\t{}\t{}\n{}{}{}{}\t{}\t{}{}",
        "1 -> Play",
        match runtime_blob.comunication.msg.as_str() {
            "" => {
                (if runtime_blob.core_functions.first_cycle {
                    "!\t\t\t"
                } else {
                    " again?\t"
                })
                .to_string()
            }
            _ => "\t\t\t".to_string(),
        },
        "1st :\t",
        runtime_blob.core_functions.high_score[0],
        runtime_blob.core_functions.high_score[1],
        runtime_blob.core_functions.high_score[2],
        "2 -> Options",
        match runtime_blob.comunication.msg.as_str() {
            _ => "\t\t\t".to_string(),
        },
        "2nd :\t",
        runtime_blob.core_functions.high_score[3],
        runtime_blob.core_functions.high_score[4],
        runtime_blob.core_functions.high_score[5],
        "3 -> Score Board",
        match runtime_blob.comunication.msg.as_str() {
            _ => "\t\t".to_string(),
        },
        "3rd :\t",
        runtime_blob.core_functions.high_score[6],
        runtime_blob.core_functions.high_score[7],
        runtime_blob.core_functions.high_score[8],
        "0 -> Quit",
        match runtime_blob.comunication.msg.as_str() {
            _ => "\t\t\t".to_string(),
        },
        "4th :\t",
        runtime_blob.core_functions.high_score[9],
        runtime_blob.core_functions.high_score[10],
        runtime_blob.core_functions.high_score[11],
        format!("\n{}", runtime_blob.comunication.msg),
    )
}
