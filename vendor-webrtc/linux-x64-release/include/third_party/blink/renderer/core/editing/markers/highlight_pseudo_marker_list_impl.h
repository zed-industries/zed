// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_HIGHLIGHT_PSEUDO_MARKER_LIST_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_HIGHLIGHT_PSEUDO_MARKER_LIST_IMPL_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/markers/document_marker_list.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

// Implementation of HighlightPseudoMarkerList for HighlightPseudo markers.
class CORE_EXPORT HighlightPseudoMarkerListImpl : public DocumentMarkerList {
 public:
  HighlightPseudoMarkerListImpl(const HighlightPseudoMarkerListImpl&) = delete;
  HighlightPseudoMarkerListImpl& operator=(
      const HighlightPseudoMarkerListImpl&) = delete;

  bool IsEmpty() const final;

  void Add(DocumentMarker*) final;
  void Clear() final;

  const HeapVector<Member<DocumentMarker>>& GetMarkers() const final;
  DocumentMarker* FirstMarkerIntersectingRange(unsigned start_offset,
                                               unsigned end_offset) const final;
  HeapVector<Member<DocumentMarker>> MarkersIntersectingRange(
      unsigned start_offset,
      unsigned end_offset) const final;

  bool MoveMarkers(int length, DocumentMarkerList* dst_list) final;
  bool RemoveMarkers(unsigned start_offset, int length) final;
  bool ShiftMarkers(const String& node_text,
                    unsigned offset,
                    unsigned old_length,
                    unsigned new_length) final;

  void Trace(blink::Visitor*) const override;

 protected:
  HighlightPseudoMarkerListImpl() = default;

  HeapVector<Member<DocumentMarker>> markers_;
};

template <>
struct DowncastTraits<HighlightPseudoMarkerListImpl> {
  static bool AllowFrom(const DocumentMarkerList& list) {
    return list.MarkerType() == DocumentMarker::kCustomHighlight ||
           list.MarkerType() == DocumentMarker::kTextFragment;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_HIGHLIGHT_PSEUDO_MARKER_LIST_IMPL_H_
