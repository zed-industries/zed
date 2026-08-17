// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_VARIANT_NUMERIC_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_VARIANT_NUMERIC_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class FontVariantNumeric {
  STACK_ALLOCATED();

 public:
  enum NumericFigure { kNormalFigure = 0, kLiningNums, kOldstyleNums };
  static WTF::String ToString(NumericFigure);

  enum NumericSpacing { kNormalSpacing = 0, kProportionalNums, kTabularNums };
  static WTF::String ToString(NumericSpacing);

  enum NumericFraction {
    kNormalFraction = 0,
    kDiagonalFractions,
    kStackedFractions
  };
  static WTF::String ToString(NumericFraction);

  enum Ordinal { kOrdinalOff = 0, kOrdinalOn };
  static WTF::String ToString(Ordinal);

  enum SlashedZero { kSlashedZeroOff = 0, kSlashedZeroOn };
  static WTF::String ToString(SlashedZero);

  FontVariantNumeric() : fields_as_unsigned_(0) {}

  static FontVariantNumeric InitializeFromUnsigned(unsigned init_value) {
    return FontVariantNumeric(init_value);
  }

  void SetNumericFigure(NumericFigure figure) {
    fields_.numeric_figure_ = figure;
  }
  void SetNumericSpacing(NumericSpacing spacing) {
    fields_.numeric_spacing_ = spacing;
  }
  void SetNumericFraction(NumericFraction fraction) {
    fields_.numeric_fraction_ = fraction;
  }
  void SetOrdinal(Ordinal ordinal) { fields_.ordinal_ = ordinal; }
  void SetSlashedZero(SlashedZero slashed_zero) {
    fields_.slashed_zero_ = slashed_zero;
  }

  NumericFigure NumericFigureValue() const {
    return static_cast<NumericFigure>(fields_.numeric_figure_);
  }
  NumericSpacing NumericSpacingValue() const {
    return static_cast<NumericSpacing>(fields_.numeric_spacing_);
  }
  NumericFraction NumericFractionValue() const {
    return static_cast<NumericFraction>(fields_.numeric_fraction_);
  }
  Ordinal OrdinalValue() const {
    return static_cast<Ordinal>(fields_.ordinal_);
  }
  SlashedZero SlashedZeroValue() const {
    return static_cast<SlashedZero>(fields_.slashed_zero_);
  }

  bool IsAllNormal() { return !fields_as_unsigned_; }

  bool operator==(const FontVariantNumeric& other) const {
    return fields_as_unsigned_ == other.fields_as_unsigned_;
  }

  WTF::String ToString() const;

 private:
  FontVariantNumeric(unsigned init_value) : fields_as_unsigned_(init_value) {}

  struct BitFields {
    unsigned numeric_figure_ : 2;
    unsigned numeric_spacing_ : 2;
    unsigned numeric_fraction_ : 2;
    unsigned ordinal_ : 1;
    unsigned slashed_zero_ : 1;
  };

  union {
    BitFields fields_;
    unsigned fields_as_unsigned_;
  };
  static_assert(sizeof(BitFields) == sizeof(unsigned),
                "Mapped union types must match in size.");

  // Used in SetVariant to store the value in m_fields.m_variantNumeric;
  friend class FontDescription;
};
}

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_VARIANT_NUMERIC_H_
