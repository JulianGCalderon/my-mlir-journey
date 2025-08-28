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
    let module = Module::new(location);

    module.body().append_operation(
        irdl::dialect(
            ctx,
            {
                let region = Region::new();
                region.append_block(Block::new(&[]));
                region
            },
            StringAttribute::new(ctx, "cmath"),
            location,
        )
        .into(),
    );

    module
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
                    OperationBuilder::new("cmath.add", location)
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
