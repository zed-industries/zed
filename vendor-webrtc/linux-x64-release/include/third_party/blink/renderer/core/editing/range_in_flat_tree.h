// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_RANGE_IN_FLAT_TREE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_RANGE_IN_FLAT_TREE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/ephemeral_range.h"
#include "third_party/blink/renderer/core/editing/relocatable_position.h"

namespace blink {

// This is a wrapper class for a range in flat tree that is relocatable by
// relocating the start and end positions in DOM tree.
class CORE_EXPORT RangeInFlatTree final
    : public GarbageCollected<RangeInFlatTree> {
 public:
  RangeInFlatTree();
  RangeInFlatTree(const PositionInFlatTree& start,
                  const PositionInFlatTree& end);

  void SetStart(const PositionInFlatTree& start);

  void SetEnd(const PositionInFlatTree& end);

  PositionInFlatTree StartPosition() const;

  PositionInFlatTree EndPosition() const;

  bool IsCollapsed() const;

  bool IsConnected() const;

  bool IsNull() const;

  EphemeralRangeInFlatTree ToEphemeralRange() const;

  void Trace(Visitor* visitor) const;

 private:
  Member<RelocatablePosition> start_;
  Member<RelocatablePosition> end_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_RANGE_IN_FLAT_TREE_H_
