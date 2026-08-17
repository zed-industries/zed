// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_TABLE_LAYOUT_ALGORITHM_TYPES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_TABLE_LAYOUT_ALGORITHM_TYPES_H_

#include <optional>

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/block_node.h"
#include "third_party/blink/renderer/core/layout/geometry/box_strut.h"
#include "third_party/blink/renderer/core/layout/min_max_sizes.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/platform/geometry/length.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class ComputedStyle;

// Define constraint classes for TableLayoutAlgorithm.
class CORE_EXPORT TableTypes {
 public:
  static constexpr LayoutUnit kTableMaxInlineSize =
      LayoutUnit(static_cast<uint64_t>(1000000));

  // Inline constraint for a single cell.
  // Takes into account the cell style, and min/max content-sizes.
  struct CellInlineConstraint {
    DISALLOW_NEW();
    LayoutUnit min_inline_size;
    LayoutUnit max_inline_size;
    std::optional<float> percent;       // 100% is stored as 100.0f
    LayoutUnit percent_border_padding;  // Border/padding used for percentage
                                        // size resolution.
    bool is_constrained;  // True if this cell has a specified inline-size.

    void Encompass(const CellInlineConstraint&);
  };

  // Inline constraints for a cell that span multiple columns.
  struct ColspanCell {
    DISALLOW_NEW();
    CellInlineConstraint cell_inline_constraint;
    wtf_size_t start_column;
    wtf_size_t span;
    ColspanCell(const CellInlineConstraint& cell_inline_constraint,
                unsigned start_column,
                unsigned span)
        : cell_inline_constraint(cell_inline_constraint),
          start_column(start_column),
          span(span) {}
  };

  // Constraint for a column.
  struct Column {
    DISALLOW_NEW();
    Column(const std::optional<LayoutUnit>& min_inline_size,
           const std::optional<LayoutUnit>& max_inline_size,
           const std::optional<float>& percent,
           LayoutUnit percent_border_padding,
           bool is_constrained,
           bool is_collapsed,
           bool is_table_fixed,
           bool is_mergeable)
        : min_inline_size(min_inline_size),
          max_inline_size(max_inline_size),
          percent(percent),
          percent_border_padding(percent_border_padding),
          is_constrained(is_constrained),
          is_collapsed(is_collapsed),
          is_table_fixed(is_table_fixed),
          is_mergeable(is_mergeable) {}
    Column() = default;

    bool operator==(const Column& other) const {
      return min_inline_size == other.min_inline_size &&
             max_inline_size == other.max_inline_size &&
             percent == other.percent &&
             percent_border_padding == other.percent_border_padding &&
             is_constrained == other.is_constrained &&
             is_collapsed == other.is_collapsed &&
             is_table_fixed == other.is_table_fixed &&
             is_mergeable == other.is_mergeable;
    }
    bool operator!=(const Column& other) const { return !(*this == other); }

    // These members are initialized from <col> and <colgroup>, then they
    // accumulate data from |CellInlineConstraint|s.
    std::optional<LayoutUnit> min_inline_size;
    std::optional<LayoutUnit> max_inline_size;
    std::optional<float> percent;       // 100% is stored as 100.0f
    LayoutUnit percent_border_padding;  // Border/padding used for percentage
                                        // size resolution.
    // True if any cell for this column is constrained.
    bool is_constrained = false;
    bool is_collapsed = false;
    bool is_table_fixed = false;
    bool is_mergeable = false;

    void Encompass(const std::optional<TableTypes::CellInlineConstraint>&);
    LayoutUnit ResolvePercentInlineSize(
        LayoutUnit percentage_resolution_inline_size) const {
      return std::max(
          min_inline_size.value_or(LayoutUnit()),
          LayoutUnit(*percent * percentage_resolution_inline_size / 100) +
              percent_border_padding);
    }
    bool IsFixed() const {
      return is_constrained && !percent && max_inline_size;
    }
  };

