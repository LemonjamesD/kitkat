use crate::ast::Program;
use crate::ast::{Expr, Expr_::{self, *}};

use std::collections::HashMap;
use std::error::Error;

use inkwell::*;
use inkwell::context::*;
use inkwell::module::*;
use inkwell::builder::*;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::types::*;
use inkwell::values::*;

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
        
        let new_module = self._compile(self.program.stmt.clone(), &context, module, &builder, None, HashMap::new(), &mut HashMap::new());
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
        function: Option<FunctionValue<'ctx>>,
        params: HashMap<String, u32>,
        variables: &mut HashMap<String, (BasicTypeEnum<'ctx>, PointerValue<'ctx>)>
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
                        AnyTypeEnum::IntType(int) => int.fn_type(&types[0..types.len()].iter().map(|x| (x.1).into()).collect::<Vec<_>>(), false),
                        AnyTypeEnum::VoidType(void) => void.fn_type(&types[0..types.len()].iter().map(|x| (x.1).into()).collect::<Vec<_>>(), false),
                        _ => todo!(),
                    };
                    let function = module.add_function(&name, fn_type, None);
                    let entry = context.append_basic_block(function, "entry");
                    builder.position_at_end(entry);

                    let mut hash = HashMap::new();

                    // Gets the idx of each arg
                    for (i, (arg_name, _)) in types.into_iter().enumerate() {
                        hash.insert(arg_name, i as u32);
                    }

                    let mut vars = HashMap::new();
                    module = self._compile(match *body.node {
                        Block(exprs_prime) => exprs_prime,
                        _ => todo!()
                    }, context, module, &builder, Some(function), hash, &mut vars);
                },
                If(cond, block) => {
                    let then_block = context.append_basic_block(function.unwrap(), "then");
                    let else_block = context.append_basic_block(function.unwrap(), "else");
                    builder.build_conditional_branch(
                        resolve_int_value(cond, context, &module, builder, function, params.clone(), variables.clone()),
                        then_block,
                        else_block,
                    ).unwrap();

                    module = self._compile(block, context, module, builder, function, params.clone(), variables);
                    builder.position_at_end(else_block);
                },
                For { init, cond, end, body } => {
                    let init_block = context.append_basic_block(function.unwrap(), "init_for");
                    let end_block = context.append_basic_block(function.unwrap(), "end_for");
                    let escape_block = context.append_basic_block(function.unwrap(), "escape_for");

                    builder.build_unconditional_branch(init_block);
                    builder.position_at_end(init_block);
                    module = self._compile(vec![init], context, module, builder, function, params.clone(), variables);
                    builder.build_unconditional_branch(end_block);
                    builder.position_at_end(end_block);
                    module = self._compile(body, context, module, builder, function, params.clone(), variables);
                    module = self._compile(vec![end], context, module, builder, function, params.clone(), variables);
                    builder.build_conditional_branch(
                        resolve_int_value(cond, context, &module, builder, function, params.clone(), variables.clone()),
                        end_block,
                        escape_block
                    );
                    builder.position_at_end(escape_block);

                },
                VarAssign { name, var_type, value } => {
                    let var_type = get_type_single(var_type, context);
                    let ptr = builder.build_alloca(var_type, &name).unwrap();
        
                    variables.insert(name.clone(), (var_type, ptr));

                    let value = resolve_value(value, context, &module, builder, function, params.clone(), variables.clone());
                    builder.build_store(ptr, value).unwrap();
                },
                VarReassign(name, value) => {
                    let (var_type, ptr) = variables.get(&name).unwrap();

                    let value = resolve_value(value, context, &module, builder, function, params.clone(), variables.clone());
                    builder.build_store(*ptr, value);
                }
                Return(expr) => {
                    builder.build_return(Some(&resolve_value(expr, context, &module, builder, function, params.clone(), variables.clone()))).unwrap();
                }
                _ => todo!(),
            }
        }
        return module;
    }
}

