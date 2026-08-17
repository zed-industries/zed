/*
 * Copyright (C) 2004, 2006 Apple Computer, Inc.  All rights reserved.
 * Copyright (C) 2006 Alexey Proskuryakov <ap@nypop.com>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_TEXT_CODEC_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_TEXT_CODEC_H_

#include <memory>
#include "base/notreached.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_uchar.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace WTF {

class TextEncoding;

// Specifies what will happen when a character is encountered that is
// not encodable in the character set.
enum UnencodableHandling {
  // Encodes the character as an XML entity. For example, U+06DE
  // would be "&#1758;" (0x6DE = 1758 in octal).
  kEntitiesForUnencodables,

  // Encodes the character as en entity as above, but escaped
  // non-alphanumeric characters. This is used in URLs.
  // For example, U+6DE would be "%26%231758%3B".
  kURLEncodedEntitiesForUnencodables,

  // Encodes the character as a CSS entity.  For example U+06DE
  // would be \06de.  See: https://www.w3.org/TR/css-syntax-3/#escaping
  kCSSEncodedEntitiesForUnencodables,

  // Used when all characters can be encoded in the character set. Only
  // applicable to UTF-N encodings.
  kNoUnencodables,
};

enum class FlushBehavior {
  // More bytes are coming, don't flush the codec.
  kDoNotFlush = 0,

  // A fetch has hit EOF. Some codecs handle fetches differently, for compat
  // reasons.
  kFetchEOF,

  // Do a full flush of the codec.
  kDataEOF
};

class WTF_EXPORT TextCodec {
  USING_FAST_MALLOC(TextCodec);

 public:
  TextCodec() = default;
  TextCodec(const TextCodec&) = delete;
  TextCodec& operator=(const TextCodec&) = delete;
  virtual ~TextCodec();

  struct EncodeIntoResult {
    wtf_size_t code_units_read;
    size_t bytes_written;
  };

  String Decode(base::span<const uint8_t> data,
                FlushBehavior flush = FlushBehavior::kDoNotFlush) {
    bool ignored;
    return Decode(data, flush, false, ignored);
  }

  virtual String Decode(base::span<const uint8_t> data,
                        FlushBehavior,
                        bool stop_on_error,
                        bool& saw_error) = 0;
  virtual std::string Encode(base::span<const UChar>, UnencodableHandling) = 0;
  virtual std::string Encode(base::span<const LChar>, UnencodableHandling) = 0;

  // EncodeInto is meant only to encode UTF8 bytes into an unsigned char*
  // buffer; therefore this method is only usefully overridden by TextCodecUTF8.
  virtual EncodeIntoResult EncodeInto(base::span<const LChar>,
                                      base::span<uint8_t> destination) {
    NOTREACHED();
  }
  virtual EncodeIntoResult EncodeInto(base::span<const UChar>,
                                      base::span<uint8_t> destination) {
    NOTREACHED();
  }

  static std::string GetUnencodableReplacement(UChar32 code_point,
                                               UnencodableHandling);
};

typedef void (*EncodingNameRegistrar)(const char* alias, const char* name);

typedef std::unique_ptr<TextCodec> (
    *NewTextCodecFunction)(const TextEncoding&, const void* additional_data);
typedef void (*TextCodecRegistrar)(const char* name,
                                   NewTextCodecFunction,
                                   const void* additional_data);

}  // namespace WTF

using WTF::TextCodec;

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_TEXT_CODEC_H_
