use std::env;
use std::fs;

const CMD_NAME: &'static str = "minigrep";

fn main() -> Result<(), String> {
    let argv: Vec<String> = env::args().collect();
    let (_cmd, query, filename) = parse_argv(&argv)?;
    grep(&query, &filename);
    Ok(())
}

fn parse_argv(argv: &Vec<String>) -> Result<(&str, &str, &str), String> {
    match argv.as_slice() {
        [cmd, query, filename] => Ok((&cmd, &query, &filename)),
        _ => {
            let cmd: &str = argv.get(0).map_or(CMD_NAME, |cmd| cmd.as_str());
            Err(format!("Usage: {} QUERY FILENAME", cmd))
        }
    }
}

fn grep(_query: &str, filename: &str) {
    println!("In file {}", filename);

    let file_contents = fs::read_to_string(filename)
        .expect("Couldn't read specified file.");

    println!("--- text contents: ---");
    println!("{}", file_contents);
    println!("----------------------");
}
