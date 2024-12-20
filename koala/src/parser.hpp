#ifndef PARSER_HPP
#define PARSER_HPP

#include "ast.hpp"
#include "expected"
#include "lexer.hpp"

template <class T> using ParseResult = std::expected<T, std::string>;

class Parser {
public:
  Parser(std::vector<Token>);

  ParseResult<AST::Define> parse_define();
  ParseResult<AST::Module> parse_module();
  ParseResult<AST::Statement> parse_statement();
  ParseResult<AST::Expression> parse_expression();

private:
  std::vector<Token> tokens;
  size_t current = 0;

  Token peek_token();
  Token next_token();
  ParseResult<Token> expect_token(Token::Kind);
  bool at_end();
};

ParseResult<AST::Module> parse(std::vector<Token>);

#endif
