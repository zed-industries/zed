// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_SUGGESTION_MARKER_LIST_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_SUGGESTION_MARKER_LIST_IMPL_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/markers/document_marker_list.h"
#include "third_party/blink/renderer/core/editing/markers/suggestion_marker.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

// Implementation of DocumentMarkerList for Suggestion markers. Suggestion
// markers are somewhat unusual compared to some other MarkerTypes in that they
// can overlap. Since sorting by start offset doesn't help very much when the
// markers can overlap, and other ways of doing this efficiently would add a lot
// of complexity, suggestion markers are currently stored unsorted.
class CORE_EXPORT SuggestionMarkerListImpl final : public DocumentMarkerList {
 public:
  SuggestionMarkerListImpl() = default;
  SuggestionMarkerListImpl(const SuggestionMarkerListImpl&) = delete;
  SuggestionMarkerListImpl& operator=(const SuggestionMarkerListImpl&) = delete;

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

  // SuggestionMarkerListImpl-specific
  bool RemoveMarkerByTag(int32_t tag);
  bool RemoveMarkerByType(const SuggestionMarker::SuggestionType& type);

 private:
  bool ShiftMarkersForSuggestionReplacement(unsigned offset,
                                            unsigned old_length,
                                            unsigned new_length);
  bool ShiftMarkersForNonSuggestionEditingOperation(const String& node_text,
                                                    unsigned offset,
                                                    unsigned old_length,
                                                    unsigned new_length);

  HeapVector<Member<DocumentMarker>> markers_;
};

template <>
struct DowncastTraits<SuggestionMarkerListImpl> {
  static bool AllowFrom(const DocumentMarkerList& list) {
    return list.MarkerType() == DocumentMarker::kSuggestion;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_SUGGESTION_MARKER_LIST_IMPL_H_
