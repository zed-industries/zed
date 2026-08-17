// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_SORTED_DOCUMENT_MARKER_LIST_EDITOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_SORTED_DOCUMENT_MARKER_LIST_EDITOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/markers/document_marker_list.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"

namespace blink {

class DocumentMarker;

class CORE_EXPORT SortedDocumentMarkerListEditor final {
 public:
  using MarkerList = HeapVector<Member<DocumentMarker>>;

  static void AddMarkerWithoutMergingOverlapping(MarkerList*, DocumentMarker*);

  // Returns true if a marker was moved, false otherwise.
  static bool MoveMarkers(MarkerList* src_list,
                          int length,
                          DocumentMarkerList* dst_list);

  // Returns true if a marker was removed, false otherwise.
  static bool RemoveMarkers(MarkerList*, unsigned start_offset, int length);

  // The following two methods both update the position of a list's
  // DocumentMarkers in response to editing operations. The difference is that
  // if an editing operation actually changes the text spanned by a marker (as
  // opposed to only changing text before or after the marker),
  // ShiftMarkersContentDependent will remove the marker, and
  // ShiftMarkersContentIndependent will attempt to keep tracking the marked
  // region across edits.

  // Returns true if a marker was shifted or removed, false otherwise.
  static bool ShiftMarkersContentDependent(MarkerList*,
                                           unsigned offset,
                                           unsigned old_length,
                                           unsigned new_length);

  // Returns true if a marker was shifted or removed, false otherwise.
  static bool ShiftMarkersContentIndependent(MarkerList*,
                                             unsigned offset,
                                             unsigned old_length,
                                             unsigned new_length);

  // Returns the first marker in the specified MarkerList whose interior
  // overlaps overlap with the range [start_offset, end_offset], or null if
  // there is no such marker.
  static DocumentMarker* FirstMarkerIntersectingRange(const MarkerList&,
                                                      unsigned start_offset,
                                                      unsigned end_offset);

  static HeapVector<Member<DocumentMarker>> MarkersIntersectingRange(
      const MarkerList&,
      unsigned start_offset,
      unsigned end_offset);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_SORTED_DOCUMENT_MARKER_LIST_EDITOR_H_
