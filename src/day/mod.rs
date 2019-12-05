/// Template
// pub mod template {
//     fn first_problem(file: &String) {
//     }

//     fn second_problem(file: &String) {
//     }

//     pub fn do_work(input: Option<String>) {
//         let contents = match input {
//             Some(x) => x,
//             None => String::from(""),
//         };
//         println!("\t>> First Problem <<");
//         first_problem(&contents);
//         println!("\t>> Second Problem <<");
//         second_problem(&contents);
//     }
// }
///////

pub mod one {
    fn first_problem(file: &String) {}

    fn second_problem(file: &String) {}

    pub fn do_work(input: Option<String>) {
        let contents = match input {
            Some(x) => x,
            None => String::from(""),
        };
        println!("\t>> First Problem <<");
        first_problem(&contents);
        println!("\t>> Second Problem <<");
        second_problem(&contents);
    }
}
