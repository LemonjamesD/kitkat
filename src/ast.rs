use crate::lexer::Span;

#[derive(Debug)]
pub struct Program {
    pub stmt: Vec<Expr>
}

#[derive(Debug)]
pub struct Expr {
    pub span: Span,
    pub node: Box<Expr_>,
}

#[derive(Debug)]
pub enum Expr_ {
    Define {
        attrs: Vec<Expr>,
        name: Box<Expr>,
        type_signature: Expr,
    },
    Declare {
        name: Box<Expr>,
    },
    EmptyTuple,
    Byte,
    TypeSig(Vec<Expr>),
    Add(Expr, Expr),
    Sub(Expr, Expr),
    Mul(Expr, Expr),
    Div(Expr, Expr),
    Return(Vec<Expr>),
    Block(Vec<Expr>),
}