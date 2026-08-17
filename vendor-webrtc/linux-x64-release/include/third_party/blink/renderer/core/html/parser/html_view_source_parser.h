/*
 * Copyright (C) 2010 Google, Inc. All Rights Reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_VIEW_SOURCE_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_VIEW_SOURCE_PARSER_H_

#include <memory>
#include "base/notreached.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/decoded_data_document_parser.h"
#include "third_party/blink/renderer/core/html/html_view_source_document.h"
#include "third_party/blink/renderer/core/html/parser/html_input_stream.h"
#include "third_party/blink/renderer/core/html/parser/html_tokenizer.h"

namespace blink {

class CORE_EXPORT HTMLViewSourceParser final
    : public DecodedDataDocumentParser {
 public:
  HTMLViewSourceParser(HTMLViewSourceDocument&, const String& mime_type);
  ~HTMLViewSourceParser() override = default;

 private:
  // DocumentParser
  void insert(const String&) override { NOTREACHED(); }
  void Append(const String&) override;
  void Finish() override;

  HTMLViewSourceDocument* GetDocument() const {
    return static_cast<HTMLViewSourceDocument*>(
        DecodedDataDocumentParser::GetDocument());
  }

  void PumpTokenizer();
  void UpdateTokenizerState();

  void StartTracker(SegmentedString&, HTMLTokenizer*, HTMLToken*);
  void EndTracker(SegmentedString&, HTMLTokenizer*);
  String SourceForToken(const HTMLToken&);
  bool NeedToCheckTokenizerBuffer(HTMLTokenizer*);

  HTMLInputStream input_;
  // Owned by `tokenizer_`.
  HTMLToken* token_ = nullptr;
  std::unique_ptr<HTMLTokenizer> tokenizer_;
  bool tracker_is_started_;

  SegmentedString previous_source_;
  SegmentedString current_source_;

  String cached_source_for_token_;

  // Offset into `current_source_` that the current token starts at.
  int token_start_ = 0;

  // Offset into `current_source_` that the current token ends at.
  int token_end_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_VIEW_SOURCE_PARSER_H_
