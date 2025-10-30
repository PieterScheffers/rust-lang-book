// use minigrep::search;
use minigrep::{search, search_case_insensitive};
use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    // let query = &args[1];
    // let file_path = &args[2];

    // let (query, file_path) = parse_config(&args);

    // println!("Searching for {query}");
    // println!("In file {file_path}");

    // println!("In file {file_path}");

    // let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // let config = parse_config(&args);
    // let config = Config::new(&args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    // let contents =
    //     fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    // println!("With text:\n{contents}");

    // run(config);

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

// struct Config {
//     query: String,
//     file_path: String,
// }

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

// impl Config {
//     fn new(args: &[String]) -> Config {
//         if args.len() < 3 {
//             panic!("not enough arguments");
//         }

//         let query = args[1].clone();
//         let file_path = args[2].clone();

//         Config { query, file_path }
//     }
// }

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        // Ok(Config { query, file_path })

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

// fn parse_config(args: &[String]) -> (&str, &str) {
//     let query = &args[1];
//     let file_path = &args[2];

//     (query, file_path)
// }
// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let file_path = args[2].clone();

//     Config { query, file_path }
// }

// fn run(config: Config) {
//     let contents =
//         fs::read_to_string(config.file_path).expect("Should have been able to read the file");

//     println!("With text:\n{contents}");
// }

// fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     let contents = fs::read_to_string(config.file_path)?;

//     // println!("With text:\n{contents}");

//     for line in search(&config.query, &contents) {
//         println!("{line}");
//     }

//     Ok(())
// }

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
