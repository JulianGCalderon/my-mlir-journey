#include "ast.hpp"
#include "lexer.hpp"
#include "parser.hpp"
#include "print.hpp"
#include <fstream>
#include <iostream>
#include <sstream>

void print_tokens(std::vector<Token> tokens) {
  for (Token t : tokens) {
    std::cout << static_cast<int>(t.kind) << ":" << t.lexeme << ' ';
  }
  std::cout << '\n';
}

int main(int argc, char **argv) {
  if (argc < 2) {
    std::cerr << "Source file missing" << '\n';
    exit(EXIT_FAILURE);
  }

  char *file_path = argv[1];

  std::ifstream file(file_path);
  std::stringstream buffer;
  buffer << file.rdbuf();
  std::string source = buffer.str();

  std::vector<Token> tokens = lex(source);
  print_tokens(tokens);

  ParseResult<AST::Module> module = parse(tokens);
  if (!module) {
    std::cerr << "Syntax Error: " << module.error() << '\n';
    exit(EXIT_FAILURE);
  }

  print_ast(module.value());

  return 0;
}
