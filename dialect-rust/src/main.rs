use std::ptr;

use melior::{
    Context,
    dialect::{
        DialectRegistry, func, llvm,
        ods::{irdl, pdl},
    },
    helpers::{BuiltinBlockExt, GepIndex, LlvmBlockExt},
    ir::{
        Attribute, Block, BlockLike, Identifier, Location, Module, Region, Type,
        attribute::{
            ArrayAttribute, DenseI32ArrayAttribute, IntegerAttribute, StringAttribute,
            TypeAttribute,
        },
        operation::OperationBuilder,
        r#type::{FunctionType, IntegerType},
    },
    pass::{self, PassManager},
    utility::{
        load_irdl_dialects, register_all_dialects, register_all_llvm_translations,
        register_all_passes,
    },
};
use mlir_sys::{
    MlirGreedyRewriteDriverConfig, mlirApplyPatternsAndFoldGreedily, mlirFreezeRewritePattern,
    mlirPDLOperationTypeGet, mlirPDLPatternModuleFromModule, mlirPDLTypeTypeGet,
    mlirPDLValueTypeGet, mlirRewritePatternSetFromPDLPatternModule,
};

fn main() {
    let context = initialize_context();

    let mut dialect_module = build_dialect_module(&context);
    canonicalize(&context, &mut dialect_module);
    eprintln!("{}", dialect_module.as_operation());

    load_irdl_dialects(&dialect_module);

    let mut core_module = build_core_module(&context);
    canonicalize(&context, &mut core_module);
    eprintln!("{}", core_module.as_operation());

    let mut pattern_module = build_pattern_module(&context);
    canonicalize(&context, &mut pattern_module);
    eprintln!("{}", pattern_module.as_operation());

    convert_pdl_to_pdl_interop(&context, &mut pattern_module);
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

fn canonicalize(context: &Context, module: &mut Module<'_>) {
    let pass_manager = PassManager::new(context);
    pass_manager.enable_verifier(true);
    pass_manager.add_pass(pass::transform::create_canonicalizer());
    pass_manager.run(module).unwrap();
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

fn build_dialect_module(ctx: &'_ Context) -> Module<'_> {
    let location = Location::unknown(ctx);
    let module = Module::new(location);

    module.body().append_operation(
        irdl::dialect(
            ctx,
            {
                let region = Region::new();
                let block = region.append_block(Block::new(&[]));

                let irdl_attribute_type = Type::parse(ctx, "!irdl.attribute").unwrap();
                let u32_type: Type<'_> = IntegerType::new(ctx, 32).into();

                block.append_operation(
                    irdl::_operation(
                        ctx,
                        {
                            let region = Region::new();
                            let block = region.append_block(Block::new(&[]));

                            let is_u252 = block
                                .append_op_result(
                                    irdl::is(
                                        ctx,
                                        irdl_attribute_type,
                                        TypeAttribute::new(u32_type).into(),
                                        location,
                                    )
                                    .into(),
                                )
                                .unwrap();

                            block.append_operation(
                                irdl::operands(
                                    ctx,
                                    &[is_u252, is_u252],
                                    Attribute::parse(
                                        ctx,
                                        "#irdl<variadicity_array[single, single]>",
                                    )
                                    .unwrap(),
                                    location,
                                )
                                .into(),
                            );

                            block.append_operation(
                                irdl::results(
                                    ctx,
                                    &[is_u252],
                                    Attribute::parse(ctx, "#irdl<variadicity_array[single]>")
                                        .unwrap(),
                                    location,
                                )
                                .into(),
                            );

                            region
                        },
                        StringAttribute::new(ctx, "add"),
                        location,
                    )
                    .into(),
                );

                region
            },
            StringAttribute::new(ctx, "felt"),
            location,
        )
        .into(),
    );

    module
}

fn build_core_module(ctx: &'_ Context) -> Module<'_> {
    let location = Location::unknown(ctx);
    let module = Module::new(location);

    let u32_type: Type<'_> = IntegerType::new(ctx, 32).into();
    let ptr_type: Type<'_> = llvm::r#type::pointer(ctx, 0);

    module.body().append_operation(func::func(
        ctx,
        StringAttribute::new(ctx, "main"),
        TypeAttribute::new(FunctionType::new(ctx, &[u32_type, ptr_type], &[u32_type]).into()),
        {
            let region = Region::new();
            let block =
                region.append_block(Block::new(&[(u32_type, location), (ptr_type, location)]));

            let argv = block.arg(1).unwrap();

            let v1_ptr_ptr = block
                .gep(ctx, location, argv, &[GepIndex::Const(1)], ptr_type)
                .unwrap();
            let v2_ptr_ptr = block
                .gep(ctx, location, argv, &[GepIndex::Const(2)], ptr_type)
                .unwrap();

            let v1_ptr = block.load(ctx, location, v1_ptr_ptr, ptr_type).unwrap();
            let v2_ptr = block.load(ctx, location, v2_ptr_ptr, ptr_type).unwrap();

            let v1 = block.load(ctx, location, v1_ptr, u32_type).unwrap();
            let v2 = block.load(ctx, location, v2_ptr, u32_type).unwrap();

            let result = block
                .append_op_result(
                    OperationBuilder::new("felt.add", location)
                        .add_operands(&[v1, v2])
                        .add_results(&[u32_type])
                        .build()
                        .unwrap(),
                )
                .unwrap();

            block.append_operation(func::r#return(&[result], location));

            region
        },
        &[],
        location,
    ));

    module
}

fn build_pattern_module(ctx: &'_ Context) -> Module<'_> {
    let location = Location::unknown(ctx);
    let module = Module::new(location);

    module.body().append_operation(
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
                                    Identifier::new(ctx, "opName"),
                                    StringAttribute::new(ctx, "felt.add").into(),
                                ),
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

                            let arith_addi_operation = block
                                .append_op_result(
                                    OperationBuilder::new("pdl.operation", location)
                                        .add_operands(&[operand1, operand2, result])
                                        .add_attributes(&[
                                            (
                                                Identifier::new(ctx, "opName"),
                                                StringAttribute::new(ctx, "arith.addi").into(),
                                            ),
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
                                OperationBuilder::new("pdl.replace", location)
                                    .add_operands(&[operation, arith_addi_operation])
                                    .add_attributes(&[(
                                        Identifier::new(ctx, "operandSegmentSizes"),
                                        DenseI32ArrayAttribute::new(ctx, &[1, 1, 0]).into(),
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

    module
}
