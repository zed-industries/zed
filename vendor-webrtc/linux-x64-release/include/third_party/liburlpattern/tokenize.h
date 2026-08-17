// Copyright 2020 The Chromium Authors
// Copyright 2014 Blake Embrey (hello@blakeembrey.com)
// Use of this source code is governed by an MIT-style license that can be
// found in the LICENSE file or at https://opensource.org/licenses/MIT.

#ifndef THIRD_PARTY_LIBURLPATTERN_LEXER_H_
#define THIRD_PARTY_LIBURLPATTERN_LEXER_H_

#include <string_view>
#include <vector>


#include "base/component_export.h"
#include "third_party/abseil-cpp/absl/status/statusor.h"

namespace liburlpattern {

enum class TokenType {
  // Open a scope with a '{'.
  kOpen,

  // Close a scope with a '}'.
  kClose,

  // A regular expression group like '(...)'.
  kRegex,

  // A named group like ':foo'.
  kName,

  // A single character.
  kChar,

  // The '\' escape character.
  kEscapedChar,

  // A '+' or '?' modifier.
  kOtherModifier,

  // A '*' character which can be a wildcard or modifier.
  kAsterisk,

  // The end of the token stream.
  kEnd,

  // A character that is not valid in a properly formed pattern; e.g. the colon
  // in `https://`.  This is only generated when TokenizerPolicy::kLenient is
  // used.
  kInvalidChar,
};

const char* TokenTypeToString(TokenType type);

// Simple structure representing a single lexical token.
struct COMPONENT_EXPORT(LIBURLPATTERN) Token {
  // Indicate the token type.
  TokenType type = TokenType::kEnd;

  // Index of the start of this token in the original pattern string.
  size_t index = 0;

  // The value of the token.  May be one or many characters depending on type.
  // May be null zero characters for the kEnd type.
  std::string_view value;

  Token(TokenType t, size_t i, std::string_view v)
      : type(t), index(i), value(v) {}
  Token() = default;
};

enum class TokenizePolicy {
  // The strict policy causes any problems found during tokenization to be
  // thrown as errors.
  kStrict,

  // The lenient policy converts problems detected during tokenization into
  // kInvalidChar tokens in the returned token list.  For something like a
  // `\` at the end of the string, this simply returns the immediate `\`
  // character.  For validation errors that cause a group to be invalid, the
  // first character of the group is instead returned.  For example, `https://`
  // returns the `:` as a kInvalidChar.  For `(foo(bar))` where capture groups
  // are illegal it causes the first `(` to be returned as a kInvalidChar.
  // Tokenization then continues with the next character after the kInvalidChar.
  kLenient,
};

COMPONENT_EXPORT(LIBURLPATTERN)
inline bool operator==(const Token& lh, const Token& rh) {
  return lh.type == rh.type && lh.index == rh.index && lh.value == rh.value;
}

inline bool operator!=(const Token& lh, const Token& rh) {
  return !(lh == rh);
}

COMPONENT_EXPORT(LIBURLPATTERN)
std::ostream& operator<<(std::ostream& o, Token token);

// Split the given input pattern string into a list of lexical tokens.
// Tokenizing will fail if |pattern| is not valid UTF-8.  Note, the generated
// Token objects simply reference positions within the input |pattern|.  The
// |pattern| must be kept alive as long as the Token objects.
COMPONENT_EXPORT(LIBURLPATTERN)
absl::StatusOr<std::vector<Token>> Tokenize(
    std::string_view pattern,
    TokenizePolicy policy = TokenizePolicy::kStrict);

}  // namespace liburlpattern

#endif  // THIRD_PARTY_LIBURLPATTERN_LEXER_H_
