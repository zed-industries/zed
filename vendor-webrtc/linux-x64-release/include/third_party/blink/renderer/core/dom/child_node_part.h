// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_CHILD_NODE_PART_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_CHILD_NODE_PART_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_union_node_string_trustedscript.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/container_node.h"
#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/core/dom/node_part.h"
#include "third_party/blink/renderer/core/dom/part_root.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/runtime_enabled_features.h"

namespace blink {

class PartInit;
class PartRootCloneOptions;

// Implementation of the ChildNodePart class, which is part of the DOM Parts
// API. A ChildNodePart stores a reference to a range of nodes within the
// children of a single parent |Node| in the DOM tree.
class CORE_EXPORT ChildNodePart : public Part, public PartRoot {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static ChildNodePart* Create(PartRootUnion* root_union,
                               Node* previous_sibling,
                               Node* next_sibling,
                               const PartInit* init,
                               ExceptionState& exception_state);
  ChildNodePart(PartRoot& root,
                Node& previous_sibling,
                Node& next_sibling,
                const PartInit* init)
      : ChildNodePart(root,
                      previous_sibling,
                      next_sibling,
                      init && init->hasMetadata() ? init->metadata()
                                                  : Vector<String>()) {}
  ChildNodePart(PartRoot& root,
                Node& previous_sibling,
                Node& next_sibling,
                Vector<String> metadata);
  ChildNodePart(const ChildNodePart&) = delete;
  ~ChildNodePart() override = default;

  void Trace(Visitor* visitor) const override;
  bool IsValid() const override;
  Node* NodeToSortBy() const override;
  Part* ClonePart(NodeCloningData&, Node&) const override;
  PartRoot* GetAsPartRoot() const override {
    return const_cast<ChildNodePart*>(this);
  }

  Document& GetDocument() const override;
  bool IsDocumentPartRoot() const override { return false; }
  Node* FirstIncludedChildNode() const override {
    return previous_sibling_.Get();
  }
  Node* LastIncludedChildNode() const override { return next_sibling_.Get(); }

  // ChildNodePart API
  void disconnect() override;
  PartRootUnion* clone(ExceptionState&);
  ContainerNode* rootContainer() const override;
  ContainerNode* parentNode() const { return previous_sibling_->parentNode(); }
  Node* previousSibling() const { return previous_sibling_.Get(); }
  Node* nextSibling() const { return next_sibling_.Get(); }
  void setNextSibling(Node& next_sibling);
  HeapVector<Member<Node>> children() const;
  void replaceChildren(const HeapVector<Member<V8UnionNodeOrString>>& nodes,
                       ExceptionState& exception_state);

 protected:
  const PartRoot* GetParentPartRoot() const override { return root(); }

 private:
  // Checks if this ChildNodePart is valid. This should only be called if the
  // cached validity is dirty.
  bool CheckValidity();

  Member<Node> previous_sibling_;
  Member<Node> next_sibling_;
};

// A ChildNodePart is valid if:
//  1. The base |Part| is valid (it has a |root|).
//  2. previous_sibling_ and next_sibling_ are non-null.
//  3. previous_sibling_ and next_sibling_ have the same (non-null) parent.
//  4. previous_sibling_ comes strictly before next_sibling_ in the tree.
inline bool ChildNodePart::IsValid() const {
  if (!Part::IsValid()) {
    return false;
  }
  if (!previous_sibling_ || !next_sibling_) {
    return false;
  }
  DCHECK(!RuntimeEnabledFeatures::DOMPartsAPIMinimalEnabled() ||
         (previous_sibling_->HasNodePart() && next_sibling_->HasNodePart()));
  ContainerNode* parent = parentNode();
  if (!parent) {
    return false;
  }
  if (next_sibling_->parentNode() != parent) {
    return false;
  }
  if (previous_sibling_ == next_sibling_) {
    return false;
  }
  Node* left = previous_sibling_.Get();
  do {
    left = left->nextSibling();
    if (left == next_sibling_) {
      return true;
    }
  } while (left);
  return false;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_CHILD_NODE_PART_H_
