mod model;
mod parser;
mod errors;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let parser = parser::Parser::new(file_path);
    let parsing_result = parser.parse();
    if let Err(parsing_result) = parsing_result{
        println!("{}", parsing_result); // Just print error message for now
    }

}
