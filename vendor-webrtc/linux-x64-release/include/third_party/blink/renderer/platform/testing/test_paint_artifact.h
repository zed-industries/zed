// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_TEST_PAINT_ARTIFACT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_TEST_PAINT_ARTIFACT_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/graphics/paint/display_item_list.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_artifact.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/testing/fake_display_item_client.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace cc {
class Layer;
}

namespace blink {

class ClipPaintPropertyNodeOrAlias;
class EffectPaintPropertyNodeOrAlias;
class PaintArtifact;
class TransformPaintPropertyNodeOrAlias;

// Useful for quickly making a paint artifact in unit tests.
//
// Usage:
//   auto artifact = TestPaintArtifact().Chunk(0).Chunk(1).Build();
//   DoSomethingWithArtifact(artifact);
//  or
//   DoSomethingWithArtifact(TestPaintArtifact().Chunk(0).Chunk(1).Build());
//
class TestPaintArtifact {
  STACK_ALLOCATED();

 public:
  // Add a chunk to the artifact. Each chunk will have a different automatically
  // created client.
  TestPaintArtifact& Chunk() { return Chunk(NewClient()); }

  // Add a chunk with the specified client.
  TestPaintArtifact& Chunk(const DisplayItemClient&,
                           DisplayItem::Type = DisplayItem::kDrawingFirst);

  // This is for RasterInvalidatorTest, to create a chunk with specific id and
  // bounds calculated with a function from the id.
  TestPaintArtifact& Chunk(int id);

  TestPaintArtifact& Properties(const PropertyTreeStateOrAlias&);
  TestPaintArtifact& Properties(
      const TransformPaintPropertyNodeOrAlias& transform,
      const ClipPaintPropertyNodeOrAlias& clip,
      const EffectPaintPropertyNodeOrAlias& effect) {
    return Properties(PropertyTreeStateOrAlias(transform, clip, effect));
  }

  // Shorthands of Chunk().Properties(...).
  TestPaintArtifact& Chunk(const TransformPaintPropertyNodeOrAlias& transform,
                           const ClipPaintPropertyNodeOrAlias& clip,
                           const EffectPaintPropertyNodeOrAlias& effect) {
    return Chunk().Properties(transform, clip, effect);
  }
  TestPaintArtifact& Chunk(const PropertyTreeStateOrAlias& properties) {
    return Chunk().Properties(properties);
  }

  TestPaintArtifact& ScrollHitTestChunk(
      const DisplayItemClient&,
      const PropertyTreeState& contents_state);
  TestPaintArtifact& ScrollHitTestChunk(
      const PropertyTreeState& contents_state) {
    return ScrollHitTestChunk(NewClient(), contents_state);
  }

  TestPaintArtifact& ScrollingContentsChunk(const DisplayItemClient&,
                                            const PropertyTreeState& state,
                                            bool opaque = false);
  TestPaintArtifact& ScrollingContentsChunk(const PropertyTreeState& state,
                                            bool opaque = false) {
    return ScrollingContentsChunk(NewClient(), state, opaque);
  }

  TestPaintArtifact& ScrollChunks(const PropertyTreeState& contents_state,
                                  bool contents_opaque = false);

  // Add display item in the chunk. Each display item will have a different
  // automatically created client.
  TestPaintArtifact& RectDrawing(const gfx::Rect& bounds, Color color);

  TestPaintArtifact& ForeignLayerChunk(scoped_refptr<cc::Layer> layer,
                                       const gfx::Point& origin);

  // Add display item with the specified client in the chunk.
  TestPaintArtifact& RectDrawing(const DisplayItemClient&,
                                 const gfx::Rect& bounds,
                                 Color color);

  // Sets fake bounds for the last paint chunk. Note that the bounds will be
  // overwritten when the PaintArtifact is constructed if the chunk has any
  // display items. Bounds() sets both bounds and drawable_bounds, while
  // DrawableBounds() sets drawable_bounds only.
  TestPaintArtifact& Bounds(const gfx::Rect&);
  TestPaintArtifact& DrawableBounds(const gfx::Rect&);

  TestPaintArtifact& SetRasterEffectOutset(RasterEffectOutset);
  TestPaintArtifact& RectKnownToBeOpaque(const gfx::Rect&);
  TestPaintArtifact& TextKnownToBeOnOpaqueBackground();
  TestPaintArtifact& HasText();
  TestPaintArtifact& IsSolidColor();
  TestPaintArtifact& EffectivelyInvisible();
  TestPaintArtifact& Uncacheable();
  TestPaintArtifact& IsMovedFromCachedSubsequence();

  const PaintArtifact& Build();

  // Create a new display item client.
  FakeDisplayItemClient& NewClient();

  FakeDisplayItemClient& Client(wtf_size_t) const;

 private:
  void DidAddDisplayItem();

  HeapVector<Member<FakeDisplayItemClient>> clients_;
  Persistent<PaintArtifact> paint_artifact_ =
      MakeGarbageCollected<PaintArtifact>();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_TEST_PAINT_ARTIFACT_H_
