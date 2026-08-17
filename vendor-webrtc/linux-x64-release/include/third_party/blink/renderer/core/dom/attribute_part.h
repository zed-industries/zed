// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ATTRIBUTE_PART_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ATTRIBUTE_PART_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_part_init.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/core/dom/node_part.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class ExceptionState;

// Implementation of the AttributePart class, which is part of the DOM Parts
// API. A AttributePart stores a reference to a single |Node| in the DOM tree.
class CORE_EXPORT AttributePart : public NodePart {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static AttributePart* Create(PartRootUnion* root_union,
                               Node* node,
                               AtomicString local_name,
                               const PartInit* init,
                               ExceptionState& exception_state);
  AttributePart(PartRoot& root,
                Element& element,
                AtomicString local_name,
                const PartInit* init)
      : AttributePart(root,
                      element,
                      local_name,
                      init && init->hasMetadata() ? init->metadata()
                                                  : Vector<String>()) {}
  AttributePart(PartRoot& root,
                Element& element,
                AtomicString local_name,
                Vector<String> metadata);
  AttributePart(const AttributePart&) = delete;
  ~AttributePart() override = default;

  Part* ClonePart(NodeCloningData&, Node&) const override;

  // AttributePart API
  AtomicString localName() const { return local_name_; }

 private:
  AtomicString local_name_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ATTRIBUTE_PART_H_
