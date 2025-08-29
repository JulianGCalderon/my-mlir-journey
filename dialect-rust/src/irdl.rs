use melior::{
    Context,
    dialect::ods::irdl,
    helpers::BuiltinBlockExt,
    ir::{
        Attribute, Block, BlockLike, Location, Module, Region, Type,
        attribute::{StringAttribute, TypeAttribute},
        r#type::IntegerType,
    },
};

pub fn build_dialect_module(ctx: &'_ Context) -> Module<'_> {
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
