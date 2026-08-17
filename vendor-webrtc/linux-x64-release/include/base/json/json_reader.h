// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// A JSON parser, converting from a std::string_view to a base::Value.
//
// The JSON spec is:
// https://tools.ietf.org/rfc/rfc8259.txt
// which obsoletes the earlier RFCs 4627, 7158 and 7159.
//
// This RFC should be equivalent to the informal spec:
// https://www.json.org/json-en.html
//
// Implementation choices permitted by the RFC:
// - Nesting is limited (to a configurable depth, 200 by default).
// - Numbers are limited to those representable by a finite double. The
//   conversion from a JSON number (in the std::string_view input) to a
//   double-flavored base::Value may also be lossy.
// - The input (which must be UTF-8) may begin with a BOM (Byte Order Mark).
// - Duplicate object keys (strings) are silently allowed. Last key-value pair
//   wins. Previous pairs are discarded.
//
// Configurable (see the JSONParserOptions type) deviations from the RFC:
// - Allow trailing commas: "[1,2,]".
// - Replace invalid Unicode with U+FFFD REPLACEMENT CHARACTER.
// - Allow "// etc\n" and "/* etc */" C-style comments.
// - Allow ASCII control characters, including literal (not escaped) NUL bytes
//   and new lines, within a JSON string.
// - Allow "\\v" escapes within a JSON string, producing a vertical tab.
// - Allow "\\x23" escapes within a JSON string. Subtly, the 2-digit hex value
//   is a Unicode code point, not a UTF-8 byte. For example, "\\xFF" in the
//   JSON source decodes to a base::Value whose string contains "\xC3\xBF", the
//   UTF-8 encoding of U+00FF LATIN SMALL LETTER Y WITH DIAERESIS. Converting
//   from UTF-8 to UTF-16, e.g. via UTF8ToWide, will recover a 16-bit 0x00FF.

#ifndef BASE_JSON_JSON_READER_H_
#define BASE_JSON_JSON_READER_H_

#include <optional>
#include <string>
#include <string_view>

#include "base/base_export.h"
#include "base/json/json_common.h"
#include "base/strings/string_number_conversions.h"
#include "base/types/expected.h"
#include "base/values.h"

namespace base {

enum JSONParserOptions {
  // Parses the input strictly according to RFC 8259.
  JSON_PARSE_RFC = 0,

  // Allows commas to exist after the last element in structures.
  JSON_ALLOW_TRAILING_COMMAS = 1 << 0,

  // If set the parser replaces invalid code points (i.e. lone
  // surrogates) with the Unicode replacement character (U+FFFD). If
  // not set, invalid code points trigger a hard error and parsing
  // fails.
  JSON_REPLACE_INVALID_CHARACTERS = 1 << 1,

  // Allows both C (/* */) and C++ (//) style comments.
  JSON_ALLOW_COMMENTS = 1 << 2,

  // Permits \\v vertical tab escapes.
  JSON_ALLOW_VERT_TAB = 1 << 3,

  // Permits \\xNN escapes as described above.
  JSON_ALLOW_X_ESCAPES = 1 << 4,

  // Permits exactly \r and \n to occur in strings, which is normally not
  // allowed; this is a subset of the behavior of JSON_ALLOW_CONTROL_CHARS.
  JSON_ALLOW_NEWLINES_IN_STRINGS = 1 << 5,

  // This parser historically accepted, without configuration flags,
  // non-standard JSON extensions. This flag enables that traditional parsing
  // behavior.
  //
  // This set of options is mirrored in Rust
  // base::JsonOptions::with_chromium_extensions().
  JSON_PARSE_CHROMIUM_EXTENSIONS = JSON_ALLOW_COMMENTS |
                                   JSON_ALLOW_NEWLINES_IN_STRINGS |
                                   JSON_ALLOW_X_ESCAPES,
};

class BASE_EXPORT JSONReader {
 public:
  struct BASE_EXPORT Error {
    std::string message;
    int line = 0;
    int column = 0;

    std::string ToString() const {
      return "line " + base::NumberToString(line) + ", column " +
             base::NumberToString(column) + ": " + message;
    }
  };

  using Result = base::expected<Value, Error>;

  // This class contains only static methods.
  JSONReader() = delete;
  JSONReader(const JSONReader&) = delete;
  JSONReader& operator=(const JSONReader&) = delete;

  // Reads and parses |json|, returning a Value.
  // If |json| is not a properly formed JSON string, returns std::nullopt.
  static std::optional<Value> Read(
      std::string_view json,
      int options = JSON_PARSE_CHROMIUM_EXTENSIONS,
      size_t max_depth = internal::kAbsoluteMaxDepth);

  // Reads and parses |json|, returning a Value::Dict.
  // If |json| is not a properly formed JSON dict string, returns std::nullopt.
  static std::optional<Value::Dict> ReadDict(
      std::string_view json,
      int options = JSON_PARSE_CHROMIUM_EXTENSIONS,
      size_t max_depth = internal::kAbsoluteMaxDepth);

  // Reads and parses |json|, returning a Value::List.
  // If |json| is not a properly formed JSON list string, returns std::nullopt.
  static std::optional<Value::List> ReadList(
      std::string_view json,
      int options = JSON_PARSE_CHROMIUM_EXTENSIONS,
      size_t max_depth = internal::kAbsoluteMaxDepth);

  // Reads and parses |json| like Read(). On success returns a Value as the
  // expected value. Otherwise, it returns an Error instance, populated with a
  // formatted error message, an error code, and the error location if
  // appropriate as the error value of the expected type.
  static Result ReadAndReturnValueWithError(
      std::string_view json,
      int options = JSON_PARSE_CHROMIUM_EXTENSIONS);

  // Determine whether the Rust parser is in use.
  static bool UsingRust();
};

}  // namespace base

#endif  // BASE_JSON_JSON_READER_H_
