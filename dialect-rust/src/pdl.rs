use melior::{
    Context,
    dialect::ods::pdl,
    helpers::BuiltinBlockExt,
    ir::{
        Block, BlockLike, Identifier, Location, Module, Region, Type,
        attribute::{ArrayAttribute, DenseI32ArrayAttribute, IntegerAttribute, StringAttribute},
        operation::OperationBuilder,
        r#type::IntegerType,
    },
};
use mlir_sys::{
    mlirPDLAttributeTypeGet, mlirPDLOperationTypeGet, mlirPDLTypeTypeGet, mlirPDLValueTypeGet,
};

pub fn load_pattern_module(ctx: &'_ Context) -> Module<'_> {
    Module::parse(
        ctx,
        r#"
        module {
          pdl.pattern : benefit(1) {
            %0 = type
            %1 = operand
            %2 = operand
            %3 = operation "felt.add"(%1, %2 : !pdl.value, !pdl.value)  -> (%0 : !pdl.type)
            rewrite %3 {
              %4 = operation "arith.addi"(%1, %2 : !pdl.value, !pdl.value)  -> (%0 : !pdl.type)
              %5 = result 0 of %4
              %6 = attribute = 13 : i32
              %7 = operation "arith.constant"  {"value" = %6} -> (%0 : !pdl.type)
              %8 = result 0 of %7
              %9 = operation "arith.remui"(%5, %8 : !pdl.value, !pdl.value)  -> (%0 : !pdl.type)
              replace %3 with %9
            }
          }
        }"#,
    )
    .unwrap()
}

pub fn build_pattern_module(ctx: &'_ Context) -> Module<'_> {
    let location = Location::unknown(ctx);
    let module = Module::new(location);

    let pdl_type_type = unsafe { Type::from_raw(mlirPDLTypeTypeGet(ctx.to_raw())) };
    let pdl_value_type = unsafe { Type::from_raw(mlirPDLValueTypeGet(ctx.to_raw())) };
    let pdl_attribute_type = unsafe { Type::from_raw(mlirPDLAttributeTypeGet(ctx.to_raw())) };
    let pdl_operation_type = unsafe { Type::from_raw(mlirPDLOperationTypeGet(ctx.to_raw())) };
    let u32_type: Type<'_> = IntegerType::new(ctx, 32).into();

    module.body().append_operation(
        melior::dialect::ods::pdl::PatternOperation::builder(ctx, location)
            .benefit(IntegerAttribute::new(IntegerType::new(ctx, 16).into(), 1))
            .body_region({
                let region = Region::new();
                let block = region.append_block(Block::new(&[]));

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

                            let add_operation = block
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

                            let add_result = block
                                .append_op_result(
                                    OperationBuilder::new("pdl.result", location)
                                        .add_operands(&[add_operation])
                                        .add_attributes(&[(
                                            Identifier::new(ctx, "index"),
                                            IntegerAttribute::new(u32_type, 0).into(),
                                        )])
                                        .add_results(&[pdl_value_type])
                                        .build()
                                        .unwrap(),
                                )
                                .unwrap();

                            let k13_attribute = block
                                .append_op_result(
                                    OperationBuilder::new("pdl.attribute", location)
                                        .add_attributes(&[(
                                            Identifier::new(ctx, "value"),
                                            IntegerAttribute::new(u32_type, 13).into(),
                                        )])
                                        .add_results(&[pdl_attribute_type])
                                        .build()
                                        .unwrap(),
                                )
                                .unwrap();

                            let k13_operation = block
                                .append_op_result(
                                    OperationBuilder::new("pdl.operation", location)
                                        .add_operands(&[k13_attribute, result])
                                        .add_attributes(&[
                                            (
                                                Identifier::new(ctx, "opName"),
                                                StringAttribute::new(ctx, "arith.constant").into(),
                                            ),
                                            (
                                                Identifier::new(ctx, "operandSegmentSizes"),
                                                DenseI32ArrayAttribute::new(ctx, &[0, 1, 1]).into(),
                                            ),
                                            (
                                                Identifier::new(ctx, "attributeValueNames"),
                                                ArrayAttribute::new(
                                                    ctx,
                                                    &[StringAttribute::new(ctx, "value").into()],
                                                )
                                                .into(),
                                            ),
                                        ])
                                        .add_results(&[pdl_operation_type])
                                        .build()
                                        .unwrap(),
                                )
                                .unwrap();

                            let k13_result = block
                                .append_op_result(
                                    OperationBuilder::new("pdl.result", location)
                                        .add_operands(&[k13_operation])
                                        .add_attributes(&[(
                                            Identifier::new(ctx, "index"),
                                            IntegerAttribute::new(u32_type, 0).into(),
                                        )])
                                        .add_results(&[pdl_value_type])
                                        .build()
                                        .unwrap(),
                                )
                                .unwrap();

                            let modulo_operation = block
                                .append_op_result(
                                    OperationBuilder::new("pdl.operation", location)
                                        .add_operands(&[add_result, k13_result, result])
                                        .add_attributes(&[
                                            (
                                                Identifier::new(ctx, "opName"),
                                                StringAttribute::new(ctx, "arith.remui").into(),
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
                                    .add_operands(&[operation, modulo_operation])
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

#[cfg(test)]
mod test {
    use crate::{
        initialize_context,
        pdl::{build_pattern_module, load_pattern_module},
    };

    #[test]
    fn equal_load_and_build() {
        let context = initialize_context();
        let builded_module = build_pattern_module(&context);
        let loaded_module = load_pattern_module(&context);
        assert_eq!(
            builded_module.as_operation().to_string(),
            loaded_module.as_operation().to_string()
        )
    }
}
