use std::error::Error;
use std::fs;
use std::process;
// use std::fs::File;
// use std::io::Read;

mod day;

pub fn work_to_do(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Running Command: {}", config.command);

    let file_content = read_file(config.filepath).unwrap();

    match config.command.as_ref() {
        "one" => day::one::do_work(file_content),
        "two" => day::two::do_work(file_content),
        "three" => day::three::do_work(file_content),
        _ => println!("None option"),
    }

    Ok(())
}

fn read_file(path: String) -> Result<Option<String>, Box<Error + 'static>> {
    println!("Reading file: {}\n", path);
    // let mut f = File::open(path).expect("file not found");
    // let mut contents = String::new();
    // f.read_to_string(&mut contents)
    //     .expect("something went wrong reading the file");

    let contents = match fs::read_to_string(path) {
        Ok(result) => result,
        Err(e) => {
            println!("Error reading the file: {}", e);
            process::exit(1);
        }
    };

    Ok(Some(contents))
}

struct Config {
    command: String,
    filepath: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments, please use this way: \"LOL\"");
        }

        let command = args[1].clone();
        let filepath = args[2].clone();

        Ok(Config { command, filepath })
    }
}
