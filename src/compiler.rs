use crate::ast::Program;
use crate::ast::{Expr, Expr_::{self, *}};

use inkwell::*;
use inkwell::context::*;
use inkwell::module::*;
use inkwell::builder::*;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::types::*;
use std::error::Error;

struct CodeGen {
    program: Program
}

impl CodeGen {
    fn new(program: Program) -> Self {
        Self {
            program
        }
    }

    pub fn compile(&mut self) -> Result<&mut Self, Box<dyn Error>> {
        let context = Context::create();
        let module = context.create_module("main");
        let builder = context.create_builder();
        let execution_engine = module.create_execution_engine()?;
        
        self._compile(self.program.stmt.clone(), &context, module, builder, execution_engine);
        Ok(self)
    }
    
    fn _compile<'ctx>(
        &self, 
        exprs: Vec<Expr>,
        context: &'ctx Context,
        module: Module<'ctx>,
        builder: Builder<'ctx>,
        execution_engine: ExecutionEngine<'ctx>,
    ) {
        for expr in exprs {
            let expr_ = expr.node;
            match *expr_ {
                Function {
                    ..
                } => {},
                _ => todo!(),
            }
        }
    }
}

fn get_type<'ctx>(exprs: Vec<Expr>, context: &'ctx Context) -> Vec<AnyTypeEnum<'ctx>> {
    let mut returned = vec![];
    for expr in exprs {
        let expr_ = expr.node;
        returned.push(match *expr_ {
            Byte => AnyTypeEnum::IntType(context.i8_type()),
            EmptyTuple => AnyTypeEnum::VoidType(context.void_type()),
            _ => unreachable!(),
        });
    }
    returned
}