// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_GEOMETRY_MAPPER_CLIP_CACHE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_GEOMETRY_MAPPER_CLIP_CACHE_H_

#include "third_party/blink/renderer/platform/graphics/overlay_scrollbar_clip_behavior.h"
#include "third_party/blink/renderer/platform/graphics/paint/float_clip_rect.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class ClipPaintPropertyNode;
class FloatClipRect;
class TransformPaintPropertyNode;

// A GeometryMapperClipCache hangs off a ClipPaintPropertyNode. It stores
// cached "clip visual rects" (See GeometryMapper.h) from that node in
// ancestor spaces.
class PLATFORM_EXPORT GeometryMapperClipCache
    : public GarbageCollected<GeometryMapperClipCache> {
 public:
  GeometryMapperClipCache() = default;
  GeometryMapperClipCache(const GeometryMapperClipCache&) = delete;
  GeometryMapperClipCache& operator=(const GeometryMapperClipCache&) = delete;

  struct ClipAndTransform {
    DISALLOW_NEW();

   public:
    Member<const ClipPaintPropertyNode> ancestor_clip;
    Member<const TransformPaintPropertyNode> ancestor_transform;
    OverlayScrollbarClipBehavior clip_behavior;
    bool operator==(const ClipAndTransform& other) const = default;
    ClipAndTransform(const ClipPaintPropertyNode* ancestor_clip_arg,
                     const TransformPaintPropertyNode* ancestor_transform_arg,
                     OverlayScrollbarClipBehavior clip_behavior_arg)
        : ancestor_clip(ancestor_clip_arg),
          ancestor_transform(ancestor_transform_arg),
          clip_behavior(clip_behavior_arg) {
      DCHECK(ancestor_clip);
      DCHECK(ancestor_transform);
    }

    void Trace(Visitor*) const;
  };

  void UpdateIfNeeded(const ClipPaintPropertyNode& node) {
    if (cache_generation_ != s_global_generation_)
      Update(node);
    DCHECK_EQ(cache_generation_, s_global_generation_);
  }

  struct ClipCacheEntry {
    DISALLOW_NEW();

   public:
    const ClipAndTransform clip_and_transform;
    // The clip visual rect of the associated clip node in the space of
    // |clip_and_transform|.
    const FloatClipRect clip_rect;

    // Whether there is any transform animation between the transform space
    // of the associated clip node (inclusive) and |clip_and_transform|
    // (exclusive).
    const bool has_transform_animation = false;
    // Similarly, for sticky transform.
    const bool has_sticky_transform = false;

    void Trace(Visitor* visitor) const { visitor->Trace(clip_and_transform); }
  };

  void Trace(Visitor*) const;

  // Returns the clip visual rect  of the owning clip of |this| in the space of
  // |ancestors|, if there is one cached. Otherwise returns null.
  const ClipCacheEntry* GetCachedClip(const ClipAndTransform& ancestors);

  // Stores cached the "clip visual rect" of |this| in the space of |ancestors|,
  // into a local cache.
  void SetCachedClip(const ClipCacheEntry&);

  static void ClearCache();
  bool IsValid() const;

  const ClipPaintPropertyNode* NearestPixelMovingFilterClip() const {
    return nearest_pixel_moving_filter_clip_;
  }

 private:
  void Update(const ClipPaintPropertyNode&);

  HeapVector<ClipCacheEntry> clip_cache_;
  // The nearest ancestor that has non-null PixelMovingFilter().
  Member<const ClipPaintPropertyNode> nearest_pixel_moving_filter_clip_;

  unsigned cache_generation_ = s_global_generation_ - 1;
  static unsigned s_global_generation_;
};

}  // namespace blink

WTF_ALLOW_MOVE_AND_INIT_WITH_MEM_FUNCTIONS(
    blink::GeometryMapperClipCache::ClipCacheEntry)

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_GEOMETRY_MAPPER_CLIP_CACHE_H_
