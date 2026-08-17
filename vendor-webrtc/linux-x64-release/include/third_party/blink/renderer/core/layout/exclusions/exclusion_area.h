// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_EXCLUSIONS_EXCLUSION_AREA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_EXCLUSIONS_EXCLUSION_AREA_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/geometry/bfc_rect.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"

namespace blink {

class LayoutBox;

struct CORE_EXPORT ExclusionShapeData final
    : public GarbageCollected<ExclusionShapeData> {
  ExclusionShapeData(const LayoutBox* layout_box,
                     const BoxStrut& margins,
                     const BoxStrut& shape_insets)
      : layout_box(layout_box), margins(margins), shape_insets(shape_insets) {}
  ExclusionShapeData(const ExclusionShapeData& other)
      : layout_box(other.layout_box),
        margins(other.margins),
        shape_insets(other.shape_insets) {}

  void Trace(Visitor*) const;

  Member<const LayoutBox> layout_box;
  const BoxStrut margins;
  const BoxStrut shape_insets;
};

// Struct that represents an exclusion for float and initial letter box.
struct CORE_EXPORT ExclusionArea final
    : public GarbageCollected<ExclusionArea> {
  enum Kind {
    kFloat,
    kInitialLetterBox,
  };

  ExclusionArea(const BfcRect& rect,
                const EFloat type,
                const Kind kind,
                bool is_hidden_for_paint,
                const ExclusionShapeData* shape_data)
      : rect(rect),
        type(type),
        kind(kind),
        is_hidden_for_paint(is_hidden_for_paint),
        shape_data(std::move(shape_data)) {}

  static const ExclusionArea* Create(const BfcRect& rect,
                                     const EFloat type,
                                     bool is_hidden_for_paint,
                                     ExclusionShapeData* shape_data = nullptr) {
    return MakeGarbageCollected<ExclusionArea>(
        rect, type, kFloat, is_hidden_for_paint, std::move(shape_data));
  }

  static const ExclusionArea* CreateForInitialLetterBox(
      const BfcRect& rect,
      const EFloat type,
      bool is_hidden_for_paint) {
    return MakeGarbageCollected<ExclusionArea>(rect, type, kInitialLetterBox,
                                               is_hidden_for_paint, nullptr);
  }

  const ExclusionArea* CopyWithOffset(const BfcDelta& offset_delta) const {
    if (!offset_delta.line_offset_delta && !offset_delta.block_offset_delta)
      return this;

    BfcRect new_rect = rect;
    new_rect.start_offset += offset_delta;
    new_rect.end_offset += offset_delta;

    return MakeGarbageCollected<ExclusionArea>(
        new_rect, type, kind, is_hidden_for_paint,
        shape_data ? MakeGarbageCollected<ExclusionShapeData>(*shape_data)
                   : nullptr);
  }

  bool IsForInitialLetterBox() const { return kind == kInitialLetterBox; }

  void Trace(Visitor* visitor) const { visitor->Trace(shape_data); }

  const BfcRect rect;
  const EFloat type;
  const Kind kind;
  bool is_past_other_exclusions = false;
  const bool is_hidden_for_paint = false;
  const Member<const ExclusionShapeData> shape_data;

  bool operator==(const ExclusionArea& other) const;
  bool operator!=(const ExclusionArea& other) const {
    return !(*this == other);
  }
};

using ExclusionAreaPtrArray = HeapVector<Member<const ExclusionArea>>;
using GCedExclusionAreaPtrArray = GCedHeapVector<Member<const ExclusionArea>>;

std::ostream& operator<<(std::ostream& os, const ExclusionArea& exclusion);
std::ostream& operator<<(std::ostream& os, const ExclusionArea* exclusion);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_EXCLUSIONS_EXCLUSION_AREA_H_
