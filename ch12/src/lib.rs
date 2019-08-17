use std::fs;
use std::error::Error;

const CMD_NAME: &'static str = "minigrep";

pub struct Config {
    command: String,
    query: String,
    filename: String,
}

impl Config {
    pub fn from_argv(argv: &[String]) -> Result<Config, String> {
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

pub fn grep(config: &Config) -> Result<(), Box<dyn Error>> {
    eprintln!("In file {}", &config.filename);

    let file_contents = fs::read_to_string(&config.filename)?;

    println!("--- text contents: ---");
    println!("{}", file_contents);
    println!("----------------------");
    Ok(())
}
