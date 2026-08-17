/*
 * Copyright (C) 2004, 2006, 2007, 2008, 2009 Apple Inc. All rights reserved.
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
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_TEXT_ENCODING_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_TEXT_ENCODING_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/text_codec.h"
#include "third_party/blink/renderer/platform/wtf/wtf_export.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace WTF {

class WTF_EXPORT TextEncoding final {
  USING_FAST_MALLOC(TextEncoding);

 public:
  TextEncoding() = default;
  explicit TextEncoding(const char* name);
  explicit TextEncoding(const String& name);

  bool IsValid() const { return !name_.IsNull(); }
  const AtomicString& GetName() const { return name_; }
  bool UsesVisualOrdering() const;
  const TextEncoding& ClosestByteBasedEquivalent() const;
  const TextEncoding& EncodingForFormSubmission() const;

  String Decode(base::span<const uint8_t> data) const {
    bool ignored;
    return Decode(data, false, ignored);
  }
  String Decode(base::span<const uint8_t> data,
                bool stop_on_error,
                bool& saw_error) const;

  std::string Encode(const StringView&, UnencodableHandling) const;

  bool IsNonByteBasedEncoding() const;

 private:
  AtomicString name_;
};

inline bool operator==(const TextEncoding& a, const TextEncoding& b) {
  return a.GetName() == b.GetName();
}
inline bool operator!=(const TextEncoding& a, const TextEncoding& b) {
  return a.GetName() != b.GetName();
}

WTF_EXPORT const TextEncoding& ASCIIEncoding();
WTF_EXPORT const TextEncoding& Latin1Encoding();
WTF_EXPORT const TextEncoding& UnknownEncoding();
WTF_EXPORT const TextEncoding& UTF16BigEndianEncoding();
WTF_EXPORT const TextEncoding& UTF16LittleEndianEncoding();
WTF_EXPORT const TextEncoding& UTF8Encoding();
WTF_EXPORT const TextEncoding& WindowsLatin1Encoding();
}  // namespace WTF

using WTF::ASCIIEncoding;
using WTF::Latin1Encoding;
using WTF::UnknownEncoding;
using WTF::UTF16BigEndianEncoding;
using WTF::UTF16LittleEndianEncoding;
using WTF::UTF8Encoding;
using WTF::WindowsLatin1Encoding;

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_TEXT_ENCODING_H_
