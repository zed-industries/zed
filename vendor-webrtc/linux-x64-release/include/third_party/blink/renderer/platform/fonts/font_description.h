/*
 * Copyright (C) 2000 Lars Knoll (knoll@kde.org)
 *           (C) 2000 Antti Koivisto (koivisto@kde.org)
 *           (C) 2000 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2003, 2004, 2005, 2006, 2007, 2008 Apple Inc. All rights
 * reserved.
 * Copyright (C) 2007 Nicholas Shanks <webkit@nickshanks.com>
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
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_DESCRIPTION_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_DESCRIPTION_H_

#include <unicode/uscript.h>

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/platform/font_family_names.h"
#include "third_party/blink/renderer/platform/fonts/font_cache_key.h"
#include "third_party/blink/renderer/platform/fonts/font_family.h"
#include "third_party/blink/renderer/platform/fonts/font_optical_sizing.h"
#include "third_party/blink/renderer/platform/fonts/font_orientation.h"
#include "third_party/blink/renderer/platform/fonts/font_palette.h"
#include "third_party/blink/renderer/platform/fonts/font_selection_types.h"
#include "third_party/blink/renderer/platform/fonts/font_size_adjust.h"
#include "third_party/blink/renderer/platform/fonts/font_smoothing_mode.h"
#include "third_party/blink/renderer/platform/fonts/font_variant_alternates.h"
#include "third_party/blink/renderer/platform/fonts/font_variant_east_asian.h"
#include "third_party/blink/renderer/platform/fonts/font_variant_emoji.h"
#include "third_party/blink/renderer/platform/fonts/font_variant_numeric.h"
#include "third_party/blink/renderer/platform/fonts/font_width_variant.h"
#include "third_party/blink/renderer/platform/fonts/opentype/font_settings.h"
#include "third_party/blink/renderer/platform/fonts/shaping/text_spacing_trim.h"
#include "third_party/blink/renderer/platform/fonts/text_rendering_mode.h"
#include "third_party/blink/renderer/platform/fonts/typesetting_features.h"
#include "third_party/blink/renderer/platform/text/layout_locale.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/math_extras.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/skia/include/core/SkFontStyle.h"

namespace blink {

typedef struct { uint32_t parts[2]; } FieldsAsUnsignedType;

class PLATFORM_EXPORT FontDescription {
  USING_FAST_MALLOC(FontDescription);

 public:
  enum HashCategory {
    kHashEmptyValue = 0,
    kHashDeletedValue,
    kHashRegularValue
  };

  enum GenericFamilyType : uint8_t {
    kNoFamily,
    kStandardFamily,
    kWebkitBodyFamily,
    kSerifFamily,
    kSansSerifFamily,
    kMonospaceFamily,
    kCursiveFamily,
    kFantasyFamily
  };
  static String ToString(GenericFamilyType);

  enum LigaturesState {
    kNormalLigaturesState,
    kDisabledLigaturesState,
    kEnabledLigaturesState
  };
  static String ToString(LigaturesState);

  enum Kerning { kAutoKerning, kNormalKerning, kNoneKerning };
  static String ToString(Kerning);

  static String ToString(FontSelectionValue);

  enum FontVariantCaps {
    kCapsNormal,
    kSmallCaps,
    kAllSmallCaps,
    kPetiteCaps,
    kAllPetiteCaps,
    kUnicase,
    kTitlingCaps
  };
  static String ToString(FontVariantCaps);
  static String ToStringForIdl(FontVariantCaps);

  enum FontSynthesisWeight {
    kAutoFontSynthesisWeight,
    kNoneFontSynthesisWeight
  };
  static String ToString(FontSynthesisWeight);

  enum FontSynthesisStyle { kAutoFontSynthesisStyle, kNoneFontSynthesisStyle };
  static String ToString(FontSynthesisStyle);

  enum FontSynthesisSmallCaps {
    kAutoFontSynthesisSmallCaps,
    kNoneFontSynthesisSmallCaps
  };
  static String ToString(FontSynthesisSmallCaps);

  enum FontVariantPosition {
    kNormalVariantPosition,
    kSubVariantPosition,
    kSuperVariantPosition
  };
  static String ToString(FontVariantPosition);

  FontDescription();
  FontDescription(const FontDescription&);

  static FontDescription CreateHashTableEmptyValue();
  explicit FontDescription(WTF::HashTableDeletedValueType);

  FontDescription& operator=(const FontDescription&);

  bool operator==(const FontDescription&) const;
  bool operator!=(const FontDescription& other) const {
    return !(*this == other);
  }

  struct VariantLigatures {
    STACK_ALLOCATED();

   public:
    VariantLigatures(LigaturesState state = kNormalLigaturesState)
        : common(state),
          discretionary(state),
          historical(state),
          contextual(state) {}

    String ToString() const;

    unsigned common : 2;
    unsigned discretionary : 2;
    unsigned historical : 2;
    unsigned contextual : 2;

    bool operator==(const VariantLigatures& other) const {
      return common == other.common && discretionary == other.discretionary
        && historical == other.historical && contextual == other.contextual;
    }
  };

  struct Size {
    STACK_ALLOCATED();

   public:
    Size(unsigned keyword, float value, bool is_absolute)
        : keyword(keyword), is_absolute(is_absolute), value(value) {}

    String ToString() const;

    unsigned keyword : 4;      // FontDescription::KeywordSize
    unsigned is_absolute : 1;  // FontDescription::IsAbsoluteSize
    float value;               // FontDescription::SpecifiedSize
  };

  struct FamilyDescription {
    STACK_ALLOCATED();

   public:
    FamilyDescription(GenericFamilyType generic_family)
        : generic_family(generic_family) {}
    FamilyDescription(GenericFamilyType generic_family,
                      const FontFamily& family)
        : generic_family(generic_family), family(family) {}

    String ToString() const;

    GenericFamilyType generic_family;
    FontFamily family;
  };

  const FontFamily& Family() const { return family_list_; }
  FamilyDescription GetFamilyDescription() const {
    return FamilyDescription(GenericFamily(), Family());
  }
  const FontFamily& FirstFamily() const { return family_list_; }
  Size GetSize() const {
    return Size(KeywordSize(), SpecifiedSize(), IsAbsoluteSize());
  }
  float SpecifiedSize() const { return specified_size_; }
  // Returns the result of applying font-size-adjust to the specified size. This
  // is useful as an input to optical sizing and takes zooming out of the
  // equation for determining the font size to be used for font-optical-sizing:
  // auto;.
  float AdjustedSpecifiedSize() const;
  float ComputedSize() const { return computed_size_; }

  // TODO(xiaochengh): The functions and members for size-adjust descriptor and
  // font-size-adjust property have similar names and are very confusing. Rename
  // them for better clarity.

  // For CSS font-size-adjust property
  FontSizeAdjust SizeAdjust() const { return size_adjust_; }
  bool HasSizeAdjust() const { return !!size_adjust_; }

  // Return a copy with the size-adjust descriptor applied.
  // https://drafts.csswg.org/css-fonts-5/#descdef-font-face-size-adjust
  FontDescription SizeAdjustedFontDescription(float size_adjust) const;

  // The used value of font-size applying font-size-adjust or size-adjust.
  // TODO(crbug.com/451346): Make font-size-adjust and size-adjust work
  // together.
  float AdjustedSize() const { return adjusted_size_; }

  int ComputedPixelSize() const { return int(computed_size_ + 0.5f); }
  FontVariantCaps VariantCaps() const {
    return static_cast<FontVariantCaps>(fields_.variant_caps_);
  }
  bool IsAbsoluteSize() const { return fields_.is_absolute_size_; }
  FontSelectionValue Weight() const { return font_selection_request_.weight; }
  FontSelectionValue Style() const { return font_selection_request_.slope; }
  FontSelectionValue Stretch() const { return font_selection_request_.width; }
  static FontSelectionValue LighterWeight(FontSelectionValue);
  static FontSelectionValue BolderWeight(FontSelectionValue);
  static Size LargerSize(const Size&);
  static Size SmallerSize(const Size&);
  GenericFamilyType GenericFamily() const {
    return static_cast<GenericFamilyType>(fields_.generic_family_);
  }

  // only use fixed default size when there is only one font family, and that
  // family is "monospace"
  bool IsMonospace() const {
    return Family().FamilyName() == font_family_names::kMonospace &&
           Family().FamilyIsGeneric() && !Family().Next();
  }
  Kerning GetKerning() const { return static_cast<Kerning>(fields_.kerning_); }
  TextSpacingTrim GetTextSpacingTrim() const {
    return static_cast<TextSpacingTrim>(fields_.text_spacing_trim_);
  }
  FontVariantEastAsian VariantEastAsian() const {
    return FontVariantEastAsian::InitializeFromUnsigned(
        fields_.variant_east_asian_);
  }
  VariantLigatures GetVariantLigatures() const;
  FontVariantNumeric VariantNumeric() const {
    return FontVariantNumeric::InitializeFromUnsigned(fields_.variant_numeric_);
  }
  LigaturesState CommonLigaturesState() const {
    return static_cast<LigaturesState>(fields_.common_ligatures_state_);
  }
  LigaturesState DiscretionaryLigaturesState() const {
    return static_cast<LigaturesState>(fields_.discretionary_ligatures_state_);
  }
  LigaturesState HistoricalLigaturesState() const {
    return static_cast<LigaturesState>(fields_.historical_ligatures_state_);
  }
  LigaturesState ContextualLigaturesState() const {
    return static_cast<LigaturesState>(fields_.contextual_ligatures_state_);
  }
  unsigned KeywordSize() const { return fields_.keyword_size_; }
  FontSmoothingMode FontSmoothing() const {
    return static_cast<FontSmoothingMode>(fields_.font_smoothing_);
  }
  OpticalSizing FontOpticalSizing() const {
    return static_cast<OpticalSizing>(fields_.font_optical_sizing_);
  }
  const FontPalette* GetFontPalette() const { return font_palette_.get(); }
  const FontVariantAlternates* GetFontVariantAlternates() const {
    return font_variant_alternates_.get();
  }
  TextRenderingMode TextRendering() const {
    return static_cast<TextRenderingMode>(fields_.text_rendering_);
  }
  const LayoutLocale* Locale() const { return locale_.get(); }
  const LayoutLocale& LocaleOrDefault() const {
    return LayoutLocale::ValueOrDefault(locale_.get());
  }
  UScriptCode GetScript() const { return LocaleOrDefault().GetScript(); }
  bool IsSyntheticBold() const { return fields_.synthetic_bold_; }
  bool IsSyntheticItalic() const { return fields_.synthetic_italic_; }
  bool IsSyntheticOblique() const { return fields_.synthetic_oblique_; }
  bool UseSubpixelPositioning() const {
    return fields_.subpixel_text_position_;
  }
  FontSynthesisWeight GetFontSynthesisWeight() const {
    return static_cast<FontSynthesisWeight>(fields_.font_synthesis_weight_);
  }
  bool SyntheticBoldAllowed() const {
    return fields_.font_synthesis_weight_ == kAutoFontSynthesisWeight;
  }
  FontSynthesisStyle GetFontSynthesisStyle() const {
    return static_cast<FontSynthesisStyle>(fields_.font_synthesis_style_);
  }
  bool SyntheticItalicAllowed() const {
    return fields_.font_synthesis_style_ == kAutoFontSynthesisStyle;
  }
  FontSynthesisSmallCaps GetFontSynthesisSmallCaps() const {
    return static_cast<FontSynthesisSmallCaps>(
        fields_.font_synthesis_small_caps_);
  }

  FontSelectionRequest GetFontSelectionRequest() const;
  float WordSpacing() const { return word_spacing_; }
  float LetterSpacing() const { return letter_spacing_; }
  FontOrientation Orientation() const {
    return static_cast<FontOrientation>(fields_.orientation_);
  }
  bool IsVerticalAnyUpright() const {
    return blink::IsVerticalAnyUpright(Orientation());
  }
  bool IsVerticalNonCJKUpright() const {
    return blink::IsVerticalNonCJKUpright(Orientation());
  }
  bool IsVerticalUpright(UChar32 character) const {
    return blink::IsVerticalUpright(Orientation(), character);
  }
  bool IsVerticalBaseline() const {
    return blink::IsVerticalBaseline(Orientation());
  }
  FontWidthVariant WidthVariant() const {
    return static_cast<FontWidthVariant>(fields_.width_variant_);
  }
  const FontFeatureSettings* FeatureSettings() const {
    return feature_settings_.get();
  }
  const FontVariationSettings* VariationSettings() const {
    return variation_settings_.get();
  }
  FontVariantPosition VariantPosition() const {
    return static_cast<FontVariantPosition>(fields_.variant_position_);
  }
  FontVariantEmoji VariantEmoji() const {
    return static_cast<FontVariantEmoji>(fields_.variant_emoji_);
  }

  float EffectiveFontSize()
      const;  // Returns either the computedSize or the computedPixelSize
  FontCacheKey CacheKey(const FontFaceCreationParams&,
                        bool is_unique_match) const;

  void SetFamily(const FontFamily& family) { family_list_ = family; }
  void SetComputedSize(float s) { computed_size_ = ClampTo<float>(s); }
  void SetSpecifiedSize(float s) { specified_size_ = ClampTo<float>(s); }
  void SetAdjustedSize(float s) { adjusted_size_ = ClampTo<float>(s); }
  void SetSizeAdjust(const FontSizeAdjust& size_adjust) {
    size_adjust_ = size_adjust;
  }

  void SetStyle(FontSelectionValue i);
  void SetWeight(FontSelectionValue w) { font_selection_request_.weight = w; }
  void SetStretch(FontSelectionValue s) { font_selection_request_.width = s; }

  void SetVariantCaps(FontVariantCaps);
  void SetVariantEastAsian(const FontVariantEastAsian);
  void SetVariantLigatures(const VariantLigatures&);
  void SetVariantNumeric(const FontVariantNumeric&);
  void SetIsAbsoluteSize(bool s) { fields_.is_absolute_size_ = s; }

  void SetGenericFamily(GenericFamilyType generic_family) {
    fields_.generic_family_ = generic_family;
  }
  void SetKerning(Kerning kerning) {
    fields_.kerning_ = kerning;
    UpdateTypesettingFeatures();
  }
  void SetTextSpacingTrim(TextSpacingTrim text_spacing_trim) {
    fields_.text_spacing_trim_ = static_cast<unsigned>(text_spacing_trim);
  }
  void SetKeywordSize(unsigned s) { fields_.keyword_size_ = s; }
  void SetFontSmoothing(FontSmoothingMode smoothing) {
    fields_.font_smoothing_ = smoothing;
  }
  void SetFontOpticalSizing(OpticalSizing font_optical_sizing) {
    fields_.font_optical_sizing_ = font_optical_sizing;
  }
  void SetFontPalette(scoped_refptr<const FontPalette> palette) {
    font_palette_ = std::move(palette);
  }
  void SetFontVariantAlternates(
      scoped_refptr<const FontVariantAlternates> alternates) {
    font_variant_alternates_ = std::move(alternates);
  }
  void SetTextRendering(TextRenderingMode rendering) {
    fields_.text_rendering_ = rendering;
    UpdateTypesettingFeatures();
  }
  void SetOrientation(FontOrientation orientation);
  void SetWidthVariant(FontWidthVariant width_variant) {
    fields_.width_variant_ = width_variant;
  }
  void SetLocale(scoped_refptr<const LayoutLocale> locale) {
    locale_ = std::move(locale);
  }
  void SetSyntheticBold(bool synthetic_bold) {
    fields_.synthetic_bold_ = synthetic_bold;
  }
  void SetSyntheticItalic(bool synthetic_italic) {
    fields_.synthetic_italic_ = synthetic_italic;
  }
  void SetFontSynthesisWeight(FontSynthesisWeight font_synthesis_weight) {
    fields_.font_synthesis_weight_ = font_synthesis_weight;
  }
  void SetFontSynthesisStyle(FontSynthesisStyle font_synthesis_style) {
    fields_.font_synthesis_style_ = font_synthesis_style;
  }
  void SetFontSynthesisSmallCaps(
      FontSynthesisSmallCaps font_synthesis_small_caps) {
    fields_.font_synthesis_small_caps_ = font_synthesis_small_caps;
  }
  void SetFeatureSettings(scoped_refptr<const FontFeatureSettings> settings) {
    feature_settings_ = std::move(settings);
  }
  void SetVariationSettings(
      scoped_refptr<const FontVariationSettings> settings) {
    variation_settings_ = std::move(settings);
  }
  void SetVariantPosition(FontVariantPosition variant_position) {
    fields_.variant_position_ = variant_position;
  }
  void SetVariantEmoji(FontVariantEmoji variant_emoji) {
    fields_.variant_emoji_ = variant_emoji;
  }
  void SetWordSpacing(float s) { word_spacing_ = s; }
  void SetLetterSpacing(float s) {
    letter_spacing_ = s;
    UpdateTypesettingFeatures();
  }

  TypesettingFeatures GetTypesettingFeatures() const {
    return static_cast<TypesettingFeatures>(fields_.typesetting_features_);
  }

  static void SetSubpixelPositioning(bool b) {
    use_subpixel_text_positioning_ = b;
  }
  static bool SubpixelPositioning() { return use_subpixel_text_positioning_; }

  void SetSubpixelAscentDescent(bool sp) {
    fields_.subpixel_ascent_descent_ = sp;
  }

  bool SubpixelAscentDescent() const {
    return fields_.subpixel_ascent_descent_;
  }

  HashCategory GetHashCategory() const {
    return static_cast<HashCategory>(fields_.hash_category_);
  }

  bool IsHashTableEmptyValue() const {
    return GetHashCategory() == kHashEmptyValue;
  }

  bool IsHashTableDeletedValue() const {
    return GetHashCategory() == kHashDeletedValue;
  }

  unsigned StyleHashWithoutFamilyList() const;
  unsigned GetHash() const;

  // TODO(drott): We should not expose internal structure here, but rather
  // introduce a hash function here.
  unsigned BitmapFields() const { return fields_as_unsigned_.parts[0]; }
  unsigned AuxiliaryBitmapFields() const {
    return fields_as_unsigned_.parts[1];
  }

  SkFontStyle SkiaFontStyle() const;

  void UpdateFromSkiaFontStyle(const SkFontStyle& font_style);

  int MinimumPrefixWidthToHyphenate() const;

  ResolvedFontFeatures ResolveFontFeatures() const;

  String ToString() const;

 private:
  void UpdateSyntheticOblique();

  FontFamily family_list_;  // The list of font families to be used.
  scoped_refptr<const FontFeatureSettings> feature_settings_;
  scoped_refptr<const FontVariationSettings> variation_settings_;
  scoped_refptr<const LayoutLocale> locale_;
  scoped_refptr<const FontPalette> font_palette_;
  scoped_refptr<const FontVariantAlternates> font_variant_alternates_;

  void UpdateTypesettingFeatures();

  // Specified CSS value. Independent of rendering issues such as integer
  // rounding, minimum font sizes, and zooming.
  float specified_size_;
  // Computed size adjusted for the minimum font size and the zoom factor.
  float computed_size_;

  // (Given aspect value / aspect value of a font family) * specifiedSize.
  // This value is adjusted for the minimum font size and the zoom factor
  // as well as a computed size is.
  float adjusted_size_;

  float letter_spacing_;
  float word_spacing_;

  FontSizeAdjust size_adjust_;

  // Covers stretch, style, weight.
  FontSelectionRequest font_selection_request_;
  FontSelectionValue original_slope;

  struct BitFields {
    DISALLOW_NEW();

    String ToString() const;

    unsigned orientation_ : kFontOrientationBitCount;

    unsigned width_variant_ : 2;  // FontWidthVariant

    unsigned variant_caps_ : 3;  // FontVariantCaps
    unsigned
        is_absolute_size_ : 1;  // Whether or not CSS specified an explicit size
    // (logical sizes like "medium" don't count).
    unsigned generic_family_ : 3;  // GenericFamilyType

    unsigned kerning_ : 2;  // Kerning

    unsigned common_ligatures_state_ : 2;
    unsigned discretionary_ligatures_state_ : 2;
    unsigned historical_ligatures_state_ : 2;
    unsigned contextual_ligatures_state_ : 2;

    // We cache whether or not a font is currently represented by a CSS keyword
    // (e.g., medium).  If so, then we can accurately translate across different
    // generic families to adjust for different preference settings (e.g., 13px
    // monospace vs. 16px everything else).  Sizes are 1-8 (like the HTML size
    // values for <font>).
    unsigned keyword_size_ : 4;

    unsigned font_smoothing_ : 2;  // FontSmoothingMode
    unsigned text_rendering_ : 2;  // TextRenderingMode
    unsigned synthetic_bold_ : 1;
    unsigned synthetic_italic_ : 1;
    unsigned synthetic_oblique_ : 1;
    unsigned font_synthesis_weight_ : 1;
    unsigned font_synthesis_style_ : 1;
    unsigned font_synthesis_small_caps_ : 1;
    unsigned subpixel_text_position_ : 1;
    unsigned typesetting_features_ : 3;
    unsigned variant_numeric_ : 8;
    unsigned variant_east_asian_ : 6;
    unsigned subpixel_ascent_descent_ : 1;
    unsigned font_optical_sizing_ : 1;
    unsigned has_size_adjust_descriptor_ : 1;
    unsigned variant_position_ : 2;
    unsigned variant_emoji_ : 2;
    unsigned text_spacing_trim_ : kTextSpacingTrimBitCount;

    unsigned hash_category_ : 2;  // HashCategory
  };

  static_assert(sizeof(BitFields) == sizeof(FieldsAsUnsignedType),
                "Mapped bitfield datatypes must have identical size.");
  union {
    BitFields fields_;
    FieldsAsUnsignedType fields_as_unsigned_;
  };

  static bool use_subpixel_text_positioning_;
};

}  // namespace blink

namespace WTF {

template <>
struct HashTraits<blink::FontDescription>
    : SimpleClassHashTraits<blink::FontDescription> {
  // FontDescription default constructor creates a regular value instead of the
  // empty value.
  static blink::FontDescription EmptyValue() {
    return blink::FontDescription::CreateHashTableEmptyValue();
  }
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_DESCRIPTION_H_
