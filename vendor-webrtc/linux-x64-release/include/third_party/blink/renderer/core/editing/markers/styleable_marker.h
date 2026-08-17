// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_STYLEABLE_MARKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_STYLEABLE_MARKER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/markers/document_marker.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "ui/base/ime/mojom/ime_types.mojom-blink.h"

namespace blink {

// An abstract subclass of DocumentMarker to be subclassed by DocumentMarkers
// that should be renderable with customizable formatting.
class CORE_EXPORT StyleableMarker : public DocumentMarker {
 public:
  StyleableMarker(unsigned start_offset,
                  unsigned end_offset,
                  Color underline_color,
                  ui::mojom::ImeTextSpanThickness,
                  ui::mojom::ImeTextSpanUnderlineStyle,
                  Color text_color,
                  Color background_color);
  StyleableMarker(const StyleableMarker&) = delete;
  StyleableMarker& operator=(const StyleableMarker&) = delete;

  // StyleableMarker-specific
  Color UnderlineColor() const;
  bool HasThicknessNone() const;
  bool HasThicknessThin() const;
  bool HasThicknessThick() const;
  ui::mojom::ImeTextSpanUnderlineStyle UnderlineStyle() const;
  bool UseTextColor() const;
  Color BackgroundColor() const;
  Color TextColor() const;

 private:
  const Color underline_color_;
  const Color background_color_;
  const ui::mojom::ImeTextSpanThickness thickness_;
  const ui::mojom::ImeTextSpanUnderlineStyle underline_style_;
  const Color text_color_;
};

bool CORE_EXPORT IsStyleableMarker(const DocumentMarker&);

template <>
struct DowncastTraits<StyleableMarker> {
  static bool AllowFrom(const DocumentMarker& marker) {
    return IsStyleableMarker(marker);
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_STYLEABLE_MARKER_H_
