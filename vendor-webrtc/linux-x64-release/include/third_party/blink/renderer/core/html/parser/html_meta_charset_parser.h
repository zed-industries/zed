/*
 * Copyright (C) 2010 Google Inc. All Rights Reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_META_CHARSET_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_META_CHARSET_PARSER_H_

#include <memory>

#include "base/memory/ptr_util.h"
#include "third_party/blink/renderer/platform/text/segmented_string.h"
#include "third_party/blink/renderer/platform/wtf/text/text_codec.h"
#include "third_party/blink/renderer/platform/wtf/text/text_encoding.h"

namespace blink {

class HTMLToken;
class HTMLTokenizer;

class HTMLMetaCharsetParser {
  USING_FAST_MALLOC(HTMLMetaCharsetParser);

 public:
  HTMLMetaCharsetParser();
  HTMLMetaCharsetParser(const HTMLMetaCharsetParser&) = delete;
  HTMLMetaCharsetParser& operator=(const HTMLMetaCharsetParser&) = delete;
  ~HTMLMetaCharsetParser();

  // Returns true if done checking, regardless whether an encoding is found.
  bool CheckForMetaCharset(base::span<const char> data);

  const WTF::TextEncoding& Encoding() { return encoding_; }

 private:
  bool ProcessMeta(const HTMLToken& token);

  std::unique_ptr<HTMLTokenizer> tokenizer_;
  std::unique_ptr<TextCodec> assumed_codec_;
  SegmentedString input_;
  bool in_head_section_;

  bool done_checking_;
  WTF::TextEncoding encoding_;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_META_CHARSET_PARSER_H_
