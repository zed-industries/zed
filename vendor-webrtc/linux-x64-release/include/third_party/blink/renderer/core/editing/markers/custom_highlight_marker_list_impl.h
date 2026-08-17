// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_CUSTOM_HIGHLIGHT_MARKER_LIST_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_CUSTOM_HIGHLIGHT_MARKER_LIST_IMPL_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/markers/highlight_pseudo_marker_list_impl.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

// Implementation of HighlightPseudoMarkerListImpl for Highlight markers.
class CORE_EXPORT CustomHighlightMarkerListImpl final
    : public HighlightPseudoMarkerListImpl {
 public:
  CustomHighlightMarkerListImpl() = default;
  CustomHighlightMarkerListImpl(const CustomHighlightMarkerListImpl&) = delete;
  CustomHighlightMarkerListImpl& operator=(
      const CustomHighlightMarkerListImpl&) = delete;

  DocumentMarker::MarkerType MarkerType() const final;

  void MergeOverlappingMarkers() final;
};

template <>
struct DowncastTraits<CustomHighlightMarkerListImpl> {
  static bool AllowFrom(const DocumentMarkerList& list) {
    return list.MarkerType() == DocumentMarker::kCustomHighlight;
  }
  static bool AllowFrom(const HighlightPseudoMarkerListImpl& list) {
    return list.MarkerType() == DocumentMarker::kCustomHighlight;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_CUSTOM_HIGHLIGHT_MARKER_LIST_IMPL_H_
