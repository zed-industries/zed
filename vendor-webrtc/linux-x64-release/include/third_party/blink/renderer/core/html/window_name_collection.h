// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_WINDOW_NAME_COLLECTION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_WINDOW_NAME_COLLECTION_H_

#include "third_party/blink/renderer/core/html/html_name_collection.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class WindowNameCollection final : public HTMLNameCollection {
 public:
  WindowNameCollection(ContainerNode& document, const AtomicString& name);
  WindowNameCollection(ContainerNode& document,
                       CollectionType type,
                       const AtomicString& name);

  bool ElementMatches(const Element&) const;
};

template <>
struct DowncastTraits<WindowNameCollection> {
  static bool AllowFrom(const LiveNodeListBase& collection) {
    return collection.GetType() == kWindowNamedItems;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_WINDOW_NAME_COLLECTION_H_
