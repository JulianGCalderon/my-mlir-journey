%.llvm.mlir: %.mlir
	mlir-opt --convert-scf-to-cf --convert-cf-to-llvm --convert-to-llvm $< -o $@

%.ll: %.llvm.mlir
	mlir-translate --mlir-to-llvmir $< -o $@

%.o: %.ll
	clang $< -O0 -Wno-override-module -c -o $@

%.out: %.o
	clang $< -o $@

clean:
	rm **/*.llvm.mlir **/*.ll **/*.o **/*.out
.PHONY: clean
