// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_PAINT_CHUNK_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_PAINT_CHUNK_H_

#include <iosfwd>
#include <memory>
#include <optional>

#include "base/check_op.h"
#include "base/dcheck_is_on.h"
#include "cc/input/hit_test_opaqueness.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/graphics/paint/display_item_client.h"
#include "third_party/blink/renderer/platform/graphics/paint/drawing_display_item.h"
#include "third_party/blink/renderer/platform/graphics/paint/hit_test_data.h"
#include "third_party/blink/renderer/platform/graphics/paint/layer_selection_data.h"
#include "third_party/blink/renderer/platform/graphics/paint/property_tree_state.h"
#include "third_party/blink/renderer/platform/graphics/paint/raster_invalidation_tracking.h"
#include "third_party/blink/renderer/platform/graphics/paint/region_capture_data.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/skia/include/core/SkColor.h"
#include "ui/gfx/geometry/rect.h"

namespace blink {

constexpr float kMinBackgroundColorCoverageRatio = 0.5;

// A contiguous sequence of drawings with common paint properties.
//
// This is expected to be owned by the paint artifact which also owns the
// related drawings.
struct PLATFORM_EXPORT PaintChunk {
  DISALLOW_NEW();

  using Id = DisplayItem::Id;
  PaintChunk(wtf_size_t begin,
             wtf_size_t end,
             const DisplayItemClient& client,
             const Id& id,
             const PropertyTreeStateOrAlias& properties,
             bool effectively_invisible = false)
      : begin_index(begin),
        end_index(end),
        id(id),
        properties(properties),
        text_known_to_be_on_opaque_background(true),
        has_text(false),
        is_cacheable(client.IsCacheable()),
        client_is_just_created(client.IsJustCreated()),
        is_moved_from_cached_subsequence(false),
        effectively_invisible(effectively_invisible) {}

  // Move a paint chunk from a cached subsequence.
  PaintChunk(wtf_size_t begin, PaintChunk&& other)
      : begin_index(begin),
        end_index(begin + other.size()),
        id(other.id),
        background_color(other.background_color),
        properties(other.properties),
        hit_test_data(std::move(other.hit_test_data)),
        region_capture_data(std::move(other.region_capture_data)),
        layer_selection_data(std::move(other.layer_selection_data)),
        bounds(other.bounds),
        drawable_bounds(other.drawable_bounds),
        rect_known_to_be_opaque(other.rect_known_to_be_opaque),
        raster_effect_outset(other.raster_effect_outset),
        hit_test_opaqueness(other.hit_test_opaqueness),
        text_known_to_be_on_opaque_background(
            other.text_known_to_be_on_opaque_background),
        has_text(other.has_text),
        is_cacheable(other.is_cacheable),
        client_is_just_created(false),
        is_moved_from_cached_subsequence(true),
        effectively_invisible(other.effectively_invisible) {}

  void Trace(Visitor* visitor) const {
    visitor->Trace(properties);
    visitor->Trace(hit_test_data);
    visitor->Trace(region_capture_data);
    visitor->Trace(layer_selection_data);
  }

  wtf_size_t size() const {
    DCHECK_GE(end_index, begin_index);
    return end_index - begin_index;
  }

  // Check if a new PaintChunk (this) created in the latest paint matches an old
  // PaintChunk created in the previous paint.
  bool Matches(const PaintChunk& old) const {
    return old.is_cacheable && Matches(old.id);
  }

  bool CanMatchOldChunk() const {
    if (!is_cacheable)
      return false;
    // A chunk whose client is just created should not match any cached chunk,
    // even if it's id equals the old chunk's id (which may happen if this
    // chunk's client is just created at the same address of the old chunk's
    // deleted client).
    return !client_is_just_created;
  }

  bool Matches(const Id& other_id) const {
    return CanMatchOldChunk() && id == other_id;
  }

  bool EqualsForUnderInvalidationChecking(const PaintChunk& other) const;

  HitTestData& EnsureHitTestData() {
    if (!hit_test_data) {
      hit_test_data = MakeGarbageCollected<HitTestData>();
    }
    return *hit_test_data;
  }

  LayerSelectionData& EnsureLayerSelectionData() {
    if (!layer_selection_data) {
      layer_selection_data = MakeGarbageCollected<LayerSelectionData>();
    }
    return *layer_selection_data;
  }

  bool DrawsContent() const {
    return !effectively_invisible && !drawable_bounds.IsEmpty();
  }

  size_t MemoryUsageInBytes() const;

  // The no-argument version is for operator<< which is used in DCHECK and unit
  // tests. It doesn't output the debug name of the client.
  String ToString(bool concise = false) const;
  String ToString(const PaintArtifact& paint_artifact,
                  bool concise = false) const;

  // Index of the first drawing in this chunk.
  wtf_size_t begin_index;

  // Index of the first drawing not in this chunk, so that there are
  // |endIndex - beginIndex| drawings in the chunk.
  wtf_size_t end_index;

  // Identifier of this chunk. It should be unique if |is_cacheable| is true.
  // This is used to match a new chunk to a cached old chunk to track changes
  // of chunk contents, so the id should be stable across document cycles.
  Id id;

  // Color to use for checkerboarding, derived from display item's in this
  // chunk; or Color::kTransparent if no such display item exists.
  using BackgroundColorInfo = DrawingDisplayItem::BackgroundColorInfo;
  BackgroundColorInfo background_color;

  // The paint properties which apply to this chunk.
  TraceablePropertyTreeStateOrAlias properties;

  Member<HitTestData> hit_test_data;
  Member<RegionCaptureData> region_capture_data;
  Member<LayerSelectionData> layer_selection_data;

  // The following fields depend on the display items in this chunk.
  // They are updated when a display item is added into the chunk.

  // The total bounds of visual rects of all display items in this paint chunk,
  // and extra bounds that are not from display items for e.g. hit test.
  // It's in the coordinate space of the containing transform node. This can be
  // larger than |drawble_bounds|, because of non-drawable display items and
  // extra bounds.
  gfx::Rect bounds;

  // The total bounds of visual rects of drawable display items in this paint
  // chunk.
  gfx::Rect drawable_bounds;

  gfx::Rect rect_known_to_be_opaque;

  // Some raster effects can exceed |bounds| in the rasterization space. This
  // is the maximum DisplayItemClient::VisualRectOutsetForRasterEffects() of
  // all clients of items in this chunk.
  RasterEffectOutset raster_effect_outset = RasterEffectOutset::kNone;

  cc::HitTestOpaqueness hit_test_opaqueness =
      cc::HitTestOpaqueness::kTransparent;

  // True if all text is known to be on top of opaque backgrounds or there is
  // not text. Though in theory the value doesn't matter when there is no text,
  // being true can simplify code.
  bool text_known_to_be_on_opaque_background : 1;
  bool has_text : 1;

  // End of derived data.
  // The following fields are put here to avoid memory gap.
  bool is_cacheable : 1;
  bool client_is_just_created : 1;
  bool is_moved_from_cached_subsequence : 1;
  bool effectively_invisible : 1;
};

using PaintChunks = HeapVector<PaintChunk>;

PLATFORM_EXPORT std::ostream& operator<<(std::ostream&, const PaintChunk&);

static_assert(std::is_trivially_destructible_v<PaintChunk>);

}  // namespace blink

WTF_ALLOW_MOVE_AND_INIT_WITH_MEM_FUNCTIONS(blink::PaintChunk)

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_PAINT_CHUNK_H_
