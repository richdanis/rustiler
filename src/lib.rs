use std::fs;
use std::error::Error;

mod tokenizer;
pub struct Config {
    pub file_path: String
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    // this function call returns a Result, so we need to unwrap it with ?
    let code = fs::read_to_string(&config.file_path)?;

    // here we let tokenize take ownership of code, since after this function
    // we don't need to use it anymore
    let tokens = tokenizer::tokenize(code)?;

    for token in tokens {
        println!("{token}");
    }

    Ok(())

}
