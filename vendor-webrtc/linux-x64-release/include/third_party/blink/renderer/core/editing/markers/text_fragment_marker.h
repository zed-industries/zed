// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_TEXT_FRAGMENT_MARKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_TEXT_FRAGMENT_MARKER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/markers/highlight_pseudo_marker.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

// A subclass of HighlightPseudoMarker used for indicating a text fragment on
// the page. See blink/renderer/core/page/scrolling/text_fragment_anchor.h.
class CORE_EXPORT TextFragmentMarker final : public HighlightPseudoMarker {
 public:
  TextFragmentMarker(unsigned start_offset, unsigned end_offset);
  TextFragmentMarker(const TextFragmentMarker&) = delete;
  TextFragmentMarker& operator=(const TextFragmentMarker&) = delete;

  // DocumentMarker implementations.
  MarkerType GetType() const final;

  // HighlightPseudoMarker implementations.
  PseudoId GetPseudoId() const final;
  const AtomicString& GetPseudoArgument() const final;
};

template <>
struct DowncastTraits<TextFragmentMarker> {
  static bool AllowFrom(const DocumentMarker& marker) {
    return marker.GetType() == DocumentMarker::kTextFragment;
  }
  static bool AllowFrom(const HighlightPseudoMarker& marker) {
    return marker.GetType() == DocumentMarker::kTextFragment;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_TEXT_FRAGMENT_MARKER_H_
