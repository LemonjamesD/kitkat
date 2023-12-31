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
extern new_line :: ();
extern print_fizz :: ();
extern print_buzz :: ();
    
fizz_buzz :: start:u8 -> to:u8 -> () = {
    for (let i: u8 = start; (to + 1) > i; i = i + 1;) {
        let changed: u8 = 0;
        if ((i % 3) == 0) {
            [print_fizz]();
            changed = 1;
        }
        if ((i % 5) == 0) {
            [print_buzz]();
            changed = 1;
        }
        if (changed == 0) {
            [print_int](i);
        }
        [new_line]();
        // [free](changed);
    }
    return ();
};

main :: u8 = {
    [fizz_buzz](1 100);
    return 0;
};
"#;
    let s2 = "::: = { ::: = {:::} };";
    let lexer = Lexer::new(&s).inspect(|tok| println!("tok: {:?}", tok));
    let program = parse(lexer).unwrap();
    println!("{:#?}", program.stmt);
    CodeGen::new(program).compile().unwrap();
}
