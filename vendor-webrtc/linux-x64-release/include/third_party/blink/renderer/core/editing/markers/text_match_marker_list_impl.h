// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_TEXT_MATCH_MARKER_LIST_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_TEXT_MATCH_MARKER_LIST_IMPL_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/markers/document_marker_list.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace gfx {
class Rect;
}

namespace blink {

class Node;

// Nearly-complete implementation of DocumentMarkerList for text match markers.
// Markers are kept sorted by start offset, under the assumption that
// TextMatch markers are typically inserted in an order.
class CORE_EXPORT TextMatchMarkerListImpl final : public DocumentMarkerList {
 public:
  TextMatchMarkerListImpl() = default;
  TextMatchMarkerListImpl(const TextMatchMarkerListImpl&) = delete;
  TextMatchMarkerListImpl& operator=(const TextMatchMarkerListImpl&) = delete;

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

  // TextMatchMarkerListImpl-specific
  Vector<gfx::Rect> LayoutRects(const Node&) const;
  // Returns true if markers within a range defined by |startOffset| and
  // |endOffset| are found.
  bool SetTextMatchMarkersActive(unsigned start_offset,
                                 unsigned end_offset,
                                 bool);

 private:
  HeapVector<Member<DocumentMarker>> markers_;
};

template <>
struct DowncastTraits<TextMatchMarkerListImpl> {
  static bool AllowFrom(const DocumentMarkerList& list) {
    return list.MarkerType() == DocumentMarker::kTextMatch;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_TEXT_MATCH_MARKER_LIST_IMPL_H_
