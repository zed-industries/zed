// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_SELF_ALIGNMENT_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_SELF_ALIGNMENT_DATA_H_

#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class StyleSelfAlignmentData {
  DISALLOW_NEW();

 public:
  // Style data for Self-Aligment and Default-Alignment properties: align-{self,
  // items}, justify-{self, items}.
  // [ <self-position> && <overflow-position>? ] | [ legacy && [ left | right |
  // center ] ]
  StyleSelfAlignmentData(
      ItemPosition position,
      OverflowAlignment overflow,
      ItemPositionType position_type = ItemPositionType::kNonLegacy)
      : position_(static_cast<unsigned>(position)),
        position_type_(static_cast<unsigned>(position_type)),
        overflow_(static_cast<unsigned>(overflow)) {}

  void SetPosition(ItemPosition position) {
    position_ = static_cast<unsigned>(position);
  }
  void SetPositionType(ItemPositionType position_type) {
    position_type_ = static_cast<unsigned>(position_type);
  }
  void SetOverflow(OverflowAlignment overflow) {
    overflow_ = static_cast<unsigned>(overflow);
  }

  ItemPosition GetPosition() const {
    return static_cast<ItemPosition>(position_);
  }
  ItemPositionType PositionType() const {
    return static_cast<ItemPositionType>(position_type_);
  }
  OverflowAlignment Overflow() const {
    return static_cast<OverflowAlignment>(overflow_);
  }

  bool operator==(const StyleSelfAlignmentData& o) const {
    return position_ == o.position_ && position_type_ == o.position_type_ &&
           overflow_ == o.overflow_;
  }

  bool operator!=(const StyleSelfAlignmentData& o) const {
    return !(*this == o);
  }

 private:
  unsigned position_ : 4;       // ItemPosition
  unsigned position_type_ : 1;  // Whether or not alignment uses the 'legacy'
                                // keyword.
  unsigned overflow_ : 2;       // OverflowAlignment
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_SELF_ALIGNMENT_DATA_H_
