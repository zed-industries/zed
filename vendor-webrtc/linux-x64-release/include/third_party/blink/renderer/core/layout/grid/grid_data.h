// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GRID_GRID_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GRID_GRID_DATA_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/grid/grid_line_resolver.h"
#include "third_party/blink/renderer/core/layout/grid/grid_subtree.h"
#include "third_party/blink/renderer/core/layout/grid/grid_track_collection.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

struct CORE_EXPORT GridPlacementData {
  DISALLOW_NEW();

 public:
  GridPlacementData(GridPlacementData&&) = default;
  GridPlacementData& operator=(GridPlacementData&&) = default;

  explicit GridPlacementData(const GridLineResolver& line_resolver)
      : line_resolver(line_resolver) {}

  bool operator==(const GridPlacementData& other) const {
    return grid_item_positions == other.grid_item_positions &&
           column_start_offset == other.column_start_offset &&
           row_start_offset == other.row_start_offset &&
           line_resolver == other.line_resolver;
  }

  bool operator!=(const GridPlacementData& other) const {
    return !(*this == other);
  }

  wtf_size_t AutoRepeatTrackCount(
      GridTrackSizingDirection track_direction) const {
    return line_resolver.AutoRepeatTrackCount(track_direction);
  }

  wtf_size_t ExplicitGridTrackCount(
      GridTrackSizingDirection track_direction) const {
    return line_resolver.ExplicitGridTrackCount(track_direction);
  }

  bool HasStandaloneAxis(GridTrackSizingDirection track_direction) const {
    return line_resolver.HasStandaloneAxis(track_direction);
  }

  wtf_size_t StartOffset(GridTrackSizingDirection track_direction) const {
    return (track_direction == kForColumns) ? column_start_offset
                                            : row_start_offset;
  }

  wtf_size_t SubgridSpanSize(GridTrackSizingDirection track_direction) const {
    return line_resolver.SubgridSpanSize(track_direction);
  }

  GridLineResolver line_resolver;
  Vector<GridArea> grid_item_positions;
  wtf_size_t column_start_offset{0};
  wtf_size_t row_start_offset{0};
};

namespace {

bool AreEqual(const std::unique_ptr<GridLayoutTrackCollection>& lhs,
              const std::unique_ptr<GridLayoutTrackCollection>& rhs) {
  return (lhs && rhs) ? *lhs == *rhs : !lhs && !rhs;
}

}  // namespace

// This struct contains the column and row data necessary to layout grid items.
// For grid sizing, it will store |GridSizingTrackCollection| pointers, which
// are able to modify the geometry of its sets. However, after sizing is done,
// it should only copy |GridLayoutTrackCollection| immutable data.
class CORE_EXPORT GridLayoutData {
  USING_FAST_MALLOC(GridLayoutData);

 public:
  GridLayoutData() = default;
  GridLayoutData(GridLayoutData&&) = default;
  GridLayoutData& operator=(GridLayoutData&&) = default;

  GridLayoutData(const GridLayoutData& other) {
    if (other.columns_) {
      columns_ = std::make_unique<GridLayoutTrackCollection>(other.Columns());
    }
    if (other.rows_) {
      rows_ = std::make_unique<GridLayoutTrackCollection>(other.Rows());
    }
  }

  GridLayoutData& operator=(const GridLayoutData& other) {
    return *this = GridLayoutData(other);
  }

  bool operator==(const GridLayoutData& other) const {
    return AreEqual(columns_, other.columns_) && AreEqual(rows_, other.rows_);
  }

  bool operator!=(const GridLayoutData& other) const {
    return !(*this == other);
  }

  bool HasSubgriddedAxis(GridTrackSizingDirection track_direction) const {
    return (track_direction == kForColumns)
               ? !(columns_ && columns_->IsForSizing())
               : !(rows_ && rows_->IsForSizing());
  }

  bool IsSubgridWithStandaloneAxis(
      GridTrackSizingDirection track_direction) const {
    return columns_ && rows_ &&
           ((track_direction == kForColumns)
                ? columns_->IsForSizing() && !rows_->IsForSizing()
                : rows_->IsForSizing() && !columns_->IsForSizing());
  }

  GridLayoutTrackCollection& Columns() const {
    DCHECK(columns_);
    DCHECK_EQ(columns_->Direction(), kForColumns);
    return *columns_;
  }

  GridLayoutTrackCollection& Rows() const {
    DCHECK(rows_);
    DCHECK_EQ(rows_->Direction(), kForRows);
    return *rows_;
  }

  GridSizingTrackCollection& SizingCollection(
      GridTrackSizingDirection track_direction) const {
    DCHECK(!HasSubgriddedAxis(track_direction));

    return To<GridSizingTrackCollection>(
        (track_direction == kForColumns) ? Columns() : Rows());
  }

  // This method is intended for subgrids with both a standalone and a
  // subgridded axis. Returns the only subgridded track collection.
  const GridLayoutTrackCollection& OnlySubgriddedCollection() const {
    DCHECK(columns_);
    DCHECK(rows_);
    DCHECK_NE(columns_->IsForSizing(), rows_->IsForSizing());
    return columns_->IsForSizing() ? *rows_ : *columns_;
  }