fn resolve_value<'ctx>(
    expr: Expr,
    context: &'ctx Context,
    module: &Module<'ctx>,
    builder: &Builder<'ctx>,
    function: Option<FunctionValue<'ctx>>,
    params: HashMap<String, u32>,
    variables: HashMap<String, (BasicTypeEnum<'ctx>, PointerValue<'ctx>)>
 ) -> BasicValueEnum<'ctx> {
    match *expr.node {
        Number(_) => BasicValueEnum::IntValue(resolve_int_value(expr, context, module, builder, function, params, variables.clone())),
        Var(name) => {
            if let Some(idx) = params.get(&name) {
                function.unwrap().get_nth_param(*idx).unwrap()
            } else {
                let (var_type, var_ptr) = variables.get(&name).unwrap();
                builder.build_load(*var_type, *var_ptr, "loaded_value").unwrap()
            }
        },
        FunctionCall(name, args) => {
            let args = args.into_iter().map(|x| BasicMetadataValueEnum::from(resolve_value(x, context, module, builder, function.clone(), params.clone(), variables.clone()))).collect::<Vec<_>>();
            let function = module.get_function(&name).unwrap();
            
            builder.build_call(function, &args[..], &name).unwrap().try_as_basic_value().unwrap_left()
        },
        Add(_, _) => BasicValueEnum::IntValue(
            resolve_int_value(expr, context, module, builder, function, params, variables.clone())
        ),
        Sub(_, _) => BasicValueEnum::IntValue(
            resolve_int_value(expr, context, module, builder, function, params, variables.clone())
        ),
        Mul(_, _) => BasicValueEnum::IntValue(
            resolve_int_value(expr, context, module, builder, function, params, variables.clone())
        ),
        Div(_, _) => BasicValueEnum::IntValue(
            resolve_int_value(expr, context, module, builder, function, params, variables.clone())
        ),
        Eq(_, _) => BasicValueEnum::IntValue(
            resolve_int_value(expr, context, module, builder, function, params, variables.clone())
        ),
        NEq(_, _) => BasicValueEnum::IntValue(
            resolve_int_value(expr, context, module, builder, function, params, variables.clone())
        ),
        Gt(_, _) => BasicValueEnum::IntValue(
            resolve_int_value(expr, context, module, builder, function, params, variables.clone())
        ),
        Lt(_, _) => BasicValueEnum::IntValue(
            resolve_int_value(expr, context, module, builder, function, params, variables.clone())
        ),
        _ => todo!()
    }
}

