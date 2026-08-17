// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_CLIP_PAINT_PROPERTY_NODE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_CLIP_PAINT_PROPERTY_NODE_H_

#include <algorithm>
#include <optional>

#include "base/check_op.h"
#include "third_party/blink/renderer/platform/geometry/float_rounded_rect.h"
#include "third_party/blink/renderer/platform/geometry/path.h"
#include "third_party/blink/renderer/platform/graphics/paint/float_clip_rect.h"
#include "third_party/blink/renderer/platform/graphics/paint/geometry_mapper_clip_cache.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_property_node.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

class EffectPaintPropertyNode;
class GeometryMapperClipCache;
class PropertyTreeState;
class TransformPaintPropertyNodeOrAlias;

// A clip rect created by a css property such as "overflow" or "clip".
// Along with a reference to the transform space the clip rect is based on,
// and a parent ClipPaintPropertyNode for inherited clips.
//
// The clip tree is rooted at a node with no parent. This root node should
// not be modified.
class ClipPaintPropertyNode;

class PLATFORM_EXPORT ClipPaintPropertyNodeOrAlias
    : public PaintPropertyNodeBase<ClipPaintPropertyNodeOrAlias,
                                   ClipPaintPropertyNode> {
 public:
  // Checks if the accumulated clip from |this| to |relative_to_state.Clip()|
  // has changed, at least significance of |change|, in the space of
  // |relative_to_state.Transform()|. We check for changes of not only clip
  // nodes, but also LocalTransformSpace relative to |relative_to_state
  // .Transform()| of the clip nodes. |transform_not_to_check| specifies a
  // transform node that the caller has checked or will check its change in
  // other ways and this function should treat it as unchanged.
  bool Changed(
      PaintPropertyChangeType change,
      const PropertyTreeState& relative_to_state,
      const TransformPaintPropertyNodeOrAlias* transform_not_to_check) const;

  // See PaintPropertyNode::ChangedSequenceNumber().
  void ClearChangedToRoot(int sequence_number) const;

  void AddChanged(PaintPropertyChangeType changed) final {
    DCHECK_NE(PaintPropertyChangeType::kUnchanged, changed);
    GeometryMapperClipCache::ClearCache();
    PaintPropertyNodeBase::AddChanged(changed);
  }

 protected:
  using PaintPropertyNodeBase::PaintPropertyNodeBase;
};

class ClipPaintPropertyNodeAlias final : public ClipPaintPropertyNodeOrAlias {
 public:
  static ClipPaintPropertyNodeAlias* Create(
      const ClipPaintPropertyNodeOrAlias& parent) {
    return MakeGarbageCollected<ClipPaintPropertyNodeAlias>(kParentAlias,
                                                            parent);
  }

  // These are public required by MakeGarbageCollected, but the protected tags
  // prevent these from being called from outside.
  ClipPaintPropertyNodeAlias(ParentAliasTag,
                             const ClipPaintPropertyNodeOrAlias& parent)
      : ClipPaintPropertyNodeOrAlias(kParentAlias, parent) {}
};