  void SetTrackCollection(
      std::unique_ptr<GridLayoutTrackCollection> track_collection) {
    DCHECK(track_collection);

    if (track_collection->Direction() == kForColumns) {
      columns_ = std::move(track_collection);
    } else {
      rows_ = std::move(track_collection);
    }
  }

 private:
  std::unique_ptr<GridLayoutTrackCollection> columns_;
  std::unique_ptr<GridLayoutTrackCollection> rows_;
};

// Subgrid layout relies on the root grid to perform the track sizing algorithm
// for every level of nested subgrids. This class contains the finalized layout
// data of every node in a grid tree (see `grid_subtree.h`), which will be
// passed down to the constraint space of a subgrid to perform layout.
//
// Note that this class allows subtrees to be compared for equality; this is
// important because when we store this tree within a constraint space we want
// to be able to invalidate the cached layout result of a subgrid based on
// whether the provided subtree's track were sized exactly the same.
class GridLayoutTree : public RefCounted<GridLayoutTree> {
 public:
  struct GridTreeNode {
    GridTreeNode(const GridLayoutData& layout_data, wtf_size_t subtree_size)
        : has_unresolved_geometry(layout_data.Columns().HasIndefiniteSet() ||
                                  layout_data.Rows().HasIndefiniteSet()),
          layout_data(layout_data),
          subtree_size(subtree_size) {}

    bool has_unresolved_geometry;
    GridLayoutData layout_data;
    wtf_size_t subtree_size;
  };

  explicit GridLayoutTree(Vector<GridTreeNode, 16>&& tree_data)
      : tree_data_(std::move(tree_data)) {}

  bool AreSubtreesEqual(wtf_size_t subtree_root,
                        const GridLayoutTree& other,
                        wtf_size_t other_subtree_root) const {
    const wtf_size_t subtree_size = SubtreeSize(subtree_root);

    if (subtree_size != other.SubtreeSize(other_subtree_root)) {
      return false;
    }

    for (wtf_size_t i = 0; i < subtree_size; ++i) {
      if (other.LayoutData(other_subtree_root + i) !=
          LayoutData(subtree_root + i)) {
        return false;
      }
    }
    return true;
  }

  bool HasUnresolvedGeometry(wtf_size_t index) const {
    DCHECK_LT(index, tree_data_.size());
    return tree_data_[index].has_unresolved_geometry;
  }

  const GridLayoutData& LayoutData(wtf_size_t index) const {
    DCHECK_LT(index, tree_data_.size());
    return tree_data_[index].layout_data;
  }

  wtf_size_t Size() const { return tree_data_.size(); }

  wtf_size_t SubtreeSize(wtf_size_t index) const {
    DCHECK_LT(index, tree_data_.size());
    return tree_data_[index].subtree_size;
  }

 private:
  Vector<GridTreeNode, 16> tree_data_;
};

using GridLayoutTreePtr = scoped_refptr<const GridLayoutTree>;

// This class represents a subtree in a `GridLayoutTree` and mostly serves two
// purposes: provide seamless iteration over the tree structure and compare
// input subtrees to invalidate a subgrid's cached layout result.
class GridLayoutSubtree : public GridSubtree<GridLayoutTree> {
  DISALLOW_NEW();

 public:
  GridLayoutSubtree() = default;

  explicit GridLayoutSubtree(GridLayoutTreePtr layout_tree,
                             wtf_size_t subtree_root = 0)
      : layout_tree_(std::move(layout_tree)) {
    SetSubtreeRoot(LayoutTree(), subtree_root);
  }

  GridLayoutSubtree FirstChild() const {
    return GridLayoutSubtree(layout_tree_,
                             GridSubtree::FirstChild(LayoutTree()));
  }

  GridLayoutSubtree NextSibling() const {
    return GridLayoutSubtree(layout_tree_,
                             GridSubtree::NextSibling(LayoutTree()));
  }

  // This method is meant to be used for layout invalidation, so we only care
  // about comparing the layout data of both subtrees.
  bool operator==(const GridLayoutSubtree& other) const {
    return (layout_tree_ && other.layout_tree_)
               ? layout_tree_->AreSubtreesEqual(
                     subtree_root_, *other.layout_tree_, other.subtree_root_)
               : !layout_tree_ && !other.layout_tree_;
  }

  bool HasUnresolvedGeometry() const {
    return LayoutTree().HasUnresolvedGeometry(subtree_root_);
  }

  const GridLayoutData& LayoutData() const {
    return LayoutTree().LayoutData(subtree_root_);
  }

 private:
  GridLayoutSubtree(const GridLayoutTreePtr& layout_tree, GridSubtree subtree)
      : GridSubtree(std::move(subtree)), layout_tree_(layout_tree) {}

  const GridLayoutTree& LayoutTree() const {
    DCHECK(layout_tree_);
    return *layout_tree_;
  }

  // Pointer to the layout tree shared by multiple subtree instances.
  GridLayoutTreePtr layout_tree_{nullptr};
};

}  // namespace blink

WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(
    blink::GridLayoutTree::GridTreeNode)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GRID_GRID_DATA_H_
