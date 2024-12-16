#include "ast.hpp"
#include <iostream>

void print_ast(AST::Expression expr, size_t ident = 0) {
  std::string pad = std::string(ident, ' ');

  switch (expr.kind) {
  case AST::Expression::Integer:
    std::cout << expr.value.integer;
  }
}

void print_ast(AST::Statement stmt, size_t ident = 0) {
  std::string pad = std::string(ident, ' ');

  switch (stmt.kind) {
  case AST::Statement::Kind::Return:
    std::cout << pad << "return ";
    print_ast(stmt.value.ret.expr);
    std::cout << ";\n";
  }
}

void print_ast(AST::Define module, size_t ident = 0) {
  std::string pad = std::string(ident, ' ');

  std::cout << pad << "define " << module.name << "() {\n";

  for (auto &stmt : module.stmts) {
    print_ast(stmt, ident + 4);
  }

  std::cout << pad << "}\n";
}

void print_ast(AST::Module module, size_t ident = 0) {
  std::string pad = std::string(ident, ' ');

  for (auto &define : module.definitions) {
    print_ast(define, ident);
  }
}
