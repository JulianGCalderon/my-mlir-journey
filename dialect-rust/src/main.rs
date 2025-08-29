use dialect_rust::{
    apply_pdl_patterns, canonicalize, convert_pdl_to_pdl_interop, convert_to_llvm,
    core::build_core_module, execute_entrypoint, initialize_context, irdl::build_dialect_module,
    pdl::build_pattern_module,
};
use melior::utility::load_irdl_dialects;

fn main() {
    let context = initialize_context();

    // We build the dialect module. This will contain only the dialect
    // definition, and not any conversion logic.
    let mut dialect_module = build_dialect_module(&context);
    canonicalize(&context, &mut dialect_module);
    println!("{}", dialect_module.as_operation());

    // We load the dialect into the associated context.
    // This allows use to use the dialect in other modules.
    load_irdl_dialects(&dialect_module);

    // We build the core module, using our custom dialect. Note that without
    // loading the IRDL dialects first, this step will fail.
    let mut core_module = build_core_module(&context);
    canonicalize(&context, &mut core_module);
    println!("{}", core_module.as_operation());

    // If we try to compile our core module, it will fail because our custom
    // dialect is not convertible into the llvm dialect (or any other dialect,
    // for that matter). To fix it, we need to build a pattern module that
    // declares how our custom dialect is transformed.
    let mut pattern_module = build_pattern_module(&context);
    canonicalize(&context, &mut pattern_module);
    println!("{}", pattern_module.as_operation());

    // The PDL dialect by itself cannot be applied, and needs to be converted to
    // the lower-level pdl-interop dialect.
    convert_pdl_to_pdl_interop(&context, &mut pattern_module);

    // We apply our rewrite patterns to the core module. This will rewrite our
    // custom operations with operations from known dialects.
    apply_pdl_patterns(&core_module, &pattern_module);
    println!("{}", core_module.as_operation());

    // Now that we are using known dialects, we can convert it to the LLVM
    // dialect without errors.
    convert_to_llvm(&context, &mut core_module);

    // As a test, we execute the "entrypoint" function from our core module.
    let a = 10;
    let b = 7;
    let result = execute_entrypoint(&core_module, a, b);
    println!("{a} + {b} = {result} mod 13")
}
