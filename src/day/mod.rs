/// Template
// pub mod template {
//     fn first_problem(file: &String) {
//     }

//     fn second_problem(file: &String) {
//     }

//     pub fn do_work(input: Option<String>) {
//         let contents = match input {
//             Some(x) => x,
//             None => String::from("")
//         };
//         first_problem(&contents);
//         second_problem(&contents);
//     }
// }
///////

pub mod three {
    fn first_problem(file: &String) {
    }

    fn second_problem(file: &String) {
    }

    pub fn do_work(input: Option<String>) {
        let contents = match input {
            Some(x) => x,
            None => String::from("")
        };
        first_problem(&contents);
        second_problem(&contents);
    }
}

pub mod two {
    use std::collections::HashMap;

    fn first_problem(file: &String) {
        let mut twos = 0;
        let mut threes = 0;

        for line in file.lines() {
            // println!("{}",line);
            let (a, b) = get_repeats(&line);
            if a > 0 { twos += 1; }
            if b > 0 { threes += 1; }
            // println!("Twos: {}, Threes: {}", a, b);
        }

        println!("Hashcode: {}", twos * threes);
    }

    fn get_repeats(input: &str) -> (i32, i32) {
        let mut repeat: HashMap<char, i32> = HashMap::new();
        for c in input.chars() {
            let value = repeat.get(&c).cloned();
            match value {
                Some(num) => repeat.insert(c, num + 1),
                None => repeat.insert(c, 0),
            };
        }
        // println!("{:?}", repeat);
        let mut twos = 0;
        let mut threes = 0;
        for (_character, value) in &repeat {
            if value == &1i32 {
                twos += 1;
            }
            if value == &2i32 {
                threes += 1;
            }
        }
        (twos, threes)
    }

    fn second_problem(file: &String) {
        let mut lines: Vec<&str> = Vec::new();

        for line in file.lines() {
            lines.push(line);
        }

        check_close_compare(lines);
        // println!("{:?}",check_close_compare(lines));
    }

    fn check_close_compare(lines: Vec<&str>) -> Option<bool> {
        let mut diff = 0;
        for i in 0..lines.len() {
            let char_first: Vec<char> = lines[i].chars().collect();
            // println!("{:?}", char_first);
            for j in i..lines.len() {
                let char_second: Vec<char> = lines[j].chars().collect();
                for x in 0..char_first.len() {
                    if diff > 1 {
                        diff = 0;
                        break;
                    }
                    if char_first[x] != char_second[x] {
                        diff += 1;
                    }
                }
                if diff == 1 {
                    println!("First: {:?}, Second: {:?}", lines[i], lines[j]);
                    return Some(true);
                }
                diff = 0;
            }
        }
        None
    }

    pub fn do_work(input: Option<String>) {
        let contents = match input {
            Some(x) => x,
            None => String::from("")
        };
        first_problem(&contents);
        second_problem(&contents);
    }
}

pub mod one {
    use std::collections::HashSet;
    
    pub fn do_work(input: Option<String>) {
        let contents = match input {
            Some(x) => x,
            None => String::from("")
        };
        first_problem(&contents);
        second_problem(&contents);
    }

    fn first_problem(file: &String) {
        let mut frequency: i32 = 0;

        for line in file.lines() {
            let parsed = line.parse::<i32>().unwrap();
            frequency += parsed;
        }

        // println!("Repeat len: {:?}", repeat.len());
        println!("End frequency: {}", frequency);
    }

    fn second_problem(file: &String) {
        let mut frequency: i32 = 0;
        let mut repeat = HashSet::new();
        let mut end = false;

        repeat.insert(frequency);

        while !end {
            for line in file.lines() {
                let parsed = line.parse::<i32>().unwrap();
                frequency += parsed;
                // println!("{}", frequency);

                if repeat.contains(&frequency) {
                    end = true;
                    println!("First repeated: {}", frequency);
                    break;
                } else {
                    repeat.insert(frequency);
                }
            }
        }
    }
}