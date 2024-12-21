#include "codegen.hpp"
#include "ast.hpp"

#include "llvm/ADT/APFloat.h"
#include "llvm/ADT/STLExtras.h"
#include "llvm/IR/BasicBlock.h"
#include "llvm/IR/Constants.h"
#include "llvm/IR/DerivedTypes.h"
#include "llvm/IR/Function.h"
#include "llvm/IR/IRBuilder.h"
#include "llvm/IR/Instructions.h"
#include "llvm/IR/LLVMContext.h"
#include "llvm/IR/LegacyPassManager.h"
#include "llvm/IR/Module.h"
#include "llvm/IR/Type.h"
#include "llvm/IR/Verifier.h"
#include "llvm/MC/TargetRegistry.h"
#include "llvm/Support/FileSystem.h"
#include "llvm/Support/TargetSelect.h"
#include "llvm/Support/raw_ostream.h"
#include "llvm/Target/TargetMachine.h"
#include "llvm/Target/TargetOptions.h"
#include "llvm/TargetParser/Host.h"

#include <cassert>
#include <cstdlib>
#include <iostream>

void CodeGen::build_module(AST::Module module) {
  for (auto d : module.definitions) {
    build_function(d);
  }
}

void CodeGen::build_function(AST::Define define) {
  llvm::FunctionType *FT =
      llvm::FunctionType::get(llvm::Type::getInt32Ty(*Context), false);

  llvm::Function *F = llvm::Function::Create(
      FT, llvm::Function::ExternalLinkage, define.name, Module.get());

  llvm::BasicBlock *BB = llvm::BasicBlock::Create(*Context, "entry", F);
  Builder->SetInsertPoint(BB);

  for (auto stmt : define.stmts) {
    build_statement(stmt);
  }

  assert(!llvm::verifyFunction(*F, &llvm::errs()));
}

void CodeGen::build_statement(AST::Statement stmt) {
  switch (stmt.kind) {
  case AST::Statement::Return:
    build_statement(stmt.value.ret);
    break;
  }
}

void CodeGen::build_statement(AST::ReturnStatement stmt) {
  llvm::Value *value = build_expr(stmt.expr);
  Builder->CreateRet(value);
}

llvm::Value *CodeGen::build_expr(AST::Expression expr) {
  switch (expr.kind) {
  case AST::Expression::Integer:
    return build_expr(expr.value.integer);
    break;
  }
}

llvm::Value *CodeGen::build_expr(int expr) {
  llvm::APInt v = llvm::APInt(32, (uint64_t)expr, false);
  return llvm::ConstantInt::get(*Context, v);
}

void CodeGen::compile() {
  auto llvmir_path = "a.llvmir";
  std::error_code EC;
  llvm::raw_fd_ostream llvmir_file(llvmir_path, EC, llvm::sys::fs::OF_None);
  if (EC) {
    llvm::errs() << "Failed to create a.llvmir: " << EC.message();
    exit(EXIT_FAILURE);
  }
  Module->print(llvmir_file, nullptr, true, true);

  llvm::InitializeAllTargetInfos();
  llvm::InitializeAllTargets();
  llvm::InitializeAllTargetMCs();
  llvm::InitializeAllAsmParsers();
  llvm::InitializeAllAsmPrinters();

  std::string target_triple = llvm::sys::getDefaultTargetTriple();

  std::string Error;
  const llvm::Target *target =
      llvm::TargetRegistry::lookupTarget(target_triple, Error);

  if (!target) {
    llvm::errs() << "Failed to lookup target: " << Error;
    exit(EXIT_FAILURE);
  }

  auto CPU = "generic";
  auto Features = "";

  llvm::TargetOptions opt;
  llvm::TargetMachine *target_machine = target->createTargetMachine(
      target_triple, CPU, Features, opt, llvm::Reloc::PIC_);

  Module->setDataLayout(target_machine->createDataLayout());
  Module->setTargetTriple(target_triple);

  auto object_path = "a.o";
  llvm::raw_fd_ostream object_file(object_path, EC, llvm::sys::fs::OF_None);
  if (EC) {
    llvm::errs() << "Failed to create a.o: " << EC.message();
    exit(EXIT_FAILURE);
  }

  llvm::legacy::PassManager pass;
  auto FileType = llvm::CodeGenFileType::ObjectFile;

  if (target_machine->addPassesToEmitFile(pass, object_file, nullptr,
                                          FileType)) {
    llvm::errs() << "Failed to emit to file";
    exit(EXIT_FAILURE);
  }

  pass.run(*Module);
  object_file.flush();
}
