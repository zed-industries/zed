// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_GLIC_MARKER_LIST_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_GLIC_MARKER_LIST_IMPL_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/markers/highlight_pseudo_marker_list_impl.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class CORE_EXPORT GlicMarkerListImpl final
    : public HighlightPseudoMarkerListImpl {
 public:
  GlicMarkerListImpl() = default;
  GlicMarkerListImpl(const GlicMarkerListImpl&) = delete;
  GlicMarkerListImpl& operator=(const GlicMarkerListImpl&) = delete;

  // `HighlightPseudoMarkerListImpl`:
  DocumentMarker::MarkerType MarkerType() const final;
  void MergeOverlappingMarkers() final;
};

template <>
struct DowncastTraits<GlicMarkerListImpl> {
  static bool AllowFrom(const DocumentMarkerList& list) {
    return list.MarkerType() == DocumentMarker::kGlic;
  }
  static bool AllowFrom(const GlicMarkerListImpl& list) {
    return list.MarkerType() == DocumentMarker::kGlic;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_GLIC_MARKER_LIST_IMPL_H_
