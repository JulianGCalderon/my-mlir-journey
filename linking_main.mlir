module {
  llvm.func @hello_world() -> i32

  func.func @main() -> i32 {
    %0 = llvm.call @hello_world() : () -> i32
	return %0 : i32
  }
}
