// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_EDITING_STRATEGY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_EDITING_STRATEGY_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/flat_tree_traversal.h"
#include "third_party/blink/renderer/core/dom/node_traversal.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// Editing algorithm defined on node traversal.
template <typename Traversal>
class EditingAlgorithm : public Traversal {
  STATIC_ONLY(EditingAlgorithm);

 public:
  static int CaretMaxOffset(const Node&);
  // This method is used to create positions in the DOM. It returns the
  // maximum valid offset in a node. It returns 1 for some elements even
  // though they do not have children, which creates technically invalid DOM
  // Positions. Be sure to call |parentAnchoredEquivalent()| on a Position
  // before using it to create a DOM Range, or an exception will be thrown.
  static int LastOffsetForEditing(const Node*);
  static Node* RootUserSelectAllForNode(Node*);
};

extern template class CORE_EXTERN_TEMPLATE_EXPORT
    EditingAlgorithm<NodeTraversal>;
extern template class CORE_EXTERN_TEMPLATE_EXPORT
    EditingAlgorithm<FlatTreeTraversal>;

// DOM tree version of editing algorithm
using EditingStrategy = EditingAlgorithm<NodeTraversal>;
// Flat tree version of editing algorithm
using EditingInFlatTreeStrategy = EditingAlgorithm<FlatTreeTraversal>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_EDITING_STRATEGY_H_
