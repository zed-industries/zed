/*
 * Copyright (C) 2004, 2005, 2008 Nikolas Zimmermann <zimmermann@kde.org>
 * Copyright (C) 2004, 2005, 2006, 2007 Rob Buis <buis@kde.org>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_PRESERVE_ASPECT_RATIO_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_PRESERVE_ASPECT_RATIO_H_

#include "third_party/blink/renderer/core/svg/properties/svg_property.h"
#include "third_party/blink/renderer/core/svg/svg_parsing_error.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace gfx {
class RectF;
class SizeF;
}  // namespace gfx

namespace blink {

class AffineTransform;
class SVGPreserveAspectRatioTearOff;

class SVGPreserveAspectRatio final : public SVGPropertyBase {
 public:
  enum SVGPreserveAspectRatioType {
    kSvgPreserveaspectratioUnknown = 0,
    kSvgPreserveaspectratioNone = 1,
    kSvgPreserveaspectratioXminymin = 2,
    kSvgPreserveaspectratioXmidymin = 3,
    kSvgPreserveaspectratioXmaxymin = 4,
    kSvgPreserveaspectratioXminymid = 5,
    kSvgPreserveaspectratioXmidymid = 6,
    kSvgPreserveaspectratioXmaxymid = 7,
    kSvgPreserveaspectratioXminymax = 8,
    kSvgPreserveaspectratioXmidymax = 9,
    kSvgPreserveaspectratioXmaxymax = 10
  };

  enum SVGMeetOrSliceType {
    kSvgMeetorsliceUnknown = 0,
    kSvgMeetorsliceMeet = 1,
    kSvgMeetorsliceSlice = 2
  };

  typedef SVGPreserveAspectRatioTearOff TearOffType;

  SVGPreserveAspectRatio();

  SVGPreserveAspectRatio* Clone() const;

  bool operator==(const SVGPreserveAspectRatio&) const;
  bool operator!=(const SVGPreserveAspectRatio& other) const {
    return !operator==(other);
  }

  void SetAlign(SVGPreserveAspectRatioType align) { align_ = align; }
  SVGPreserveAspectRatioType Align() const { return align_; }

  void SetMeetOrSlice(SVGMeetOrSliceType meet_or_slice) {
    meet_or_slice_ = meet_or_slice;
  }
  SVGMeetOrSliceType MeetOrSlice() const { return meet_or_slice_; }

  void TransformRect(gfx::RectF& dest_rect, gfx::RectF& src_rect) const;

  AffineTransform ComputeTransform(const gfx::RectF& view_box,
                                   const gfx::SizeF& viewport_size) const;

  String ValueAsString() const override;
  SVGParsingError SetValueAsString(const String&);
  bool Parse(const UChar*& ptr, const UChar* end, bool validate);
  bool Parse(const LChar*& ptr, const LChar* end, bool validate);

  void Add(const SVGPropertyBase*, const SVGElement*) override;
  void CalculateAnimatedValue(
      const SMILAnimationEffectParameters&,
      float percentage,
      unsigned repeat_count,
      const SVGPropertyBase* from,
      const SVGPropertyBase* to,
      const SVGPropertyBase* to_at_end_of_duration_value,
      const SVGElement* context_element) override;
  float CalculateDistance(const SVGPropertyBase* to,
                          const SVGElement* context_element) const override;

  static AnimatedPropertyType ClassType() {
    return kAnimatedPreserveAspectRatio;
  }
  AnimatedPropertyType GetType() const override { return ClassType(); }

  void SetDefault();

 private:
  template <typename CharType>
  SVGParsingError ParseInternal(const CharType*& ptr,
                                const CharType* end,
                                bool validate);

  SVGPreserveAspectRatioType align_;
  SVGMeetOrSliceType meet_or_slice_;
};

template <>
struct DowncastTraits<SVGPreserveAspectRatio> {
  static bool AllowFrom(const SVGPropertyBase& value) {
    return value.GetType() == SVGPreserveAspectRatio::ClassType();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_PRESERVE_ASPECT_RATIO_H_
