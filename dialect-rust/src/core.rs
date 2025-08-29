use melior::{
    Context,
    dialect::func,
    helpers::BuiltinBlockExt,
    ir::{
        Attribute, Block, BlockLike, Identifier, Location, Module, Region, Type,
        attribute::{StringAttribute, TypeAttribute},
        operation::OperationBuilder,
        r#type::{FunctionType, IntegerType},
    },
};

pub fn load_core_module(ctx: &'_ Context) -> Module<'_> {
    Module::parse(
        ctx,
        r#"
        module {
          func.func @entrypoint(%arg0: i32, %arg1: i32) -> i32 attributes {llvm.emit_c_interface} {
            %0 = "felt.add"(%arg0, %arg1) : (i32, i32) -> i32
            return %0 : i32
          }
        }"#,
    )
    .unwrap()
}

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

            let v1 = block.arg(0).unwrap();
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
        &[(
            Identifier::new(ctx, "llvm.emit_c_interface"),
            Attribute::unit(ctx),
        )],
        location,
    ));

    module
}

#[cfg(test)]
mod test {
    use melior::utility::load_irdl_dialects;

    use crate::{
        core::{build_core_module, load_core_module},
        initialize_context,
        irdl::build_dialect_module,
    };

    #[test]
    fn equal_load_and_build() {
        let context = initialize_context();

        let dialect_module = build_dialect_module(&context);
        load_irdl_dialects(&dialect_module);

        let builded_module = build_core_module(&context);
        let loaded_module = load_core_module(&context);
        assert_eq!(
            builded_module.as_operation().to_string(),
            loaded_module.as_operation().to_string()
        )
    }
}
