use crate::ast::Program;
use crate::ast::{Expr, Expr_::{self, *}};

use inkwell::*;
use inkwell::context::*;
use inkwell::module::*;
use inkwell::builder::*;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::types::*;
use inkwell::values::*;
use std::error::Error;

pub struct CodeGen {
    program: Program
}

impl CodeGen {
    pub fn new(program: Program) -> Self {
        Self {
            program
        }
    }

    pub fn compile(&mut self) -> Result<&mut Self, Box<dyn Error>> {
        let context = Context::create();
        let module = context.create_module("main");
        let builder = context.create_builder();
        
        let new_module = self._compile(self.program.stmt.clone(), &context, module, &builder);
        let mut path = std::path::Path::new("module.bc");
        let written = new_module.write_bitcode_to_path(&path);
        Ok(self)
    }
    
    fn _compile<'ctx>(
        &self, 
        exprs: Vec<Expr>,
        context: &'ctx Context,
        mut module: Module<'ctx>,
        builder: &Builder<'ctx>,
    ) -> Module<'ctx> {
        for expr in exprs {
            let expr_ = expr.node;
            match *expr_ {
                Function {
                    attrs,
                    name,
                    type_signature,
                    body,
                } => {
                    let (types, return_type) = get_type(type_signature, context);
                    let fn_type = match return_type {
                        AnyTypeEnum::IntType(int) => int.fn_type(&types[0..types.len()].iter().map(|x| (*x).into()).collect::<Vec<_>>(), false),
                        AnyTypeEnum::VoidType(void) => void.fn_type(&types[0..types.len()].iter().map(|x| (*x).into()).collect::<Vec<_>>(), false),
                        _ => todo!(),
                    };
                    let function = module.add_function(&name, fn_type, None);
                    let entry = context.append_basic_block(function, "entry");
                    builder.position_at_end(entry);

                    module = self._compile(match *body.node {
                        Block(exprs_prime) => exprs_prime,
                        _ => todo!()
                    }, context, module, &builder);
                },
                Return(expr) => {
                    builder.build_return(Some(&resolve_value(expr, context, builder))).unwrap();
                }
                _ => todo!(),
            }
        }
        return module;
    }
}

fn resolve_value<'ctx>(expr: Expr, context: &'ctx Context, builder: &Builder<'ctx>) -> BasicValueEnum<'ctx> {
    match *expr.node {
        Number(_) => BasicValueEnum::IntValue(resolve_int_value(expr, context, builder)),
        Add(_, _) => BasicValueEnum::IntValue(
            resolve_int_value(expr, context, builder)
        ),
        Sub(_, _) => BasicValueEnum::IntValue(
            resolve_int_value(expr, context, builder)
        ),
        Mul(_, _) => BasicValueEnum::IntValue(
            resolve_int_value(expr, context, builder)
        ),
        Div(_, _) => BasicValueEnum::IntValue(
            resolve_int_value(expr, context, builder)
        ),
        _ => todo!()
    }
}

fn resolve_int_value<'ctx>(expr: Expr, context: &'ctx Context, builder: &Builder<'ctx>) -> IntValue<'ctx> {
    match *expr.node {
        Number(number) => context.i8_type().const_int(number as u64, false),
        Add(x, y) => builder.build_int_add::<IntValue>(
            resolve_int_value(x, context, builder),
            resolve_int_value(y, context, builder),
            "added_value"
        ).unwrap(),
        Sub(x, y) => builder.build_int_sub::<IntValue>(
            resolve_int_value(x, context, builder),
            resolve_int_value(y, context, builder),
            "subbed_value"
        ).unwrap(),
        Mul(x, y) => builder.build_int_mul::<IntValue>(
            resolve_int_value(x, context, builder),
            resolve_int_value(y, context, builder),
            "multiplied_value"
        ).unwrap(),
        Div(x, y) => builder.build_int_unsigned_div::<IntValue>(
            resolve_int_value(x, context, builder),
            resolve_int_value(y, context, builder),
            "divided_value"
        ).unwrap(),
        _ => todo!(),
    }
}

fn get_type<'ctx>(exprs: Vec<Expr>, context: &'ctx Context) -> (Vec<BasicTypeEnum<'ctx>>, AnyTypeEnum<'ctx>) {
    let mut returned = vec![];
    for expr in exprs[0..exprs.len() - 1].to_vec() {
        let expr_ = expr.node;
        returned.push(match *expr_ {
            Byte => BasicTypeEnum::IntType(context.i8_type()),
            EmptyTuple => break,
            _ => unreachable!(),
        });
    }

    let return_type = match *exprs[exprs.len() - 1].node {
        Byte => AnyTypeEnum::IntType(context.i8_type()),
        EmptyTuple => AnyTypeEnum::VoidType(context.void_type()),
        _ => unreachable!(),
    };
    
    (returned, return_type)
}