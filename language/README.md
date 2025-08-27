# Language

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

## Pipeline

### Conversion

The first step is to convert our module to the `llvm` dialect. To do this, we can use the `mlir-opt` tool:
```bash
mlir-opt fibonacci.mlir --convert-scf-to-cf --convert-cf-to-llvm --convert-to-llvm -o fibonacci.llvm.mlir
```

### Translation

Then, it can be translated to LLVMIR using the `mlir-translate` tool:
```bash
mlir-translate fibonacci.llvm.mlir --mlir-to-llvmir -o fibonacci.ll
```

### Compilation

Now we can reuse the LLVM framework to compile it to an object file:
```bash
clang fibonacci.ll -Wno-override-module -c -o fibonacci.o
```

Finally, it can be linked as an executable file:
```bash
clang fibonacci.o -o fibonacci.out
```

The binary will return the fibonacci of the number of arguments
```bash
$ ./fibonacci.out 1 1 1; echo $?
24
```

## Linking

You can define extern functions and globals, and link them at compile/runtime, using the `llvm` or `func` dialect.
```bash
make linking_main.o linking_lib.o
clang linking_lib.o linking_main.o -o linking_main.out
```

You could also link with stdlib (this is done by default).
