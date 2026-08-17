// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_JSON_JSON_PARSER_H_
#define BASE_JSON_JSON_PARSER_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <optional>
#include <string>
#include <string_view>
#include <utility>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/gtest_prod_util.h"
#include "base/json/json_common.h"
#include "base/third_party/icu/icu_utf.h"
#include "base/values.h"

namespace base {

class Value;

namespace internal {

class JSONParserTest;

// The implementation behind the JSONReader interface. This class is not meant
// to be used directly; it encapsulates logic that need not be exposed publicly.
//
// This parser guarantees O(n) time through the input string. Iteration happens
// on the byte level, with the functions ConsumeChars() and ConsumeChar(). The
// conversion from byte to JSON token happens without advancing the parser in
// GetNextToken/ParseToken, that is tokenization operates on the current parser
// position without advancing.
//
// Built on top of these are a family of Consume functions that iterate
// internally. Invariant: on entry of a Consume function, the parser is wound
// to the first byte of a valid JSON token. On exit, it is on the first byte
// after the token that was just consumed, which would likely be the first byte
// of the next token.
class BASE_EXPORT JSONParser {
 public:
  // Error codes during parsing.
  enum JsonParseError {
    JSON_NO_ERROR = base::ValueDeserializer::kErrorCodeNoError,
    JSON_SYNTAX_ERROR = base::ValueDeserializer::kErrorCodeInvalidFormat,
    JSON_INVALID_ESCAPE,
    JSON_UNEXPECTED_TOKEN,
    JSON_TRAILING_COMMA,
    JSON_TOO_MUCH_NESTING,
    JSON_UNEXPECTED_DATA_AFTER_ROOT,
    JSON_UNSUPPORTED_ENCODING,
    JSON_UNQUOTED_DICTIONARY_KEY,
    JSON_UNREPRESENTABLE_NUMBER,
    JSON_PARSE_ERROR_COUNT
  };

  // String versions of parse error codes.
  static const char kSyntaxError[];
  static const char kInvalidEscape[];
  static const char kUnexpectedToken[];
  static const char kTrailingComma[];
  static const char kTooMuchNesting[];
  static const char kUnexpectedDataAfterRoot[];
  static const char kUnsupportedEncoding[];
  static const char kUnquotedDictionaryKey[];
  static const char kUnrepresentableNumber[];

  explicit JSONParser(int options, size_t max_depth = kAbsoluteMaxDepth);

  JSONParser(const JSONParser&) = delete;
  JSONParser& operator=(const JSONParser&) = delete;

  ~JSONParser();

  // Parses the input string according to the set options and returns the
  // result as a Value.
  // Wrap this in base::FooValue::From() to check the Value is of type Foo and
  // convert to a FooValue at the same time.
  std::optional<Value> Parse(std::string_view input);

  // Returns the error code.
  JsonParseError error_code() const;

  // Returns the human-friendly error message.
  std::string GetErrorMessage() const;

  // Returns the error line number if parse error happened. Otherwise always
  // returns 0.
  int error_line() const;

  // Returns the error column number if parse error happened. Otherwise always
  // returns 0.
  int error_column() const;

 private:
  enum Token {
    T_OBJECT_BEGIN,  // {
    T_OBJECT_END,    // }
    T_ARRAY_BEGIN,   // [
    T_ARRAY_END,     // ]
    T_STRING,
    T_NUMBER,
    T_BOOL_TRUE,              // true
    T_BOOL_FALSE,             // false
    T_NULL,                   // null
    T_LIST_SEPARATOR,         // ,
    T_OBJECT_PAIR_SEPARATOR,  // :
    T_END_OF_INPUT,
    T_INVALID_TOKEN,
  };

  // Returns the next |count| bytes of the input stream, or nullopt if fewer
  // than |count| bytes remain.
  std::optional<std::string_view> PeekChars(size_t count);

  // Calls PeekChars() with a |count| of 1.
  std::optional<char> PeekChar();

  // Returns the next |count| bytes of the input stream, or nullopt if fewer
  // than |count| bytes remain, and advances the parser position by |count|.
  std::optional<std::string_view> ConsumeChars(size_t count);

  // Calls ConsumeChars() with a |count| of 1.
  std::optional<char> ConsumeChar();

  // Returns a pointer to the current character position.
  const char* pos();

  // Skips over whitespace and comments to find the next token in the stream.
  // This does not advance the parser for non-whitespace or comment chars.
  Token GetNextToken();

  // Consumes whitespace characters and comments until the next non-that is
  // encountered.
  void EatWhitespaceAndComments();
  // Helper function that consumes a comment, assuming that the parser is
  // currently wound to a '/'.
  bool EatComment();

  // Calls GetNextToken() and then ParseToken().
  std::optional<Value> ParseNextToken();