  // Block constraint for a single cell.
  struct CellBlockConstraint {
    DISALLOW_NEW();
    LayoutUnit min_block_size;
    BoxStrut borders;
    wtf_size_t column_index;
    wtf_size_t effective_rowspan;
    bool is_constrained;  // True if this cell has a specified block-size.
    bool has_descendant_that_depends_on_percentage_block_size;
    CellBlockConstraint(
        LayoutUnit min_block_size,
        BoxStrut borders,
        wtf_size_t column_index,
        wtf_size_t effective_rowspan,
        bool is_constrained,
        bool has_descendant_that_depends_on_percentage_block_size)
        : min_block_size(min_block_size),
          borders(borders),
          column_index(column_index),
          effective_rowspan(effective_rowspan),
          is_constrained(is_constrained),
          has_descendant_that_depends_on_percentage_block_size(
              has_descendant_that_depends_on_percentage_block_size) {}
  };

  // RowspanCells span multiple rows.
  struct RowspanCell {
    DISALLOW_NEW();
    wtf_size_t start_row;
    wtf_size_t effective_rowspan;
    LayoutUnit min_block_size;
    RowspanCell(wtf_size_t start_row,
                wtf_size_t effective_rowspan,
                LayoutUnit min_block_size)
        : start_row(start_row),
          effective_rowspan(effective_rowspan),
          min_block_size(min_block_size) {}

    // Original Legacy sorting criteria from
    // CompareRowspanCellsInHeightDistributionOrder
    bool operator<(const TableTypes::RowspanCell& rhs) const {
      // Returns true if a |RowspanCell| is completely contained within another
      // |RowspanCell|.
      auto IsEnclosed = [](const TableTypes::RowspanCell& c1,
                           const TableTypes::RowspanCell& c2) {
        return (c1.start_row >= c2.start_row) &&
               (c1.start_row + c1.effective_rowspan) <=
                   (c2.start_row + c2.effective_rowspan);
      };

      // If cells span the same rows, the bigger cell is distributed first.
      if (start_row == rhs.start_row &&
          effective_rowspan == rhs.effective_rowspan) {
        return min_block_size > rhs.min_block_size;
      }

      // If one cell is fully enclosed by another, the inner cell wins.
      if (IsEnclosed(*this, rhs))
        return true;
      if (IsEnclosed(rhs, *this))
        return false;
      // Lowest row wins.
      return start_row < rhs.start_row;
    }
  };

  struct Row {
    DISALLOW_NEW();
    LayoutUnit block_size;
    wtf_size_t start_cell_index;
    wtf_size_t cell_count;
    std::optional<LayoutUnit> baseline;
    std::optional<float> percent;  // 100% is stored as 100.0f
    // |is_constrained| is true if row has specified block-size, or contains
    // constrained cells.
    bool is_constrained;
    bool has_rowspan_start;  // True if row originates a TD with rowspan > 1
    bool is_collapsed;
  };

  struct Section {
    wtf_size_t start_row;
    wtf_size_t row_count;
    LayoutUnit block_size;
    std::optional<float> percent;
    bool is_constrained;
    bool is_tbody;
    bool needs_redistribution;
  };

  static Column CreateColumn(const ComputedStyle&,
                             std::optional<LayoutUnit> default_inline_size,
                             bool is_table_fixed);

  static CellInlineConstraint CreateCellInlineConstraint(
      const BlockNode&,
      WritingDirectionMode table_writing_direction,
      bool is_fixed_layout,
      const BoxStrut& cell_border,
      const BoxStrut& cell_padding);

  static Section CreateSection(const LayoutInputNode&,
                               wtf_size_t start_row,
                               wtf_size_t row_count,
                               LayoutUnit block_size,
                               bool treat_as_tbody);

