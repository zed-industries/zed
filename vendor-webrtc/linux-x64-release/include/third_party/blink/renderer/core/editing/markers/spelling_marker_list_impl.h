// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_SPELLING_MARKER_LIST_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_SPELLING_MARKER_LIST_IMPL_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/markers/spell_check_marker_list_impl.h"

namespace blink {

// This is the DocumentMarkerList implementation used to store Spelling markers.
class CORE_EXPORT SpellingMarkerListImpl final
    : public SpellCheckMarkerListImpl {
 public:
  SpellingMarkerListImpl() = default;
  SpellingMarkerListImpl(const SpellingMarkerListImpl&) = delete;
  SpellingMarkerListImpl& operator=(const SpellingMarkerListImpl&) = delete;

  DocumentMarker::MarkerType MarkerType() const final;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_SPELLING_MARKER_LIST_IMPL_H_
