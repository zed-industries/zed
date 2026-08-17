// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_TABLE_ROW_BREAK_TOKEN_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_TABLE_ROW_BREAK_TOKEN_DATA_H_

#include "third_party/blink/renderer/core/layout/block_break_token_data.h"

namespace blink {

struct TableRowBreakTokenData final : BlockBreakTokenData {
  TableRowBreakTokenData(const BlockBreakTokenData* break_token_data,
                         LayoutUnit previous_consumed_row_block_size)
      : BlockBreakTokenData(kTableRowBreakTokenData, break_token_data),
        previous_consumed_row_block_size(previous_consumed_row_block_size) {}

  // Similar to |consumed_block_size| however it will stop increasing once it
  // reaches the total block-size rather than keep expanding based on its
  // content.
  LayoutUnit previous_consumed_row_block_size;
};

template <>
struct DowncastTraits<TableRowBreakTokenData> {
  static bool AllowFrom(const BlockBreakTokenData& token_data) {
    return token_data.IsTableRowType();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_TABLE_ROW_BREAK_TOKEN_DATA_H_
