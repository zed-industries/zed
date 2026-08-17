// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_COMPUTED_GRID_TEMPLATE_AREAS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_COMPUTED_GRID_TEMPLATE_AREAS_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/style/grid_area.h"
#include "third_party/blink/renderer/core/style/named_grid_lines_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"

namespace blink {

class CORE_EXPORT ComputedGridTemplateAreas
    : public GarbageCollected<ComputedGridTemplateAreas> {
 public:
  ComputedGridTemplateAreas() = default;
  ComputedGridTemplateAreas(const NamedGridAreaMap&,
                            wtf_size_t row_count,
                            wtf_size_t column_count);

  void Trace(Visitor* visitor) const {}

  bool operator==(const ComputedGridTemplateAreas& other) const {
    // No need to check `implicit_named_grid_row_lines` or
    // `implicit_named_grid_column_lines` as they're derived from `named_areas`.
    return named_areas == other.named_areas && row_count == other.row_count &&
           column_count == other.column_count;
  }

  bool operator!=(const ComputedGridTemplateAreas& other) const {
    return !(*this == other);
  }

  static NamedGridLinesMap CreateImplicitNamedGridLinesFromGridArea(
      const NamedGridAreaMap& named_areas,
      GridTrackSizingDirection direction);

  NamedGridAreaMap named_areas;

  NamedGridLinesMap implicit_named_grid_row_lines;
  NamedGridLinesMap implicit_named_grid_column_lines;

  wtf_size_t row_count{0};
  wtf_size_t column_count{0};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_COMPUTED_GRID_TEMPLATE_AREAS_H_
