/*
 * Copyright (C) 2004, 2005 Nikolas Zimmermann <zimmermann@kde.org>
 * Copyright (C) 2004, 2005, 2006 Rob Buis <buis@kde.org>
 * Copyright (C) 2007 Eric Seidel <eric@webkit.org>
 * Copyright (C) 2008 Apple Inc. All rights reserved.
 * Copyright (C) 2008 Cameron McCormack <cam@mcc.id.au>
 * Copyright (C) Research In Motion Limited 2011. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_ANIMATION_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_ANIMATION_ELEMENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/svg/animation/svg_smil_element.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "ui/gfx/geometry/cubic_bezier.h"

namespace blink {

class ExceptionState;

enum AnimationMode {
  kNoAnimation,
  kFromToAnimation,
  kFromByAnimation,
  kToAnimation,
  kByAnimation,
  kValuesAnimation,
  kPathAnimation  // Used by AnimateMotion.
};

enum CalcMode {
  kCalcModeDiscrete,
  kCalcModeLinear,
  kCalcModePaced,
  kCalcModeSpline
};

struct SMILAnimationEffectParameters;
struct SMILAnimationValue;

class CORE_EXPORT SVGAnimationElement : public SVGSMILElement {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // SVGAnimationElement
  float getStartTime(ExceptionState&) const;
  float getCurrentTime() const;
  float getSimpleDuration(ExceptionState&) const;

  void beginElement() { beginElementAt(0); }
  void beginElementAt(float offset);
  void endElement() { endElementAt(0); }
  void endElementAt(float offset);

  DEFINE_ATTRIBUTE_EVENT_LISTENER(begin, kBeginEvent)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(end, kEndEvent)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(repeat, kRepeatEvent)

  virtual SMILAnimationValue CreateAnimationValue() const = 0;
  void ApplyAnimation(SMILAnimationValue&);
  virtual void ApplyResultsToTarget(const SMILAnimationValue&) = 0;

  virtual void ClearAnimationValue() = 0;
  // Returns true if this animation "sets" the value of the animation. Thus all
  // previous animations are rendered useless.
  bool OverwritesUnderlyingAnimationValue() const;

  void Trace(Visitor* visitor) const override;

 protected:
  SVGAnimationElement(const QualifiedName&, Document&);

  SMILAnimationEffectParameters ComputeEffectParameters() const;

  void ParseAttribute(const AttributeModificationParams&) override;

  virtual AnimationMode CalculateAnimationMode();
  AnimationMode GetAnimationMode() const { return animation_mode_; }
  void SetCalcMode(CalcMode calc_mode) {
    use_paced_key_times_ = false;
    calc_mode_ = calc_mode;
  }
  CalcMode GetCalcMode() const { return calc_mode_; }

  virtual bool HasValidAnimation() const = 0;
  void UnregisterAnimation(const QualifiedName& attribute_name);
  void RegisterAnimation(const QualifiedName& attribute_name);

  // Parses a list of values as specified by SVG, stripping leading
  // and trailing whitespace, and places them in result. If the
  // format of the string is not valid, parseValues empties result
  // and returns false. See
  // http://www.w3.org/TR/SVG/animate.html#ValuesAttribute .
  static bool ParseValues(const String&, Vector<String>& result);

  void WillChangeAnimationTarget() override;
  void AnimationAttributeChanged();

 private:
  bool IsValid() const final { return SVGTests::IsValid(); }

  bool IsAdditive() const;
  bool IsAccumulated() const;

  String ToValue() const;
  String ByValue() const;
  String FromValue() const;

  bool UpdateAnimationParameters();
  bool CheckAnimationParameters() const;
  bool UpdateAnimationValues();

  virtual bool CalculateToAtEndOfDurationValue(
      const String& to_at_end_of_duration_string) = 0;
  virtual void CalculateFromAndToValues(const String& from_string,
                                        const String& to_string) = 0;
  virtual void CalculateFromAndByValues(const String& from_string,
                                        const String& by_string) = 0;
  virtual void CalculateAnimationValue(SMILAnimationValue&,
                                       float percent,
                                       unsigned repeat_count) const = 0;
  virtual float CalculateDistance(const String& /*fromString*/,
                                  const String& /*toString*/) {
    return -1.f;
  }

  float CurrentValuesForValuesAnimation(float percent,
                                        String& from,
                                        String& to) const;
  // Also decides which list is to be used, either key_times_from_attribute_
  // or key_times_for_paced_ by toggling the flag use_paced_key_times_.
  void CalculateKeyTimesForCalcModePaced();

  HeapVector<float> const& KeyTimes() const {
    return use_paced_key_times_ ? key_times_for_paced_
                                : key_times_from_attribute_;
  }

  float CalculatePercentFromKeyPoints(float percent) const;
  float CurrentValuesFromKeyPoints(float percent,
                                   String& from,
                                   String& to) const;
  float CalculatePercentForSpline(float percent, unsigned spline_index) const;
  float CalculatePercentForFromTo(float percent) const;
  unsigned CalculateKeyTimesIndex(float percent) const;

  void SetCalcMode(const AtomicString&);

  enum class AnimationValidity : unsigned char {
    kUnknown,
    kValid,
    kInvalid,
  };
  AnimationValidity animation_valid_;
  bool registered_animation_;
  bool use_paced_key_times_;

  Vector<String> values_;

  // FIXME: We should probably use doubles for this, but there's no point
  // making such a change unless all SVG logic for sampling animations is
  // changed to use doubles.

  // Storing two sets of values to avoid overwriting (or reparsing) when
  // calc-mode changes. Fix for Issue 231525. What list to use is
  // decided in CalculateKeyTimesForCalcModePaced.
  HeapVector<float> key_times_from_attribute_;
  HeapVector<float> key_times_for_paced_;

  HeapVector<float> key_points_;
  Vector<gfx::CubicBezier> key_splines_;
  String last_values_animation_from_;
  String last_values_animation_to_;
  CalcMode calc_mode_;
  AnimationMode animation_mode_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_ANIMATION_ELEMENT_H_
