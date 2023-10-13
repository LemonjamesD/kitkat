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

    FunctionCall(String, Vec<Expr>),
    VarAssign {
        name: String,
        var_type: Expr,
        value: Expr,
    },
    VarReassign(String, Expr),
    Var(String),
    Number(i32),

    If(Expr, Vec<Expr>),
    IfElse(Expr, Vec<Expr>, Vec<Expr>),

    Eq(Expr, Expr),
    NEq(Expr, Expr),
    Gt(Expr, Expr),
    Geq(Expr, Expr),
    Lt(Expr, Expr),
    Leq(Expr, Expr),
    Or(Expr, Expr),
    And(Expr, Expr),
    
    Add(Expr, Expr),
    Sub(Expr, Expr),
    Mul(Expr, Expr),
    Div(Expr, Expr),
    Return(Expr),
    Block(Vec<Expr>),
}