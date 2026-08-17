// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MATHML_MATHML_PAINT_INFO_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MATHML_MATHML_PAINT_INFO_H_

#include <unicode/uchar.h>

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/character_names.h"

namespace blink {

class ShapeResultView;

struct CORE_EXPORT MathMLPaintInfo : public GarbageCollected<MathMLPaintInfo> {
 public:
  MathMLPaintInfo(UChar operator_character,
                  const ShapeResultView* operator_shape_result_view,
                  LayoutUnit operator_inline_size,
                  LayoutUnit operator_ascent,
                  LayoutUnit operator_descent,
                  const BoxStrut& radical_base_margins = BoxStrut(),
                  const std::optional<LayoutUnit>&
                      radical_operator_inline_offset = std::nullopt)
      : operator_character(operator_character),
        operator_shape_result_view(operator_shape_result_view),
        operator_inline_size(operator_inline_size),
        operator_ascent(operator_ascent),
        operator_descent(operator_descent),
        radical_base_margins(radical_base_margins),
        radical_operator_inline_offset(radical_operator_inline_offset) {}

  void Trace(Visitor* visitor) const {
    visitor->Trace(operator_shape_result_view);
  }
  bool IsRadicalOperator() const {
    return radical_operator_inline_offset.has_value();
  }
  UChar operator_character{kNonCharacter};
  Member<const ShapeResultView> operator_shape_result_view;
  LayoutUnit operator_inline_size;
  LayoutUnit operator_ascent;
  LayoutUnit operator_descent;
  BoxStrut radical_base_margins;
  std::optional<LayoutUnit> radical_operator_inline_offset;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MATHML_MATHML_PAINT_INFO_H_
