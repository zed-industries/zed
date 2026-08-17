// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_COMPOSITION_MARKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_COMPOSITION_MARKER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/markers/styleable_marker.h"

namespace blink {

// A subclass of DocumentMarker used to store information specific to
// composition markers. We store what color to display the underline (possibly
// transparent), whether the underline should be thick or not, and what
// background color should be used under the marked text (also possibly
// transparent).
class CORE_EXPORT CompositionMarker final : public StyleableMarker {
 public:
  CompositionMarker(unsigned start_offset,
                    unsigned end_offset,
                    Color underline_color,
                    ui::mojom::ImeTextSpanThickness,
                    ui::mojom::ImeTextSpanUnderlineStyle,
                    Color text_color,
                    Color background_color);
  CompositionMarker(const CompositionMarker&) = delete;
  CompositionMarker& operator=(const CompositionMarker&) = delete;

  // DocumentMarker implementations
  MarkerType GetType() const final;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_COMPOSITION_MARKER_H_
