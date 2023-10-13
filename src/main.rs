mod parser;
mod lexer;
mod ast;
mod compiler;

use lexer::Lexer;
use parser::parse;
use compiler::CodeGen;

fn main() {
    let s = r#"
cool :: x:u8 -> u8 = {
    if (x == 10) {
        return 10 + x;
    }
    return x;
};

main :: u8 = {
    let x: u8 = 10;
    x = 30;
    return [cool](x);
};
"#;
    let lexer = Lexer::new(&s).inspect(|tok| println!("tok: {:?}", tok));
    let program = parse(lexer).unwrap();
    println!("{:#?}", program.stmt);
    CodeGen::new(program).compile().unwrap();
}
