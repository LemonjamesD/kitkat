use crate::lexer::Span;

#[derive(Debug, Clone)]
pub struct Program {
    pub stmt: Vec<Expr>
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub span: Span,
    pub node: Box<Expr_>,
}

#[derive(Debug, Clone)]
pub enum Expr_ {
    Function {
        attrs: Vec<Expr>,
        name: String,
        type_signature: Vec<(Option<String>, Expr)>,
        body: Expr,
    },

    // Attributes
    Public,
    Private,
    Eager,
    Lazy,
    Unsafe,
    Impure,
    Pure,
    NoError,
 
    EmptyTuple,
    Byte,
    Type(String),

    FunctionCalls(String, Vec<Expr>)
    Var(String),
    Number(i32),

    Add(Expr, Expr),
    Sub(Expr, Expr),
    Mul(Expr, Expr),
    Div(Expr, Expr),
    Return(Expr),
    Block(Vec<Expr>),
}