/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
 * Copyright (C) 2012 Intel Corporation. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_PARSED_CONTENT_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_PARSED_CONTENT_TYPE_H_

#include <optional>

#include "third_party/blink/renderer/platform/network/parsed_content_header_field_parameters.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// ParsedContentType parses the content of a Content-Type header field as
// specified in RFC2045 into MIME type and parameters and stores them.
// FIXME: add support for comments.
class PLATFORM_EXPORT ParsedContentType final {
  STACK_ALLOCATED();

 public:
  using Mode = ParsedContentHeaderFieldParameters::Mode;

  explicit ParsedContentType(const String&, Mode = Mode::kNormal);

  String MimeType() const { return mime_type_; }
  String Charset() const;

  // Note that in the case of multiple values for the same name, the last value
  // is returned.
  String ParameterValueForName(const String& name) const {
    return IsValid() ? parameters_->ParameterValueForName(name) : String();
  }
  const ParsedContentHeaderFieldParameters& GetParameters() const {
    DCHECK(IsValid());
    return *parameters_;
  }

  bool IsValid() const { return !!parameters_; }

 private:
  String mime_type_;
  std::optional<ParsedContentHeaderFieldParameters> parameters_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_PARSED_CONTENT_TYPE_H_
