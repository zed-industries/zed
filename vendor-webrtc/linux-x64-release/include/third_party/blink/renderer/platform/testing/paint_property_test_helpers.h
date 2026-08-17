// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_PAINT_PROPERTY_TEST_HELPERS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_PAINT_PROPERTY_TEST_HELPERS_H_

#include "third_party/blink/renderer/platform/graphics/paint/clip_paint_property_node.h"
#include "third_party/blink/renderer/platform/graphics/paint/effect_paint_property_node.h"
#include "third_party/blink/renderer/platform/graphics/paint/property_tree_state.h"
#include "third_party/blink/renderer/platform/graphics/paint/scroll_paint_property_node.h"
#include "third_party/blink/renderer/platform/graphics/paint/transform_paint_property_node.h"
#include "ui/gfx/geometry/rect_conversions.h"
#include "ui/gfx/geometry/transform.h"

namespace blink {

inline gfx::Transform MakeScaleMatrix(double tx, double ty, double tz = 1) {
  gfx::Transform t;
  t.Scale3d(tx, ty, tz);
  return t;
}

inline gfx::Transform MakeScaleMatrix(double s) {
  return MakeScaleMatrix(s, s, 1);
}

inline gfx::Transform MakeTranslationMatrix(double tx,
                                            double ty,
                                            double tz = 0) {
  gfx::Transform t;
  t.Translate3d(tx, ty, tz);
  return t;
}

inline gfx::Transform MakeRotationMatrix(double degrees) {
  gfx::Transform t;
  t.Rotate(degrees);
  return t;
}

inline gfx::Transform MakeRotationMatrix(double degrees_x,
                                         double degrees_y,
                                         double degrees_z) {
  gfx::Transform t;
  t.RotateAboutZAxis(degrees_z);
  t.RotateAboutYAxis(degrees_y);
  t.RotateAboutXAxis(degrees_x);
  return t;
}

// Convenient shorthands.
inline const TransformPaintPropertyNode& t0() {
  return TransformPaintPropertyNode::Root();
}
inline const ClipPaintPropertyNode& c0() {
  return ClipPaintPropertyNode::Root();
}
inline const EffectPaintPropertyNode& e0() {
  return EffectPaintPropertyNode::Root();
}
inline const ScrollPaintPropertyNode& s0() {
  return ScrollPaintPropertyNode::Root();
}

constexpr int c0_id = 1;
constexpr int e0_id = 1;
constexpr int t0_id = 1;

inline EffectPaintPropertyNode* CreateOpacityEffect(
    const EffectPaintPropertyNodeOrAlias& parent,
    const TransformPaintPropertyNodeOrAlias& local_transform_space,
    const ClipPaintPropertyNodeOrAlias* output_clip,
    float opacity,
    CompositingReasons compositing_reasons = CompositingReason::kNone) {
  EffectPaintPropertyNode::State state;
  state.local_transform_space = &local_transform_space;
  state.output_clip = output_clip;
  state.opacity = opacity;
  state.direct_compositing_reasons = compositing_reasons;
  state.compositor_element_id = CompositorElementIdFromUniqueObjectId(
      NewUniqueObjectId(), CompositorElementIdNamespace::kPrimary);
  return EffectPaintPropertyNode::Create(parent, std::move(state));
}

inline EffectPaintPropertyNode* CreateOpacityEffect(
    const EffectPaintPropertyNodeOrAlias& parent,
    float opacity,
    CompositingReasons compositing_reasons = CompositingReason::kNone) {
  return CreateOpacityEffect(parent, parent.Unalias().LocalTransformSpace(),
                             parent.Unalias().OutputClip(), opacity,
                             compositing_reasons);
}

inline EffectPaintPropertyNode* CreateAnimatingOpacityEffect(
    const EffectPaintPropertyNodeOrAlias& parent,
    float opacity = 1.f,
    const ClipPaintPropertyNodeOrAlias* output_clip = nullptr) {
  EffectPaintPropertyNode::State state;
  state.local_transform_space = &parent.Unalias().LocalTransformSpace();
  state.output_clip = output_clip;
  state.opacity = opacity;
  state.direct_compositing_reasons = CompositingReason::kActiveOpacityAnimation;
  state.compositor_element_id = CompositorElementIdFromUniqueObjectId(
      NewUniqueObjectId(), CompositorElementIdNamespace::kPrimaryEffect);
  return EffectPaintPropertyNode::Create(parent, std::move(state));
}

inline EffectPaintPropertyNode* CreateFilterEffect(
    const EffectPaintPropertyNodeOrAlias& parent,
    const TransformPaintPropertyNodeOrAlias& local_transform_space,
    const ClipPaintPropertyNodeOrAlias* output_clip,
    CompositorFilterOperations filter,
    CompositingReasons compositing_reasons = CompositingReason::kNone) {
  EffectPaintPropertyNode::State state;
  state.local_transform_space = &local_transform_space;
  state.output_clip = output_clip;
  state.filter_info = std::make_unique<EffectPaintPropertyNode::FilterInfo>(
      filter, filter.MapRect(gfx::ToEnclosingRect(filter.ReferenceBox())));
  state.direct_compositing_reasons = compositing_reasons;
  state.compositor_element_id = CompositorElementIdFromUniqueObjectId(
      NewUniqueObjectId(), CompositorElementIdNamespace::kEffectFilter);
  return EffectPaintPropertyNode::Create(parent, std::move(state));
}

inline EffectPaintPropertyNode* CreateFilterEffect(
    const EffectPaintPropertyNodeOrAlias& parent,
    CompositorFilterOperations filter,
    CompositingReasons compositing_reasons = CompositingReason::kNone) {
  return CreateFilterEffect(parent, parent.Unalias().LocalTransformSpace(),
                            parent.Unalias().OutputClip(), filter,
                            compositing_reasons);
}

inline EffectPaintPropertyNode* CreateAnimatingFilterEffect(
    const EffectPaintPropertyNodeOrAlias& parent,
    CompositorFilterOperations filter = CompositorFilterOperations(),
    const ClipPaintPropertyNodeOrAlias* output_clip = nullptr) {
  EffectPaintPropertyNode::State state;
  state.local_transform_space = &parent.Unalias().LocalTransformSpace();
  state.output_clip = output_clip;
  state.filter_info = std::make_unique<EffectPaintPropertyNode::FilterInfo>(
      filter, filter.MapRect(gfx::ToEnclosingRect(filter.ReferenceBox())));
  state.direct_compositing_reasons = CompositingReason::kActiveFilterAnimation;
  state.compositor_element_id = CompositorElementIdFromUniqueObjectId(
      NewUniqueObjectId(), CompositorElementIdNamespace::kEffectFilter);
  return EffectPaintPropertyNode::Create(parent, std::move(state));
}

inline EffectPaintPropertyNode* CreateBackdropFilterEffect(
    const EffectPaintPropertyNodeOrAlias& parent,
    const TransformPaintPropertyNodeOrAlias& local_transform_space,
    const ClipPaintPropertyNodeOrAlias* output_clip,
    CompositorFilterOperations backdrop_filter,
    float opacity = 1.0f,
    CompositingReasons compositing_reasons =
        CompositingReason::kBackdropFilter) {
  EffectPaintPropertyNode::State state;
  state.local_transform_space = &local_transform_space;
  state.output_clip = output_clip;
  if (!backdrop_filter.IsEmpty()) {
    state.backdrop_filter_info =
        base::WrapUnique(new EffectPaintPropertyNode::BackdropFilterInfo{
            std::move(backdrop_filter)});
  }
  state.direct_compositing_reasons = compositing_reasons;
  state.compositor_element_id = CompositorElementIdFromUniqueObjectId(
      NewUniqueObjectId(), CompositorElementIdNamespace::kPrimary);
  state.opacity = opacity;
  return EffectPaintPropertyNode::Create(parent, std::move(state));
}

inline EffectPaintPropertyNode* CreateBackdropFilterEffect(
    const EffectPaintPropertyNodeOrAlias& parent,
    CompositorFilterOperations backdrop_filter,
    CompositingReasons compositing_reasons =
        CompositingReason::kBackdropFilter) {
  return CreateBackdropFilterEffect(parent,
                                    parent.Unalias().LocalTransformSpace(),
                                    parent.Unalias().OutputClip(),
                                    backdrop_filter, 1.0f, compositing_reasons);
}

inline EffectPaintPropertyNode* CreateAnimatingBackdropFilterEffect(
    const EffectPaintPropertyNodeOrAlias& parent,
    CompositorFilterOperations backdrop_filter = CompositorFilterOperations(),
    const ClipPaintPropertyNodeOrAlias* output_clip = nullptr) {
  EffectPaintPropertyNode::State state;
  state.local_transform_space = &parent.Unalias().LocalTransformSpace();
  state.output_clip = output_clip;
  if (!backdrop_filter.IsEmpty()) {
    state.backdrop_filter_info =
        base::WrapUnique(new EffectPaintPropertyNode::BackdropFilterInfo{
            std::move(backdrop_filter)});
  }
  state.direct_compositing_reasons =
      CompositingReason::kActiveBackdropFilterAnimation;
  state.compositor_element_id = CompositorElementIdFromUniqueObjectId(
      NewUniqueObjectId(), CompositorElementIdNamespace::kPrimaryEffect);
  return EffectPaintPropertyNode::Create(parent, std::move(state));
}

inline ClipPaintPropertyNode* CreateClip(
    const ClipPaintPropertyNodeOrAlias& parent,
    const TransformPaintPropertyNodeOrAlias& local_transform_space,
    const gfx::RectF& layout_clip_rect,
    const FloatRoundedRect& paint_clip_rect) {
  ClipPaintPropertyNode::State state(local_transform_space, layout_clip_rect,
                                     paint_clip_rect);
  return ClipPaintPropertyNode::Create(parent, std::move(state));
}

inline ClipPaintPropertyNode* CreateClip(
    const ClipPaintPropertyNodeOrAlias& parent,
    const TransformPaintPropertyNodeOrAlias& local_transform_space,
    const FloatRoundedRect& clip_rect) {
  return CreateClip(parent, local_transform_space, clip_rect.Rect(), clip_rect);
}

inline ClipPaintPropertyNode* CreatePixelMovingFilterClipExpander(
    const ClipPaintPropertyNodeOrAlias& parent,
    const EffectPaintPropertyNode& pixel_moving_filter) {
  ClipPaintPropertyNode::State state(pixel_moving_filter.LocalTransformSpace(),
                                     &pixel_moving_filter);
  return ClipPaintPropertyNode::Create(parent, std::move(state));
}

inline void UpdateClip(ClipPaintPropertyNode& clip,
                       const gfx::RectF& layout_clip_rect,
                       const FloatRoundedRect& paint_clip_rect) {
  clip.Update(*clip.Parent(),
              ClipPaintPropertyNode::State(clip.LocalTransformSpace(),
                                           layout_clip_rect, paint_clip_rect));
}

inline void UpdateClip(ClipPaintPropertyNode& clip,
                       const FloatRoundedRect& clip_rect) {
  UpdateClip(clip, clip_rect.Rect(), clip_rect);
}

inline ClipPaintPropertyNode* CreateClipPathClip(
    const ClipPaintPropertyNodeOrAlias& parent,
    const TransformPaintPropertyNodeOrAlias& local_transform_space,
    const FloatRoundedRect& clip_rect) {
  ClipPaintPropertyNode::State state(local_transform_space, clip_rect.Rect(),
                                     clip_rect);
  state.clip_path = Path();
  return ClipPaintPropertyNode::Create(parent, std::move(state));
}

inline TransformPaintPropertyNode* Create2DTranslation(
    const TransformPaintPropertyNodeOrAlias& parent,
    float x,
    float y) {
  return TransformPaintPropertyNode::Create(
      parent, TransformPaintPropertyNode::State{
                  {gfx::Transform::MakeTranslation(x, y)}});
}

inline TransformPaintPropertyNode* CreateFixedPositionTranslation(
    const TransformPaintPropertyNodeOrAlias& parent,
    float offset_x,
    float offset_y,
    const TransformPaintPropertyNode& scroll_translation_for_fixed) {
  TransformPaintPropertyNode::State state{
      {gfx::Transform::MakeTranslation(offset_x, offset_y)}};
  state.scroll_translation_for_fixed = &scroll_translation_for_fixed;
  state.direct_compositing_reasons = CompositingReason::kFixedPosition;
  return TransformPaintPropertyNode::Create(parent, std::move(state));
}

inline TransformPaintPropertyNode* CreateTransform(
    const TransformPaintPropertyNodeOrAlias& parent,
    const gfx::Transform& matrix,
    const gfx::Point3F& origin = gfx::Point3F(),
    CompositingReasons compositing_reasons = CompositingReason::kNone) {
  TransformPaintPropertyNode::State state{{matrix, origin}};
  state.direct_compositing_reasons = compositing_reasons;
  return TransformPaintPropertyNode::Create(parent, std::move(state));
}

inline TransformPaintPropertyNode* CreateAnimatingTransform(
    const TransformPaintPropertyNodeOrAlias& parent,
    const gfx::Transform& matrix = gfx::Transform(),
    const gfx::Point3F& origin = gfx::Point3F()) {
  TransformPaintPropertyNode::State state{{matrix, origin}};
  state.direct_compositing_reasons =
      CompositingReason::kActiveTransformAnimation;
  state.compositor_element_id = CompositorElementIdFromUniqueObjectId(
      NewUniqueObjectId(), CompositorElementIdNamespace::kPrimaryTransform);
  return TransformPaintPropertyNode::Create(parent, std::move(state));
}

inline TransformPaintPropertyNode* CreateScrollTranslation(
    const TransformPaintPropertyNodeOrAlias& parent,
    float offset_x,
    float offset_y,
    const ScrollPaintPropertyNode& scroll,
    CompositingReasons compositing_reasons = CompositingReason::kNone) {
  TransformPaintPropertyNode::State state{
      {gfx::Transform::MakeTranslation(offset_x, offset_y)}};
  state.direct_compositing_reasons = compositing_reasons;
  state.scroll = &scroll;
  return TransformPaintPropertyNode::Create(parent, std::move(state));
}

inline TransformPaintPropertyNode* CreateScrollTranslation(
    const TransformPaintPropertyNodeOrAlias& parent_transform,
    const ScrollPaintPropertyNode& parent_scroll,
    float offset_x,
    float offset_y,
    const gfx::Rect& container_rect,
    const gfx::Size& contents_size,
    const ClipPaintPropertyNode* overflow_clip,
    CompositingReasons compositing_reasons = CompositingReason::kNone,
    MainThreadScrollingReasons main_thread_reasons =
        cc::MainThreadScrollingReason::kNotOpaqueForTextAndLCDText) {
  ScrollPaintPropertyNode::State scroll_state;
  scroll_state.container_rect = container_rect;
  scroll_state.contents_size = contents_size;
  scroll_state.overflow_clip_node = overflow_clip;
  scroll_state.compositor_element_id = CompositorElementIdFromUniqueObjectId(
      NewUniqueObjectId(), CompositorElementIdNamespace::kScroll);
  scroll_state.main_thread_repaint_reasons = main_thread_reasons;
  scroll_state.user_scrollable_horizontal = true;
  scroll_state.user_scrollable_vertical = true;
  TransformPaintPropertyNode::State translation_state{
      {gfx::Transform::MakeTranslation(offset_x, offset_y)}};
  translation_state.direct_compositing_reasons = compositing_reasons;
  translation_state.scroll =
      ScrollPaintPropertyNode::Create(parent_scroll, std::move(scroll_state));
  return TransformPaintPropertyNode::Create(parent_transform,
                                            std::move(translation_state));
}

inline const ScrollPaintPropertyNode& DefaultParentScroll(
    const TransformPaintPropertyNodeOrAlias& parent_transform) {
  return *parent_transform.Unalias()
              .NearestScrollTranslationNode()
              .ScrollNode();
}

inline TransformPaintPropertyNode* CreateCompositedScrollTranslation(
    const TransformPaintPropertyNodeOrAlias& parent_transform,
    float offset_x,
    float offset_y,
    const ScrollPaintPropertyNode& scroll) {
  return CreateScrollTranslation(parent_transform, offset_x, offset_y, scroll,
                                 CompositingReason::kOverflowScrolling);
}

inline TransformPaintPropertyNode* CreateCompositedScrollTranslation(
    const TransformPaintPropertyNodeOrAlias& parent_transform,
    const ScrollPaintPropertyNode& parent_scroll,
    float offset_x,
    float offset_y,
    const gfx::Rect& container_rect,
    const gfx::Size& contents_size,
    const ClipPaintPropertyNode* overflow_clip,
    MainThreadScrollingReasons main_thread_reasons =
        cc::MainThreadScrollingReason::kNotScrollingOnMain) {
  return CreateScrollTranslation(
      parent_transform, parent_scroll, offset_x, offset_y, container_rect,
      contents_size, overflow_clip, CompositingReason::kOverflowScrolling,
      main_thread_reasons);
}

inline PropertyTreeState CreateScrollTranslationState(
    const PropertyTreeState& parent_state,
    const ScrollPaintPropertyNode& parent_scroll,
    float offset_x,
    float offset_y,
    const gfx::Rect& container_rect,
    const gfx::Size& contents_size,
    CompositingReasons compositing_reasons = CompositingReason::kNone,
    MainThreadScrollingReasons main_thread_reasons =
        cc::MainThreadScrollingReason::kNotOpaqueForTextAndLCDText) {
  auto* clip = CreateClip(parent_state.Clip(), parent_state.Transform(),
                          FloatRoundedRect(container_rect));
  auto* transform =
      CreateScrollTranslation(parent_state.Transform(), parent_scroll, offset_x,
                              offset_y, container_rect, contents_size, clip,
                              compositing_reasons, main_thread_reasons);
  return PropertyTreeState(*transform, *clip, parent_state.Effect());
}

inline PropertyTreeState CreateScrollTranslationState(
    const PropertyTreeState& parent_state,
    float offset_x,
    float offset_y,
    const gfx::Rect& container_rect,
    const gfx::Size& contents_size,
    CompositingReasons compositing_reasons = CompositingReason::kNone,
    MainThreadScrollingReasons main_thread_reasons =
        cc::MainThreadScrollingReason::kNotOpaqueForTextAndLCDText) {
  return CreateScrollTranslationState(
      parent_state, DefaultParentScroll(parent_state.Transform()), offset_x,
      offset_y, container_rect, contents_size, compositing_reasons,
      main_thread_reasons);
}

inline PropertyTreeState CreateCompositedScrollTranslationState(
    const PropertyTreeState& parent_state,
    const ScrollPaintPropertyNode& parent_scroll,
    float offset_x,
    float offset_y,
    const gfx::Rect& container_rect,
    const gfx::Size& contents_size,
    MainThreadScrollingReasons main_thread_reasons =
        cc::MainThreadScrollingReason::kNotScrollingOnMain) {
  return CreateScrollTranslationState(parent_state, parent_scroll, offset_x,
                                      offset_y, container_rect, contents_size,
                                      CompositingReason::kOverflowScrolling,
                                      main_thread_reasons);
}

inline PropertyTreeState CreateCompositedScrollTranslationState(
    const PropertyTreeState& parent_state,
    float offset_x,
    float offset_y,
    const gfx::Rect& container_rect,
    const gfx::Size& contents_size,
    MainThreadScrollingReasons main_thread_reasons =
        cc::MainThreadScrollingReason::kNotScrollingOnMain) {
  return CreateScrollTranslationState(
      parent_state, DefaultParentScroll(parent_state.Transform()), offset_x,
      offset_y, container_rect, contents_size,
      CompositingReason::kOverflowScrolling, main_thread_reasons);
}

inline PropertyTreeState DefaultPaintChunkProperties() {
  return PropertyTreeState::Root();
}

// Checked downcast from *PaintPropertyNodeOrAlias to *PaintPropertyNode.
// This is used in tests that expect the node to be an unaliased node.
inline const ClipPaintPropertyNode& ToUnaliased(
    const ClipPaintPropertyNodeOrAlias& node) {
  DCHECK(!node.IsParentAlias());
  return static_cast<const ClipPaintPropertyNode&>(node);
}
inline const EffectPaintPropertyNode& ToUnaliased(
    const EffectPaintPropertyNodeOrAlias& node) {
  DCHECK(!node.IsParentAlias());
  return static_cast<const EffectPaintPropertyNode&>(node);
}
inline const TransformPaintPropertyNode& ToUnaliased(
    const TransformPaintPropertyNodeOrAlias& node) {
  DCHECK(!node.IsParentAlias());
  return static_cast<const TransformPaintPropertyNode&>(node);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_PAINT_PROPERTY_TEST_HELPERS_H_
