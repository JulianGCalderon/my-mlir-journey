#include "ast.hpp"
#include "lexer.hpp"
#include "parser.hpp"
#include "print.hpp"
#include <fstream>
#include <iostream>
#include <sstream>

std::string tokens_to_string(std::vector<Token> tokens) {
  std::stringstream buffer;
  for (Token t : tokens) {
    buffer << static_cast<int>(t.kind) << ":" << t.lexeme << ' ';
  }
  return buffer.str();
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

  LexResult tokens_result = lex(source);
  if (!tokens_result) {
    std::cerr << "Syntax Error: " << tokens_to_string(tokens_result.error())
              << "\n";
    exit(EXIT_FAILURE);
  }
  std::vector<Token> tokens = tokens_result.value();

  std::cout << tokens_to_string(tokens) << "\n";

  ParseResult<AST::Module> module = parse(tokens);
  if (!module) {
    std::cerr << "Syntax Error: " << module.error() << '\n';
    exit(EXIT_FAILURE);
  }

  print_ast(module.value());

  return 0;
}
