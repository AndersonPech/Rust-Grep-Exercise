use std::env;
use std::fs::File;
use std::io::Read;
use std::io::{ErrorKind, Error};

mod config;
mod file;
//
// Code for a simplified grep application with regular expressions.
// ^      Matches characters at the beginning of a line
// $      Matches characters at the end of a line
// "."    Matches any character
// [a-z]  Matches any characters between A and Z
// [^ ..] Matches anything apart from what is contained in the brackets
// \      Treat next character not as a regular expression
fn main() -> Result<(),  &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return Err("Less than 2 arguments");
    }

    let query = &args[1];
    let file_path = &args[2];

    let mut grep_command: config::QueryDetails = config::QueryDetails {
       query: String::from(query),
       file_path: String::from(file_path),
       regex : None
    };

    grep_command.data_cleaning();


    let mut file_contents = File::open(file_path).unwrap();
    let mut contents = String::new();
    file_contents.read_to_string(&mut contents).unwrap();

    file::file_search(&contents, &grep_command);

    Ok(())
}

