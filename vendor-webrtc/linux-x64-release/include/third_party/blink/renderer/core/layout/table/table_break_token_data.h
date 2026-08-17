// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_TABLE_BREAK_TOKEN_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_TABLE_BREAK_TOKEN_DATA_H_

#include "third_party/blink/renderer/core/layout/block_break_token_data.h"
#include "third_party/blink/renderer/core/layout/table/table_layout_algorithm_types.h"

namespace blink {

struct TableBreakTokenData final : BlockBreakTokenData {
  TableBreakTokenData(
      const BlockBreakTokenData* break_token_data,
      const TableTypes::Rows& rows,
      const TableTypes::CellBlockConstraints& cell_block_constraints,
      const TableTypes::Sections& sections,
      LayoutUnit total_table_min_block_size,
      LayoutUnit consumed_table_box_block_size,
      bool has_entered_table_box,
      bool is_past_table_box)
      : BlockBreakTokenData(kTableBreakTokenData, break_token_data),
        rows(rows),
        cell_block_constraints(cell_block_constraints),
        sections(sections),
        total_table_min_block_size(total_table_min_block_size),
        consumed_table_box_block_size(consumed_table_box_block_size),
        has_entered_table_box(has_entered_table_box),
        is_past_table_box(is_past_table_box) {}
  // Table layout information that will be the same for all fragments to be
  // generated (but potentially very expensive to calculate).
  TableTypes::Rows rows;
  TableTypes::CellBlockConstraints cell_block_constraints;
  TableTypes::Sections sections;
  LayoutUnit total_table_min_block_size;

  // Similar to |consumed_block_size|, but it only counts space consumed by the
  // table box itself, i.e. excluding any captions.
  LayoutUnit consumed_table_box_block_size;

  // True if we have entered the table box, i.e. we're past any top captions.
  bool has_entered_table_box;

  // True if we're past the table box, i.e. all sections (or, if there are no
  // sections, this will be set once we've added all the space required by
  // specified block-size & co).
  bool is_past_table_box;
};

template <>
struct DowncastTraits<TableBreakTokenData> {
  static bool AllowFrom(const BlockBreakTokenData& token_data) {
    return token_data.IsTableType();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_TABLE_BREAK_TOKEN_DATA_H_
