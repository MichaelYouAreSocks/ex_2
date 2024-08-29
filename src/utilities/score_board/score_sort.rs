//use radsort::sort_by_key;
//
//use crate::utilities::score_board::score_defaults::default_scores;
//
//fn _main(mut _score_vector: Vec<String>) -> Vec<String> {
//
//    struct Column {no1: String, no2: String, no3: String}
//    
//    let mut score_vector: Vec<String> = default_scores();
//    let score_amount = score_vector.len() as u8 / 3;
//    let size: String;
//    let atempts: String;
//    let name: String;
//
//    size = score_vector.remove(0);
//    atempts = score_vector.remove(0).to_string();
//    name = score_vector.remove(0).to_string();
//
//    let mut score_sorted_by_columns: [Column; score_amount] = [
//        Column {no1: size,no2: atempts,no3: name}
//    ];
//
//    //sort_by_key(&mut score_sorted_by_columns, |&column| (
//    //    column.no1, 
//    //    column.no2, 
//    //    column.no3
//    //));
//    return score_sorted_by_columns;
//}
