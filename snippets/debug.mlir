module {
  func.func @main(%arg : i32 loc("debug_c.c":1:14)) -> i32 {
    %c1_i32 = arith.constant 1 : i32
    %next = arith.addi %arg, %c1_i32 : i32 loc("debug_c.c":2:0)
    %mult = arith.muli %arg, %next : i32 loc("debug_c.c":3:0)
    %rest = arith.addi %mult, %c1_i32 : i32 loc("debug_c.c":4:12)
    func.return %rest : i32 loc("debug_c.c":4:0)
  } loc(#loc_main)
} loc(#loc_module)

#di_file = #llvm.di_file<"debug_c.c" in ".">

#distinct_compile_unit_id = distinct[0]<"compile_unit_id">
#di_compile_unit = #llvm.di_compile_unit<id = #distinct_compile_unit_id, sourceLanguage = DW_LANG_C, file = #di_file, isOptimized = false, emissionKind = Full>
#di_module = #llvm.di_module<file = #di_file, scope = #di_compile_unit>
#loc_module = loc(fused<#di_module>["debug_c.c"])

#distinct_fn_0 = distinct[1]<"fn_0">
#di_subroutine_type = #llvm.di_subroutine_type<>
#di_subprogram = #llvm.di_subprogram<id = #distinct_fn_0, compileUnit = #di_compile_unit, scope = #di_file, name = "main", file = #di_file, subprogramFlags = Definition, type = #di_subroutine_type>
#loc_main = loc(fused<#di_subprogram>["debug_c.c":1:0])
