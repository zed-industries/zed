// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_TRANSFORM_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_TRANSFORM_HELPER_H_

#include "third_party/blink/renderer/core/style/computed_style.h"
#include "third_party/blink/renderer/platform/transforms/affine_transform.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace gfx {
class RectF;
}

namespace blink {

class SVGElement;
class LayoutObject;

class TransformHelper {
  STATIC_ONLY(TransformHelper);

 public:
  static void UpdateOffsetPath(SVGElement&, const ComputedStyle*);
  // Returns true if the passed in ComputedStyle has a transform that needs to
  // resolve against the reference box.
  static bool DependsOnReferenceBox(const ComputedStyle&);

  // Check if ComputedStyle has changed in a way that could be reflected in the
  // transform reference box.
  static bool CheckReferenceBoxDependencies(const ComputedStyle& old_style,
                                            const ComputedStyle& style);

  // Computes the reference box for the LayoutObject based on the
  // 'transform-box'. Applies zoom if needed.
  static gfx::RectF ComputeReferenceBox(const LayoutObject&);

  // Compute the transform for the LayoutObject based on the various
  // 'transform*' properties.
  static AffineTransform ComputeTransform(UseCounter&,
                                          const ComputedStyle&,
                                          const gfx::RectF& reference_box,
                                          ComputedStyle::ApplyTransformOrigin);

  // Compute the transform for the SVGElement (which is assumed to have an
  // associated LayoutObject) based on the various 'transform*' properties,
  // using the specified reference box. Will also include a motion transform
  // (from <animateMotion>) if one has been specified.
  static AffineTransform ComputeTransformIncludingMotion(
      const SVGElement& element,
      const gfx::RectF& reference_box);

  // Like the above, but also computes the reference box.
  static AffineTransform ComputeTransformIncludingMotion(
      const SVGElement& element);

  static gfx::PointF ComputeTransformOrigin(const ComputedStyle&,
                                            const gfx::RectF& reference_box);
};

// The following enumeration is used to optimize cases where the scale is known
// to be invariant (see: LayoutSVGContainer::UpdateSVGLayout and
// LayoutSVGRoot). The value 'Full' can be used in the general case when the
// scale change is unknown, or known to have changed.
enum class SVGTransformChange {
  kNone,
  kScaleInvariant,
  kFull,
};

// Helper for computing ("classifying") a change to a transform using the
// categories defined above.
class SVGTransformChangeDetector {
  STACK_ALLOCATED();

 public:
  explicit SVGTransformChangeDetector(const AffineTransform& previous)
      : previous_transform_(previous) {}

  SVGTransformChange ComputeChange(const AffineTransform& current) {
    if (previous_transform_ == current)
      return SVGTransformChange::kNone;
    if (ScaleReference(previous_transform_) == ScaleReference(current))
      return SVGTransformChange::kScaleInvariant;
    return SVGTransformChange::kFull;
  }

 private:
  static std::pair<double, double> ScaleReference(
      const AffineTransform& transform) {
    return std::make_pair(transform.XScaleSquared(), transform.YScaleSquared());
  }
  AffineTransform previous_transform_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_TRANSFORM_HELPER_H_
