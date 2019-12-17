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
    fn first_problem(file: &String) {
        let mut sum: i32 = 0;
        for line in file.lines() {
            let parsed = line.parse::<i32>().unwrap();
            let gas_needed = parsed / 3;
            let gas_needed = gas_needed - 2;

            sum += gas_needed;
        }

        println!("Ammount of fuel needed is: {}", sum);
    }

    fn second_problem(file: &String) {
        let mut total_fuel: i32 = 0;
        for line in file.lines() {
            let parsed = line.parse::<i32>().unwrap();
            let gas_needed = parsed / 3;
            let gas_needed = gas_needed - 2;

            total_fuel += gas_needed;

            let mut temp = gas_needed;
            while temp > 0 {
                // println!("1: {}", temp);
                temp = temp / 3;
                temp = temp - 2;
                if temp > 0 {
                    total_fuel += temp;
                }
            }
        }

        println!("Ammount of total fuel needed is: {}", total_fuel);
    }

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
