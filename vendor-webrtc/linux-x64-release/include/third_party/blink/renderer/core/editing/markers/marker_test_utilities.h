// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_MARKER_TEST_UTILITIES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_MARKER_TEST_UTILITIES_H_

#include "third_party/blink/renderer/core/editing/markers/suggestion_marker.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {
inline bool compare_markers(const Member<DocumentMarker>& marker1,
                            const Member<DocumentMarker>& marker2) {
  if (marker1->StartOffset() != marker2->StartOffset())
    return marker1->StartOffset() < marker2->StartOffset();

  return marker1->EndOffset() < marker2->EndOffset();
}
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_MARKER_TEST_UTILITIES_H_
