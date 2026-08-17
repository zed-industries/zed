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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_STRING_UTF8_ADAPTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_STRING_UTF8_ADAPTOR_H_

#include <string_view>

#include "base/containers/span.h"
#include "base/memory/raw_span.h"
#include "third_party/blink/renderer/platform/wtf/text/string_view.h"
#include "third_party/blink/renderer/platform/wtf/text/text_encoding.h"
#include "third_party/blink/renderer/platform/wtf/wtf_export.h"

namespace WTF {

// This class lets you get UTF-8 data out of a String without mallocing a
// separate buffer to hold the data if the String happens to be 8 bit and
// contain only ASCII characters.
class WTF_EXPORT StringUTF8Adaptor final {
  DISALLOW_NEW();

 public:
  using iterator = base::raw_span<const char>::iterator;

  explicit StringUTF8Adaptor(
      StringView string,
      Utf8ConversionMode mode = Utf8ConversionMode::kLenient);
  ~StringUTF8Adaptor();

  const char* data() const { return span_.data(); }
  wtf_size_t size() const { return span_.size(); }

  // Iterators, so this type meets the requirements of
  // `std::ranges::contiguous_range`.
  iterator begin() const { return span_.begin(); }
  iterator end() const { return span_.end(); }

  std::string_view AsStringView() const { return base::as_string_view(span_); }

 private:
  std::string utf8_buffer_;
  base::raw_span<const char> span_;
};

}  // namespace WTF

using WTF::StringUTF8Adaptor;

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_STRING_UTF8_ADAPTOR_H_
