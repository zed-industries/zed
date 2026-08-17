// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_PARSED_CONTENT_DISPOSITION_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_PARSED_CONTENT_DISPOSITION_H_

#include <optional>

#include "third_party/blink/renderer/platform/network/parsed_content_header_field_parameters.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// Parses the content of a Content-Disposition header field into disposition
// type and parameters and stores them.
class PLATFORM_EXPORT ParsedContentDisposition final {
  STACK_ALLOCATED();

 public:
  using Mode = ParsedContentHeaderFieldParameters::Mode;

  explicit ParsedContentDisposition(const String&, Mode = Mode::kNormal);

  String Type() const { return type_; }
  String Filename() const;

  // Note that in the case of multiple values for the same name, the last value
  // is returned.
  String ParameterValueForName(const String& name) const {
    return IsValid() ? parameters_->ParameterValueForName(name) : String();
  }
  bool IsValid() const { return !!parameters_; }

 private:
  String type_;
  std::optional<ParsedContentHeaderFieldParameters> parameters_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_PARSED_CONTENT_DISPOSITION_H_
