#include "lexer.hpp"

Lexer::Lexer(std::string source) : source(source) {}

Token Lexer::next() {
  skip_space();

  if (at_end()) {
    return EndToken;
  };

  start = current;

  char c = next_char();

  // match symbols
  switch (c) {
  case '(':
    return build_token(Token::Kind::OpenParen);
  case ')':
    return build_token(Token::Kind::CloseParen);
  case '{':
    return build_token(Token::Kind::OpenBrace);
  case '}':
    return build_token(Token::Kind::CloseBrace);
  case ';':
    return build_token(Token::Kind::SemiColon);
  }

  // if is a number
  if (isnumber(c)) {
    // advance all numeric characters
    while (isnumber(peek_char()))
      next_char();

    return build_token(Token::Kind::Integer);
  }

  // if is alphabetic
  if (isalpha(c)) {
    // advance all alphanumeric characters
    while (isalnum(peek_char()))
      next_char();

    // match keywords or identifiers
    if (matches("define")) {
      return build_token(Token::Kind::Define);
    } else if (matches("return")) {
      return build_token(Token::Kind::Return);
    } else {
      return build_token(Token::Kind::Identifier);
    }
  }

  return build_token(Token::Kind::Unknown);
}

void Lexer::skip_space() {
  while (isspace(peek_char()))
    next_char();
}

char Lexer::peek_char() { return at_end() ? '\0' : source[current]; }
char Lexer::next_char() { return at_end() ? '\0' : source[current++]; }
bool Lexer::at_end() { return current >= source.length(); }

// Returns true if the current lexeme matches the given string
//
// This function doesn't clone the string, so it's more performant
// than using `current_lexeme`.
bool Lexer::matches(std::string_view s) {
  return source.compare(start, current - start, s) == 0;
}

// Builds the current token with the given kind.
//
// Never fails, but it's wrapped with optional for convenience.
Token Lexer::build_token(Token::Kind kind) {
  return Token{
      .kind = kind,
      .lexeme = current_lexeme(),
  };
}

std::string Lexer::current_lexeme() {
  return source.substr(start, current - start);
}

std::vector<Token> lex(std::string source) {
  Lexer lexer = {source};

  std::vector<Token> vector;

  Token token = lexer.next();
  while (token.kind != Token::Kind::End) {
    vector.push_back(token);
    token = lexer.next();
  };

  return vector;
}