  // Columns are cached by LayoutTable, and need to be RefCounted.
  typedef base::RefCountedData<WTF::Vector<Column>> Columns;
  // Inline constraints are optional because we need to distinguish between an
  // empty cell, and a non-existent cell.
  using CellInlineConstraints = Vector<std::optional<CellInlineConstraint>>;
  using ColspanCells = Vector<ColspanCell>;
  using Caption = MinMaxSizes;
  using CellBlockConstraints = Vector<CellBlockConstraint>;
  using RowspanCells = Vector<RowspanCell>;
  using Rows = Vector<Row>;
  using Sections = Vector<Section>;
};

class TableGroupedChildrenIterator;

// Table's children grouped by type.
// When iterating through members, make sure to handle out_of_flows correctly.
struct TableGroupedChildren {
  DISALLOW_NEW();

 public:
  explicit TableGroupedChildren(const BlockNode& table);
  ~TableGroupedChildren() {
    captions.clear();
    columns.clear();
    bodies.clear();
  }

  void Trace(Visitor*) const;

  HeapVector<BlockNode> captions;  // CAPTION
  HeapVector<BlockNode> columns;   // COLGROUP, COL

  BlockNode header;  // first THEAD

  // These cannot be modified except in ctor to ensure
  // TableGroupedChildrenIterator works correctly.
  HeapVector<BlockNode> bodies;  // TBODY/multiple THEAD/TFOOT

  BlockNode footer;  // first TFOOT

  // Default iterators iterate over tbody-like (THEAD/TBODY/TFOOT) elements.
  TableGroupedChildrenIterator begin() const;
  TableGroupedChildrenIterator end() const;
};

// Iterates table's sections in order:
// thead, tbody, tfoot
class TableGroupedChildrenIterator {
  STACK_ALLOCATED();

  enum CurrentSection { kNone, kHead, kBody, kFoot, kEnd };

 public:
  explicit TableGroupedChildrenIterator(
      const TableGroupedChildren& grouped_children,
      bool is_end = false);

  TableGroupedChildrenIterator& operator++();
  TableGroupedChildrenIterator& operator--();
  BlockNode operator*() const;
  bool operator==(const TableGroupedChildrenIterator& rhs) const;
  bool operator!=(const TableGroupedChildrenIterator& rhs) const;
  // True if section should be treated as tbody
  bool TreatAsTBody() const { return current_section_ == kBody; }

 private:
  void AdvanceForwardToNonEmptySection();
  void AdvanceBackwardToNonEmptySection();
  const TableGroupedChildren& grouped_children_;
  CurrentSection current_section_{kNone};

  // |body_vector_| can be modified only in ctor and
  // |AdvanceToNonEmptySection()|.
  const HeapVector<BlockNode>* body_vector_ = nullptr;
  wtf_size_t position_ = 0;
};

}  // namespace blink

WTF_ALLOW_MOVE_INIT_AND_COMPARE_WITH_MEM_FUNCTIONS(
    blink::TableTypes::CellInlineConstraint)
WTF_ALLOW_MOVE_INIT_AND_COMPARE_WITH_MEM_FUNCTIONS(
    blink::TableTypes::ColspanCell)
WTF_ALLOW_MOVE_INIT_AND_COMPARE_WITH_MEM_FUNCTIONS(blink::TableTypes::Column)
WTF_ALLOW_MOVE_INIT_AND_COMPARE_WITH_MEM_FUNCTIONS(
    blink::TableTypes::CellBlockConstraint)
WTF_ALLOW_MOVE_INIT_AND_COMPARE_WITH_MEM_FUNCTIONS(
    blink::TableTypes::RowspanCell)
WTF_ALLOW_MOVE_INIT_AND_COMPARE_WITH_MEM_FUNCTIONS(blink::TableTypes::Row)
WTF_ALLOW_MOVE_INIT_AND_COMPARE_WITH_MEM_FUNCTIONS(blink::TableTypes::Section)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_TABLE_LAYOUT_ALGORITHM_TYPES_H_
