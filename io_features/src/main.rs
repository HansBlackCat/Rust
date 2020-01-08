use std::env;
use std::fs;

struct Config {
    query: String,
    filename: String,
}
impl Config {
    fn new(args: &[String]) ->Result<Config, &'static str> {
        if args.len() < 3 { return Err("not enough arguments"); }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config {query, filename})
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    let config = match config {
        Err(_) => {
            println!("It's Error time!");
            Config {query: String::from("Error!"), filename: String::from("Error.txt")}
        },
        Ok(Config{query, filename}) => Config {query, filename },
    };
    // let config = parse_config_v2(&args);
    /*
    let (query, filename) = parse_config(&args);
    let query = &args[1];
    let filename = &args[2];
    */
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename).expect("Something went wrong reading file");
    println!("With text:\n{}", contents);
}

fn parse_config(args: &[String]) ->(&str, &str) {
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}
fn parse_config_v2(args: &[String]) ->Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    Config {query, filename}
}