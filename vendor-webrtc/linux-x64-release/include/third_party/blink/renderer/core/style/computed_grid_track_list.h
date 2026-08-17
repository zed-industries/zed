// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_COMPUTED_GRID_TRACK_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_COMPUTED_GRID_TRACK_LIST_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/style/grid_track_list.h"
#include "third_party/blink/renderer/core/style/grid_track_size.h"
#include "third_party/blink/renderer/core/style/named_grid_lines_map.h"
#include "third_party/blink/renderer/core/style/ordered_named_grid_lines.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

struct CORE_EXPORT ComputedGridTrackList {
  ComputedGridTrackList() = default;

  static ComputedGridTrackList DefaultMasonryTrackList() {
    ComputedGridTrackList default_track_list;
    default_track_list.track_list.AddRepeater({GridTrackSize(Length::Auto())});
    return default_track_list;
  }

  bool operator==(const ComputedGridTrackList& other) const {
    return track_list == other.track_list &&
           named_grid_lines == other.named_grid_lines &&
           auto_repeat_named_grid_lines == other.auto_repeat_named_grid_lines &&
           ordered_named_grid_lines == other.ordered_named_grid_lines &&
           auto_repeat_ordered_named_grid_lines ==
               other.auto_repeat_ordered_named_grid_lines &&
           auto_repeat_insertion_point == other.auto_repeat_insertion_point &&
           auto_repeat_type == other.auto_repeat_type &&
           axis_type == other.axis_type;
  }

  bool operator!=(const ComputedGridTrackList& other) const {
    return !(*this == other);
  }

  bool IsSubgriddedAxis() const {
    return axis_type == GridAxisType::kSubgriddedAxis;
  }

  NGGridTrackList track_list;

  NamedGridLinesMap named_grid_lines;
  NamedGridLinesMap auto_repeat_named_grid_lines;
  OrderedNamedGridLines ordered_named_grid_lines;
  OrderedNamedGridLines auto_repeat_ordered_named_grid_lines;

  wtf_size_t auto_repeat_insertion_point{0};
  AutoRepeatType auto_repeat_type{AutoRepeatType::kNoAutoRepeat};
  GridAxisType axis_type{GridAxisType::kStandaloneAxis};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_COMPUTED_GRID_TRACK_LIST_H_
