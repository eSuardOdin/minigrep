use std::env;
use std::fs;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    // let config = parse_config(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments : {}",err);
        process::exit(1);
    });

    // println!("In file: {}", config.file_path);
    // let content = fs::read_to_string(config.file_path)
    //     .expect("Should have opened file");
    // println!("With text:\n{}", content);
}



struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        else {
            let query = args[1].clone();
            let file_path = args[2].clone();
            return Ok(Config {query, file_path});
        }
    }
}

/*
fn parse_config(args: &[String]) -> Config {
    
    let query = args[1].clone();
    let file_path = args[2].clone();

    
    let conf = Config {
        query,
        file_path,
    };

    conf
}
*/









































/*
Conjoncture: ensemble des facteurs situationnels qui ont un impact sur l'économie
 
L'Etat a mis en oeuvre des mesures conjoncturelles : 
    - Exonération de charges
    - Subventions directes
    - Aides pour maintenir le salaire des salariés
*/


















