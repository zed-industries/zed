// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_FIND_PAINT_OFFSET_NEEDING_UPDATE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_FIND_PAINT_OFFSET_NEEDING_UPDATE_H_

#include "base/dcheck_is_on.h"

#if DCHECK_IS_ON()

#include <optional>

#include "base/check_op.h"
#include "third_party/blink/renderer/core/layout/layout_object.h"
#include "third_party/blink/renderer/core/paint/fragment_data.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "ui/gfx/geometry/vector2d_f.h"

namespace blink {

// FindPaintOffsetNeedingUpdateScope catches cases where paint offset needed
// an update but was not marked as such. If paint offset will change, the
// object must be marked as such by
// LayoutObject::SetShouldCheckLayoutForPaintInvalidation()
// (which is a private function called by several public paint-invalidation-flag
// setting functions).
class FindPaintOffsetNeedingUpdateScope {
  STACK_ALLOCATED();

 public:
  FindPaintOffsetNeedingUpdateScope(const LayoutObject& object,
                                    const FragmentData& fragment_data,
                                    const bool& is_actually_needed)
      : object_(object),
        fragment_data_(fragment_data),
        is_actually_needed_(is_actually_needed),
        old_paint_offset_(fragment_data.PaintOffset()) {
    if (const auto* properties = fragment_data.PaintProperties()) {
      if (const auto* translation = properties->PaintOffsetTranslation()) {
        old_parent_ = translation->Parent();
        old_translation_ = translation->Get2dTranslation();
      }
    }
  }

  ~FindPaintOffsetNeedingUpdateScope() {
    if (is_actually_needed_)
      return;
    auto paint_offset = fragment_data_.PaintOffset();
    DCHECK_EQ(old_paint_offset_, paint_offset) << object_;

    const TransformPaintPropertyNodeOrAlias* new_parent = nullptr;
    std::optional<gfx::Vector2dF> new_translation;
    if (const auto* properties = fragment_data_.PaintProperties()) {
      if (const auto* translation = properties->PaintOffsetTranslation()) {
        new_parent = translation->Parent();
        new_translation = translation->Get2dTranslation();
      }
    }
    DCHECK_EQ(!!old_translation_, !!new_translation) << object_;
    DCHECK_EQ(old_parent_, new_parent) << object_.DebugName();
    if (old_translation_ && new_translation)
      DCHECK_EQ(*old_translation_, *new_translation) << object_;
  }

 private:
  const LayoutObject& object_;
  const FragmentData& fragment_data_;
  const bool& is_actually_needed_;
  PhysicalOffset old_paint_offset_;
  const TransformPaintPropertyNodeOrAlias* old_parent_ = nullptr;
  std::optional<gfx::Vector2dF> old_translation_;
};

}  // namespace blink

#endif  // DCHECK_IS_ON()

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_FIND_PAINT_OFFSET_NEEDING_UPDATE_H_
