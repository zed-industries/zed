// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_JSON_JSON_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_JSON_JSON_PARSER_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

#include <memory>

namespace blink {

class JSONValue;

enum class JSONParseErrorType {
  kNoError,
  kUnexpectedToken,
  kSyntaxError,
  kInvalidEscape,
  kTooMuchNesting,
  kUnexpectedDataAfterRoot,
  kUnsupportedEncoding,
};

struct PLATFORM_EXPORT JSONParseError {
  JSONParseErrorType type;
  int line;
  int column;
  String message;

  // These aren't errors per se, but irregularities that clients may wish to
  // surface to the user.

  // Keys for which one object had the same key twice.
  Vector<String> duplicate_keys;
};

enum class JSONCommentState {
  kDisallowed = 0,
  kAllowedButAbsent,
  kAllowedAndPresent,
};

// Parses |json| string and returns a value it represents.
// In case of parsing failure, returns nullptr.
// Optional error struct may be passed in, which will contain
// error details or |kNoError| if parsing succeeded.
PLATFORM_EXPORT std::unique_ptr<JSONValue> ParseJSON(
    const String& json,
    JSONParseError* opt_error = nullptr);

// Do not introduce new uses of this function; JSON comments are not standard.
//
// If provided, |opt_has_comments| will indicate whether comments were found.
PLATFORM_EXPORT std::unique_ptr<JSONValue> ParseJSONWithCommentsDeprecated(
    const String& json,
    JSONParseError* opt_error = nullptr,
    bool* opt_has_comments = nullptr);

// Exposed for testing.
PLATFORM_EXPORT std::unique_ptr<JSONValue> ParseJSON(
    const String& json,
    JSONCommentState& comment_state,
    int max_depth,
    JSONParseError* opt_error = nullptr);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_JSON_JSON_PARSER_H_
