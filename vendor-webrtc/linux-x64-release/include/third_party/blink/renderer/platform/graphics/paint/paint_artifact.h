// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_PAINT_ARTIFACT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_PAINT_ARTIFACT_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/platform/graphics/paint/display_item_list.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_chunk.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

// A PaintArtifact represents the output of painting, consisting of paint chunks
// and display items (in DisplayItemList).
//
// A PaintArtifact not only represents the output of the current painting, but
// also serves as cache of individual display items and paint chunks for later
// paintings as long as the display items and paint chunks are valid.
//
// It represents a particular state of the world, and is immutable (const) and
// promises to be in a reasonable state (e.g. chunk bounding boxes computed) to
// all users, except for PaintController and unit tests.
class PLATFORM_EXPORT PaintArtifact final
    : public GarbageCollected<PaintArtifact> {
 public:
  PaintArtifact() = default;
  PaintArtifact(const PaintArtifact& other) = delete;
  PaintArtifact& operator=(const PaintArtifact& other) = delete;
  PaintArtifact(PaintArtifact&& other) = delete;
  PaintArtifact& operator=(PaintArtifact&& other) = delete;

  void Trace(Visitor* visitor) const { visitor->Trace(chunks_); }

  bool IsEmpty() const { return chunks_.empty(); }

  DisplayItemList& GetDisplayItemList() { return display_item_list_; }
  const DisplayItemList& GetDisplayItemList() const {
    return display_item_list_;
  }

  PaintChunks& GetPaintChunks() { return chunks_; }
  const PaintChunks& GetPaintChunks() const { return chunks_; }

  DisplayItemRange DisplayItemsInChunk(wtf_size_t chunk_index) const {
    DCHECK_LT(chunk_index, chunks_.size());
    auto& chunk = chunks_[chunk_index];
    return UNSAFE_TODO(
        display_item_list_.ItemsInRange(chunk.begin_index, chunk.end_index));
  }

  // Returns the approximate memory usage, excluding memory likely to be
  // shared with the embedder after copying to cc::DisplayItemList.
  size_t ApproximateUnsharedMemoryUsage() const;

  PaintRecord GetPaintRecord(const PropertyTreeState& replay_state,
                             const gfx::Rect* cull_rect = nullptr) const;

  void RecordDebugInfo(DisplayItemClientId, const String&, DOMNodeId);
  // Note that ClientDebugName() returns the debug name at the time the client
  // was last painted, which may be out-of-date for a client whose debug name
  // has changed, but not in a way that caused it to be repainted.  This can
  // happen, for example, when the 'id' or 'class' attribute on a DOM element
  // changes, but the change doesn't cause a style invalidation.
  String ClientDebugName(DisplayItemClientId) const;
  DOMNodeId ClientOwnerNodeId(DisplayItemClientId) const;
  String IdAsString(const DisplayItem::Id& id) const;

  std::unique_ptr<JSONArray> ToJSON() const;
  void AppendChunksAsJSON(
      wtf_size_t start_chunk_index,
      wtf_size_t end_chunk_index,
      JSONArray&,
      DisplayItemList::JsonOption = DisplayItemList::kDefault) const;

  void clear();

 private:
  struct ClientDebugInfo {
    String name;
    DOMNodeId owner_node_id;
    DISALLOW_NEW();
  };

  using DebugInfo = HashMap<DisplayItemClientId, ClientDebugInfo>;

  DisplayItemList display_item_list_;
  PaintChunks chunks_;
  DebugInfo debug_info_;
};

PLATFORM_EXPORT std::ostream& operator<<(std::ostream&, const PaintArtifact&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_PAINT_ARTIFACT_H_
