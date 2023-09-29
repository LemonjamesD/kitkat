use plex::lexer;

#[derive(Debug)]
pub enum Token {
    Whitespace,
    Comment,
    Integer(i32),
    String(String),
    Ident(String),

    // Keywords
    Public,
    Eager,
    Lazy,
    Static,
    Constant,
    Void,
    Extern,
    Async,
    Await,
    Abstract,
    Final,
    Virtual,
    Yield,
    Private,
    // Purity
    Unsafe,
    Impure,
    Pure,
    // Errority
    NoError,

    // Built-in Types,
    EmptyTuple,
    Byte,

    // Arrows
    RArrow,
    LArrow,
    
    // Operators
    Equality,
    LtEqual,
    Lt,
    GtEqual,
    Gt,
    NotEquality,
    Equal,
    Star,
    Slash,
    Plus,
    Minus,
    
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    
    Semi,
    Colon,
    DoubleColon,
}

lexer! {
    fn next_token(tok: 'a) -> Token;

    r"[ \t\r\n]" => Token::Whitespace,

    r"//.*" => Token::Comment,

    // Literals
    "[0-9]+" => Token::Integer(tok.parse().unwrap()),
    r#""[^"]*""# => Token::String(tok[1..tok.len() - 1].to_owned()),

    // Built in types
    r"\(\)" => Token::EmptyTuple,
    r"u8" => Token::Byte,

    // Keywords
    r"pub" => Token::Public,
    r"eager" => Token::Eager,
    r"lazy" => Token::Lazy,
    r"static" => Token::Static,
    r"constant" => Token::Constant,
    r"void" => Token::Void,
    r"extern" => Token::Extern,
    r"async" => Token::Async,
    r"await" => Token::Await,
    r"abstract" => Token::Abstract,
    r"final" => Token::Final,
    r"virtual" => Token::Virtual,
    r"yield" => Token::Yield,
    r"private" => Token::Private,
    r"unsafe" => Token::Unsafe,
    r"impure" => Token::Impure,
    r"pure" => Token::Pure,
    // Errority
    r"noerror" => Token::NoError,

    // Ident
    r"[a-zA-Z_][a-zA-Z0-9_]*" => Token::Ident(tok.to_owned()),

    // Arrows
    r"->" => Token::RArrow,
    r"<-" => Token::LArrow,

    // Operators
    r"==" => Token::Equality,
    r">=" => Token::LtEqual,
    r">" => Token::Lt,
    r"<=" => Token::GtEqual,
    r">" => Token::Gt,
    r"!=" => Token::NotEquality,
    r"=" => Token::Equal,
    r"\*" => Token::Star,
    r"/" => Token::Slash,
    r"\+" => Token::Plus,
    r"-" => Token::Minus,

    r"\(" => Token::LParen,
    r"\)" => Token::RParen,
    r"\[" => Token::LBracket,
    r"\]" => Token::RBracket,
    r"\{" => Token::LBrace,
    r"\}" => Token::RBrace,
    
    r";" => Token::Semi,
    r"::" => Token::DoubleColon,
    r":" => Token::Colon,

    r"." => panic!("Unexpected character: {}", tok),
}

pub struct Lexer<'a> {
    original: &'a str,
    remaining: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Lexer<'a> {
        Lexer {
            original: s,
            remaining: s,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Token, Span);
    fn next(&mut self) -> Option<(Token, Span)> {
        loop {
            let (tok, span) = if let Some((tok, new_remaining)) = next_token(self.remaining) {
                let lo = self.original.len() - self.remaining.len();
                let hi = self.original.len() - new_remaining.len();
                self.remaining = new_remaining;
                (tok, Span { lo, hi })
            } else {
                return None;
            };
            match tok {
                Token::Whitespace | Token::Comment => {
                    continue;
                }
                tok => {
                    return Some((tok, span));
                }
            }
        }
    }
}
