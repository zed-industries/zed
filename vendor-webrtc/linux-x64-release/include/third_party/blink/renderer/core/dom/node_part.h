// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NODE_PART_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NODE_PART_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_part_init.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/core/dom/part_root.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class ExceptionState;

// Implementation of the NodePart class, which is part of the DOM Parts API.
// A NodePart stores a reference to a single |Node| in the DOM tree.
class CORE_EXPORT NodePart : public Part {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static NodePart* Create(PartRootUnion* root_union,
                          Node* node,
                          const PartInit* init,
                          ExceptionState& exception_state);
  NodePart(PartRoot& root, Node& node, const PartInit* init)
      : NodePart(root,
                 node,
                 init && init->hasMetadata() ? init->metadata()
                                             : Vector<String>()) {}
  NodePart(PartRoot& root,
           Node& node,
           Vector<String> metadata);
  ~NodePart() override = default;

  void Trace(Visitor* visitor) const override;
  void disconnect() override;
  bool IsValid() const override {
    // A NodePart is valid if the base Part is valid (has a root), and if there
    // is a node reference.
    return Part::IsValid() && node_;
  }
  Node* NodeToSortBy() const override;
  Part* ClonePart(NodeCloningData&, Node&) const override;
  Document& GetDocument() const override;

  // NodePart API
  Node* node() const { return node_.Get(); }

 private:
  Member<Node> node_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NODE_PART_H_
