// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_COMPOSITION_MARKER_LIST_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_COMPOSITION_MARKER_LIST_IMPL_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/markers/document_marker_list.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"

namespace blink {

// Implementation of DocumentMarkerList for Composition markers. Composition
// markers can overlap (e.g. an IME might pass two markers on the same region of
// text, one to underline it and one to add a background color), so we store
// them unsorted.
class CORE_EXPORT CompositionMarkerListImpl final : public DocumentMarkerList {
 public:
  CompositionMarkerListImpl() = default;
  CompositionMarkerListImpl(const CompositionMarkerListImpl&) = delete;
  CompositionMarkerListImpl& operator=(const CompositionMarkerListImpl&) =
      delete;

  // DocumentMarkerList implementations
  DocumentMarker::MarkerType MarkerType() const final;

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

  void MergeOverlappingMarkers() final {}

  void Trace(Visitor*) const override;

 private:
  HeapVector<Member<DocumentMarker>> markers_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_COMPOSITION_MARKER_LIST_IMPL_H_
