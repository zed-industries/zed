// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_POSITIONED_FLOAT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_POSITIONED_FLOAT_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/geometry/bfc_offset.h"
#include "third_party/blink/renderer/core/layout/layout_result.h"

namespace blink {

class LayoutResult;

// Contains the information necessary for copying back data to a FloatingObject.
struct CORE_EXPORT PositionedFloat {
  DISALLOW_NEW();

 public:
  PositionedFloat() = default;
  PositionedFloat(const LayoutResult* layout_result,
                  const BlockBreakToken* break_before_token,
                  const BfcOffset& bfc_offset,
                  LayoutUnit minimum_space_shortage)
      : layout_result(layout_result),
        break_before_token(break_before_token),
        bfc_offset(bfc_offset),
        minimum_space_shortage(minimum_space_shortage) {}
  PositionedFloat(PositionedFloat&&) noexcept = default;
  PositionedFloat(const PositionedFloat&) = default;
  PositionedFloat& operator=(PositionedFloat&&) = default;
  PositionedFloat& operator=(const PositionedFloat&) = default;

  void Trace(Visitor*) const;

  const BlockBreakToken* BreakToken() const;

  Member<const LayoutResult> layout_result;
  Member<const BlockBreakToken> break_before_token;
  BfcOffset bfc_offset;
  LayoutUnit minimum_space_shortage;
};

}  // namespace blink

WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(blink::PositionedFloat)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_POSITIONED_FLOAT_H_
