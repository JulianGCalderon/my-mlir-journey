%.llvm.mlir: %.mlir
	mlir-opt --convert-scf-to-cf --convert-to-llvm $< -o $@

%.ll: %.llvm.mlir
	mlir-translate --mlir-to-llvmir $< -o $@

%.o: %.ll
	clang $< -O0 -Wno-override-module -c -o $@

%.out: %.o
	clang $< -o $@

clean:
	find . -name '*.llvm.mlir' -type f -delete
	find . -name '*.ll'        -type f -delete
	find . -name '*.o'         -type f -delete
	find . -name '*.out'       -type f -delete
.PHONY: clean
