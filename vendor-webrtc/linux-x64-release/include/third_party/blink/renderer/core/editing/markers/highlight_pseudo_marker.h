// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_HIGHLIGHT_PSEUDO_MARKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_HIGHLIGHT_PSEUDO_MARKER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/markers/document_marker.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

// DocumentMarker subclass that represent the CSS highlight pseudos-elements and
// the CSS custom highlight pseudo-element:
// * https://drafts.csswg.org/css-pseudo-4/#highlight-pseudos
// * https://drafts.csswg.org/css-highlight-api/#custom-highlight-pseudo
class CORE_EXPORT HighlightPseudoMarker : public DocumentMarker {
 public:
  HighlightPseudoMarker(unsigned start_offset, unsigned end_offset);
  HighlightPseudoMarker(const HighlightPseudoMarker&) = delete;
  HighlightPseudoMarker& operator=(const HighlightPseudoMarker&) = delete;

  virtual PseudoId GetPseudoId() const = 0;
  virtual const AtomicString& GetPseudoArgument() const = 0;
};

template <>
struct DowncastTraits<HighlightPseudoMarker> {
  static bool AllowFrom(const DocumentMarker& document_marker) {
    return document_marker.GetType() == DocumentMarker::kCustomHighlight ||
           document_marker.GetType() == DocumentMarker::kTextFragment;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_HIGHLIGHT_PSEUDO_MARKER_H_
