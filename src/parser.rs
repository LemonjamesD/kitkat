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
        }
    }

    type_sig: Vec<(Option<std::string::String>, Expr)> {
        => vec![],
        #[overriding]
        Ident(string) Colon _type[_type] => vec![(Some(string), _type)],
        _type[_type] => vec![(None, _type)],
        #[overriding]
        Ident(string) Colon type_sig[mut sig] RArrow _type[_type] => {
            sig.push((Some(string), _type));
            sig
        }
        type_sig[mut sig] RArrow _type[_type] => {
            sig.push((None, _type));
            sig
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
        Return term[a] Semi => Expr {
            span: span!(),
            node: Box::new(Expr_::Return(a)),
        },
        #[overriding]
        term[a] => a,
    }

    term: Expr {
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
        Ident(i) => Expr {
            span: span!(),
            node: Box::new(Expr_::Var(i))
        },
        Integer(i) => Expr {
            span: span!(),
            node: Box::new(Expr_::Number(i))
        },
        LParen term[a] RParen => a
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