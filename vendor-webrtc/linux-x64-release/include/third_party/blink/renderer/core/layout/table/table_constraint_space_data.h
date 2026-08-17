// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_TABLE_CONSTRAINT_SPACE_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_TABLE_CONSTRAINT_SPACE_DATA_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/core/layout/geometry/logical_size.h"
#include "third_party/blink/renderer/core/layout/table/table_column_location.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/text/writing_mode.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

// The same TableConstraintSpaceData object gets passed on the constraint
// space to all rows and sections within a single table. It contains all the
// geometry information needed to layout sections/rows/cells. This is different
// from most other algorithms, where the constraint space data is not shared.
class TableConstraintSpaceData : public RefCounted<TableConstraintSpaceData> {
 public:
  // |Section| holds the row index information used to map between table and
  // section row indexes.
  struct Section {
    Section(wtf_size_t start_row_index, wtf_size_t row_count)
        : start_row_index(start_row_index), row_count(row_count) {}

    bool MaySkipLayout(const Section& other) const {
      // We don't compare |start_row_index| as this is allowed to change.
      return row_count == other.row_count;
    }

    const wtf_size_t start_row_index;  // First section row in table grid.
    const wtf_size_t row_count;
  };

  // Data needed by row layout algorithm.
  struct Row {
    Row(LayoutUnit block_size,
        wtf_size_t start_cell_index,
        wtf_size_t cell_count,
        std::optional<LayoutUnit> baseline,
        bool is_collapsed)
        : block_size(block_size),
          start_cell_index(start_cell_index),
          cell_count(cell_count),
          baseline(baseline),
          is_collapsed(is_collapsed) {}

    bool MaySkipLayout(const Row& other) const {
      // We don't compare |start_cell_index| as this is allowed to change.
      return block_size == other.block_size && cell_count == other.cell_count &&
             baseline == other.baseline && is_collapsed == other.is_collapsed;
    }

    const LayoutUnit block_size;
    const wtf_size_t start_cell_index;
    const wtf_size_t cell_count;
    const std::optional<LayoutUnit> baseline;
    const bool is_collapsed;
  };

  // Data needed to layout a single cell.
  struct Cell {
    Cell(BoxStrut borders,
         LayoutUnit rowspan_block_size,
         wtf_size_t start_column,
         bool is_initial_block_size_indefinite,
         bool has_descendant_that_depends_on_percentage_block_size)
        : borders(borders),
          rowspan_block_size(rowspan_block_size),
          start_column(start_column),
          is_initial_block_size_indefinite(is_initial_block_size_indefinite),
          has_descendant_that_depends_on_percentage_block_size(
              has_descendant_that_depends_on_percentage_block_size) {}

    bool operator==(const Cell& other) const {
      return borders == other.borders &&
             rowspan_block_size == other.rowspan_block_size &&
             start_column == other.start_column &&
             is_initial_block_size_indefinite ==
                 other.is_initial_block_size_indefinite &&
             has_descendant_that_depends_on_percentage_block_size ==
                 other.has_descendant_that_depends_on_percentage_block_size;
    }
    bool operator!=(const Cell& other) const { return !(*this == other); }

    // Size of borders drawn on the inside of the border box.
    const BoxStrut borders;
    // Size of the cell. Need this for cells that span multiple rows.
    const LayoutUnit rowspan_block_size;
    const wtf_size_t start_column;
    const bool is_initial_block_size_indefinite;
    const bool has_descendant_that_depends_on_percentage_block_size;
  };

  bool IsTableSpecificDataEqual(const TableConstraintSpaceData& other) const {
    return column_locations == other.column_locations &&
           table_writing_direction == other.table_writing_direction &&
           table_border_spacing == other.table_border_spacing &&
           is_table_block_size_specified ==
               other.is_table_block_size_specified &&
           has_collapsed_borders == other.has_collapsed_borders;
  }

  bool MaySkipRowLayout(const TableConstraintSpaceData& other,
                        wtf_size_t new_row_index,
                        wtf_size_t old_row_index) const {
    DCHECK_LT(new_row_index, rows.size());
    DCHECK_LT(old_row_index, other.rows.size());

    const Row& new_row = rows[new_row_index];
    const Row& old_row = other.rows[old_row_index];
    if (!new_row.MaySkipLayout(old_row))
      return false;

    DCHECK_EQ(new_row.cell_count, old_row.cell_count);

    const wtf_size_t new_start_cell_index = new_row.start_cell_index;
    const wtf_size_t old_start_cell_index = old_row.start_cell_index;

    const wtf_size_t new_end_cell_index =
        new_start_cell_index + new_row.cell_count;
    const wtf_size_t old_end_cell_index =
        old_start_cell_index + old_row.cell_count;

    for (wtf_size_t new_cell_index = new_start_cell_index,
                    old_cell_index = old_start_cell_index;
         new_cell_index < new_end_cell_index &&
         old_cell_index < old_end_cell_index;
         ++new_cell_index, ++old_cell_index) {
      if (cells[new_cell_index] != other.cells[old_cell_index])
        return false;
    }

    return true;
  }

  bool MaySkipSectionLayout(const TableConstraintSpaceData& other,
                            wtf_size_t new_section_index,
                            wtf_size_t old_section_index) const {
    DCHECK_LE(new_section_index, sections.size());
    DCHECK_LE(old_section_index, other.sections.size());

    const Section& new_section = sections[new_section_index];
    const Section& old_section = other.sections[old_section_index];
    if (!new_section.MaySkipLayout(old_section))
      return false;

    DCHECK_EQ(new_section.row_count, old_section.row_count);

    const wtf_size_t new_start_row_index = new_section.start_row_index;
    const wtf_size_t old_start_row_index = old_section.start_row_index;

    // Collapsed-border painting has a dependency on the row-index.
    DCHECK_EQ(has_collapsed_borders, other.has_collapsed_borders);
    if (has_collapsed_borders && new_start_row_index != old_start_row_index)
      return false;

    const wtf_size_t new_end_row_index =
        new_start_row_index + new_section.row_count;
    const wtf_size_t old_end_row_index =
        old_start_row_index + old_section.row_count;

    for (wtf_size_t new_row_index = new_start_row_index,
                    old_row_index = old_start_row_index;
         new_row_index < new_end_row_index && old_row_index < old_end_row_index;
         ++new_row_index, ++old_row_index) {
      if (!MaySkipRowLayout(other, new_row_index, old_row_index))
        return false;
    }

    return true;
  }

  Vector<TableColumnLocation> column_locations;
  Vector<Section> sections;
  Vector<Row> rows;
  Vector<Cell> cells;
  WritingDirectionMode table_writing_direction =
      WritingDirectionMode(WritingMode::kHorizontalTb, TextDirection::kLtr);
  LogicalSize table_border_spacing;

  // If the block-size of the table is specified (not 'auto').
  bool is_table_block_size_specified;
  bool has_collapsed_borders;
};

}  // namespace blink

WTF_ALLOW_MOVE_INIT_AND_COMPARE_WITH_MEM_FUNCTIONS(
    blink::TableConstraintSpaceData::Section)
WTF_ALLOW_MOVE_AND_INIT_WITH_MEM_FUNCTIONS(blink::TableConstraintSpaceData::Row)
WTF_ALLOW_MOVE_AND_INIT_WITH_MEM_FUNCTIONS(
    blink::TableConstraintSpaceData::Cell)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_TABLE_CONSTRAINT_SPACE_DATA_H_
