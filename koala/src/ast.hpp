#ifndef AST_HPP
#define AST_HPP

#include <optional>
#include <string>
#include <vector>

namespace AST {

struct Expression {
  enum Kind { Integer } kind;
  union {
    int integer;
  } value;
};

struct ReturnStatement {
  Expression expr;
};

struct Statement {
  enum Kind { Return } kind;
  union {
    ReturnStatement ret;
  } value;
};

struct Define {
  std::string name;
  std::vector<Statement> stmts;
};

struct Module {
  std::vector<Define> definitions;
};

} // namespace AST

#endif
