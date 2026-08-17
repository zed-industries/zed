/*
 * Copyright (C) 2017 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS''
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
 * THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS
 * BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF
 * THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_SELECTION_TYPES_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_SELECTION_TYPES_H_

#include <algorithm>

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_table_deleted_value_type.h"
#include "third_party/blink/renderer/platform/wtf/hash_traits.h"
#include "third_party/blink/renderer/platform/wtf/math_extras.h"
#include "third_party/blink/renderer/platform/wtf/std_lib_extras.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// Unclamped, unchecked, signed fixed-point number representing a value used for
// font variations. Sixteen bits in total, one sign bit, two fractional bits,
// means the smallest positive representable value is 0.25, the maximum
// representable value is 8191.75, and the minimum representable value is -8192.
class PLATFORM_EXPORT FontSelectionValue {
  USING_FAST_MALLOC(FontSelectionValue);

 public:
  FontSelectionValue() = default;

  // Explicit because it is lossy.
  explicit constexpr FontSelectionValue(int x)
      : backing_(ClampTo<int16_t>(x * fractionalEntropy)) {}

  // Explicit because it is lossy.
  explicit constexpr FontSelectionValue(float x)
      : backing_(ClampTo<int16_t>(x * fractionalEntropy)) {}

  // Explicit because it is lossy.
  explicit constexpr FontSelectionValue(double x)
      : backing_(ClampTo<int16_t>(x * fractionalEntropy)) {}

  constexpr operator float() const {
    // floats have 23 fractional bits, but only 14 fractional bits are
    // necessary, so every value can be represented losslessly.
    return backing_ / static_cast<float>(fractionalEntropy);
  }

  constexpr FontSelectionValue operator+(const FontSelectionValue& other) const;
  constexpr FontSelectionValue operator-(const FontSelectionValue& other) const;
  constexpr FontSelectionValue operator*(const FontSelectionValue& other) const;
  constexpr FontSelectionValue operator/(const FontSelectionValue& other) const;
  constexpr FontSelectionValue operator-() const;
  constexpr bool operator==(const FontSelectionValue& other) const;
  constexpr bool operator!=(const FontSelectionValue& other) const;
  constexpr bool operator<(const FontSelectionValue& other) const;
  constexpr bool operator<=(const FontSelectionValue& other) const;
  constexpr bool operator>(const FontSelectionValue& other) const;
  constexpr bool operator>=(const FontSelectionValue& other) const;

  int16_t RawValue() const { return backing_; }

  String ToString() const;

  static constexpr FontSelectionValue MaximumValue() {
    return FontSelectionValue(std::numeric_limits<int16_t>::max(),
                              RawTag::RawTag);
  }

  static constexpr FontSelectionValue MinimumValue() {
    return FontSelectionValue(std::numeric_limits<int16_t>::min(),
                              RawTag::RawTag);
  }

 protected:
  enum class RawTag { RawTag };

  constexpr FontSelectionValue(int16_t rawValue, RawTag) : backing_(rawValue) {}

 private:
  static constexpr int fractionalEntropy = 4;
  // TODO(drott) crbug.com/745910 - Consider making this backed by a checked
  // arithmetic type.
  int16_t backing_{0};
};

inline constexpr FontSelectionValue FontSelectionValue::operator+(
    const FontSelectionValue& other) const {
  return FontSelectionValue(backing_ + other.backing_, RawTag::RawTag);
}

inline constexpr FontSelectionValue FontSelectionValue::operator-(
    const FontSelectionValue& other) const {
  return FontSelectionValue(backing_ - other.backing_, RawTag::RawTag);
}

inline constexpr FontSelectionValue FontSelectionValue::operator*(
    const FontSelectionValue& other) const {
  return FontSelectionValue(
      static_cast<int32_t>(backing_) * other.backing_ / fractionalEntropy,
      RawTag::RawTag);
}

inline constexpr FontSelectionValue FontSelectionValue::operator/(
    const FontSelectionValue& other) const {
  return FontSelectionValue(
      static_cast<int32_t>(backing_) / other.backing_ * fractionalEntropy,
      RawTag::RawTag);
}

inline constexpr FontSelectionValue FontSelectionValue::operator-() const {
  return FontSelectionValue(-backing_, RawTag::RawTag);
}

inline constexpr bool FontSelectionValue::operator==(
    const FontSelectionValue& other) const {
  return backing_ == other.backing_;
}

inline constexpr bool FontSelectionValue::operator!=(
    const FontSelectionValue& other) const {
  return !operator==(other);
}

inline constexpr bool FontSelectionValue::operator<(
    const FontSelectionValue& other) const {
  return backing_ < other.backing_;
}

inline constexpr bool FontSelectionValue::operator<=(
    const FontSelectionValue& other) const {
  return backing_ <= other.backing_;
}

inline constexpr bool FontSelectionValue::operator>(
    const FontSelectionValue& other) const {
  return backing_ > other.backing_;
}

inline constexpr bool FontSelectionValue::operator>=(
    const FontSelectionValue& other) const {
  return backing_ >= other.backing_;
}

inline constexpr FontSelectionValue kItalicThreshold = FontSelectionValue(14);

static constexpr inline bool isItalic(FontSelectionValue fontStyle) {
  return fontStyle >= kItalicThreshold;
}

inline constexpr FontSelectionValue kFontSelectionZeroValue =
    FontSelectionValue(0);

inline constexpr FontSelectionValue kNormalSlopeValue = FontSelectionValue();

inline constexpr FontSelectionValue kItalicSlopeValue = FontSelectionValue(14);

inline constexpr FontSelectionValue kMaxObliqueValue = FontSelectionValue(90);

inline constexpr FontSelectionValue kMinObliqueValue = FontSelectionValue(-90);

inline constexpr FontSelectionValue kBoldThreshold = FontSelectionValue(600);

inline constexpr FontSelectionValue kMinWeightValue = FontSelectionValue(1);

inline constexpr FontSelectionValue kMaxWeightValue = FontSelectionValue(1000);

inline constexpr FontSelectionValue kBlackWeightValue = FontSelectionValue(900);

inline constexpr FontSelectionValue kExtraBoldWeightValue =
    FontSelectionValue(800);

inline constexpr FontSelectionValue kBoldWeightValue = FontSelectionValue(700);

inline constexpr FontSelectionValue kSemiBoldWeightValue =
    FontSelectionValue(600);

inline constexpr FontSelectionValue kMediumWeightValue =
    FontSelectionValue(500);

inline constexpr FontSelectionValue kNormalWeightValue =
    FontSelectionValue(400);

inline constexpr FontSelectionValue kLightWeightValue = FontSelectionValue(300);

inline constexpr FontSelectionValue kExtraLightWeightValue =
    FontSelectionValue(200);

inline constexpr FontSelectionValue kThinWeightValue = FontSelectionValue(100);

static constexpr inline bool isFontWeightBold(FontSelectionValue fontWeight) {
  return fontWeight >= kBoldThreshold;
}

inline constexpr FontSelectionValue kUpperWeightSearchThreshold =
    FontSelectionValue(500);

inline constexpr FontSelectionValue kLowerWeightSearchThreshold =
    FontSelectionValue(400);

inline constexpr FontSelectionValue kUltraCondensedWidthValue =
    FontSelectionValue(50);

inline constexpr FontSelectionValue kExtraCondensedWidthValue =
    FontSelectionValue(62.5f);

inline constexpr FontSelectionValue kCondensedWidthValue =
    FontSelectionValue(75);

inline constexpr FontSelectionValue kSemiCondensedWidthValue =
    FontSelectionValue(87.5f);

inline constexpr FontSelectionValue kNormalWidthValue = FontSelectionValue(100);

inline constexpr FontSelectionValue kSemiExpandedWidthValue =
    FontSelectionValue(112.5f);

inline constexpr FontSelectionValue kExpandedWidthValue =
    FontSelectionValue(125);

inline constexpr FontSelectionValue kExtraExpandedWidthValue =
    FontSelectionValue(150);

inline constexpr FontSelectionValue kUltraExpandedWidthValue =
    FontSelectionValue(200);

struct FontSelectionRange {
  enum RangeType { kSetFromAuto, kSetExplicitly };

  explicit FontSelectionRange(FontSelectionValue single_value)
      : minimum(single_value), maximum(single_value) {}

  FontSelectionRange(const FontSelectionValue& minimum,
                     const FontSelectionValue maximum)
      : minimum(minimum), maximum(maximum) {}

  FontSelectionRange(const FontSelectionValue& minimum,
                     const FontSelectionValue& maximum,
                     RangeType type)
      : minimum(minimum), maximum(maximum), type(type) {}

  bool operator==(const FontSelectionRange& other) const {
    return minimum == other.minimum && maximum == other.maximum;
  }

  bool IsValid() const { return minimum <= maximum; }

  bool IsRange() const { return maximum > minimum; }

  bool IsRangeSetFromAuto() const { return type == kSetFromAuto; }

  void Expand(const FontSelectionRange& other) {
    DCHECK(other.IsValid());
    if (!IsValid()) {
      *this = other;
    } else {
      minimum = std::min(minimum, other.minimum);
      maximum = std::max(maximum, other.maximum);
    }
    DCHECK(IsValid());
  }

  bool Includes(FontSelectionValue target) const {
    return target >= minimum && target <= maximum;
  }

  uint32_t UniqueValue() const {
    return minimum.RawValue() << 16 | maximum.RawValue();
  }

  FontSelectionValue clampToRange(FontSelectionValue selection_value) const {
    return std::clamp(selection_value, minimum, maximum);
  }

  FontSelectionValue minimum{FontSelectionValue(1)};
  FontSelectionValue maximum{FontSelectionValue(0)};

  RangeType type = kSetFromAuto;
};

struct PLATFORM_EXPORT FontSelectionRequest {
  FontSelectionRequest() = default;

  FontSelectionRequest(FontSelectionValue weight,
                       FontSelectionValue width,
                       FontSelectionValue slope)
      : weight(weight), width(width), slope(slope) {}

  unsigned GetHash() const;

  bool operator==(const FontSelectionRequest& other) const {
    return weight == other.weight && width == other.width &&
           slope == other.slope;
  }

  bool operator!=(const FontSelectionRequest& other) const {
    return !operator==(other);
  }

  String ToString() const;

  FontSelectionValue weight;
  FontSelectionValue width;
  FontSelectionValue slope;
};

// Only used for HashMaps. We don't want to put the bool into
// FontSelectionRequest because FontSelectionRequest needs to be as small as
// possible because it's inside every FontDescription.
struct FontSelectionRequestKey {
  FontSelectionRequestKey() = default;

  FontSelectionRequestKey(FontSelectionRequest request) : request(request) {}

  explicit FontSelectionRequestKey(WTF::HashTableDeletedValueType)
      : isDeletedValue(true) {}

  bool IsHashTableDeletedValue() const { return isDeletedValue; }

  bool operator==(const FontSelectionRequestKey& other) const {
    return request == other.request && isDeletedValue == other.isDeletedValue;
  }

  FontSelectionRequest request;
  bool isDeletedValue{false};
};

struct PLATFORM_EXPORT FontSelectionRequestKeyHashTraits
    : SimpleClassHashTraits<FontSelectionRequestKey> {
  static unsigned GetHash(const FontSelectionRequestKey&);
};

struct FontSelectionCapabilities {
  FontSelectionCapabilities() = default;

  FontSelectionCapabilities(FontSelectionRange width,
                            FontSelectionRange slope,
                            FontSelectionRange weight)
      : width(width), slope(slope), weight(weight), is_deleted_value_(false) {}

  FontSelectionCapabilities(WTF::HashTableDeletedValueType)
      : is_deleted_value_(true) {}

  bool IsHashTableDeletedValue() const { return is_deleted_value_; }

  void Expand(const FontSelectionCapabilities& capabilities) {
    width.Expand(capabilities.width);
    slope.Expand(capabilities.slope);
    weight.Expand(capabilities.weight);
  }

  bool IsValid() const {
    return width.IsValid() && slope.IsValid() && weight.IsValid() &&
           !is_deleted_value_;
  }

  bool HasRange() const {
    return width.IsRange() || slope.IsRange() || weight.IsRange();
  }

  bool operator==(const FontSelectionCapabilities& other) const {
    return width == other.width && slope == other.slope &&
           weight == other.weight &&
           is_deleted_value_ == other.is_deleted_value_;
  }

  bool operator!=(const FontSelectionCapabilities& other) const {
    return !(*this == other);
  }

  FontSelectionRange width{kFontSelectionZeroValue, kFontSelectionZeroValue};
  FontSelectionRange slope{kFontSelectionZeroValue, kFontSelectionZeroValue};
  FontSelectionRange weight{kFontSelectionZeroValue, kFontSelectionZeroValue};
  bool is_deleted_value_{false};
};

struct PLATFORM_EXPORT FontSelectionCapabilitiesHashTraits
    : SimpleClassHashTraits<FontSelectionCapabilities> {
  static unsigned GetHash(const FontSelectionCapabilities& key);
};

}  // namespace blink

namespace WTF {

template <>
struct HashTraits<blink::FontSelectionRequestKey>
    : blink::FontSelectionRequestKeyHashTraits {};

template <>
struct HashTraits<blink::FontSelectionCapabilities>
    : blink::FontSelectionCapabilitiesHashTraits {};

}  // namespace WTF

// Used for ClampTo for example in StyleBuilderConverter
template <>
inline blink::FontSelectionValue
DefaultMinimumForClamp<blink::FontSelectionValue>() {
  return blink::FontSelectionValue::MinimumValue();
}

template <>
inline blink::FontSelectionValue
DefaultMaximumForClamp<blink::FontSelectionValue>() {
  return blink::FontSelectionValue::MaximumValue();
}

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_SELECTION_TYPES_H_
