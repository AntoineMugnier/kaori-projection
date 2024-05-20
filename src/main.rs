mod share;
mod parser;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let parser = parser::Parser::new(file_path);
    parser.parse().unwrap();

}
