// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GRID_GRID_NAMED_LINE_COLLECTION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GRID_GRID_NAMED_LINE_COLLECTION_H_

#include "third_party/blink/renderer/core/style/grid_enums.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

using NamedGridLinesMap = HashMap<String, Vector<wtf_size_t>>;
struct ComputedGridTrackList;

class GridNamedLineCollection {
 public:
  GridNamedLineCollection(const String& named_line,
                          GridTrackSizingDirection track_direction,
                          const NamedGridLinesMap& implicit_grid_line_names,
                          const NamedGridLinesMap& explicit_grid_line_names,
                          const ComputedGridTrackList& computed_grid_track_list,
                          wtf_size_t last_line,
                          wtf_size_t auto_repeat_tracks_count,
                          bool is_subgridded_to_parent);

  GridNamedLineCollection(const GridNamedLineCollection&) = delete;
  GridNamedLineCollection& operator=(const GridNamedLineCollection&) = delete;

  bool HasNamedLines() const;
  wtf_size_t FirstPosition() const;

  bool Contains(wtf_size_t line) const;

 private:
  bool HasExplicitNamedLines() const;
  // Returns true if the author specified auto repeat tracks, but they were
  // collapsed to zero repeats. Only possible for subgrids.
  bool HasCollapsedAutoRepeat() const;
  wtf_size_t FirstExplicitPosition() const;
  const Vector<wtf_size_t>* named_lines_indexes_ = nullptr;
  const Vector<wtf_size_t>* auto_repeat_named_lines_indexes_ = nullptr;
  const Vector<wtf_size_t>* implicit_named_lines_indexes_ = nullptr;

  bool is_standalone_grid_;
  wtf_size_t insertion_point_;
  wtf_size_t last_line_;
  wtf_size_t auto_repeat_total_tracks_;
  wtf_size_t auto_repeat_track_list_length_;
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GRID_GRID_NAMED_LINE_COLLECTION_H_
