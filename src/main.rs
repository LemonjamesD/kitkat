mod parser;
mod lexer;
mod ast;
mod compiler;

use lexer::Lexer;
use parser::parse;
use compiler::CodeGen;

fn main() {
    let s = r#"
extern print_int :: num:u8 -> ();
    
cool :: x:u8 -> u8 = {
    if (x == 10) {
        return 10 + x;
    }
    return x;
};

main :: u8 = {
    for (let i: u8 = 1; 10 > i; i = i + 1;) {
        [print_int](i);
    }
    return i;
};
"#;
    let lexer = Lexer::new(&s).inspect(|tok| println!("tok: {:?}", tok));
    let program = parse(lexer).unwrap();
    println!("{:#?}", program.stmt);
    CodeGen::new(program).compile().unwrap();
}
