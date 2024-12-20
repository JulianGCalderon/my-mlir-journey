#include "parser.hpp"
#include "ast.hpp"
#include "lexer.hpp"
#include <cstdlib>
#include <format>
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
  ParseResult<Token> token_result = expect_token(Token::Kind::Define);
  if (!token_result) {
    return std::unexpected(token_result.error());
  }

  Token name_token = next_token(); // get name

  token_result = expect_token(Token::Kind::OpenParen);
  if (!token_result) {
    return std::unexpected(token_result.error());
  }
  token_result = expect_token(Token::Kind::CloseParen);
  if (!token_result) {
    return std::unexpected(token_result.error());
  }
  token_result = expect_token(Token::Kind::OpenBrace);
  if (!token_result) {
    return std::unexpected(token_result.error());
  }

  std::vector<AST::Statement> vector;

  while (peek_token().kind != Token::Kind::CloseBrace) {
    ParseResult<AST::Statement> stmt = parse_statement();
    if (!stmt) {
      return std::unexpected(stmt.error());
    }

    vector.push_back(stmt.value());
  }

  token_result = expect_token(Token::Kind::CloseBrace);
  if (!token_result) {
    return std::unexpected(token_result.error());
  }

  return ParseResult<AST::Define>(AST::Define{
      .name = name_token.lexeme,
      .stmts = vector,
  });
}

ParseResult<AST::Statement> Parser::parse_statement() {
  ParseResult<Token> token_result = expect_token(Token::Kind::Return);
  if (!token_result) {
    return std::unexpected(token_result.error());
  }

  ParseResult<AST::Expression> expression = parse_expression();
  if (!expression) {
    return std::unexpected(expression.error());
  }

  token_result = expect_token(Token::Kind::SemiColon);
  if (!token_result) {
    return std::unexpected(token_result.error());
  }

  AST::Statement statement{
      .kind = AST::Statement::Kind::Return,
      .value.ret = AST::ReturnStatement{.expr = expression.value()},
  };

  return ParseResult<AST::Statement>(statement);
}

ParseResult<AST::Expression> Parser::parse_expression() {
  ParseResult<Token> integer_token_result = expect_token(Token::Kind::Integer);
  if (!integer_token_result) {
    return std::unexpected(integer_token_result.error());
  }
  Token integer_token = integer_token_result.value();

  AST::Expression expression = AST::Expression{
      .kind = AST::Expression::Kind::Integer,
      .value.integer = std::stoi(integer_token.lexeme),
  };

  return ParseResult<AST::Expression>(expression);
}

Token Parser::peek_token() { return at_end() ? EndToken : tokens[current]; }
Token Parser::next_token() { return at_end() ? EndToken : tokens[current++]; }
ParseResult<Token> Parser::expect_token(Token::Kind kind) {
  Token token = at_end() ? EndToken : tokens[current++];

  if (token.kind != kind) {
    return std::unexpected(
        std::format("expected token of type {}, but got {}", kind, token.kind));
  } else {
    return ParseResult<Token>(token);
  }
}
bool Parser::at_end() { return current >= tokens.size(); }
