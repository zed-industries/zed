// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_DOCUMENT_MARKER_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_DOCUMENT_MARKER_LIST_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/markers/document_marker.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class DocumentMarker;

// This is an interface implemented by classes that DocumentMarkerController
// uses to store DocumentMarkers. Different implementations can be written
// to handle different MarkerTypes (e.g. to provide more optimized handling of
// MarkerTypes with different insertion/retrieval patterns, or to provide
// different behavior for certain MarkerTypes).
class CORE_EXPORT DocumentMarkerList
    : public GarbageCollected<DocumentMarkerList> {
 public:
  DocumentMarkerList(const DocumentMarkerList&) = delete;
  DocumentMarkerList& operator=(const DocumentMarkerList&) = delete;
  virtual ~DocumentMarkerList();

  // Returns the single marker type supported by the list implementation.
  virtual DocumentMarker::MarkerType MarkerType() const = 0;

  virtual bool IsEmpty() const = 0;

  virtual void Add(DocumentMarker*) = 0;
  virtual void Clear() = 0;

  // Returns all markers
  virtual const HeapVector<Member<DocumentMarker>>& GetMarkers() const = 0;
  // Returns the first marker whose interior overlaps with the range
  // [start_offset, end_offset], or null if there is no such marker.
  virtual DocumentMarker* FirstMarkerIntersectingRange(
      unsigned start_offset,
      unsigned end_offset) const = 0;
  // Returns markers whose interiors have non-empty overlap with the range
  // [start_offset, end_offset]. Note that the range can be collapsed, in which
  // case markers containing the offset in their interiors are returned.
  virtual HeapVector<Member<DocumentMarker>> MarkersIntersectingRange(
      unsigned start_offset,
      unsigned end_offset) const = 0;

  // Returns true if at least one marker is copied, false otherwise
  virtual bool MoveMarkers(int length, DocumentMarkerList* dst_list) = 0;

  // Returns true if at least one marker is removed, false otherwise
  virtual bool RemoveMarkers(unsigned start_offset, int length) = 0;

  // Returns true if at least one marker is shifted or removed, false otherwise.
  // Called in response to an edit replacing the range
  // [offset, offset + old_length] by a string of length new_length.
  // node_text is the full text of the affected node *after* the edit is
  // applied.
  virtual bool ShiftMarkers(const String& node_text,
                            unsigned offset,
                            unsigned old_length,
                            unsigned new_length) = 0;

  // Update the marker list by merging any overlapping markers that should be
  // merged. Only Custom Highlights and Text Fragment markers need to merge.
  virtual void MergeOverlappingMarkers() = 0;

  virtual void Trace(Visitor* visitor) const {}

 protected:
  DocumentMarkerList();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_DOCUMENT_MARKER_LIST_H_