class PLATFORM_EXPORT ClipPaintPropertyNode final
    : public ClipPaintPropertyNodeOrAlias {
 public:
  // To make it less verbose and more readable to construct and update a node,
  // a struct with default values is used to represent the state.
  struct PLATFORM_EXPORT State {
    DISALLOW_NEW();
   public:
    State(const TransformPaintPropertyNodeOrAlias& local_transform_space,
          const gfx::RectF& layout_clip_rect,
          const FloatRoundedRect& paint_clip_rect,
          bool requires_expanded_rect = false)
        : local_transform_space(&local_transform_space),
          requires_expanded_rect_(requires_expanded_rect) {
      SetClipRect(layout_clip_rect, paint_clip_rect);
    }
    State(const TransformPaintPropertyNodeOrAlias& local_transform_space,
          const EffectPaintPropertyNode* pixel_moving_filter)
        : local_transform_space(&local_transform_space),
          pixel_moving_filter(pixel_moving_filter),
          requires_expanded_rect_(false) {
      DCHECK(layout_clip_rect_.IsInfinite());
      paint_clip_rect_ = FloatRoundedRect(layout_clip_rect_.Rect());
    }

    Member<const TransformPaintPropertyNodeOrAlias> local_transform_space;
    std::optional<FloatClipRect> layout_clip_rect_excluding_overlay_scrollbars;
    std::optional<Path> clip_path;
    // If this is not nullptr, this clip node will generate a cc clip node to
    // expand clip rect for a pixel-moving filter.
    Member<const EffectPaintPropertyNode> pixel_moving_filter;

    void SetClipRect(const gfx::RectF& layout_clip_rect_arg,
                     const FloatRoundedRect& paint_clip_rect_arg) {
      layout_clip_rect_.SetRect(layout_clip_rect_arg);
      if (paint_clip_rect_arg.IsRounded())
        layout_clip_rect_.SetHasRadius();
      paint_clip_rect_ = paint_clip_rect_arg;
    }

    PaintPropertyChangeType ComputeChange(const State& other) const;

    bool ClipPathEquals(const std::optional<Path>& p) const {
      return (!clip_path && !p) || (clip_path && p && *clip_path == *p);
    }

    void Trace(Visitor*) const;

   private:
    friend class ClipPaintPropertyNode;
    bool requires_expanded_rect_;
    FloatClipRect layout_clip_rect_;
    FloatRoundedRect paint_clip_rect_;
  };

  // This node is really a sentinel, and does not represent a real clip space.
  static const ClipPaintPropertyNode& Root();

  static ClipPaintPropertyNode* Create(
      const ClipPaintPropertyNodeOrAlias& parent,
      State&& state) {
    return MakeGarbageCollected<ClipPaintPropertyNode>(kNonParentAlias, parent,
                                                       std::move(state));
  }

  static const FloatClipRect& ExpandedLayoutClipRect();
  static const FloatRoundedRect& ExpandedPaintClipRect();

  void Trace(Visitor* visitor) const final {
    ClipPaintPropertyNodeOrAlias::Trace(visitor);
    visitor->Trace(state_);
    visitor->Trace(clip_cache_);
  }

  // The empty AnimationState struct is to meet the requirement of
  // ObjectPaintProperties.
  struct AnimationState {
    STACK_ALLOCATED();
  };
  PaintPropertyChangeType Update(const ClipPaintPropertyNodeOrAlias& parent,
                                 State&& state,
                                 const AnimationState& = AnimationState()) {
    auto parent_changed = SetParent(parent);
    auto state_changed = state_.ComputeChange(state);
    if (state_changed != PaintPropertyChangeType::kUnchanged) {
      state_ = std::move(state);
      AddChanged(state_changed);
    }
    return std::max(parent_changed, state_changed);
  }

  const ClipPaintPropertyNode& Unalias() const = delete;
  bool IsParentAlias() const = delete;

  const TransformPaintPropertyNodeOrAlias& LocalTransformSpace() const {
    return *state_.local_transform_space;
  }
  // The clip rect for painting and compositing. It may be pixel snapped, or
  // not (e.g. for SVG).
  const FloatRoundedRect& PaintClipRect() const {
    if (state_.requires_expanded_rect_) {
      return ExpandedPaintClipRect();
    }
    return state_.paint_clip_rect_;
  }
  // The clip rect used for GeometryMapper to map in layout coordinates.
  const FloatClipRect& LayoutClipRect() const {
    if (state_.requires_expanded_rect_) {
      return ExpandedLayoutClipRect();
    }
    return state_.layout_clip_rect_;
  }
  // The clip rect used for GeometryMapper to map in layout coordinates,
  // ignoring cc clip paths
  const FloatClipRect& PreciseLayoutClipRect() const {
    return state_.layout_clip_rect_;
  }

  bool IsForCompositeClipPathAnimation() const {
    return state_.requires_expanded_rect_;
  }

  const FloatClipRect& LayoutClipRectExcludingOverlayScrollbars() const {
    return state_.layout_clip_rect_excluding_overlay_scrollbars
               ? *state_.layout_clip_rect_excluding_overlay_scrollbars
               : state_.layout_clip_rect_;
  }

  const std::optional<Path>& ClipPath() const { return state_.clip_path; }
  bool ClipPathEquals(const std::optional<Path>& p) const {
    return state_.ClipPathEquals(p);
  }

  const EffectPaintPropertyNode* PixelMovingFilter() const {
    return state_.pixel_moving_filter.Get();
  }

  const ClipPaintPropertyNode* NearestPixelMovingFilterClip() const {
    return GetClipCache().NearestPixelMovingFilterClip();
  }

  std::unique_ptr<JSONObject> ToJSON() const final;

  // These are public required by MakeGarbageCollected, but the protected tags
  // prevent these from being called from outside.
  explicit ClipPaintPropertyNode(RootTag);
  ClipPaintPropertyNode(NonParentAliasTag,
                        const ClipPaintPropertyNodeOrAlias& parent,
                        State&& state)
      : ClipPaintPropertyNodeOrAlias(kNonParentAlias, parent),
        state_(std::move(state)) {}

 private:
  // For access to GetClipCache();
  friend class GeometryMapper;
  friend class GeometryMapperClipCache;
  friend class GeometryMapperTest;

  GeometryMapperClipCache& GetClipCache() const {
    if (!clip_cache_) {
      clip_cache_ = MakeGarbageCollected<GeometryMapperClipCache>();
    }
    clip_cache_->UpdateIfNeeded(*this);
    return *clip_cache_;
  }

  State state_;
  mutable Member<GeometryMapperClipCache> clip_cache_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_CLIP_PAINT_PROPERTY_NODE_H_
