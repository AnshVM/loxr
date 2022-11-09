pub mod lexer;
pub use loxr::loxr_;

use loxr::lexer::lexer::Literal;
use std::fs;
use std::env;


fn main() {
    let args:Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Could not read file");
    let mut loxr = loxr::loxr_::Loxr::new();
    let mut scanner = loxr::lexer::lexer::Scanner::new(&contents[..],&mut loxr);
    scanner.scan_tokens();
    for token in scanner.tokens.iter() {
        let literal = token.literal.as_ref().unwrap_or(&Literal::NONE(' '));
        println!("{:?} {:?}",token.token_type,literal);
    }
    println!("file path : {}",file_path);
    println!("Hello, world!");
}
