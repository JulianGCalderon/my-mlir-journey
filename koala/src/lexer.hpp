#ifndef LEXER_HPP
#define LEXER_HPP

#include <expected>
#include <format>
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

using LexResult = std::expected<std::vector<Token>, std::vector<Token>>;

// Returns all the tokens in the given string
LexResult lex(std::string);

template <> struct std::formatter<Token::Kind> {
  constexpr auto parse(auto &ctx) { return ctx.begin(); }

  auto format(const Token::Kind &obj, auto &ctx) const {
    std::string name;
    switch (obj) {
    case Token::Kind::Identifier:
      name = "Identifier";
      break;
    case Token::Kind::Integer:
      name = "Integer";
      break;
    case Token::Kind::Define:
      name = "Define";
      break;
    case Token::Kind::Return:
      name = "Return";
      break;
    case Token::Kind::OpenParen:
      name = "OpenParen";
      break;
    case Token::Kind::CloseParen:
      name = "CloseParen";
      break;
    case Token::Kind::OpenBrace:
      name = "OpenBrace";
      break;
    case Token::Kind::CloseBrace:
      name = "CloseBrace";
      break;
    case Token::Kind::SemiColon:
      name = "SemiColon";
      break;
    case Token::Kind::Unknown:
      name = "Unknown";
      break;
    case Token::Kind::End:
      name = "End";
      break;
    }
    return std::format_to(ctx.out(), "{}", name);
  }
};

#endif
