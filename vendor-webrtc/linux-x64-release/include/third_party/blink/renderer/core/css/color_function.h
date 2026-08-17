// Copyright 2024 The Chromium Authors Use of this source code is governed by a
// BSD-style license that can be found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_COLOR_FUNCTION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_COLOR_FUNCTION_H_

#include "base/containers/fixed_flat_map.h"
#include "third_party/blink/renderer/core/css_value_keywords.h"
#include "third_party/blink/renderer/platform/graphics/color.h"

namespace blink {

class ColorFunction {
 public:
  struct Metadata {
    // The name/binding for positional color channels 0, 1 and 2.
    std::array<CSSValueID, 3> channel_name;

    // The value (number) that equals 100% for the corresponding positional
    // color channel.
    std::array<double, 3> channel_percentage;
  };

  // Unique entries in kMetadataMap.
  enum class MetadataEntry : uint8_t {
    kLegacyRgb,  // Color::ColorSpace::kSRGBLegacy
    kColorRgb,   // Color::ColorSpace::kSRGB,
                 // Color::ColorSpace::kSRGBLinear,
                 // Color::ColorSpace::kDisplayP3, Color::ColorSpace::kA98RGB,
                 // Color::ColorSpace::kProPhotoRGB, Color::ColorSpace::kRec2020
    kColorXyz,   // Color::ColorSpace::kXYZD50,
                 // Color::ColorSpace::kXYZD65
    kLab,        // Color::ColorSpace::kLab
    kOkLab,      // Color::ColorSpace::kOklab
    kLch,        // Color::ColorSpace::kLch
    kOkLch,      // Color::ColorSpace::kOklch
    kHsl,        // Color::ColorSpace::kHSL
    kHwb,        // Color::ColorSpace::kHWB
  };

  static constexpr double kPercentNotApplicable =
      std::numeric_limits<double>::quiet_NaN();

  static constexpr auto kMetadataMap =
      base::MakeFixedFlatMap<MetadataEntry, Metadata>({
          // rgb(); percentage mapping: r,g,b=255
          {MetadataEntry::kLegacyRgb,
           {{CSSValueID::kR, CSSValueID::kG, CSSValueID::kB}, {255, 255, 255}}},

          // color(... <predefined-rgb-params> ...); percentage mapping: r,g,b=1
          {MetadataEntry::kColorRgb,
           {{CSSValueID::kR, CSSValueID::kG, CSSValueID::kB}, {1, 1, 1}}},

          // color(... <xyz-params> ...); percentage mapping: x,y,z=1
          {MetadataEntry::kColorXyz,
           {{CSSValueID::kX, CSSValueID::kY, CSSValueID::kZ}, {1, 1, 1}}},

          // lab(); percentage mapping: l=100 a,b=125
          {MetadataEntry::kLab,
           {{CSSValueID::kL, CSSValueID::kA, CSSValueID::kB}, {100, 125, 125}}},

          // oklab(); percentage mapping: l=1 a,b=0.4
          {MetadataEntry::kOkLab,
           {{CSSValueID::kL, CSSValueID::kA, CSSValueID::kB}, {1, 0.4, 0.4}}},

          // lch(); percentage mapping: l=100 c=150 h=n/a
          {MetadataEntry::kLch,
           {{CSSValueID::kL, CSSValueID::kC, CSSValueID::kH},
            {100, 150, kPercentNotApplicable}}},

          // oklch(); percentage mapping: l=1 c=0.4 h=n/a
          {MetadataEntry::kOkLch,
           {{CSSValueID::kL, CSSValueID::kC, CSSValueID::kH},
            {1, 0.4, kPercentNotApplicable}}},

          // hsl(); percentage mapping: h=n/a s,l=100
          {MetadataEntry::kHsl,
           {{CSSValueID::kH, CSSValueID::kS, CSSValueID::kL},
            {kPercentNotApplicable, 100, 100}}},

          // hwb(); percentage mapping: h=n/a w,b=100
          {MetadataEntry::kHwb,
           {{CSSValueID::kH, CSSValueID::kW, CSSValueID::kB},
            {kPercentNotApplicable, 100, 100}}},
      });

  static constexpr auto kColorSpaceMap =
      base::MakeFixedFlatMap<Color::ColorSpace, MetadataEntry>({
          {Color::ColorSpace::kSRGBLegacy, MetadataEntry::kLegacyRgb},
          {Color::ColorSpace::kSRGB, MetadataEntry::kColorRgb},
          {Color::ColorSpace::kSRGBLinear, MetadataEntry::kColorRgb},
          {Color::ColorSpace::kDisplayP3, MetadataEntry::kColorRgb},
          {Color::ColorSpace::kA98RGB, MetadataEntry::kColorRgb},
          {Color::ColorSpace::kProPhotoRGB, MetadataEntry::kColorRgb},
          {Color::ColorSpace::kRec2020, MetadataEntry::kColorRgb},
          {Color::ColorSpace::kXYZD50, MetadataEntry::kColorXyz},
          {Color::ColorSpace::kXYZD65, MetadataEntry::kColorXyz},
          {Color::ColorSpace::kLab, MetadataEntry::kLab},
          {Color::ColorSpace::kOklab, MetadataEntry::kOkLab},
          {Color::ColorSpace::kLch, MetadataEntry::kLch},
          {Color::ColorSpace::kOklch, MetadataEntry::kOkLch},
          {Color::ColorSpace::kHSL, MetadataEntry::kHsl},
          {Color::ColorSpace::kHWB, MetadataEntry::kHwb},
      });

  static const Metadata& MetadataForColorSpace(Color::ColorSpace color_space) {
    auto function_entry = kColorSpaceMap.find(color_space);
    CHECK(function_entry != ColorFunction::kColorSpaceMap.end());
    auto function_metadata_entry = kMetadataMap.find(function_entry->second);
    CHECK(function_metadata_entry != kMetadataMap.end());
    return function_metadata_entry->second;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_COLOR_FUNCTION_H_
