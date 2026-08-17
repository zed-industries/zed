/*
 * Copyright (C) 2010 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_STRING_CONCATENATE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_STRING_CONCATENATE_H_

#include <string_view>

#include "base/containers/span.h"
#include "base/notreached.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace WTF {

template <typename StringType>
class StringTypeAdapter {
  DISALLOW_NEW();
};

template <>
class StringTypeAdapter<char> {
  DISALLOW_NEW();

 public:
  explicit StringTypeAdapter<char>(char buffer) : buffer_(buffer) {}

  size_t length() const { return 1; }
  bool Is8Bit() const { return true; }

  void WriteTo(base::span<LChar> destination) const {
    destination[0] = buffer_;
  }
  void WriteTo(base::span<UChar> destination) const {
    destination[0] = buffer_;
  }

 private:
  const LChar buffer_;
};

template <>
class StringTypeAdapter<LChar> : public StringTypeAdapter<char> {
 public:
  explicit StringTypeAdapter<LChar>(LChar buffer)
      : StringTypeAdapter<char>(buffer) {}
};

template <>
class StringTypeAdapter<UChar> {
  DISALLOW_NEW();

 public:
  explicit StringTypeAdapter<UChar>(UChar buffer) : buffer_(buffer) {}

  size_t length() const { return 1; }
  bool Is8Bit() const { return buffer_ <= 0xff; }

  void WriteTo(base::span<LChar> destination) const {
    DCHECK(Is8Bit());
    destination[0] = static_cast<LChar>(buffer_);
  }

  void WriteTo(base::span<UChar> destination) const {
    destination[0] = buffer_;
  }

 private:
  const UChar buffer_;
};

template <>
class WTF_EXPORT StringTypeAdapter<const char*> {
  DISALLOW_NEW();

 public:
  explicit StringTypeAdapter<const char*>(const char* buffer)
      : buffer_(base::as_byte_span(std::string_view(buffer))) {}

  size_t length() const { return buffer_.size(); }
  bool Is8Bit() const { return true; }

  void WriteTo(base::span<LChar> destination) const;
  void WriteTo(base::span<UChar> destination) const;

 private:
  const base::span<const LChar> buffer_;
};

template <>
class WTF_EXPORT StringTypeAdapter<const LChar*>
    : StringTypeAdapter<const char*> {
 public:
  explicit StringTypeAdapter<const LChar*>(const LChar* buffer)
      : StringTypeAdapter<const char*>(reinterpret_cast<const char*>(buffer)) {}
};

template <>
class WTF_EXPORT StringTypeAdapter<char*>
    : public StringTypeAdapter<const char*> {
 public:
  explicit StringTypeAdapter<char*>(char* buffer)
      : StringTypeAdapter<const char*>(buffer) {}
};

template <>
class WTF_EXPORT StringTypeAdapter<LChar*>
    : public StringTypeAdapter<const LChar*> {
 public:
  explicit StringTypeAdapter<LChar*>(LChar* buffer)
      : StringTypeAdapter<const LChar*>(buffer) {}
};

template <>
class WTF_EXPORT StringTypeAdapter<const UChar*> {
  DISALLOW_NEW();

 public:
  explicit StringTypeAdapter(const UChar* buffer);

  size_t length() const { return buffer_.size(); }
  bool Is8Bit() const { return false; }

  void WriteTo(base::span<LChar> destination) const { NOTREACHED(); }
  void WriteTo(base::span<UChar> destination) const;

 private:
  const base::span<const UChar> buffer_;
};

template <>
class WTF_EXPORT StringTypeAdapter<StringView> {
  DISALLOW_NEW();

 public:
  explicit StringTypeAdapter(const StringView& view) : view_(view) {}

  size_t length() const { return view_.length(); }
  bool Is8Bit() const { return view_.Is8Bit(); }

  void WriteTo(base::span<LChar> destination) const;
  void WriteTo(base::span<UChar> destination) const;

 private:
  const StringView view_;
};

template <>
class StringTypeAdapter<String> : public StringTypeAdapter<StringView> {
 public:
  explicit StringTypeAdapter(const String& string)
      : StringTypeAdapter<StringView>(string) {}
};

template <>
class StringTypeAdapter<AtomicString> : public StringTypeAdapter<StringView> {
 public:
  explicit StringTypeAdapter(const AtomicString& string)
      : StringTypeAdapter<StringView>(string) {}
};

}  // namespace WTF

#include "third_party/blink/renderer/platform/wtf/text/string_operators.h"
#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_STRING_CONCATENATE_H_
