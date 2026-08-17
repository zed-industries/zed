// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_PART_ROOT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_PART_ROOT_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_union_childnodepart_documentpartroot.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/part.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class ContainerNode;
class Document;

using PartRootUnion = V8UnionChildNodePartOrDocumentPartRoot;

// These should be the data (text content) of the start and end comment nodes
// that represent the endpoints of a ChildNodePart range.
#define kChildNodePartStartCommentData "#"
#define kChildNodePartEndCommentData "/"

// Implementation of the PartRoot class, which is part of the DOM Parts API.
// PartRoot is the base of the class hierarchy.
class CORE_EXPORT PartRoot : public GarbageCollectedMixin {
 public:
  PartRoot(const PartRoot&) = delete;
  void operator=(const PartRoot&) = delete;
  ~PartRoot() = default;

  void Trace(Visitor* visitor) const override;

  // Adds a new part to this PartRoot's collection of maintained parts.
  void AddPart(Part& new_part);
  void RemovePart(Part& part);
  static void CloneParts(const Node& source_node,
                         Node& destination_node,
                         NodeCloningData& data);
  void MarkPartsDirty() {
    DCHECK(!RuntimeEnabledFeatures::DOMPartsAPIMinimalEnabled());
    cached_parts_list_dirty_ = true;
  }
  void SwapPartsList(PartRoot& other);

  virtual Document& GetDocument() const = 0;
  virtual bool IsDocumentPartRoot() const = 0;
  virtual Node* FirstIncludedChildNode() const = 0;
  virtual Node* LastIncludedChildNode() const = 0;

  // Utilities to convert to/from the IDL union.
  static PartRootUnion* GetUnionFromPartRoot(PartRoot* root);
  static PartRoot* GetPartRootFromUnion(PartRootUnion* root_union);
  static const PartRoot* GetPartRootFromUnion(const PartRootUnion* root_union) {
    return GetPartRootFromUnion(const_cast<PartRootUnion*>(root_union));
  }

  // PartRoot API
  using PartNodeList = GCedHeapVector<Member<Node>, 20>;
  using PartList = GCedHeapVector<Member<Part>, 20>;
  const PartList& getParts();
  const PartNodeList& getNodePartNodes();
  const PartNodeList& getChildNodePartNodes();
  virtual ContainerNode* rootContainer() const = 0;

 protected:
  PartRoot();
  virtual const PartRoot* GetParentPartRoot() const = 0;

 private:
  void RebuildPartsList();
  Member<PartList> cached_ordered_parts_;
  bool cached_parts_list_dirty_{false};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_PART_ROOT_H_
