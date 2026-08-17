/*
 * Copyright (C) 2007 Eric Seidel <eric@webkit.org>
 * Copyright (C) 2008 Apple Inc. All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_ANIMATE_MOTION_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_ANIMATE_MOTION_ELEMENT_H_

#include "third_party/blink/renderer/core/svg/svg_animation_element.h"
#include "third_party/blink/renderer/platform/geometry/path.h"

namespace blink {

class SVGAnimateMotionElement final : public SVGAnimationElement {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit SVGAnimateMotionElement(Document&);
  ~SVGAnimateMotionElement() override;

  void ChildMPathChanged();

 private:
  bool HasValidAnimation() const override;
  void WillChangeAnimationTarget() override;
  void DidChangeAnimationTarget() override;

  void ParseAttribute(const AttributeModificationParams&) override;

  SMILAnimationValue CreateAnimationValue() const override;
  void ClearAnimationValue() override;
  bool CalculateToAtEndOfDurationValue(
      const String& to_at_end_of_duration_string) override;
  void CalculateFromAndToValues(const String& from_string,
                                const String& to_string) override;
  void CalculateFromAndByValues(const String& from_string,
                                const String& by_string) override;
  void CalculateAnimationValue(SMILAnimationValue&,
                               float percentage,
                               unsigned repeat_count) const override;
  void ApplyResultsToTarget(const SMILAnimationValue&) override;
  float CalculateDistance(const String& from_string,
                          const String& to_string) override;

  enum RotateMode { kRotateAngle, kRotateAuto, kRotateAutoReverse };
  RotateMode GetRotateMode() const;

  AnimationMode CalculateAnimationMode() override;
  void UpdateAnimationPath();

  // Note: we do not support percentage values for to/from coords as the spec
  // implies we should (opera doesn't either)
  gfx::PointF from_point_;
  gfx::PointF to_point_;
  gfx::PointF to_point_at_end_of_duration_;

  Path path_;
  Path animation_path_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_ANIMATE_MOTION_ELEMENT_H_
