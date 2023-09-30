mod parser;
mod lexer;
mod ast;
mod compiler;

use lexer::Lexer;
use parser::parse;

fn main() {
    let s = r#"
pub eager noerror impure main :: u8 = {
    return 0;
};
"#;
    let lexer = Lexer::new(&s).inspect(|tok| println!("tok: {:?}", tok));
    let program = parse(lexer).unwrap().stmt;
    println!("{program:#?}");
}
