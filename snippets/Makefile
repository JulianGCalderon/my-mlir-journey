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
	find . -name '*.llvm.mlir' -type f -delete
	find . -name '*.ll'        -type f -delete
	find . -name '*.o'         -type f -delete
	find . -name '*.out'       -type f -delete
.PHONY: clean
