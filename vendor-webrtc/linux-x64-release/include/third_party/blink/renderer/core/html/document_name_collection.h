// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_DOCUMENT_NAME_COLLECTION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_DOCUMENT_NAME_COLLECTION_H_

#include "third_party/blink/renderer/core/html/html_element.h"
#include "third_party/blink/renderer/core/html/html_name_collection.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class DocumentNameCollection final : public HTMLNameCollection {
 public:
  DocumentNameCollection(ContainerNode& document, const AtomicString& name);
  DocumentNameCollection(ContainerNode& document,
                         CollectionType type,
                         const AtomicString& name);

  HTMLElement* Item(unsigned offset) const {
    return To<HTMLElement>(HTMLNameCollection::item(offset));
  }

  bool ElementMatches(const HTMLElement&) const;
};

template <>
struct DowncastTraits<DocumentNameCollection> {
  static bool AllowFrom(const LiveNodeListBase& collection) {
    return collection.GetType() == kDocumentNamedItems;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_DOCUMENT_NAME_COLLECTION_H_
