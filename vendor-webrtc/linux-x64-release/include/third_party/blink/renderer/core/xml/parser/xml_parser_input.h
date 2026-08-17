/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_XML_PARSER_XML_PARSER_INPUT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_XML_PARSER_XML_PARSER_INPUT_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class XMLParserInput {
  STACK_ALLOCATED();

 public:
  explicit XMLParserInput(const String& source)
      : source_(source), encoding_(nullptr), data_(nullptr), size_(0) {
    if (source_.empty())
      return;

    const UChar kBOM = 0xFEFF;
    const unsigned char bom_high_byte =
        *reinterpret_cast<const unsigned char*>(&kBOM);

    if (source_.Is8Bit()) {
      encoding_ = "iso-8859-1";
    } else {
      encoding_ = bom_high_byte == 0xFF ? "UTF-16LE" : "UTF-16BE";
    }
    auto byte_span = base::as_chars(source.RawByteSpan());
    data_ = byte_span.data();
    size_ = base::checked_cast<int>(byte_span.size());
  }

  const char* Encoding() const { return encoding_; }
  const char* Data() const { return data_; }
  int size() const { return size_; }

 private:
  String source_;
  const char* encoding_;
  const char* data_;
  int size_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_XML_PARSER_XML_PARSER_INPUT_H_
