// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_HEADER_FIELD_TOKENIZER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_HEADER_FIELD_TOKENIZER_H_

#include "third_party/blink/renderer/platform/network/parsed_content_header_field_parameters.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// Parses header fields into tokens, quoted strings and separators.
// Commonly used by ParsedContent* classes.
class PLATFORM_EXPORT HeaderFieldTokenizer final {
  STACK_ALLOCATED();

 public:
  using Mode = ParsedContentHeaderFieldParameters::Mode;

  explicit HeaderFieldTokenizer(const String& header_field);
  HeaderFieldTokenizer(HeaderFieldTokenizer&&);

  // Try to parse a separator character, a token or either a token or a quoted
  // string from the |header_field| input. Return |true| on success. Return
  // |false| if the separator character, the token or the quoted string is
  // missing or invalid.
  bool Consume(char);
  bool ConsumeToken(Mode, StringView& output);
  bool ConsumeTokenOrQuotedString(Mode, String& output);

  // Consume all characters before (but excluding) any of the characters from
  // the Vector parameter are found.
  // Because we potentially have to iterate through the entire Vector for each
  // character of the base string, the Vector should be small (< 3 members).
  void ConsumeBeforeAnyCharMatch(Vector<LChar>);

  unsigned Index() const { return index_; }
  bool IsConsumed() const { return index_ >= input_.length(); }

 private:
  bool ConsumeQuotedString(String& output);
  void SkipOptionalWhitespace();

  unsigned index_;
  const String input_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_HEADER_FIELD_TOKENIZER_H_
