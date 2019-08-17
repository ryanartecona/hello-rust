use std::env;

const CMD_NAME: &'static str = "minigrep";

fn main() {
    let argv: Vec<String> = env::args().collect();
    match argv.as_slice() {
        [_cmd, query, filename] => println!("{:?}, file {:?}", query, filename),
        _ => {
            let cmd: &str = argv.get(0).map_or(CMD_NAME, |cmd| cmd.as_str());
            println!("Usage: {} QUERY FILENAME", cmd);
            std::process::exit(1);
        }
    }
}
