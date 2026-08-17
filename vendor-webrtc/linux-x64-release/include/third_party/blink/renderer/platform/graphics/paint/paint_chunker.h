// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_PAINT_CHUNKER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_PAINT_CHUNKER_H_

#include <optional>

#include "base/dcheck_is_on.h"
#include "base/memory/stack_allocated.h"
#include "cc/input/hit_test_opaqueness.h"
#include "cc/input/layer_selection_bound.h"
#include "third_party/blink/renderer/platform/graphics/paint/display_item.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_artifact.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_chunk.h"
#include "third_party/blink/renderer/platform/graphics/paint/property_tree_state.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

// Accepts information about changes to chunk properties as drawings are
// accumulated, and produces a series of paint chunks: contiguous ranges of the
// display list with identical properties. Finish() must be called at end of
// painting to ensure the last chunk is complete.
class PLATFORM_EXPORT PaintChunker final {
  STACK_ALLOCATED();

 public:
  // If `clients_to_validate` is given, it will contain DisplayItemClients of
  // newly created paint chunks on finish.
  explicit PaintChunker(PaintChunks& chunks,
                        HeapVector<Member<const DisplayItemClient>>*
                            clients_to_validate = nullptr)
      : chunks_(chunks), clients_to_validate_(clients_to_validate) {
    DCHECK(chunks.empty());
  }
  PaintChunker(const PaintChunker&) = delete;
  PaintChunker& operator=(const PaintChunker&) = delete;

  // Finishes current chunks if any. New function calls are disallowed.
  void Finish();

  void MarkClientForValidation(const DisplayItemClient& client);

  const PropertyTreeStateOrAlias& CurrentPaintChunkProperties() const {
    return current_properties_;
  }
  void UpdateCurrentPaintChunkProperties(const PropertyTreeStateOrAlias&);
  void UpdateCurrentPaintChunkProperties(const PaintChunk::Id&,
                                         const DisplayItemClient&,
                                         const PropertyTreeStateOrAlias&);

  // Sets the forcing new chunk status on. When the status is on, even the
  // properties haven't change, we'll force a new paint chunk for the next
  // display item and then automatically resets the status. Some special display
  // item (e.g. ForeignLayerDisplayItem) also automatically sets the status on
  // before and after the item to force a dedicated paint chunk.
  void SetWillForceNewChunk() {
    will_force_new_chunk_ = true;
    next_chunk_id_ = std::nullopt;
  }
  bool WillForceNewChunkForTesting() const { return will_force_new_chunk_; }

  void AppendByMoving(PaintChunk&&);

  // Returns true if a new chunk is created.
  bool IncrementDisplayItemIndex(const DisplayItemClient&, const DisplayItem&);

  // The id will be used when we need to create a new current chunk.
  // Otherwise it's ignored. Returns true if a new chunk is added.
  bool AddHitTestDataToCurrentChunk(const PaintChunk::Id&,
                                    const DisplayItemClient&,
                                    const gfx::Rect&,
                                    TouchAction,
                                    bool blocking_wheel,
                                    cc::HitTestOpaqueness);

  bool CurrentChunkIsNonEmptyAndTransparentToHitTest() const;

  void CreateScrollHitTestChunk(
      const PaintChunk::Id&,
      const DisplayItemClient&,
      const TransformPaintPropertyNode* scroll_translation,
      const gfx::Rect& scroll_hit_test_rect,
      cc::HitTestOpaqueness,
      const gfx::Rect& scrolling_contents_cull_rect);

  // The id will be used when we need to create a new current chunk.
  // Otherwise it's ignored. Returns true if a new chunk is added.
  bool AddRegionCaptureDataToCurrentChunk(const PaintChunk::Id& id,
                                          const DisplayItemClient& client,
                                          const RegionCaptureCropId& crop_id,
                                          const gfx::Rect& bounds);

  // The id will be used when we need to create a new current chunk.
  // Otherwise it's ignored. Returns true if a new chunk is added.
  void AddSelectionToCurrentChunk(std::optional<PaintedSelectionBound> start,
                                  std::optional<PaintedSelectionBound> end,
                                  String debug_info);
  void RecordAnySelectionWasPainted();

  // Returns true if a new chunk is created.
  bool EnsureChunk() {
    return EnsureCurrentChunk(next_chunk_id_->first, next_chunk_id_->second);
  }

  bool CurrentEffectivelyInvisible() const {
    return current_effectively_invisible_;
  }
  void SetCurrentEffectivelyInvisible(bool invisible) {
    current_effectively_invisible_ = invisible;
  }

 private:
  // Returns true if a new chunk is created.
  bool EnsureCurrentChunk(const PaintChunk::Id&, const DisplayItemClient&);
  bool WillCreateNewChunk() const;

  void UnionBounds(const gfx::Rect&, cc::HitTestOpaqueness);

  void ProcessBackgroundColorCandidate(const DisplayItem&);

  void FinalizeLastChunkProperties();

  void CheckNotFinished() const {
#if DCHECK_IS_ON()
    DCHECK(!finished_);
#endif
  }

  PaintChunks& chunks_;
  HeapVector<Member<const DisplayItemClient>>* const clients_to_validate_;

  // The id specified by UpdateCurrentPaintChunkProperties(). If it is not
  // nullopt, we will use it as the id of the next new chunk. Otherwise we will
  // use the id of the first display item of the new chunk as the id.
  // It's cleared when we create a new chunk with the id, or decide not to
  // create a chunk with it (e.g. when properties don't change and we are not
  // forced to create a new chunk).
  using NextChunkId = std::pair<PaintChunk::Id, const DisplayItemClient&>;
  std::optional<NextChunkId> next_chunk_id_;

  PropertyTreeStateOrAlias current_properties_{
      PropertyTreeStateOrAlias::kUninitialized};

  // True when an item forces a new chunk (e.g., foreign display items), and for
  // the item following a forced chunk. PaintController also forces new chunks
  // before and after subsequences by calling ForceNewChunk().
  bool will_force_new_chunk_ = true;

  bool current_effectively_invisible_ = false;

#if DCHECK_IS_ON()
  bool finished_ = false;
#endif
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_PAINT_CHUNKER_H_
