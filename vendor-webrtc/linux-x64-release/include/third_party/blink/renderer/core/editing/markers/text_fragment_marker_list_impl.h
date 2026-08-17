// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_TEXT_FRAGMENT_MARKER_LIST_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_TEXT_FRAGMENT_MARKER_LIST_IMPL_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/markers/highlight_pseudo_marker_list_impl.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

// Implementation of TextMarkerBaseListImpl for TextFragment markers.
class CORE_EXPORT TextFragmentMarkerListImpl final
    : public HighlightPseudoMarkerListImpl {
 public:
  TextFragmentMarkerListImpl() = default;
  TextFragmentMarkerListImpl(const TextFragmentMarkerListImpl&) = delete;
  TextFragmentMarkerListImpl& operator=(const TextFragmentMarkerListImpl&) =
      delete;

  // DocumentMarkerList implementations
  DocumentMarker::MarkerType MarkerType() const final;

  void MergeOverlappingMarkers() final;
};

template <>
struct DowncastTraits<TextFragmentMarkerListImpl> {
  static bool AllowFrom(const DocumentMarkerList& list) {
    return list.MarkerType() == DocumentMarker::kTextFragment;
  }
  static bool AllowFrom(const HighlightPseudoMarkerListImpl& list) {
    return list.MarkerType() == DocumentMarker::kTextFragment;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_TEXT_FRAGMENT_MARKER_LIST_IMPL_H_
