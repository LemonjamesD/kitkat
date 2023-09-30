use crate::ast::Program;

struct CodeGen {
    context: Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
    program: Program
}

impl<'ctx> CodeGen<'ctx> {
    fn new(program: Program) -> Result<Self, Box<dyn Error>> {
        Self {
            context: Context::create(),
            module: context.create_module("main"),
            execution_engine: module.create_jit_execution_engine(OptimizationLevel::None)?,
            program
        }
    }
    
    fn jit_compile_sum(&self) -> Option<JitFunction<SumFunc>> {
        let i64_type = self.context.i64_type();
        let fn_type = i64_type.fn_type(&vec![], false);
        let function = self.module.add_function("sum", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);

        let x = function.get_nth_param(0)?.into_int_value();
        let y = function.get_nth_param(1)?.into_int_value();
        let z = function.get_nth_param(2)?.into_int_value();

        let sum = self.builder.build_int_add(x, y, "sum").unwrap();
        let sum = self.builder.build_int_add(sum, z, "sum").unwrap();

        self.builder.build_return(Some(&sum)).unwrap();

        unsafe { self.execution_engine.get_function("sum").ok() }
    }
}
