#ifndef CODEGEN_HPP
#define CODEGEN_HPP

#include "ast.hpp"
#include "llvm/IR/IRBuilder.h"
#include "llvm/IR/LLVMContext.h"
#include "llvm/IR/Module.h"

class CodeGen {
  std::unique_ptr<llvm::LLVMContext> Context;
  std::unique_ptr<llvm::Module> Module;
  std::unique_ptr<llvm::IRBuilder<>> Builder;

public:
  CodeGen() {
    Context = std::make_unique<llvm::LLVMContext>();
    Module = std::make_unique<llvm::Module>("Koala", *Context);
    Builder = std::make_unique<llvm::IRBuilder<>>(*Context);
  }

  void build_module(AST::Module);
  void build_function(AST::Define);
  void build_statement(AST::Statement);
  void build_statement(AST::ReturnStatement);
  llvm::Value *build_expr(AST::Expression);
  llvm::Value *build_expr(int);

  void compile();
};

#endif
