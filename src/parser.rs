use crate::lexer::Token::*;
use crate::lexer::*;
use crate::ast::*;
use plex::parser;

parser! {
    fn parse_(Token, Span);

    (a, b) {
        Span {
            lo: a.lo,
            hi: b.hi,
        }
    }

    program: Program {
        statements[s] => Program { stmt: s }
    }

    statements: Vec<Expr> {
        => vec![],
        statements[mut st] toplevel[e] => {
            st.push(e);
            st
        }
    }

    toplevel: Expr {
        function_attrs[attrs] Ident(name) DoubleColon type_sig[sig] Equal body[block] Semi => Expr {
            span: span!(),
            node: Box::new(Expr_::Function {
                attrs,
                name,
                type_signature: sig,
                body: Expr {
                    span: span!(),
                    node: Box::new(Expr_::Block(block)),
                }
            })
        },
        Extern function_attrs[attrs] Ident(name) DoubleColon type_sig[sig] Semi => Expr {
            span: span!(),
            node: Box::new(Expr_::BlankFunction {
                attrs,
                name,
                type_signature: sig
            })
        },
    }

    type_sig: Vec<(Option<std::string::String>, Expr)> {
        => vec![],
        _type[_type] => vec![(None, _type)],
        #[overriding]
        Ident(string) Colon _type[_type] RArrow type_sig[mut sig] => {
            sig.push((Some(string), _type));
            sig
        }
        _type[_type] => {
            vec![(None, _type)]
        }
    }

    _type: Expr {
        Ident(string) => Expr {
            span: span!(),
            node: Box::new(Expr_::Type(string))
        },
        Byte => Expr {
            span: span!(),
            node: Box::new(Expr_::Byte)
        },
        EmptyTuple => Expr {
            span: span!(),
            node: Box::new(Expr_::EmptyTuple)
        },
    }

    body: Vec<Expr> {
        LBrace equations[equations] RBrace => equations
    }

    equations: Vec<Expr> {
        => vec![],
        equations[mut equations] assign[contents] => {
            equations.push(contents);
            equations
        }
    }

    assign: Expr {
        Let Ident(name) Colon _type[var_type] Equal term[value] Semi => Expr {
            span: span!(),
            node: Box::new(Expr_::VarAssign {
                name,
                var_type,
                value
            })
        },
        Return term[a] Semi => Expr {
            span: span!(),
            node: Box::new(Expr_::Return(a)),
        },
        Ident(name) Equal term[a] Semi => Expr {
            span: span!(),
            node: Box::new(Expr_::VarReassign(name, a)),
        },
        #[overriding]
        LBracket Ident(i) RBracket LParen function_call[a] RParen Semi => Expr {
            span: span!(),
            node: Box::new(Expr_::FunctionCall(i, a))
        },
        LBracket Ident(i) RBracket EmptyTuple Semi => Expr {
            span: span!(),
            node: Box::new(Expr_::FunctionCall(i, vec![]))
        },
        LBracket Free RBracket LParen function_call[a] RParen Semi => Expr {
            span: span!(),
            node: Box::new(Expr_::Free(a))
        },
        #[overriding]
        term[a] => a,
    }

    term: Expr {
        term[lhs] Ampersand Ampersand fact[rhs] => Expr {
            span: span!(),
            node: Box::new(Expr_::And(lhs, rhs))
        },
        term[lhs] Bar Bar fact[rhs] => Expr {
            span: span!(),
            node: Box::new(Expr_::Or(lhs, rhs))
        },
        term[lhs] Percent fact[rhs] => Expr {
            span: span!(),
            node: Box::new(Expr_::Mod(lhs, rhs))
        },
        term[lhs] Equality fact[rhs] => Expr {
            span: span!(),
            node: Box::new(Expr_::Eq(lhs, rhs))
        },
        term[lhs] NotEquality fact[rhs] => Expr {
            span: span!(),
            node: Box::new(Expr_::NEq(lhs, rhs))
        },
        term[lhs] Gt fact[rhs] => Expr {
            span: span!(),
            node: Box::new(Expr_::Gt(lhs, rhs))
        },
        term[lhs] Plus fact[rhs] => Expr {
            span: span!(),
            node: Box::new(Expr_::Add(lhs, rhs))
        },
        term[lhs] Minus fact[rhs] => Expr {
            span: span!(),
            node: Box::new(Expr_::Sub(lhs, rhs))
        },
        #[overriding]
        fact[a] => a
    }

    fact: Expr {
        term[lhs] Star atom[rhs] => Expr {
            span: span!(),
            node: Box::new(Expr_::Mul(lhs, rhs))
        },
        term[lhs] Slash atom[rhs] => Expr {
            span: span!(),
            node: Box::new(Expr_::Div(lhs, rhs))
        },
        #[overriding]
        atom[a] => a,
    }

    atom: Expr {
        #[overriding]
        Ident(i) => Expr {
            span: span!(),
            node: Box::new(Expr_::Var(i))
        },
        EmptyTuple => Expr {
            span: span!(),
            node: Box::new(Expr_::EmptyTuple)
        },
        Integer(i) => Expr {
            span: span!(),
            node: Box::new(Expr_::Number(i))
        },
        LBracket Ident(i) RBracket LParen function_call[a] RParen => Expr {
            span: span!(),
            node: Box::new(Expr_::FunctionCall(i, a))
        },
        If LParen term[a] RParen body[body] => Expr {
            span: span!(),
            node: Box::new(Expr_::If(a, body))
        },
        #[overriding]
        If LParen term[a] RParen body[body1] Else body[body2] => Expr {
            span: span!(),
            node: Box::new(Expr_::IfElse(a, body1, body2))
        },
        For LParen assign[init] term[cond] Semi assign[end] RParen body[body] => Expr {
            span: span!(),
            node: Box::new(Expr_::For { init, cond, end, body })
        },
        #[overriding]
        LParen term[a] RParen => a
    }

    function_call: Vec<Expr> {
        => vec![],
        term[arg] function_call[mut args] => {
            args.push(arg);
            args
        }
    }

    function_attrs: Vec<Expr> {
        attr[attr] function_attrs[mut attrs] => { attrs.push(attr); attrs },
        => vec![],
    }

    attr: Expr {
        Private => Expr {
            span: span!(),
            node: Box::new(Expr_::Private)
        },
        Public => Expr {
            span: span!(),
            node: Box::new(Expr_::Public)
        },
        Lazy => Expr {
            span: span!(),
            node: Box::new(Expr_::Lazy)
        },
        Eager => Expr {
            span: span!(),
            node: Box::new(Expr_::Eager)
        },
        Unsafe => Expr {
            span: span!(),
            node: Box::new(Expr_::Unsafe)
        },
        Impure => Expr {
            span: span!(),
            node: Box::new(Expr_::Impure)
        },
        Pure => Expr {
            span: span!(),
            node: Box::new(Expr_::Pure)
        },
        NoError => Expr {
            span: span!(),
            node: Box::new(Expr_::NoError)
        }
    }
}

pub fn parse<I: Iterator<Item = (Token, Span)>>(
    i: I,
) -> Result<Program, (Option<(Token, Span)>, &'static str)> {
    parse_(i)
}