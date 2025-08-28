use std::ptr;

use melior::{
    Context,
    dialect::{DialectRegistry, func},
    helpers::{ArithBlockExt, BuiltinBlockExt},
    ir::{
        Block, BlockLike, Identifier, Location, Module, Region, Type,
        attribute::{
            ArrayAttribute, DenseI32ArrayAttribute, IntegerAttribute, StringAttribute,
            TypeAttribute,
        },
        operation::OperationBuilder,
        r#type::{FunctionType, IntegerType},
    },
    pass::{self, PassManager},
    utility::{register_all_dialects, register_all_llvm_translations, register_all_passes},
};

use mlir_sys::{
    MlirGreedyRewriteDriverConfig, mlirApplyPatternsAndFoldGreedily, mlirFreezeRewritePattern,
    mlirPDLOperationTypeGet, mlirPDLPatternModuleFromModule, mlirPDLTypeTypeGet,
    mlirPDLValueTypeGet, mlirRewritePatternSetFromPDLPatternModule,
};

use melior::dialect::ods::pdl;

fn main() {
    let context = initialize_context();

    let mut pattern_module = build_pattern_module(&context);
    println!("{}", pattern_module.as_operation());

    convert_pdl_to_pdl_interop(&context, &mut pattern_module);
    println!("{}", pattern_module.as_operation());

    let core_module = build_core_module(&context);
    println!("{}", core_module.as_operation());

    apply_pdl_patterns(&core_module, &pattern_module);
    println!("{}", core_module.as_operation());
}

fn initialize_context() -> Context {
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

fn build_pattern_module(ctx: &'_ Context) -> Module<'_> {
    let location = Location::unknown(ctx);
    let pattern_module = Module::new(location);

    pattern_module.body().append_operation(
        melior::dialect::ods::pdl::PatternOperation::builder(ctx, location)
            .benefit(IntegerAttribute::new(IntegerType::new(ctx, 16).into(), 1))
            .body_region({
                let region = Region::new();
                let block = region.append_block(Block::new(&[]));

                let pdl_type_type = unsafe { Type::from_raw(mlirPDLTypeTypeGet(ctx.to_raw())) };
                let pdl_value_type = unsafe { Type::from_raw(mlirPDLValueTypeGet(ctx.to_raw())) };
                let pdl_operation_type =
                    unsafe { Type::from_raw(mlirPDLOperationTypeGet(ctx.to_raw())) };

                let result = block
                    .append_op_result(pdl::r#type(ctx, pdl_type_type, location).into())
                    .unwrap();
                let operand1 = block
                    .append_op_result(pdl::operand(ctx, pdl_value_type, location).into())
                    .unwrap();
                let operand2 = block
                    .append_op_result(pdl::operand(ctx, pdl_value_type, location).into())
                    .unwrap();

                let operation = block
                    .append_op_result(
                        OperationBuilder::new("pdl.operation", location)
                            .add_operands(&[operand1, operand2, result])
                            .add_attributes(&[
                                (
                                    Identifier::new(ctx, "operandSegmentSizes"),
                                    DenseI32ArrayAttribute::new(ctx, &[2, 0, 1]).into(),
                                ),
                                (
                                    Identifier::new(ctx, "attributeValueNames"),
                                    ArrayAttribute::new(ctx, &[]).into(),
                                ),
                            ])
                            .add_results(&[pdl_operation_type])
                            .build()
                            .unwrap(),
                    )
                    .unwrap();

                block.append_operation(
                    OperationBuilder::new("pdl.rewrite", location)
                        .add_operands(&[operation])
                        .add_attributes(&[(
                            Identifier::new(ctx, "operandSegmentSizes"),
                            DenseI32ArrayAttribute::new(ctx, &[1, 0]).into(),
                        )])
                        .add_regions([{
                            let region = Region::new();
                            let block = region.append_block(Block::new(&[]));

                            block.append_operation(
                                OperationBuilder::new("pdl.replace", location)
                                    .add_operands(&[operation, operand1])
                                    .add_attributes(&[(
                                        Identifier::new(ctx, "operandSegmentSizes"),
                                        DenseI32ArrayAttribute::new(ctx, &[1, 0, 1]).into(),
                                    )])
                                    .build()
                                    .unwrap(),
                            );

                            region
                        }])
                        .build()
                        .unwrap(),
                );

                region
            })
            .build()
            .into(),
    );

    pattern_module
}

fn build_core_module(ctx: &'_ Context) -> Module<'_> {
    let location = Location::unknown(ctx);
    let core_module = Module::new(location);
    core_module.body().append_operation(func::func(
        ctx,
        StringAttribute::new(ctx, "main"),
        TypeAttribute::new(
            FunctionType::new(
                ctx,
                &[IntegerType::new(ctx, 32).into()],
                &[IntegerType::new(ctx, 32).into()],
            )
            .into(),
        ),
        {
            let region = Region::new();
            let block =
                region.append_block(Block::new(&[(IntegerType::new(ctx, 32).into(), location)]));

            let k1 = block.const_int(ctx, location, 1, 32).unwrap();
            let k3 = block
                .addi(block.argument(0).unwrap().into(), k1, location)
                .unwrap();

            block.append_operation(func::r#return(&[k3], location));

            region
        },
        &[],
        location,
    ));
    core_module
}

fn convert_pdl_to_pdl_interop(ctx: &Context, module: &mut Module) {
    let pass_manager = PassManager::new(ctx);
    pass_manager.enable_verifier(true);
    pass_manager.add_pass(pass::conversion::create_pdl_to_pdl_interp());
    pass_manager.run(module).unwrap();
}

fn apply_pdl_patterns(target_module: &Module, pattern_module: &Module) {
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
