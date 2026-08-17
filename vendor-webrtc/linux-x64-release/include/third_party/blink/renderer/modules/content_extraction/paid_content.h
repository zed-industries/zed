// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CONTENT_EXTRACTION_PAID_CONTENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CONTENT_EXTRACTION_PAID_CONTENT_H_

#include "base/memory/stack_allocated.h"
#include "third_party/blink/renderer/core/dom/container_node.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/json/json_values.h"

namespace blink {

class PaidContent final {
  STACK_ALLOCATED();

 public:
  // Queries the document for elements marked as isAccessibleForFree=false.
  bool QueryPaidElements(Document& document);

  // Returns true if the element is marked as isAccessibleForFree=false.
  bool IsPaidElement(const Element* element) const;

 private:
  // Whether to check for microdata annotations while walking.
  HeapHashMap<WeakMember<Document>, bool> check_microdata_;

  // Appends elements found by the cssSelector in the hasPart object.
  bool AppendHasPartElements(Document& document, JSONObject& hasPart_obj);

  // List of nodes marked as isAccessibleForFree=false.
  HeapVector<Member<Element>> paid_elements_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CONTENT_EXTRACTION_PAID_CONTENT_H_
