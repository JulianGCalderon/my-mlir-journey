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

/// Builds the pattern module using PDL.
///
/// The built module should be equal to the one in `load_pattern_module`.
pub fn build_pattern_module(ctx: &'_ Context) -> Module<'_> {
    let location = Location::unknown(ctx);
    let module = Module::new(location);

    // PDL is a meta-dialect (a dialect that reason about MLIR itself). Here,
    // everything is a value. Types are values, values are values, attributes
    // and operations are also values.
    //
    // PDL defines custom types that determine what a value refers to.

    // To build the PDL types, we need to call the C API directly.
    let pdl_type_type = unsafe { Type::from_raw(mlirPDLTypeTypeGet(ctx.to_raw())) };
    let pdl_value_type = unsafe { Type::from_raw(mlirPDLValueTypeGet(ctx.to_raw())) };
    let pdl_attribute_type = unsafe { Type::from_raw(mlirPDLAttributeTypeGet(ctx.to_raw())) };
    let pdl_operation_type = unsafe { Type::from_raw(mlirPDLOperationTypeGet(ctx.to_raw())) };
    let u32_type: Type<'_> = IntegerType::new(ctx, 32).into();

    // We will define a single pattern, that rewrites the felt.add operation
    // into an arith.addi, followed by a arith.remui with a constant value of 13 as
    // the divisor.
    //
    // The benefit attribute states the expected benefit of applying the rewrite
    // pattern. See https://mlir.llvm.org/docs/PatternRewriter/#introduction for
    // more information.
    module.body().append_operation(
        melior::dialect::ods::pdl::PatternOperation::builder(ctx, location)
            .benefit(IntegerAttribute::new(IntegerType::new(ctx, 16).into(), 1))
            .body_region({
                let region = Region::new();
                let block = region.append_block(Block::new(&[]));

                // We declare the existence of a result type, and two operands.
                // This can be done with the pdl.type and pdl.operand operations.
                //
                // %0 = type
                // %1 = operand
                // %2 = operand

                let result = block
                    .append_op_result(pdl::r#type(ctx, pdl_type_type, location).into())
                    .unwrap();
                let operand1 = block
                    .append_op_result(pdl::operand(ctx, pdl_value_type, location).into())
                    .unwrap();
                let operand2 = block
                    .append_op_result(pdl::operand(ctx, pdl_value_type, location).into())
                    .unwrap();

                // By itself, operands and types don't mean anything. With the
                // pd.operation operation, we define a pattern that ties this
                // operands values together.
                //
                // %3 = operation "felt.add"(%1, %2)  -> (%0)

                // In MLIR, operations have operands, result values, and
                // attributes. As in PDL everything is a value, we specify all
                // of these through the pdl.operation operands.
                //
                // To let MLIR know what these values actually mean, we use the
                // `operandSegmentSizes` to annotate the size of each of these
                // segments. In this case, we are passing a two operands, and a
                // single return type.
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

                // So far, we only declared a pattern that matches an operation
                // called "felt.add", that receives two operands, and returns a
                // single element.
                //
                // Now, we will define how this pattern should be rewriten,
                // using the pdl.rewrite operation.
                //
                // When directly nested in a pdl.pattern region,
                // the pdl.operation corresponds to input operations
                // that should be matched. When nested in a
                // pdl.rewrite region, the pdl.operation corresponds
                // to operations that should be created as part of
                // a rewrite.
                //
                // rewrite %3 {
                //   %4 = operation "arith.addi"(%1, %2 : !pdl.value, !pdl.value)  -> (%0 : !pdl.type)
                //   %5 = result 0 of %4
                //   %6 = attribute = 13 : i32
                //   %7 = operation "arith.constant"  {"value" = %6} -> (%0 : !pdl.type)
                //   %8 = result 0 of %7
                //   %9 = operation "arith.remui"(%5, %8 : !pdl.value, !pdl.value)  -> (%0 : !pdl.type)
                //   replace %3 with %9
                // }
                //
                // Like the pdl.operation operation, we use the
                // "operandSegmentSizes" attribute to differentiate between the
                // `root` operand, and the `externalArgs` operand. I do not yet
                // know what the `externalArgs` operand means.
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

                            // We create an arith.addi operation that receives
                            // the same arguments as the felt.add operation, and
                            // returns the same value type.
                            //
                            // %4 = operation "arith.addi"(%1, %2)  -> (%0)
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

                            // We bind the add_result value, to the result of
                            // the previously defined arith.addi operation.
                            //
                            // %5 = result 0 of %4
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

                            // Our goal is now to create a value with a constant
                            // value of 13, so that we can use it as our modulo.
                            // We want something like this:
                            //
                            // %c13_i32 = arith.constant 13 : i32
                            //
                            // The problem is that in PDL, everything is an value, so we need to:
                            // - Define an attribute with a constant value of 13.
                            // - Define the arith.constant operation, that receives this attribute..
                            // - Take the result of this operation.

                            // We define an attribute value, with a constant value of 13.
                            //
                            // %6 = attribute = 13 : i32
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

                            // We define the arith.constant operation, that receives the attribute.
                            //
                            // %7 = operation "arith.constant"  {"value" = %6} -> (%0 : !pdl.type)
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

                            // We take the result value of the arith.constant operation
                            //
                            // %8 = result 0 of %7
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

                            // With the constant 13 value, we can now define the
                            // remui operation that receives both the arith.addi
                            // result, and the constant 13 value.
                            //
                            // %9 = operation "arith.remui"(%5, %8)  -> (%0)
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

                            // Now, we just replace the root operation
                            // "felt.add", with the result of the "arith.remui"
                            // operation.
                            //
                            // replace %3 with %9
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
