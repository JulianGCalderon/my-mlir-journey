module {
  func.func @main(%argc: i32) -> i32 {
    %0 = call @fib(%argc) : (i32) -> i32
    return %0 : i32
  }

  func.func @fib(%n: i32) -> i32 {
    %c1_i32 = arith.constant 1 : i32
    %c2_i32 = arith.constant 2 : i32
    %n_plus_1 = arith.addi %n, %c1_i32 : i32

    %fib = scf.for %i = %c2_i32 to %n_plus_1 step %c1_i32
        iter_args(%sum = %c1_i32) -> (i32) : i32 {

      %new_sum = arith.muli %sum, %i : i32

      scf.yield %new_sum : i32
    }

    return %fib : i32
  }

  func.func @main_8() -> i32 {
    %c8_i32 = arith.constant 8 : i32
    %0 = call @main(%c8_i32) : (i32) -> i32
    return %0 : i32
  }
}
