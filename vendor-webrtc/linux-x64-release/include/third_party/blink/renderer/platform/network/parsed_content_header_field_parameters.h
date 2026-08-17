// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_PARSED_CONTENT_HEADER_FIELD_PARAMETERS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_PARSED_CONTENT_HEADER_FIELD_PARAMETERS_H_

#include <optional>

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class HeaderFieldTokenizer;

// Parses content header field parameters as specified in RFC2045 and stores
// them. It is used internally by ParsedContent* classes.
// FIXME: add support for comments.
class PLATFORM_EXPORT ParsedContentHeaderFieldParameters final {
  DISALLOW_NEW();

 public:
  struct NameValue {
    NameValue(String name, String value) : name(name), value(value) {}

    String name;
    String value;
  };

  using NameValuePairs = Vector<NameValue>;
  using const_iterator = NameValuePairs::const_iterator;
  using reverse_const_iterator = NameValuePairs::const_reverse_iterator;

  // When |Relaxed| is specified, the parser parses parameter values in a sloppy
  // manner, i.e., only ';' and '"' are treated as special characters.
  // See https://chromiumcodereview.appspot.com/23043002.
  enum class Mode {
    kNormal,
    kRelaxed,
  };

  // We use std::optional instead of WTF::Optional which requires its content
  // type to be fully defined. They are essentially same, so uses of this class
  // can (and should) use WTF::Optional to store the returned value.
  static std::optional<ParsedContentHeaderFieldParameters> Parse(
      HeaderFieldTokenizer,
      Mode);

  // Note that in the case of multiple values for the same name, the last value
  // is returned.
  String ParameterValueForName(const String&) const;
  size_t ParameterCount() const;
  bool HasDuplicatedNames() const;

  const_iterator begin() const { return parameters_.begin(); }
  const_iterator end() const { return parameters_.end(); }

  reverse_const_iterator rbegin() const { return parameters_.rbegin(); }
  reverse_const_iterator rend() const { return parameters_.rend(); }

 private:
  explicit ParsedContentHeaderFieldParameters(NameValuePairs parameters)
      : parameters_(std::move(parameters)) {}

  NameValuePairs parameters_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_PARSED_CONTENT_HEADER_FIELD_PARAMETERS_H_