  // Takes a token that represents the start of a Value ("a structural token"
  // in RFC terms) and consumes it, returning the result as a Value.
  std::optional<Value> ParseToken(Token token);

  // Assuming that the parser is currently wound to '{', this parses a JSON
  // object into a Value.
  std::optional<Value> ConsumeDictionary();

  // Assuming that the parser is wound to '[', this parses a JSON list into a
  // Value.
  std::optional<Value> ConsumeList();

  // Calls through ConsumeStringRaw and wraps it in a value.
  std::optional<Value> ConsumeString();

  // Assuming that the parser is wound to a double quote, this parses a string,
  // decoding any escape sequences and validating UTF-8. Returns the string on
  // success or std::nullopt on error, with error information set.
  std::optional<std::string> ConsumeStringRaw();

  enum class StringResult {
    // Parsing stopped because of invalid input. Error information has been set.
    // The caller should return failure.
    kError,
    // Parsing stopped because the string is finished. The parser is wound to
    // just paste the closing quote. The caller should stop parsing the string.
    kDone,
    // Parsing stopped because of invalid Unicode which should be replaced with
    // a replacement character. The parser is wound to just past the input that
    // should be a replacement character. The caller should add a replacement
    // character and continue parsing.
    kReplacementCharacter,
    // Parsing stopped because of an escape sequence. The parser is wound to
    // just past the backslash. The caller should consume the escape sequence
    // and continue parsing.
    kEscape,
  };

  // Consumes the portion of a JavaScript string which may be copied to the
  // input with no conversions, stopping at one of the events above. Returns the
  // reason parsing stopped and the data that was consumed. This should be
  // called in a loop, handling all the cases above until reaching kDone.
  std::pair<StringResult, std::string_view> ConsumeStringPart();

  // Helper function for ConsumeStringRaw() that consumes the next four or 10
  // bytes (parser is wound to the first character of a HEX sequence, with the
  // potential for consuming another \uXXXX for a surrogate). Returns true on
  // success and places the code point |out_code_point|, and false on failure.
  bool DecodeUTF16(base_icu::UChar32* out_code_point);

  // Assuming that the parser is wound to the start of a valid JSON number,
  // this parses and converts it to either an int or double value.
  std::optional<Value> ConsumeNumber();
  // Helper that reads characters that are ints. Returns true if a number was
  // read and false on error.
  bool ReadInt(bool allow_leading_zeros);

  // Consumes the literal values of |true|, |false|, and |null|, assuming the
  // parser is wound to the first character of any of those.
  std::optional<Value> ConsumeLiteral();

  // Helper function that returns true if the byte squence |match| can be
  // consumed at the current parser position. Returns false if there are fewer
  // than |match|-length bytes or if the sequence does not match, and the
  // parser state is unchanged.
  bool ConsumeIfMatch(std::string_view match);

  // Sets the error information to |code| at the current column, based on
  // |index_| and |index_last_line_|, with an optional positive/negative
  // adjustment by |column_adjust|.
  void ReportError(JsonParseError code, int column_adjust);

  // Given the line and column number of an error, formats one of the error
  // message contants from json_reader.h for human display.
  static std::string FormatErrorMessage(int line,
                                        int column,
                                        const std::string& description);

  // base::JSONParserOptions that control parsing.
  const int options_;

  // Maximum depth to parse.
  const size_t max_depth_;

  // The input stream being parsed. Note: Not guaranteed to NUL-terminated.
  std::string_view input_;

  // The index in the input stream to which the parser is wound.
  size_t index_;

  // The number of times the parser has recursed (current stack depth).
  size_t stack_depth_;

  // The line number that the parser is at currently.
  int line_number_;

  // The last value of |index_| on the previous line.
  size_t index_last_line_;

  // Error information.
  JsonParseError error_code_;
  int error_line_;
  int error_column_;

  friend class JSONParserTest;
  FRIEND_TEST_ALL_PREFIXES(JSONParserTest, NextChar);
  FRIEND_TEST_ALL_PREFIXES(JSONParserTest, ConsumeDictionary);
  FRIEND_TEST_ALL_PREFIXES(JSONParserTest, ConsumeList);
  FRIEND_TEST_ALL_PREFIXES(JSONParserTest, ConsumeString);
  FRIEND_TEST_ALL_PREFIXES(JSONParserTest, ConsumeLiterals);
  FRIEND_TEST_ALL_PREFIXES(JSONParserTest, ConsumeNumbers);
  FRIEND_TEST_ALL_PREFIXES(JSONParserTest, ErrorMessages);
};

// Used when decoding and an invalid utf-8 sequence is encountered.
BASE_EXPORT extern const char kUnicodeReplacementString[];

}  // namespace internal
}  // namespace base

#endif  // BASE_JSON_JSON_PARSER_H_
