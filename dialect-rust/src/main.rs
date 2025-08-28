use melior::{
    Context,
    dialect::{DialectRegistry, func, llvm, ods::irdl},
    helpers::{BuiltinBlockExt, GepIndex, LlvmBlockExt},
    ir::{
        Attribute, Block, BlockLike, Location, Module, Region, Type,
        attribute::{StringAttribute, TypeAttribute},
        operation::OperationBuilder,
        r#type::{FunctionType, IntegerType},
    },
    pass::{self, PassManager},
    utility::{
        load_irdl_dialects, register_all_dialects, register_all_llvm_translations,
        register_all_passes,
    },
};

fn main() {
    let context = initialize_context();

    let mut dialect_module = build_dialect_module(&context);
    canonicalize(&context, &mut dialect_module);
    println!("{}", dialect_module.as_operation());

    load_irdl_dialects(&dialect_module);

    let mut core_module = build_core_module(&context);
    canonicalize(&context, &mut core_module);
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
                    OperationBuilder::new("arith.addi", location)
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
