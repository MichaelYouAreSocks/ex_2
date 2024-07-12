use crate::RuntimeFunctionBlob;

pub fn start_menu_layout(runtime_blob: &RuntimeFunctionBlob, high_scores: &Vec<String>) -> String {
    format!(
        "{}{}{}{}\t{}\t{}\n{}{}{}{}\t{}\t{}\n{}{}{}{}\t{}\t{}\n{}{}{}{}\t{}\t{}{}",
        "1 -> Play",
        match runtime_blob.comunication.msg.as_str() {
            "" => {
                format!(
                    "{tmp}",
                    tmp = if runtime_blob.core_functions.first_cycle == true {
                        "!\t\t\t"
                    } else {
                        " again?\t"
                    }
                )
            }
            _ => format!("\t\t\t"),
        },
        "1st :\t",
        high_scores[0],
        high_scores[1],
        high_scores[2],
        "2 -> Options",
        match runtime_blob.comunication.msg.as_str() {
            _ => format!("\t\t\t"),
        },
        "2nd :\t",
        high_scores[3],
        high_scores[4],
        high_scores[5],
        "3 -> Score Board",
        match runtime_blob.comunication.msg.as_str() {
            _ => format!("\t\t"),
        },
        "3rd :\t",
        high_scores[6],
        high_scores[7],
        high_scores[8],
        "0 -> Quit",
        match runtime_blob.comunication.msg.as_str() {
            _ => format!("\t\t\t"),
        },
        "4th :\t",
        high_scores[9],
        high_scores[10],
        high_scores[11],
        match runtime_blob.comunication.msg.as_str() {
            _ => format!("\n{}", runtime_blob.comunication.msg),
        },
    )
}