fn resolve_int_value<'ctx>(
    expr: Expr, 
    context: &'ctx Context, 
    module: &Module<'ctx>,
    builder: &Builder<'ctx>,
    function: Option<FunctionValue<'ctx>>,
    params: HashMap<String, u32>,
    variables: HashMap<String, (BasicTypeEnum<'ctx>, PointerValue<'ctx>)>
) -> IntValue<'ctx> {
    match *expr.node {
        Number(number) => context.i8_type().const_int(number as u64, false),
        Add(x, y) => builder.build_int_add::<IntValue>(
            resolve_int_value(x, context, module, builder, function, params.clone(), variables.clone()),
            resolve_int_value(y, context, module, builder, function, params, variables.clone()),
            "added_value"
        ).unwrap(),
        Sub(x, y) => builder.build_int_sub::<IntValue>(
            resolve_int_value(x, context, module, builder, function, params.clone(), variables.clone()),
            resolve_int_value(y, context, module, builder, function, params, variables.clone()),
            "subbed_value"
        ).unwrap(),
        Mul(x, y) => builder.build_int_mul::<IntValue>(
            resolve_int_value(x, context, module, builder, function, params.clone(), variables.clone()),
            resolve_int_value(y, context, module, builder, function, params, variables.clone()),
            "multiplied_value"
        ).unwrap(),
        Div(x, y) => builder.build_int_unsigned_div::<IntValue>(
            resolve_int_value(x, context, module, builder, function, params.clone(), variables.clone()),
            resolve_int_value(y, context, module, builder, function, params, variables.clone()),
            "divided_value"
        ).unwrap(),
        Eq(x, y) => builder.build_int_compare::<IntValue>(
                IntPredicate::EQ,
            resolve_int_value(x, context, module, builder, function, params.clone(), variables.clone()),
            resolve_int_value(y, context, module, builder, function, params, variables.clone()),
            "equaled_value"
        ).unwrap(),
        NEq(x, y) => builder.build_int_compare::<IntValue>(
                IntPredicate::NE,
            resolve_int_value(x, context, module, builder, function, params.clone(), variables.clone()),
            resolve_int_value(y, context, module, builder, function, params, variables.clone()),
            "not_equaled_value"
        ).unwrap(),
        Gt(x, y) => builder.build_int_compare::<IntValue>(
                IntPredicate::UGT,
            resolve_int_value(x, context, module, builder, function, params.clone(), variables.clone()),
            resolve_int_value(y, context, module, builder, function, params, variables.clone()),
            "gt_value"
        ).unwrap(),
        Lt(x, y) => builder.build_int_compare::<IntValue>(
                IntPredicate::ULT,
            resolve_int_value(x, context, module, builder, function, params.clone(), variables.clone()),
            resolve_int_value(y, context, module, builder, function, params, variables.clone()),
            "lt_value"
        ).unwrap(),
        Var(name) => {
            if let Some(idx) = params.get(&name) {
                match function.unwrap().get_nth_param(*idx).unwrap() {
                    BasicValueEnum::IntValue(int) => int,
                    _ => panic!("You can't do an operation on an {{integer}} to a non int")
                }
            } else {
                let (var_type, var_ptr) = variables.get(&name).unwrap();
                match builder.build_load(*var_type, *var_ptr, "loaded_value").unwrap() {
                    BasicValueEnum::IntValue(int) => int,
                    _ => panic!("You can't do an operation on an {{integer}} to a non int")
                }
            }
        },
        FunctionCall(name, args) => {
            let args = args.into_iter().map(|x| BasicMetadataValueEnum::from(resolve_value(x, context, module, builder, function.clone(), params.clone(), variables.clone()))).collect::<Vec<_>>();
            let function = module.get_function(&name).unwrap();
            
            match builder.build_call(function, &args[..], &name).unwrap().try_as_basic_value().unwrap_left() {
                BasicValueEnum::IntValue(int) => int,
                _ => panic!("You can't do an operation on an {{integer}} to a non int")
            }
        },
        _ => todo!(),
    }
}

fn get_type_single<'ctx>(expr: Expr, context: &'ctx Context) -> BasicTypeEnum<'ctx> {
    match *expr.node {
        Byte => BasicTypeEnum::IntType(context.i8_type()),
        EmptyTuple => todo!(),
        _ => unreachable!(),
    }
}

fn get_type<'ctx>(exprs: Vec<(Option<String>, Expr)>, context: &'ctx Context) -> (Vec<(String, BasicTypeEnum<'ctx>)>, AnyTypeEnum<'ctx>) {
    let mut returned = vec![];
    for expr in exprs[0..exprs.len() - 1].to_vec() {
        let expr_ = expr.1.node;
        returned.push(match *expr_ {
            Byte => (expr.0.unwrap(), BasicTypeEnum::IntType(context.i8_type())),
            EmptyTuple => break,
            _ => unreachable!(),
        });
    }

    let return_type = match *exprs[exprs.len() - 1].1.node {
        Byte => AnyTypeEnum::IntType(context.i8_type()),
        EmptyTuple => AnyTypeEnum::VoidType(context.void_type()),
        _ => unreachable!(),
    };
    
    (returned, return_type)
}