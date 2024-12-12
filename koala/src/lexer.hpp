#ifndef LEXER_HPP
#define LEXER_HPP

#include <optional>
#include <string>
#include <vector>

struct Token {
  enum class Kind {
    Identifier,

    // literals
    Integer,

    // keywords
    Define,
    Return,

    // symbols
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    SemiColon,

    // unique
    Unknown,
    End,
  };

  Kind kind;
  std::string lexeme;
};

const Token EndToken{Token::Kind::End, ""};

class Lexer {
public:
  Lexer(std::string source);
  // Returns the next token in source, if available.
  Token next();

private:
  std::string source;
  size_t start = 0;
  size_t current = 0;

  char peek_char();
  char next_char();
  bool at_end();
  bool matches(std::string_view s);
  void skip_space();

  Token build_token(Token::Kind kind);
  std::string current_lexeme();
};

// Returns all the tokens in the given string
std::vector<Token> lex(std::string);

#endif
