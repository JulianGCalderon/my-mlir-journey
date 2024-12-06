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
clang fibonacci.ll -Wno-override-module -o fibonacci.o
```

I automated the build process with Makefile
```bash
make fibonacci.o
```

The binary will return the fibonacci of the number of arguments
```bash
$ ./fibonacci.o 1 1 1; echo $?
24
```
