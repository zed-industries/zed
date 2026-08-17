// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_DOCUMENT_METADATA_DOCUMENT_METADATA_EXTRACTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_DOCUMENT_METADATA_DOCUMENT_METADATA_EXTRACTOR_H_

#include "components/schema_org/common/metadata.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/document_metadata/document_metadata.mojom-blink-forward.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Document;

// Extract structured metadata (currently schema.org in JSON-LD). The extraction
// must be done after the document has finished parsing.
class MODULES_EXPORT DocumentMetadataExtractor final {
  STATIC_ONLY(DocumentMetadataExtractor);

 public:
  static mojom::blink::WebPagePtr Extract(const Document&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_DOCUMENT_METADATA_DOCUMENT_METADATA_EXTRACTOR_H_
