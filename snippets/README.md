# Snippets

This directory explores MLIR in its textual form, and how it's transformed to an executable object.

## Language Structure

MLIR is a graph-shaped IR with `Operation`s as nodes, and `Value`s as edges. A value is generated as the result of an operation, and they itself are used as arguments to other operations.

An `Operation` can contain any number of ordered `Region`s, and each region contains any number of ordered `Block`s. A block can itself contain any number of ordered operations, allowing for a recursive structure.

The top-level element is the `builtin.module` operation. It contains a single region with a single block, which can contain any number of operations. To define functions inside of a module, the `func.func` operation is used.

`Dialect`s are the mechanism to extend MLIR with custom operations and types. Every operation is defined inside of a dialect. The `builtin` layout defines core elements used in other layouts. As we are interested on compiling it to LLVMIR, we will use the `llvm` dialect a lot.

`Attribute`s are used to specify constant arguments to operations. For example, the `sym_name` attribute of `func.func` specifies the function name.

See:
- [Language Reference](https://mlir.llvm.org/docs/LangRef/)
- [Understanding the IR Structure](https://mlir.llvm.org/docs/Tutorials/UnderstandingTheIRStructure/)
