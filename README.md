This repository attempts to document my journey throughout MLIR. It will contain snippets, commands, and overall anything I find useful.

## Resources

- https://github.com/j2kun/mlir-tutorial

## Commands

# mlir-opt

Applies a series of optimizations/passes to an mlir file.
```bash
mlir-opt fibonacci.mlir --convert-scf-to-cf --convert-cf-to-llvm --convert-to-llvm
```

# mlir-cpu-runner

An mlir interpreter for low level dialects (llvm)
```bash
mlir-opt fibonacci.mlir --convert-scf-to-cf --convert-cf-to-llvm --convert-to-llvm \
  | mlir-cpu-runner -e main_8 -entry-point-result=i32
```

# mlir-translate

Converts mlir in llvm dialect to llvmir
```bash
mlir-opt fibonacci.mlir --convert-scf-to-cf --convert-cf-to-llvm --convert-to-llvm \
 | mlir-translate --mlir-to-llvmir -o fibonacci.ll
```

Now we case reuse the llvm framework to compile it native code
```bash
clang fibonacci.ll -Wno-override-module -o fibonacci.out
```

I automated the build process with Makefile
```bash
make fibonacci.out
```

The binary will return the fibonacci of the number of arguments
```bash
$ ./fibonacci.out 1 1 1; echo $?
24
```

## Linking

You can define extern functions and globals, and link them at compile/runtime, using the `llvm` or `func` dialect. See [linking_main.mlir](./linking_main.mlir).
```bash
make linking_main.o linking_lib.o
clang linking_lib.o linking_main.o -o linking_main.out
```

You could also link with stdlib (this is done by default).

## Advent of Code

For some reason, I decided to attempt the advent of code in pure MLIR.

```bash
make ./advent-of-code-2024/day1-part1.out
./advent-of-code-2024/day1-part1.out advent-of-code-2024/day1-part1.input
```

I managed to solve the first part, but I ended up using many stdlib functions
