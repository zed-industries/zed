// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_BLOCK_BREAK_TOKEN_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_BLOCK_BREAK_TOKEN_DATA_H_

#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

struct BlockBreakTokenData : public GarbageCollected<BlockBreakTokenData> {
 public:
  enum BreakTokenDataType {
    kBlockBreakTokenData,
    kFieldsetBreakTokenData,
    kFlexBreakTokenData,
    kGridBreakTokenData,
    kTableBreakTokenData,
    kTableRowBreakTokenData
    // When adding new values, ensure |type| below has enough bits.
  };
  BreakTokenDataType Type() const {
    return static_cast<BreakTokenDataType>(type);
  }

  explicit BlockBreakTokenData(BreakTokenDataType type = kBlockBreakTokenData,
                               const BlockBreakTokenData* other_data = nullptr)
      : type(type) {
    if (other_data) {
      consumed_block_size = other_data->consumed_block_size;
      consumed_block_size_legacy_adjustment =
          other_data->consumed_block_size_legacy_adjustment;
      sequence_number = other_data->sequence_number;
      monolithic_overflow = other_data->monolithic_overflow;
    }
  }

  virtual ~BlockBreakTokenData() = default;

  // One note about type checking and downcasting: It's generally not safe to
  // assume that a node has a specific break token data type. Break tokens
  // aren't always created by the layout algorithm normally associated with a
  // given node type, e.g. if we add a break-before break token.
  bool IsFieldsetType() const { return Type() == kFieldsetBreakTokenData; }
  bool IsFlexType() const { return Type() == kFlexBreakTokenData; }
  bool IsGridType() const { return Type() == kGridBreakTokenData; }
  bool IsTableType() const { return Type() == kTableBreakTokenData; }
  bool IsTableRowType() const { return Type() == kTableRowBreakTokenData; }

  virtual void Trace(Visitor* visitor) const {}

  LayoutUnit consumed_block_size;
  LayoutUnit consumed_block_size_legacy_adjustment;
  LayoutUnit monolithic_overflow;

  unsigned sequence_number = 0;
  unsigned type : 3;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_BLOCK_BREAK_TOKEN_DATA_H_
