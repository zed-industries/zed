// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_DOCUMENT_MARKER_GROUP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_DOCUMENT_MARKER_GROUP_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/text.h"
#include "third_party/blink/renderer/core/editing/markers/document_marker.h"
#include "third_party/blink/renderer/core/editing/position.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

// This class enables DocumentMarkers to span across Node boundaries by grouping
// together the DocumentMarkers that belong to the same reason the text is being
// marked and maps them with their respective TextNode.
class CORE_EXPORT DocumentMarkerGroup
    : public GarbageCollected<DocumentMarkerGroup> {
 public:
  DocumentMarkerGroup() = default;
  ~DocumentMarkerGroup() = default;

  void Set(const DocumentMarker* marker, const Text* text) {
    marker_text_map_.Set(marker, text);
  }
  void Erase(const DocumentMarker* marker) { marker_text_map_.erase(marker); }

  Position StartPosition() const;
  Position EndPosition() const;
  PositionInFlatTree StartPositionInFlatTree() const {
    return ToPositionInFlatTree(StartPosition());
  }
  PositionInFlatTree EndPositionInFlatTree() const {
    return ToPositionInFlatTree(EndPosition());
  }

  const DocumentMarker* GetMarkerForText(const Text* text) const;

  void Trace(Visitor*) const;

 private:
  HeapHashMap<WeakMember<const DocumentMarker>, WeakMember<const Text>>
      marker_text_map_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_MARKERS_DOCUMENT_MARKER_GROUP_H_
