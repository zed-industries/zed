// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_JSON_DOCUMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_JSON_DOCUMENT_H_

#include "third_party/blink/renderer/core/html/html_document.h"
#include "third_party/blink/renderer/core/html/parser/html_document_parser.h"

namespace blink {
class JSONDocument : public HTMLDocument {
 public:
  JSONDocument(const DocumentInit&);
  bool IsJSONDocument() const override { return true; }

 private:
  DocumentParser* CreateParser() override;
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_JSON_DOCUMENT_H_
