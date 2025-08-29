use melior::{
    Context,
    dialect::func,
    helpers::BuiltinBlockExt,
    ir::{
        Block, BlockLike, Location, Module, Region, Type,
        attribute::{StringAttribute, TypeAttribute},
        operation::OperationBuilder,
        r#type::{FunctionType, IntegerType},
    },
};

pub fn build_core_module(ctx: &'_ Context) -> Module<'_> {
    let location = Location::unknown(ctx);
    let module = Module::new(location);

    let u32_type: Type<'_> = IntegerType::new(ctx, 32).into();

    module.body().append_operation(func::func(
        ctx,
        StringAttribute::new(ctx, "entrypoint"),
        TypeAttribute::new(FunctionType::new(ctx, &[u32_type, u32_type], &[u32_type]).into()),
        {
            let region = Region::new();
            let block =
                region.append_block(Block::new(&[(u32_type, location), (u32_type, location)]));

            let v1 = block.arg(1).unwrap();
            let v2 = block.arg(1).unwrap();

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
