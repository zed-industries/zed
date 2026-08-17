// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_GLIC_MARKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_GLIC_MARKER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/markers/document_marker.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class CORE_EXPORT GlicMarker final : public DocumentMarker {
 public:
  GlicMarker(unsigned start_offset, unsigned end_offset);
  GlicMarker(const GlicMarker&) = delete;
  GlicMarker& operator=(const GlicMarker&) = delete;

  // `DocumentMarker`:
  MarkerType GetType() const final;

  Color BackgroundColor() const;

  // Returns if the last frame is reached.
  bool UpdateOpacityForDuration(base::TimeDelta duration);

 private:
  float opacity_ = 0.f;
};

template <>
struct DowncastTraits<GlicMarker> {
  static bool AllowFrom(const DocumentMarker& marker) {
    return marker.GetType() == DocumentMarker::kGlic;
  }
  static bool AllowFrom(const GlicMarker& marker) {
    return marker.GetType() == DocumentMarker::kGlic;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_GLIC_MARKER_H_
