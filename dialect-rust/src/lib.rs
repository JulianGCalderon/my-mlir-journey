use std::ptr;

use melior::{
    Context,
    dialect::DialectRegistry,
    ir::Module,
    pass::{self, PassManager},
    utility::{register_all_dialects, register_all_llvm_translations, register_all_passes},
};

use mlir_sys::{
    MlirGreedyRewriteDriverConfig, mlirApplyPatternsAndFoldGreedily, mlirFreezeRewritePattern,
    mlirPDLPatternModuleFromModule, mlirRewritePatternSetFromPDLPatternModule,
};

pub mod core;
pub mod irdl;
pub mod pdl;

pub fn initialize_context() -> Context {
    let context = Context::new();
    context.append_dialect_registry(&{
        let registry = DialectRegistry::new();
        register_all_dialects(&registry);
        registry
    });
    context.load_all_available_dialects();
    register_all_passes();
    register_all_llvm_translations(&context);
    context
}

pub fn canonicalize(context: &Context, module: &mut Module<'_>) {
    let pass_manager = PassManager::new(context);
    pass_manager.enable_verifier(true);
    pass_manager.add_pass(pass::transform::create_canonicalizer());
    pass_manager.run(module).unwrap();
}

pub fn convert_pdl_to_pdl_interop(ctx: &Context, module: &mut Module) {
    let pass_manager = PassManager::new(ctx);
    pass_manager.enable_verifier(true);
    pass_manager.add_pass(pass::conversion::create_pdl_to_pdl_interp());
    pass_manager.run(module).unwrap();
}

pub fn convert_to_llvm(context: &Context, module: &mut Module<'_>) {
    let pass_manager = PassManager::new(context);
    pass_manager.enable_verifier(true);
    pass_manager.add_pass(pass::transform::create_canonicalizer());
    pass_manager.add_pass(pass::conversion::create_to_llvm());
    pass_manager.run(module).unwrap();
}

pub fn apply_pdl_patterns(target_module: &Module, pattern_module: &Module) {
    let pdl_module = unsafe { mlirPDLPatternModuleFromModule(pattern_module.to_raw()) };
    let rewrite_patterns = unsafe { mlirRewritePatternSetFromPDLPatternModule(pdl_module) };
    let frozen_patterns = unsafe { mlirFreezeRewritePattern(rewrite_patterns) };

    unsafe {
        mlirApplyPatternsAndFoldGreedily(
            target_module.to_raw(),
            frozen_patterns,
            MlirGreedyRewriteDriverConfig {
                ptr: ptr::null_mut(),
            },
        )
    };
}
