use dialect_rust::{
    apply_pdl_patterns, canonicalize, convert_pdl_to_pdl_interop, convert_to_llvm,
    core::build_core_module, initialize_context, irdl::build_dialect_module,
    pdl::build_pattern_module,
};
use melior::{ExecutionEngine, utility::load_irdl_dialects};

fn main() {
    let context = initialize_context();

    let mut dialect_module = build_dialect_module(&context);
    canonicalize(&context, &mut dialect_module);
    println!("{}", dialect_module.as_operation());

    load_irdl_dialects(&dialect_module);

    let mut core_module = build_core_module(&context);
    canonicalize(&context, &mut core_module);
    println!("{}", core_module.as_operation());

    let mut pattern_module = build_pattern_module(&context);
    canonicalize(&context, &mut pattern_module);
    println!("{}", pattern_module.as_operation());

    convert_pdl_to_pdl_interop(&context, &mut pattern_module);
    apply_pdl_patterns(&core_module, &pattern_module);
    println!("{}", core_module.as_operation());

    convert_to_llvm(&context, &mut core_module);

    {
        let execution_engine = ExecutionEngine::new(&core_module, 0, &[], false);

        let mut a: u64 = 10;
        let mut b: u64 = 7;
        let mut c: u64 = 0;
        let mut arguments = [
            &mut a as *mut u64 as *mut (),
            &mut b as *mut u64 as *mut (),
            &mut c as *mut u64 as *mut (),
        ];
        unsafe {
            execution_engine
                .invoke_packed("entrypoint", &mut arguments)
                .unwrap()
        }
        println!("{} + {} = {}", a, b, c);
    }
}
