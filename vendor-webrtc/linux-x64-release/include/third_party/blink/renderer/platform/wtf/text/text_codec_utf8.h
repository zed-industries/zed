/*
 * Copyright (C) 2011 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_TEXT_CODEC_UTF8_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_TEXT_CODEC_UTF8_H_

#include <unicode/utf8.h>
#include <memory>

#include "third_party/blink/renderer/platform/wtf/text/text_codec.h"

namespace WTF {

class TextCodecUTF8 : public TextCodec {
 public:
  static void RegisterEncodingNames(EncodingNameRegistrar);
  static void RegisterCodecs(TextCodecRegistrar);

 protected:
  TextCodecUTF8() : partial_sequence_size_(0) {}

 private:
  static std::unique_ptr<TextCodec> Create(const TextEncoding&, const void*);

  String Decode(base::span<const uint8_t> data,
                FlushBehavior,
                bool stop_on_error,
                bool& saw_error) override;
  std::string Encode(base::span<const UChar>, UnencodableHandling) override;
  std::string Encode(base::span<const LChar>, UnencodableHandling) override;

  // See comment above TextCodec::EncodeInto for more information.
  // This implementation writes as many code points to |destination| as will
  // fit, while never writing partial code points. If EncodeIntoResult's
  // |bytes_written| member is less than |capacity|, the remaining
  // |capacity| - |bytes_written| bytes remain untouched.
  EncodeIntoResult EncodeInto(base::span<const UChar>,
                              base::span<uint8_t> destination) override;
  EncodeIntoResult EncodeInto(base::span<const LChar>,
                              base::span<uint8_t> destination) override;

  template <typename CharType>
  std::string EncodeCommon(base::span<const CharType> characters);
  template <typename CharType>
  EncodeIntoResult EncodeIntoCommon(base::span<const CharType> characters,
                                    base::span<uint8_t> destination);

  template <typename CharType>
  bool HandlePartialSequence(CharType*& destination,
                             const uint8_t*& source,
                             const uint8_t* end,
                             bool flush,
                             bool stop_on_error,
                             bool& saw_error);
  void HandleError(int character,
                   UChar*& destination,
                   bool stop_on_error,
                   bool& saw_error);
  void ConsumePartialSequenceBytes(int num_bytes);

  int partial_sequence_size_;
  uint8_t partial_sequence_[U8_MAX_LENGTH];
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_TEXT_CODEC_UTF8_H_
