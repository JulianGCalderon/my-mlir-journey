module {
  func.func private @fopen(!llvm.ptr, !llvm.ptr) -> !llvm.ptr
  func.func private @fclose(!llvm.ptr) -> i32
  func.func private @getline(!llvm.ptr, !llvm.ptr, !llvm.ptr) -> i64
  func.func private @fputs(!llvm.ptr, !llvm.ptr) -> i32
  func.func private @realloc(!llvm.ptr, index) -> !llvm.ptr
  func.func private @free(!llvm.ptr)
  func.func private @strtok(!llvm.ptr, !llvm.ptr) -> !llvm.ptr
  func.func private @atoi(!llvm.ptr) -> i32
  func.func private @qsort(!llvm.ptr, index, index, !llvm.ptr)

  llvm.mlir.global @__stdoutp() : !llvm.ptr

  llvm.func @printf(!llvm.ptr, ...) -> i32

  func.func @main(%argc: i32, %argv: !llvm.ptr) -> i32 {
    // define constants
    %null = llvm.mlir.zero : !llvm.ptr
    %c0_i64 = arith.constant 0 : i64
    %c1_i64 = arith.constant 1 : i64
    %c0_i32 = arith.constant 0 : i32
    %c1_i32 = arith.constant 1 : i32
    %c2_i32 = arith.constant 2 : i32
    %c4_i32 = arith.constant 4 : i32
    %c0 = arith.constant 0 : index
    %c1 = arith.constant 1 : index
    %c4 = arith.constant 4 : index

    // get path
    %path_ptr = llvm.getelementptr %argv[1] : (!llvm.ptr) -> !llvm.ptr, !llvm.ptr
    %path = llvm.load %path_ptr : !llvm.ptr -> !llvm.ptr

    // create mode
    %c_mode = llvm.mlir.constant("r\00") : !llvm.array<2 x i8>
    %mode = llvm.alloca %c2_i32 x i8 : (i32) -> !llvm.ptr
    llvm.store %c_mode, %mode : !llvm.array<2 x i8>, !llvm.ptr

    // open file
    %input = call @fopen(%path, %mode) : (!llvm.ptr, !llvm.ptr) -> !llvm.ptr

    // get stdout
    %stdout_ptr = llvm.mlir.addressof @__stdoutp : !llvm.ptr
    %stdout = llvm.load %stdout_ptr : !llvm.ptr -> !llvm.ptr

    // declare line pointer
    %line_ptr = llvm.alloca %c1_i32 x i8 : (i32) -> !llvm.ptr
    llvm.store %null, %line_ptr: !llvm.ptr, !llvm.ptr

    // declare line size pointer
    %line_size_ptr = llvm.alloca %c1_i32 x i64 : (i32) -> !llvm.ptr
    llvm.store %c0_i64, %line_size_ptr: i64, !llvm.ptr

    // collect file into two dynamic arrays
    %len, %left, %right = scf.while(%i = %c0, %left = %null, %right = %null) : (index, !llvm.ptr, !llvm.ptr) -> (index, !llvm.ptr, !llvm.ptr) {
      %read = func.call @getline(%line_ptr, %line_size_ptr, %input) : (!llvm.ptr, !llvm.ptr, !llvm.ptr) -> i64

      %has_read = arith.cmpi "sgt", %read, %c0_i64 : i64
      scf.condition(%has_read) %i, %left, %right : index, !llvm.ptr, !llvm.ptr
    } do {
    ^after(%len : index, %left : !llvm.ptr, %right : !llvm.ptr):
      %len_i64 = index.castu %len : index to i64
      %len_next = index.add %len, %c1
      %len_next_i64 = index.castu %len_next : index to i64
      %cap_next = index.mul %len_next, %c4

      // extend arrays
      %left_next = func.call @realloc(%left, %cap_next) : (!llvm.ptr, index) -> !llvm.ptr
      %right_next = func.call @realloc(%right, %cap_next) : (!llvm.ptr, index) -> !llvm.ptr

      // get new element pointer
      %left_new_ptr = llvm.getelementptr %left_next[%len_i64] : (!llvm.ptr, i64) -> !llvm.ptr, i32
      %right_new_ptr = llvm.getelementptr %right_next[%len_i64] : (!llvm.ptr, i64) -> !llvm.ptr, i32

      %line = llvm.load %line_ptr : !llvm.ptr -> !llvm.ptr

      %c_sep = llvm.mlir.constant("   \00") : !llvm.array<4 x i8>
      %sep = llvm.alloca %c4_i32 x i8 : (i32) -> !llvm.ptr
      llvm.store %c_sep, %sep : !llvm.array<4 x i8>, !llvm.ptr

      // extract tokens
      %value_left_str = func.call @strtok(%line, %sep) : (!llvm.ptr, !llvm.ptr) -> !llvm.ptr
      %value_right_str = func.call @strtok(%null, %sep) : (!llvm.ptr, !llvm.ptr) -> !llvm.ptr
      %value_left = func.call @atoi(%value_left_str) : (!llvm.ptr) -> i32
      %value_right = func.call @atoi(%value_right_str) : (!llvm.ptr) -> i32

      llvm.store %value_left, %left_new_ptr : i32, !llvm.ptr
      llvm.store %value_right, %right_new_ptr : i32, !llvm.ptr

      scf.yield %len_next, %left_next, %right_next : index, !llvm.ptr, !llvm.ptr
    }

    // sort arrays
    %cmp_func = llvm.mlir.addressof @compare_i32 : !llvm.ptr
    func.call @qsort(%left, %len, %c4, %cmp_func) : (!llvm.ptr, index, index, !llvm.ptr) -> ()
    func.call @qsort(%right, %len, %c4, %cmp_func) : (!llvm.ptr, index, index, !llvm.ptr) -> ()

    // calculate sum of distances
    %sum = scf.for %i = %c0 to %len step %c1 iter_args(%sum = %c0_i32) -> (i32) {
      %i_i64 = index.castu %i : index to i64

      %left_ptr = llvm.getelementptr %left[%i_i64] : (!llvm.ptr, i64) -> !llvm.ptr, i32
      %left_number = llvm.load %left_ptr : !llvm.ptr -> i32

      %right_ptr = llvm.getelementptr %right[%i_i64] : (!llvm.ptr, i64) -> !llvm.ptr, i32
      %right_number = llvm.load %right_ptr : !llvm.ptr -> i32

      %in_order = arith.cmpi "sle", %left_number, %right_number : i32
      %min, %max = scf.if %in_order -> (i32, i32) {
        scf.yield %left_number, %right_number : i32, i32
      } else {
        scf.yield %right_number, %left_number : i32, i32
      }

      %diff = arith.subi %max, %min : i32
      %sum_next = arith.addi %sum, %diff : i32

      scf.yield %sum_next : i32
    }

    // print result
    %c_format = llvm.mlir.constant("%d\n\00") : !llvm.array<4 x i8>
    %format = llvm.alloca %c4_i32 x i8 : (i32) -> !llvm.ptr
    llvm.store %c_format, %format : !llvm.array<4 x i8>, !llvm.ptr
    llvm.call @printf(%format, %sum) vararg(!llvm.func<i32 (ptr, ...)>) : (!llvm.ptr, i32) -> i32

    // free resources
    call @free(%left) : (!llvm.ptr) -> ()
    call @free(%right) : (!llvm.ptr) -> ()
    call @fclose(%input) : (!llvm.ptr) -> i32

    return %c0_i32 : i32
  }

  llvm.func @compare_i32(%a_ptr : !llvm.ptr, %b_ptr : !llvm.ptr) -> i32 {
      %a = llvm.load %a_ptr : !llvm.ptr -> i32
      %b = llvm.load %b_ptr : !llvm.ptr -> i32
      %c = arith.subi %a, %b : i32
      llvm.return %c : i32
  }
}
