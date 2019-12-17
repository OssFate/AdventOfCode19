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

pub mod two {
    fn first_problem(file: &String) {
        let intcode: Vec<&str> = file.trim().split(',').collect();
        let mut ic: Vec<usize> = intcode
            .iter()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        ic[1] = 12;
        ic[2] = 2;

        let tmp = ic.clone();

        for (index, c) in tmp.iter().enumerate() {
            // println!("{} on {}", c, index);
            if index % 4 == 0 {
                // println!("{} on {}", c, index);
                match c {
                    1 => {
                        let sum = ic[ic[index + 1]] + ic[ic[index + 2]];
                        let ni = ic[index + 3];
                        ic[ni] = sum;
                    }
                    2 => {
                        let mul = ic[ic[index + 1]] * ic[ic[index + 2]];
                        let ni = ic[index + 3];
                        ic[ni] = mul;
                    }
                    99 => break,
                    _ => println!("This shouldnt had happened at: {}", index),
                }
            }
        }

        println!("End value at index 0: {}", ic[0]);
    }

    fn second_problem(file: &String) {
        let intcode: Vec<&str> = file.trim().split(',').collect();
        let mut ic: Vec<usize> = intcode
            .iter()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let original = ic.clone();

        for i in 0..100 {
            ic[1] = i;
            for j in 0..100 {
                ic[2] = j;
                // println!("1: {} -> 2: {}", i, j);

                let tmp = ic.clone();

                for (index, c) in tmp.iter().enumerate() {
                    // println!("{} on {}", c, index);
                    if index % 4 == 0 {
                        // println!("{} on {}", c, index);
                        match c {
                            1 => {
                                let sum = ic[ic[index + 1]] + ic[ic[index + 2]];
                                let ni = ic[index + 3];
                                ic[ni] = sum;
                            }
                            2 => {
                                let mul = ic[ic[index + 1]] * ic[ic[index + 2]];
                                let ni = ic[index + 3];
                                ic[ni] = mul;
                            }
                            99 => break,
                            _ => println!(
                                "This shouldnt had happened at: {} with value {}",
                                index, c
                            ),
                        }
                    }
                }
                if ic[0] == 19690720 {
                    println!(
                        "Combo values are {} - {} and answer is: {}",
                        i,
                        j,
                        100 * i + j
                    );
                    return;
                } else {
                    ic = original.clone();
                    ic[1] = i;
                }
            }
        }
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
