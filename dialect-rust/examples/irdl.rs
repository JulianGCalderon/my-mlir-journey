use melior::{
    Context,
    dialect::{DialectRegistry, arith, func},
    helpers::BuiltinBlockExt,
    ir::{
        Attribute, Block, BlockLike, Location, Module, Region, Type,
        attribute::{StringAttribute, TypeAttribute},
        operation::OperationBuilder,
        r#type::FunctionType,
    },
    pass::{self, PassManager},
    utility::{
        load_irdl_dialects, register_all_dialects, register_all_llvm_translations,
        register_all_passes,
    },
};

use melior::dialect::ods::irdl;

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

fn build_dialect_module(ctx: &'_ Context) -> Module<'_> {
    let location = Location::unknown(ctx);
    let dialect_module = Module::new(location);

    dialect_module.body().append_operation(
        irdl::dialect(
            ctx,
            {
                let region = Region::new();
                let block = region.append_block(Block::new(&[]));

                let irdl_attribute_type = Type::parse(ctx, "!irdl.attribute").unwrap();

                block.append_operation(
                    irdl::_operation(
                        ctx,
                        {
                            let region = Region::new();
                            let block = region.append_block(Block::new(&[]));

                            let is_f64 = block
                                .append_op_result(
                                    irdl::is(
                                        ctx,
                                        irdl_attribute_type,
                                        TypeAttribute::new(Type::float64(ctx)).into(),
                                        location,
                                    )
                                    .into(),
                                )
                                .unwrap();

                            let variadicity =
                                Attribute::parse(ctx, "#irdl<variadicity_array[ single,  single]>")
                                    .unwrap();
                            block.append_operation(
                                irdl::operands(ctx, &[is_f64, is_f64], variadicity, location)
                                    .into(),
                            );

                            let variadicity =
                                Attribute::parse(ctx, "#irdl<variadicity_array[ single ]>")
                                    .unwrap();
                            block.append_operation(
                                irdl::results(ctx, &[is_f64], variadicity, location).into(),
                            );

                            region
                        },
                        StringAttribute::new(ctx, "mul"),
                        location,
                    )
                    .into(),
                );

                region
            },
            StringAttribute::new(ctx, "cmath"),
            location,
        )
        .into(),
    );

    dialect_module
}

fn build_core_module(ctx: &'_ Context) -> Module<'_> {
    let location = Location::unknown(ctx);
    let core_module = Module::new(location);
    core_module.body().append_operation(func::func(
        ctx,
        StringAttribute::new(ctx, "main"),
        TypeAttribute::new(
            FunctionType::new(ctx, &[Type::float64(ctx)], &[Type::float64(ctx)]).into(),
        ),
        {
            let region = Region::new();
            let block = region.append_block(Block::new(&[(Type::float64(ctx), location)]));

            let k1 = block
                .append_op_result(arith::constant(
                    ctx,
                    Attribute::parse(ctx, "1.0 : f64").unwrap(),
                    location,
                ))
                .unwrap();

            let result = block
                .append_op_result(
                    OperationBuilder::new("cmath.mul", location)
                        .add_operands(&[block.arg(0).unwrap(), k1])
                        .add_results(&[Type::float64(ctx)])
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
    core_module
}

fn canonicalize(context: &Context, module: &mut Module<'_>) {
    let pass_manager = PassManager::new(context);
    pass_manager.enable_verifier(true);
    pass_manager.add_pass(pass::transform::create_canonicalizer());
    pass_manager.run(module).unwrap();
}
