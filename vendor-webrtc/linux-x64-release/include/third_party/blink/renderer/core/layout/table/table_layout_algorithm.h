// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_TABLE_LAYOUT_ALGORITHM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_TABLE_LAYOUT_ALGORITHM_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/box_fragment_builder.h"
#include "third_party/blink/renderer/core/layout/layout_algorithm.h"
#include "third_party/blink/renderer/core/layout/table/table_layout_algorithm_types.h"
#include "third_party/blink/renderer/core/layout/table/table_node.h"

namespace blink {

class BlockBreakToken;
class TableBorders;

class CORE_EXPORT TableLayoutAlgorithm
    : public LayoutAlgorithm<TableNode, BoxFragmentBuilder, BlockBreakToken> {
 public:
  explicit TableLayoutAlgorithm(const LayoutAlgorithmParams& params)
      : LayoutAlgorithm(params) {}

  void SetupRelayoutData(const TableLayoutAlgorithm& previous, RelayoutType);

  const LayoutResult* Layout();

  MinMaxSizesResult ComputeMinMaxSizes(const MinMaxSizesFloatInput&);

  static LayoutUnit ComputeTableInlineSize(const TableNode& node,
                                           const ConstraintSpace& space,
                                           const BoxStrut& border_padding);

  // Useful when trying to compute table's block sizes.
  // Table's css block size specifies size of the grid, not size
  // of the wrapper. Wrapper's block size = grid size + caption block size.
  LayoutUnit ComputeCaptionBlockSize();

  // In order to correctly determine the available block-size given to the
  // table-grid, we need to layout all the captions ahead of time. This struct
  // stores the necessary information to add them to the fragment later.
  struct CaptionResult {
    DISALLOW_NEW();

   public:
    void Trace(Visitor* visitor) const {
      visitor->Trace(node);
      visitor->Trace(layout_result);
    }

    BlockNode node;
    Member<const LayoutResult> layout_result;
    const BoxStrut margins;
  };

 private:
  void ComputeRows(const LayoutUnit table_grid_inline_size,
                   const TableGroupedChildren& grouped_children,
                   const Vector<TableColumnLocation>& column_locations,
                   const TableBorders& table_borders,
                   const LogicalSize& border_spacing,
                   const BoxStrut& table_border_padding,
                   const LayoutUnit captions_block_size,
                   TableTypes::Rows* rows,
                   TableTypes::CellBlockConstraints* cell_block_constraints,
                   TableTypes::Sections* sections,
                   LayoutUnit* minimal_table_grid_block_size);

  void ComputeTableSpecificFragmentData(
      const TableGroupedChildren& grouped_children,
      const Vector<TableColumnLocation>& column_locations,
      const TableTypes::Rows& rows,
      const TableBorders& table_borders,
      const LogicalRect& table_grid_rect,
      LayoutUnit table_grid_block_size);

  const LayoutResult* GenerateFragment(
      LayoutUnit table_inline_size,
      LayoutUnit minimal_table_grid_block_size,
      const TableGroupedChildren& grouped_children,
      const Vector<TableColumnLocation>& column_locations,
      const TableTypes::Rows& rows,
      const TableTypes::CellBlockConstraints& cell_block_constraints,
      const TableTypes::Sections& sections,
      const HeapVector<CaptionResult>& captions,
      const TableBorders& table_borders,
      const LogicalSize& border_spacing);

  LayoutUnit total_table_min_block_size_;

  // Set to true when we're re-laying out without repeating table headers and
  // footers.
  bool is_known_to_be_last_table_box_ = false;
};

}  // namespace blink

WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(
    blink::TableLayoutAlgorithm::CaptionResult)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_TABLE_LAYOUT_ALGORITHM_H_
