// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_GRAMMAR_MARKER_LIST_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_GRAMMAR_MARKER_LIST_IMPL_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/markers/spell_check_marker_list_impl.h"

namespace blink {

// This is the DocumentMarkerList implementation used to store Grammar markers.
class CORE_EXPORT GrammarMarkerListImpl final
    : public SpellCheckMarkerListImpl {
 public:
  GrammarMarkerListImpl() = default;
  GrammarMarkerListImpl(const GrammarMarkerListImpl&) = delete;
  GrammarMarkerListImpl& operator=(const GrammarMarkerListImpl&) = delete;

  DocumentMarker::MarkerType MarkerType() const final;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_GRAMMAR_MARKER_LIST_IMPL_H_
