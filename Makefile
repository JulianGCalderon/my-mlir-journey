%.llvm.mlir: %.mlir
	mlir-opt --convert-scf-to-cf --convert-cf-to-llvm --convert-to-llvm --mlir-print-debuginfo $< -o $@

%.ll: %.llvm.mlir
	mlir-translate --mlir-to-llvmir $< -o $@

%.o: %.ll
	clang $< -O0 -Wno-override-module -g -c -o $@

%.o: %.c
	clang $< -O0 -Wno-override-module -g -c -o $@

%.out: %.o
	clang $< -o $@

linking_main.out: linking_main.o linking_lib.o
	clang $^ -o $@

clean:
	rm $(wildcard *.llvm.mlir) $(wildcard *.ll) $(wildcard *.o) $(wildcard *.out)
.PHONY: clean
