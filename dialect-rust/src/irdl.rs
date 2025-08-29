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

pub fn load_dialect_module(ctx: &'_ Context) -> Module<'_> {
    Module::parse(
        ctx,
        "\
        module {
          irdl.dialect @felt {
            irdl.operation @add {
              %0 = irdl.is i32
              irdl.operands(%0, %0)
              irdl.results(%0)
            }
          }
        }",
    )
    .unwrap()
}

pub fn build_dialect_module(ctx: &'_ Context) -> Module<'_> {
    let location = Location::unknown(ctx);
    let module = Module::new(location);

    let irdl_attribute_type = Type::parse(ctx, "!irdl.attribute").unwrap();
    let u32_type: Type<'_> = IntegerType::new(ctx, 32).into();
    let u32_type_attribute: Attribute<'_> = TypeAttribute::new(u32_type).into();

    module.body().append_operation(
        irdl::dialect(
            ctx,
            {
                let region = Region::new();
                let block = region.append_block(Block::new(&[]));

                block.append_operation(
                    irdl::_operation(
                        ctx,
                        {
                            let region = Region::new();
                            let block = region.append_block(Block::new(&[]));

                            let is_u32 = block
                                .append_op_result(
                                    irdl::is(
                                        ctx,
                                        irdl_attribute_type,
                                        u32_type_attribute,
                                        location,
                                    )
                                    .into(),
                                )
                                .unwrap();

                            block.append_operation(
                                irdl::operands(
                                    ctx,
                                    &[is_u32, is_u32],
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
                                    &[is_u32],
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

#[cfg(test)]
mod test {
    use crate::{
        initialize_context,
        irdl::{build_dialect_module, load_dialect_module},
    };

    #[test]
    fn equal_load_and_build() {
        let context = initialize_context();
        let builded_module = build_dialect_module(&context);
        let loaded_module = load_dialect_module(&context);
        assert_eq!(
            builded_module.as_operation().to_string(),
            loaded_module.as_operation().to_string()
        )
    }
}
