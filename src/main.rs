mod parser;
mod lexer;

use lexer::Lexer;

fn main() {
    let mut s = r#"
pub eager noerror impure main :: () -> u8;
main = {
    return 0;
};
"#;
    let lexer = Lexer::new(&s).inspect(|tok| println!("tok: {:?}", tok)).map(|_| ()).collect::<Vec<()>>();
}
