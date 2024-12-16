#ifndef PRINT_HPP
#define PRINT_HPP

#include "ast.hpp"

void print_ast(AST::Expression expr, size_t ident = 0);
void print_ast(AST::Statement stmt, size_t ident = 0);
void print_ast(AST::Define module, size_t ident = 0);
void print_ast(AST::Module module, size_t ident = 0);

#endif
