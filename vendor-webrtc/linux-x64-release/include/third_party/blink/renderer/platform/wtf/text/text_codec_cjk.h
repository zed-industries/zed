/*
 * Copyright (C) 2020 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_TEXT_CODEC_CJK_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_TEXT_CODEC_CJK_H_

#include <optional>

#include "third_party/blink/renderer/platform/wtf/functional.h"
#include "third_party/blink/renderer/platform/wtf/text/code_point_iterator.h"
#include "third_party/blink/renderer/platform/wtf/text/string_builder.h"
#include "third_party/blink/renderer/platform/wtf/text/text_codec.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/wtf_export.h"

namespace WTF {

// TextCodecCJK supports following encodings:
// * Japanese characters (EUC-JP, ISO-2022-JP, ShiftJIS)
// * Korean characters (EUC-KR)
// * Simplified Chinese characters (GB18030, GBK)
// Note: since setting up Big5 encode table failed with overflow error
// when we use it with ICU4C bundled with Chromium, we did not include
// Big5.
//
// ICU4C behaves much different from the WHATWG specification
// (https://encoding.spec.whatwg.org/). It was difficult to fulfill the
// specification by using TextCodecICU.
class TextCodecCJK final : public TextCodec {
 public:
  class Decoder;
  static void RegisterEncodingNames(EncodingNameRegistrar);
  static void RegisterCodecs(TextCodecRegistrar);
  // Returns true if the given `name` is supported.
  WTF_EXPORT static bool IsSupported(StringView name);

 private:
  enum class Encoding : uint8_t;
  explicit TextCodecCJK(Encoding);
  WTF_EXPORT static std::unique_ptr<TextCodec> Create(const TextEncoding&,
                                                      const void*);
  String Decode(base::span<const uint8_t> data,
                FlushBehavior,
                bool stop_on_error,
                bool& saw_error) override;
  std::string Encode(base::span<const UChar>, UnencodableHandling) override;
  std::string Encode(base::span<const LChar>, UnencodableHandling) override;

  Vector<uint8_t> EncodeCommon(StringView string, UnencodableHandling) const;

  const Encoding encoding_;
  std::unique_ptr<Decoder> decoder_;
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_TEXT_CODEC_CJK_H_
