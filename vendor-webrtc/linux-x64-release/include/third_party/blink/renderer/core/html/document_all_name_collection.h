// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_DOCUMENT_ALL_NAME_COLLECTION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_DOCUMENT_ALL_NAME_COLLECTION_H_

#include "third_party/blink/renderer/core/html/html_name_collection.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

// DocumentAllNameCollection implements document.all namedItem as
// HTMLCollection.
class DocumentAllNameCollection final : public HTMLNameCollection {
 public:
  DocumentAllNameCollection(ContainerNode& document, const AtomicString& name);
  DocumentAllNameCollection(ContainerNode& document,
                            CollectionType type,
                            const AtomicString& name);

  bool ElementMatches(const Element&) const;
};

template <>
struct DowncastTraits<DocumentAllNameCollection> {
  static bool AllowFrom(const LiveNodeListBase& collection) {
    return collection.GetType() == kDocumentAllNamedItems;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_DOCUMENT_ALL_NAME_COLLECTION_H_
