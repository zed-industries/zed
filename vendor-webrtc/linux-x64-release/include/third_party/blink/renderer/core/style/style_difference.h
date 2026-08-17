// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_DIFFERENCE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_DIFFERENCE_H_

#include <algorithm>
#include <iosfwd>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class StyleDifference {
  STACK_ALLOCATED();

 public:
  enum PropertyDifference {
    kTransformPropertyChanged = 1 << 0,
    // Other transform properties include changes other than the transform
    // property itself such as individual transform properties, motion
    // path, etc. See: |ComputedStyle::HasTransform|.
    kOtherTransformPropertyChanged = 1 << 1,
    kOpacityChanged = 1 << 2,
    kZIndexChanged = 1 << 3,
    kFilterChanged = 1 << 4,
    kCSSClipChanged = 1 << 5,
    // The object needs to issue paint invalidations if it is affected by text
    // decorations or properties dependent on color (e.g., border or outline).
    // TextDecorationLine changes must also invalidate ink overflow.
    kTextDecorationOrColorChanged = 1 << 6,
    kBlendModeChanged = 1 << 7,
    kMaskChanged = 1 << 8,
    kBackgroundColorChanged = 1 << 9,
    kClipPathChanged = 1 << 10
    // If you add a value here, be sure to update kPropertyDifferenceCount.
  };

  StyleDifference()
      : paint_invalidation_type_(
            static_cast<unsigned>(PaintInvalidationType::kNone)),
        layout_type_(kNoLayout),
        needs_reshape_(false),
        recompute_visual_overflow_(false),
        property_specific_differences_(0),
        scroll_anchor_disabling_property_changed_(false),
        compositing_reasons_changed_(false),
        compositable_paint_effect_changed_(false),
        border_radius_changed_(false),
        transform_data_changed_(false) {}

  void Merge(StyleDifference other) {
    paint_invalidation_type_ =
        std::max(paint_invalidation_type_, other.paint_invalidation_type_);
    layout_type_ = std::max(layout_type_, other.layout_type_);
    needs_reshape_ |= other.needs_reshape_;
    recompute_visual_overflow_ |= other.recompute_visual_overflow_;
    property_specific_differences_ |= other.property_specific_differences_;
    scroll_anchor_disabling_property_changed_ |=
        other.scroll_anchor_disabling_property_changed_;
    compositing_reasons_changed_ |= other.compositing_reasons_changed_;
    compositable_paint_effect_changed_ |=
        other.compositable_paint_effect_changed_;
    border_radius_changed_ |= other.border_radius_changed_;
    transform_data_changed_ |= other.transform_data_changed_;
  }

  bool HasDifference() const {
    return (paint_invalidation_type_ !=
            static_cast<unsigned>(PaintInvalidationType::kNone)) ||
           layout_type_ || needs_reshape_ || property_specific_differences_ ||
           recompute_visual_overflow_ ||
           scroll_anchor_disabling_property_changed_ ||
           compositing_reasons_changed_ || compositable_paint_effect_changed_ ||
           border_radius_changed_ || transform_data_changed_;
  }

  // For simple paint invalidation, we can directly invalidate the
  // DisplayItemClients during style update, without paint invalidation during
  // PrePaintTreeWalk.
  bool NeedsSimplePaintInvalidation() const {
    return paint_invalidation_type_ ==
           static_cast<unsigned>(PaintInvalidationType::kSimple);
  }
  bool NeedsNormalPaintInvalidation() const {
    return paint_invalidation_type_ ==
           static_cast<unsigned>(PaintInvalidationType::kNormal);
  }
  void SetNeedsSimplePaintInvalidation() {
    DCHECK(!NeedsNormalPaintInvalidation());
    paint_invalidation_type_ =
        static_cast<unsigned>(PaintInvalidationType::kSimple);
  }
  void SetNeedsNormalPaintInvalidation() {
    paint_invalidation_type_ =
        static_cast<unsigned>(PaintInvalidationType::kNormal);
  }

  bool NeedsLayout() const { return layout_type_ != kNoLayout; }
  void ClearNeedsLayout() { layout_type_ = kNoLayout; }

  // The offset of this positioned object has been updated.
  bool NeedsPositionedMovementLayout() const {
    return layout_type_ == kPositionedMovement;
  }
  void SetNeedsPositionedMovementLayout() {
    DCHECK(!NeedsFullLayout());
    layout_type_ = kPositionedMovement;
  }

  bool NeedsFullLayout() const { return layout_type_ == kFullLayout; }
  void SetNeedsFullLayout() { layout_type_ = kFullLayout; }

  bool NeedsReshape() const { return needs_reshape_; }
  void SetNeedsReshape() { needs_reshape_ = true; }

  bool NeedsRecomputeVisualOverflow() const {
    return recompute_visual_overflow_;
  }
  void SetNeedsRecomputeVisualOverflow() { recompute_visual_overflow_ = true; }

  // True if the transform property itself changed, or properties related to
  // transform changed (e.g., individual transform properties, motion path,
  // etc.). See: |ComputedStyle::HasTransform|.
  bool TransformChanged() const {
    return property_specific_differences_ & kTransformPropertyChanged ||
           property_specific_differences_ & kOtherTransformPropertyChanged;
  }
  bool OtherTransformPropertyChanged() const {
    return property_specific_differences_ & kOtherTransformPropertyChanged;
  }
  void SetTransformPropertyChanged() {
    property_specific_differences_ |= kTransformPropertyChanged;
  }
  void SetOtherTransformPropertyChanged() {
    property_specific_differences_ |= kOtherTransformPropertyChanged;
  }

  bool OpacityChanged() const {
    return property_specific_differences_ & kOpacityChanged;
  }
  void SetOpacityChanged() {
    property_specific_differences_ |= kOpacityChanged;
  }

  bool ZIndexChanged() const {
    return property_specific_differences_ & kZIndexChanged;
  }
  void SetZIndexChanged() { property_specific_differences_ |= kZIndexChanged; }

  bool FilterChanged() const {
    return property_specific_differences_ & kFilterChanged;
  }
  void SetFilterChanged() { property_specific_differences_ |= kFilterChanged; }

  bool CssClipChanged() const {
    return property_specific_differences_ & kCSSClipChanged;
  }
  void SetCSSClipChanged() {
    property_specific_differences_ |= kCSSClipChanged;
  }

  bool BlendModeChanged() const {
    return property_specific_differences_ & kBlendModeChanged;
  }
  void SetBlendModeChanged() {
    property_specific_differences_ |= kBlendModeChanged;
  }

  bool TextDecorationOrColorChanged() const {
    return property_specific_differences_ & kTextDecorationOrColorChanged;
  }
  void SetTextDecorationOrColorChanged() {
    property_specific_differences_ |= kTextDecorationOrColorChanged;
  }

  bool MaskChanged() const {
    return property_specific_differences_ & kMaskChanged;
  }
  void SetMaskChanged() { property_specific_differences_ |= kMaskChanged; }

  bool BackgroundColorChanged() const {
    return property_specific_differences_ & kBackgroundColorChanged;
  }
  void SetBackgroundColorChanged() {
    property_specific_differences_ |= kBackgroundColorChanged;
  }

  bool ClipPathChanged() const {
    return property_specific_differences_ & kClipPathChanged;
  }
  void SetClipPathChanged() {
    property_specific_differences_ |= kClipPathChanged;
  }

  bool ScrollAnchorDisablingPropertyChanged() const {
    return scroll_anchor_disabling_property_changed_;
  }
  void SetScrollAnchorDisablingPropertyChanged() {
    scroll_anchor_disabling_property_changed_ = true;
  }
  bool CompositingReasonsChanged() const {
    return compositing_reasons_changed_;
  }
  void SetCompositingReasonsChanged() { compositing_reasons_changed_ = true; }
  bool CompositablePaintEffectChanged() const {
    return compositable_paint_effect_changed_;
  }
  void SetCompositablePaintEffectChanged() {
    compositable_paint_effect_changed_ = true;
  }
  bool BorderRadiusChanged() const { return border_radius_changed_; }
  void SetBorderRadiusChanged() { border_radius_changed_ = true; }
  bool TransformDataChanged() const { return transform_data_changed_; }
  void SetTransformDataChanged() { transform_data_changed_ = true; }

 private:
  static constexpr int kPropertyDifferenceCount = 11;

  friend CORE_EXPORT std::ostream& operator<<(std::ostream&,
                                              const StyleDifference&);

  enum class PaintInvalidationType { kNone, kSimple, kNormal };
  unsigned paint_invalidation_type_ : 2;

  enum LayoutType { kNoLayout = 0, kPositionedMovement, kFullLayout };
  unsigned layout_type_ : 2;
  unsigned needs_reshape_ : 1;
  unsigned recompute_visual_overflow_ : 1;
  unsigned property_specific_differences_ : kPropertyDifferenceCount;
  unsigned scroll_anchor_disabling_property_changed_ : 1;
  unsigned compositing_reasons_changed_ : 1;
  // Designed for the effects such as background-color, whose animation can be
  // composited using paint worklet infra.
  unsigned compositable_paint_effect_changed_ : 1;
  unsigned border_radius_changed_ : 1;
  unsigned transform_data_changed_ : 1;

  // This exists only to get the object up to exactly 32 bits,
  // which keeps Clang from making partial writes of it when copying
  // (making two small writes to the stack and then reading the same
  // data back again with a large read can cause store-to-load forward
  // stalls). Feel free to take bits from here if you need them
  // for something else.
  unsigned padding_ [[maybe_unused]] : 10;
};
static_assert(sizeof(StyleDifference) == 4, "Remove some padding bits!");

CORE_EXPORT std::ostream& operator<<(std::ostream&, const StyleDifference&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_DIFFERENCE_H_
