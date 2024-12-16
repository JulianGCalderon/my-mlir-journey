#include "parser.hpp"
#include "ast.hpp"
#include "lexer.hpp"
#include <__expected/unexpect.h>
#include <__expected/unexpected.h>
#include <cstdlib>
#include <optional>
#include <string>

Parser::Parser(std::vector<Token> tokens) : tokens(std::move(tokens)) {}

ParseResult<AST::Module> parse(std::vector<Token> tokens) {
  Parser parser = {tokens};
  return parser.parse_module();
}

ParseResult<AST::Module> Parser::parse_module() {
  std::vector<AST::Define> vector;

  while (peek_token().kind == Token::Kind::Define) {
    ParseResult<AST::Define> define = parse_define();
    if (!define) {
      return std::unexpected(define.error());
    }

    vector.push_back(define.value());
  }

  return ParseResult<AST::Module>(AST::Module{
      vector,
  });
}

ParseResult<AST::Define> Parser::parse_define() {
  next_token(); // consume `define`

  Token name_token = next_token(); // get name

  next_token(); // consume `(`
  next_token(); // consume `)`
  next_token(); // consume `{`

  std::vector<AST::Statement> vector;

  while (peek_token().kind != Token::Kind::CloseBrace) {
    ParseResult<AST::Statement> stmt = parse_statement();
    if (!stmt) {
      return std::unexpected(stmt.error());
    }

    vector.push_back(stmt.value());
  }

  next_token(); // consume `}`

  return ParseResult<AST::Define>(AST::Define{
      .name = name_token.lexeme,
      .stmts = vector,
  });
}

ParseResult<AST::Statement> Parser::parse_statement() {
  next_token(); // consume `return`

  ParseResult<AST::Expression> expression = parse_expression();
  if (!expression) {
    return std::unexpected(expression.error());
  }

  next_token(); // consume `;`

  AST::Statement statement{
      .kind = AST::Statement::Kind::Return,
      .value.ret = AST::ReturnStatement{.expr = expression.value()},
  };

  return ParseResult<AST::Statement>(statement);
}

ParseResult<AST::Expression> Parser::parse_expression() {
  Token integer_token = next_token(); // get integer

  AST::Expression expression = AST::Expression{
      .kind = AST::Expression::Kind::Integer,
      .value.integer = std::stoi(integer_token.lexeme),
  };

  return ParseResult<AST::Expression>(expression);
}

Token Parser::peek_token() { return at_end() ? EndToken : tokens[current]; }
Token Parser::next_token() { return at_end() ? EndToken : tokens[current++]; }
bool Parser::at_end() { return current >= tokens.size(); }
