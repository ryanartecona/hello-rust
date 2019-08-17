use std::env;
use std::fs;
use std::error::Error;

const CMD_NAME: &'static str = "minigrep";

pub struct Config {
    command: String,
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn from_argv(argv: &[String]) -> Result<Config, String> {
        match argv {
            [command, query, filename] => Ok(Config {
                command: command.to_string(),
                query: query.to_string(),
                filename: filename.to_string(),
                case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
            }),
            _ => {
                let cmd: &str = argv.get(0).map_or(CMD_NAME, |cmd| cmd.as_str());
                Err(format!("Usage: {} QUERY FILENAME", cmd))
            }
        }
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(&config.filename)?;

    for matching_line in search(&config.query, &file_contents) {
      println!("{}", matching_line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut matching_lines = Vec::new();

  for line in contents.lines() {
    if line.contains(query) {
      matching_lines.push(line);
    }
  }

  matching_lines
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let insensitive_query = query.to_lowercase();
  let mut matching_lines = Vec::new();

  for line in contents.lines() {
    if line.to_lowercase().contains(&insensitive_query) {
      matching_lines.push(line);
    }
  }

  matching_lines
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(
      vec!["safe, fast, productive."],
      search(query, contents)
    )
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, contents),
    )
  }
}
