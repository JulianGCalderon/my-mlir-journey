# MLIR Dialects in Rust

In directory explores how to define custom dialects without C++.

## Overview

`Dialect`s are the mechanism to extend MLIR with custom operations and types. Every operation is defined inside of a dialect. Usually, we use the dialects already included in MLIR (builtin, arith, func, llvm).

Building a custom dialect is actually composed of two parts:

- Dialect declaration: We define the types, operations, and other properties of the dialect. It does not contain any logic. It is usually done with [TableGen](https://mlir.llvm.org/docs/DefiningDialects/Operations/), although it can be done manually.
- Conversion pass: We define the actual logic of the dialect, and how it can be translated to other dialects. It is usually done in C++.

## Declaring the Dialect

Instead of tablegen, we explore the [IRDL](https://mlir.llvm.org/docs/Dialects/IRDL/) dialect. An MLIR dialect used to declare new dialects.

The following example declares a dialect `felt`, with a single operation `add`.
```mlir
module {
  irdl.dialect @felt {
    irdl.operation @add {
      %0 = irdl.is i32
      irdl.operands(%0, %0)
      irdl.results(%0)
    }
  }
}
```

To use that dialect, we need to register it in the context. For this, the C API exposes the `mlirLoadIRDLDialects` function. If we are using `mlir-opt`, we can achieve the same result with the `--irdl-file` flag.

## Implementing a Conversion Pass

Instead of C++, we explore the [PDL](https://mlir.llvm.org/docs/Dialects/PDLOps/) dialect. An MLIR dialect used to define rewrite patterns.

The following example declares a pattern for rewriting the `felt.add` operation, into a `arith.addi`, followed by a `arith.remui` with a constant value of 13.
```mlir
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
}
```

We need to apply the patterns to the target module. For this, the C API exposes the `mlirApplyPatternsAndFoldGreedily` function.

## Putting it all Together

This directory contains a small Rust binary, which combines this dialects to fully implement a custom dialect.

```sh
cargo run
```
