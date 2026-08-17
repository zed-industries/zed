// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_ACTIVE_SUGGESTION_MARKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_ACTIVE_SUGGESTION_MARKER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/markers/styleable_marker.h"

namespace blink {

// A subclass of StyleableMarker used to represent ActiveSuggestion markers,
// which are used to mark the region of text an open spellcheck or suggestion
// menu pertains to.
class CORE_EXPORT ActiveSuggestionMarker final : public StyleableMarker {
 public:
  ActiveSuggestionMarker(unsigned start_offset,
                         unsigned end_offset,
                         Color underline_color,
                         ui::mojom::ImeTextSpanThickness,
                         ui::mojom::ImeTextSpanUnderlineStyle underline_style,
                         Color text_color,
                         Color background_color);
  ActiveSuggestionMarker(const ActiveSuggestionMarker&) = delete;
  ActiveSuggestionMarker& operator=(const ActiveSuggestionMarker&) = delete;

  // DocumentMarker implementations
  MarkerType GetType() const final;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_ACTIVE_SUGGESTION_MARKER_H_
