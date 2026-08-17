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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DECODED_DATA_DOCUMENT_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DECODED_DATA_DOCUMENT_PARSER_H_

#include <memory>
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/document_parser.h"

namespace blink {
class TextResourceDecoder;

class CORE_EXPORT DecodedDataDocumentParser : public DocumentParser {
 public:
  // Only used by the XMLDocumentParser to communicate back to
  // XMLHttpRequest if the responseXML was well formed.
  virtual bool WellFormed() const { return true; }

  // The below functions are used by DocumentWriter (the loader).
  void AppendBytes(base::span<const uint8_t> bytes) override;
  virtual void Flush();
  bool NeedsDecoder() const final { return needs_decoder_; }
  void SetDecoder(std::unique_ptr<TextResourceDecoder>) override;
  void AppendDecodedData(const String& data,
                         const DocumentEncodingData& encoding_data) override;

 protected:
  explicit DecodedDataDocumentParser(Document&);
  ~DecodedDataDocumentParser() override;

 private:
  void UpdateDocument(const String& decoded_data);

  bool needs_decoder_;
  std::unique_ptr<TextResourceDecoder> decoder_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DECODED_DATA_DOCUMENT_PARSER_H_
