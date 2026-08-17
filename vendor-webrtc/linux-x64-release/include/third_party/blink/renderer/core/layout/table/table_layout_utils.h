// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_TABLE_LAYOUT_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_TABLE_LAYOUT_UTILS_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/table/table_layout_algorithm_types.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"

namespace blink {

class BlockNode;
class BoxFragmentBuilder;
class ConstraintSpaceBuilder;
class LogicalBoxFragment;
class TableBorders;
class TableNode;
enum class BlockContentAlignment;
enum class LayoutResultCacheSlot;
struct TableColumnLocation;

// Table size distribution algorithms.

// Computes a cell's block-size, and if its initial block-size should be
// considered indefinite.
struct CellBlockSizeData {
  LayoutUnit block_size;
  bool is_initial_block_size_indefinite;
};
CellBlockSizeData ComputeCellBlockSize(
    const TableTypes::CellBlockConstraint& cell_block_constraint,
    const TableTypes::Rows& rows,
    wtf_size_t row_index,
    const LogicalSize& border_spacing,
    bool is_table_block_size_specified);

// Sets up a constraint space builder for a table-cell.
//
// In order to make the cache as effective as possible, we try and keep
// creating the constraint-space for table-cells as consistent as possible.
void SetupTableCellConstraintSpaceBuilder(
    const WritingDirectionMode table_writing_direction,
    const BlockNode cell,
    const BoxStrut& cell_borders,
    const Vector<TableColumnLocation>& column_locations,
    LayoutUnit cell_block_size,
    LayoutUnit percentage_inline_size,
    std::optional<LayoutUnit> alignment_baseline,
    wtf_size_t start_column,
    bool is_initial_block_size_indefinite,
    bool is_restricted_block_size_table,
    bool has_collapsed_borders,
    LayoutResultCacheSlot,
    ConstraintSpaceBuilder*);

wtf_size_t ComputeMaximumNonMergeableColumnCount(
    const HeapVector<BlockNode>& columns,
    bool is_fixed_layout);

scoped_refptr<TableTypes::Columns> ComputeColumnConstraints(
    const BlockNode& table,
    const TableGroupedChildren&,
    const TableBorders& table_borders,
    const BoxStrut& border_padding);

void ComputeSectionMinimumRowBlockSizes(
    const BlockNode& section,
    const LayoutUnit cell_percentage_resolution_inline_size,
    const bool is_table_block_size_specified,
    const Vector<TableColumnLocation>& column_locations,
    const TableBorders& table_borders,
    const LayoutUnit block_border_spacing,
    wtf_size_t section_index,
    bool treat_section_as_tbody,
    TableTypes::Sections* sections,
    TableTypes::Rows* rows,
    TableTypes::CellBlockConstraints* cell_block_constraints);

// Performs any final adjustments for table-cells at the end of layout.
void FinalizeTableCellLayout(LayoutUnit unconstrained_intrinsic_block_size,
                             BoxFragmentBuilder*);

// ColspanCellTabulator keeps track of columns occupied by colspanned cells
// when traversing rows in a section. It is used to compute cell's actual
// column.
// Usage:
//   ColspanCellTabulator colspan_cell_tabulator;
//   for (Row r : section.rows) {
//      colspan_cell_tabulator.StartRow();
//      for (Cell c : row.cells) {
//        colspan_cell_tabulator.FindNextFreeColumn();
//        // colspan_cell_tabulator.CurrentColumn() has a valid value here.
//        colspan_cell_tabulator.ProcessCell();
//      }
//      colspan_cell_tabulator.EndRow();
//   }
class ColspanCellTabulator {
 public:
  unsigned CurrentColumn() { return current_column_; }
  void StartRow();
  void FindNextFreeColumn();
  void ProcessCell(const BlockNode& cell);
  void EndRow();

  struct Cell {
    Cell(unsigned column_start, unsigned span, unsigned remaining_rows)
        : column_start(column_start),
          span(span),
          remaining_rows(remaining_rows) {}
    unsigned column_start;
    unsigned span;
    unsigned remaining_rows;
  };

 private:
  unsigned current_column_ = 0;
  Vector<Cell> colspanned_cells_;
};

// RowBaselineTabulator computes baseline information for row.
// Standard: https://www.w3.org/TR/css-tables-3/#row-layout
// Baseline is either max-baseline of baseline-aligned cells,
// or bottom content edge of non-baseline-aligned cells.
class RowBaselineTabulator {
 public:
  void ProcessCell(const LogicalBoxFragment& fragment,
                   BlockContentAlignment align,
                   bool is_rowspanned,
                   bool descendant_depends_on_percentage_block_size);

  LayoutUnit ComputeRowBlockSize(const LayoutUnit max_cell_block_size);

  LayoutUnit ComputeBaseline(const LayoutUnit row_block_size);

  bool BaselineDependsOnPercentageBlockDescendant();

 private:
  // Cell baseline is computed from baseline-aligned cells.
  std::optional<LayoutUnit> max_cell_ascent_;
  std::optional<LayoutUnit> max_cell_descent_;
  bool max_cell_baseline_depends_on_percentage_block_descendant_ = false;

  // Non-baseline aligned cells are used to compute baseline if baseline
  // cells are not available.
  std::optional<LayoutUnit> fallback_cell_descent_;
  bool fallback_cell_depends_on_percentage_block_descendant_ = false;
};

// Compute maximum number of table columns that can deduced from single cell
// and its colspan.
constexpr wtf_size_t ComputeMaxColumn(wtf_size_t current_column,
                                      wtf_size_t colspan,
                                      bool is_fixed_table_layout) {
  // In fixed mode, every column is preserved.
  if (is_fixed_table_layout) {
    return current_column + colspan;
  }
  return current_column + 1;
}

// |undistributable_space| is size of space not occupied by cells
// (borders, border spacing).
CORE_EXPORT MinMaxSizes
ComputeGridInlineMinMax(const TableNode& node,
                        const TableTypes::Columns& column_constraints,
                        LayoutUnit undistributable_space,
                        bool is_fixed_layout,
                        bool is_layout_pass);

CORE_EXPORT void DistributeColspanCellsToColumns(
    const TableTypes::ColspanCells& colspan_cells,
    LayoutUnit inline_border_spacing,
    bool is_fixed_layout,
    TableTypes::Columns* column_constraints);

CORE_EXPORT Vector<LayoutUnit> SynchronizeAssignableTableInlineSizeAndColumns(
    LayoutUnit assignable_table_inline_size,
    bool is_fixed_layout,
    const TableTypes::Columns& column_constraints);

CORE_EXPORT void DistributeRowspanCellToRows(
    const TableTypes::RowspanCell& rowspan_cell,
    LayoutUnit border_block_spacing,
    TableTypes::Rows* rows);

CORE_EXPORT void DistributeSectionFixedBlockSizeToRows(
    const wtf_size_t start_row,
    const wtf_size_t end_row,
    LayoutUnit section_fixed_block_size,
    LayoutUnit border_block_spacing,
    LayoutUnit percentage_resolution_block_size,
    TableTypes::Rows* rows);

CORE_EXPORT void DistributeTableBlockSizeToSections(
    LayoutUnit border_block_spacing,
    LayoutUnit table_block_size,
    TableTypes::Sections* sections,
    TableTypes::Rows* rows);

}  // namespace blink

WTF_ALLOW_MOVE_INIT_AND_COMPARE_WITH_MEM_FUNCTIONS(
    blink::ColspanCellTabulator::Cell)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_TABLE_LAYOUT_UTILS_H_
