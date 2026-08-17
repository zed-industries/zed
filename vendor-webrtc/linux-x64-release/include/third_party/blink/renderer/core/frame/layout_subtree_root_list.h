// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_LAYOUT_SUBTREE_ROOT_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_LAYOUT_SUBTREE_ROOT_LIST_H_

#include "third_party/blink/renderer/core/layout/depth_ordered_layout_object_list.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// This class keeps track of layout objects that have identified to be
// independent layout roots meaning they won't affect other parts of the tree
// by their layout. This is an optimization to avoid doing extra work and tree
// walking during layout. See objectIsRelayoutBoundary for the criteria for
// being a root.
// These roots are sorted into a vector ordered by their depth in the tree,
// and returned one by one deepest first for layout. This is necessary in the
// case of nested subtree roots where a positioned object is added to the
// contained root but its containing block is above that root.
// It ensures we add positioned objects to their containing block's positioned
// descendant lists before laying out those objects if they're contained in
// a higher root.
// TODO(leviw): This should really be something akin to a LayoutController
// that FrameView delegates layout work to.
class LayoutSubtreeRootList : public DepthOrderedLayoutObjectList {
  DISALLOW_NEW();

 public:
  LayoutSubtreeRootList() = default;

  void ClearAndMarkContainingBlocksForLayout();

  void CountObjectsNeedingLayout(unsigned& needs_layout_objects,
                                 unsigned& total_objects);

  static void CountObjectsNeedingLayoutInRoot(const LayoutObject* root,
                                              unsigned& needs_layout_objects,
                                              unsigned& total_objects);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_LAYOUT_SUBTREE_ROOT_LIST_H_
