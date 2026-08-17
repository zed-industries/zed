// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_HARFBUZZ_FONT_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_HARFBUZZ_FONT_DATA_H_

#include <hb-cplusplus.hh>

#include "base/check_op.h"
#include "third_party/blink/renderer/platform/fonts/font_platform_data.h"
#include "third_party/blink/renderer/platform/fonts/opentype/open_type_vertical_data.h"
#include "third_party/blink/renderer/platform/fonts/shaping/harfbuzz_face.h"
#include "third_party/skia/include/core/SkFont.h"

struct hb_font_t;

namespace blink {

const unsigned kInvalidFallbackMetricsValue = static_cast<unsigned>(-1);

// The HarfBuzzFontData struct carries user-pointer data for
// |hb_font_t| callback functions/operations. It contains metrics and OpenType
// layout information related to a font scaled to a particular size.
struct HarfBuzzFontData final : public GarbageCollected<HarfBuzzFontData> {
 public:
  explicit HarfBuzzFontData(hb_font_t* unscaled_font)
      : unscaled_font_(hb::unique_ptr<hb_font_t>(unscaled_font)),
        vertical_data_(nullptr),
        range_set_(nullptr) {}

  HarfBuzzFontData(const HarfBuzzFontData&) = delete;
  HarfBuzzFontData& operator=(const HarfBuzzFontData&) = delete;

  void Trace(Visitor* visitor) const {
    visitor->Trace(vertical_data_);
    visitor->Trace(range_set_);
  }

  // The vertical origin and vertical advance functions in HarfBuzzFace require
  // the ascent and height metrics as fallback in case no specific vertical
  // layout information is found from the font.
  void UpdateFallbackMetricsAndScale(
      const FontPlatformData& platform_data,
      HarfBuzzFace::VerticalLayoutCallbacks vertical_layout) {
    float ascent = 0;
    float descent = 0;

    font_ = platform_data.CreateSkFont();

    if (vertical_layout == HarfBuzzFace::kPrepareForVerticalLayout)
        [[unlikely]] {
      FontMetrics::AscentDescentWithHacks(ascent, descent, platform_data,
                                          font_);
      ascent_fallback_ = ascent;
      // Simulate the rounding that FontMetrics does so far for returning the
      // integer Height()
      height_fallback_ = lroundf(ascent) + lroundf(descent);

      int units_per_em =
          platform_data.GetHarfBuzzFace()->UnitsPerEmFromHeadTable();
      if (!units_per_em) {
        DLOG(ERROR)
            << "Units per EM is 0 for font used in vertical writing mode.";
      }
      size_per_unit_ = platform_data.size() / (units_per_em ? units_per_em : 1);
    } else {
      ascent_fallback_ = kInvalidFallbackMetricsValue;
      height_fallback_ = kInvalidFallbackMetricsValue;
      size_per_unit_ = kInvalidFallbackMetricsValue;
    }
  }

  OpenTypeVerticalData* VerticalData() {
    if (!vertical_data_) {
      DCHECK_NE(ascent_fallback_, kInvalidFallbackMetricsValue);
      DCHECK_NE(height_fallback_, kInvalidFallbackMetricsValue);
      DCHECK_NE(size_per_unit_, kInvalidFallbackMetricsValue);

      vertical_data_ =
          MakeGarbageCollected<OpenTypeVerticalData>(font_.refTypeface());
    }
    vertical_data_->SetScaleAndFallbackMetrics(size_per_unit_, ascent_fallback_,
                                               height_fallback_);
    return vertical_data_.Get();
  }

  const hb::unique_ptr<hb_font_t> unscaled_font_;
  SkFont font_;

  // Capture these scaled fallback metrics from FontPlatformData so that a
  // OpenTypeVerticalData object can be constructed from them when needed.
  float size_per_unit_;
  float ascent_fallback_;
  float height_fallback_;

  enum class SpaceGlyphInOpenTypeTables { kUnknown, kPresent, kNotPresent };

  SpaceGlyphInOpenTypeTables space_in_gpos_ =
      SpaceGlyphInOpenTypeTables::kUnknown;
  SpaceGlyphInOpenTypeTables space_in_gsub_ =
      SpaceGlyphInOpenTypeTables::kUnknown;

  Member<OpenTypeVerticalData> vertical_data_;
  Member<const UnicodeRangeSet> range_set_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_HARFBUZZ_FONT_DATA_H_
