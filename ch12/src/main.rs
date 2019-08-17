use std::env;
use std::fs;
use std::process;
use std::error::Error;

const CMD_NAME: &'static str = "minigrep";

struct Config {
    command: String,
    query: String,
    filename: String,
}

impl Config {
    fn from_argv(argv: &[String]) -> Result<Config, String> {
        match argv {
            [command, query, filename] => Ok(Config {
                command: command.to_string(),
                query: query.to_string(),
                filename: filename.to_string(),
            }),
            _ => {
                let cmd: &str = argv.get(0).map_or(CMD_NAME, |cmd| cmd.as_str());
                Err(format!("Usage: {} QUERY FILENAME", cmd))
            }
        }
    }
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    let config = Config::from_argv(&argv).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1)
    });
    if let Err(e) = grep(&config) {
        println!("Runtime error: {}", e);
        process::exit(1)
    };
}

fn grep(config: &Config) -> Result<(), Box<dyn Error>> {
    println!("In file {}", &config.filename);

    let file_contents = fs::read_to_string(&config.filename)?;

    println!("--- text contents: ---");
    println!("{}", file_contents);
    println!("----------------------");
    Ok(())
}
