/*
 * Copyright (C) 2006 Alexey Proskuryakov <ap@webkit.org>
 * Copyright (C) 2010 Patrick Gansterer <paroga@paroga.com>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_BASE64_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_BASE64_H_

#include "third_party/blink/renderer/platform/wtf/text/string_view.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/blink/renderer/platform/wtf/wtf_export.h"

namespace WTF {

// Compliant with https://infra.spec.whatwg.org/#forgiving-base64-encode.
WTF_EXPORT void Base64Encode(base::span<const uint8_t>, Vector<char>&);
[[nodiscard]] WTF_EXPORT String Base64Encode(base::span<const uint8_t>);

enum class Base64DecodePolicy {
  // Compliant with https://infra.spec.whatwg.org/#forgiving-base64-decode.
  kForgiving,

  // Same behavior as kForgiving except:
  // - Step 1 (removing HTML whitespace) is omitted.
  // - Step 2.1 is modified to remove all padding chars from the input instead
  //   of a maximum of 2 chars.
  kNoPaddingValidation,
};
WTF_EXPORT bool Base64Decode(
    const StringView&,
    Vector<uint8_t>&,
    Base64DecodePolicy policy = Base64DecodePolicy::kNoPaddingValidation);

WTF_EXPORT bool Base64UnpaddedURLDecode(const String& in, Vector<uint8_t>&);

// Given an encoding in either base64 or base64url, returns a normalized
// encoding in plain base64.
WTF_EXPORT String NormalizeToBase64(const String&);

WTF_EXPORT String Base64URLEncode(base::span<const uint8_t>);

}  // namespace WTF

using WTF::Base64Decode;
using WTF::Base64DecodePolicy;
using WTF::Base64Encode;

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_BASE64_H_
