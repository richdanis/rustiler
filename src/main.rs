use std::env;
use std::fs;
use std::process;

fn main() {

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Compiling {}", config.file_path);

    // let code = fs::read_to_string(&config.file_path);

    println!("{}", config.file_path);
}

struct Config {
    file_path: String
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        
        // ensure there is at least one argument
        if args.len() < 2 {
            return Err("No file provided");
        }

        let file_path = args[1].clone();

        // throw error if the file ending is not .c
        if !file_path.ends_with(".c") {
            return Err("File does not end in .c");
        }

        Ok(Config { file_path })

    }
}