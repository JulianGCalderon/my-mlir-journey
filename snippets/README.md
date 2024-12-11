# Snippets

This directory contains standalone MLIR snippets, useful for testing particular features.

It contains a Makefile for quickly compiling source files.

## Commands

There are many commands for interacting with MLIR source files.

### `mlir-opt`

Applies a series of optimizations/passes to an mlir file.
```bash
mlir-opt factorial.mlir --convert-scf-to-cf --convert-to-llvm
```

### `mlir-cpu-runner`

An MLIR interpreter for low level dialects (LLVM).
```bash
make factorial.llvm.mlir
mlir-cpu-runner factorial.llvm.mlir -e main_8 -entry-point-result=i32
```

The entrypoint should not have any parameters.

### `mlir-translate`

A translator from/to MLIR/LLVMIR.
```bash
make factorial.llvm.mlir
mlir-translate factorial.llvm.mlir --mlir-to-llvmir
```

### `clang`

We can use clang to compile LLVMIR source files.
```bash
make factorial.ll
clang factorial.ll -o factorial.out # or: make factorial.out
```

The executable will return the factorial of the number of arguments
```bash
$ ./factorial.out 1 1 1; echo $?
24
```

## Linking

You can define extern functions and globals, and link them at compile/runtime, using the `llvm` or `func` dialect. See [linking_main.mlir](./linking_main.mlir).
```bash
make linking_main.o linking_lib.o
clang linking_lib.o linking_main.o -o linking_main.out
```

You could also link with stdlib (this is done by default).

## Debug Info

In LLVMIR, debug info is stored in specialized metadata nodes, interleaved with the code. In MLIR, debug info is stored as location metadata, attached to each instruction. A location can be of type `fuzed`, in which case it can contain arbitrary attributes. If the attribute shape is compatible with LLVMIR metadata nodes, then the debug info can be maintained across translation.

- https://llvm.org/docs/SourceLevelDebugging.html
- https://mlir.llvm.org/docs/Dialects/Builtin/#location-attributes
- https://mlir.llvm.org/docs/Dialects/LLVM/#debug-info

As an example, I constructed the file [debug.mlir](debug.mlir), which contains hand-made locations refering to [debug_c.c](debug_c.c). If we compile it with debug info, the debugger can correlate the compiled object with the source file.
```bash
make debug.o debug.out
lldb -- debug.out
```

I found little documentation on this subject, so this example is very incomplete.
