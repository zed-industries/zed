// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be found
// in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GRID_GRID_SUBTREE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GRID_GRID_SUBTREE_H_

#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace blink {

// A grid tree is represented by a vector satisfying the following conditions:
//   - The nodes in the tree are indexed using preorder traversal.
//   - Each position in the vector represents a grid node in the tree and it
//   stores the size of the subtree rooted at that node.
//   - Each subtree is guaranteed to be contained in a single contiguous range;
//   i.e., for a given index `k`, the range [k, k + subtree_size[k]) in the
//   vector represents the data of the subtree rooted at grid node `k`.
//   - We can iterate over a node's children by skipping over their subtrees;
//   i.e., the first child of a node `k` is always at position `k+1`, the next
//   sibling comes `subtree_size[k+1]` positions later, and so on.
//
//         (0)
//        /   \
//     (1)     (7)
//     / \     / \
//   (2) (5) (8) (9)
//   / \   \
// (3) (4) (6)       (0)
//                      (1)               (7)
//                         (2)      (5)      (8)(9)
//                            (3)(4)   (6)
//   subtree_size = [10, 6, 3, 1, 1, 2, 1, 3, 1, 1]
//
// A subtree is represented by a pointer to the entire grid tree and the index
// of the subtree's root. In order to iterate over the siblings of a subtree we
// need to store the index of the next sibling of its parent, aka the parent's
// end index, so that we don't traverse outside of the parent's subtree.
//
// In the example above, we can compute the next sibling of a subtree rooted at
// index 2 by adding its subtree size (2 + 3 = 5). However, when we compute the
// next sibling for the subtree at index 5, by adding its subtree size (5 + 2)
// it's equal to its parent's next sibling (aka parent's end index), so we can
// determine that such subtree doesn't have a next sibling.
template <typename GridTree>
class GridSubtree {
 public:
  explicit operator bool() const { return subtree_root_ != kNotFound; }

 protected:
  GridSubtree() = default;

  void SetSubtreeRoot(const GridTree& grid_tree, wtf_size_t subtree_root) {
    subtree_root_ = subtree_root;
    parent_end_index_ = NextSiblingIndex(grid_tree);
  }

  GridSubtree FirstChild(const GridTree& grid_tree) const {
    return GridSubtree(
        /*parent_end_index=*/NextSiblingIndex(grid_tree),
        /*subtree_root=*/subtree_root_ + 1);
  }

  GridSubtree NextSibling(const GridTree& grid_tree) const {
    return GridSubtree(/*parent_end_index=*/parent_end_index_,
                       /*subtree_root=*/NextSiblingIndex(grid_tree));
  }

  // Index of this subtree's root node.
  wtf_size_t subtree_root_{kNotFound};

 private:
  GridSubtree(wtf_size_t parent_end_index, wtf_size_t subtree_root) {
    DCHECK_LE(subtree_root, parent_end_index);

    // If the subtree root is beyond the parent's end index, we will keep this
    // instance as a null subtree to indicate the end iterator for siblings.
    if (subtree_root < parent_end_index) {
      parent_end_index_ = parent_end_index;
      subtree_root_ = subtree_root;
    }
  }

  wtf_size_t NextSiblingIndex(const GridTree& grid_tree) const {
    const auto subtree_size = grid_tree.SubtreeSize(subtree_root_);

    DCHECK_GT(subtree_size, 0u);
    return subtree_root_ + subtree_size;
  }

  // Index of the next sibling of this subtree's parent; used to avoid iterating
  // outside of the parent's subtree when computing this subtree's next sibling.
  wtf_size_t parent_end_index_{kNotFound};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GRID_GRID_SUBTREE_H_
