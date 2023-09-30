mod parser;
mod lexer;
mod ast;

use lexer::Lexer;

fn main() {
    let mut s = r#"
pub eager noerror impure fn main :: u8 = {
    return 0;
}
"#;
    let lexer = Lexer::new(&s).inspect(|tok| println!("tok: {:?}", tok)).map(|i| i).collect::<Vec<_>>();
}
