// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_VARIANT_EAST_ASIAN_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_VARIANT_EAST_ASIAN_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class FontVariantEastAsian {
  STACK_ALLOCATED();

 public:
  enum EastAsianForm {
    kNormalForm,
    kJis78,
    kJis83,
    kJis90,
    kJis04,
    kSimplified,
    kTraditional
    // Ensure |BitFields| has enough bits when adding values.
  };
  static WTF::String ToString(EastAsianForm);

  enum EastAsianWidth {
    kNormalWidth,
    kFullWidth,
    kProportionalWidth
    // Ensure |BitFields| has enough bits when adding values.
  };
  static WTF::String ToString(EastAsianWidth);

  FontVariantEastAsian() : fields_as_unsigned_(0) {}

  static FontVariantEastAsian InitializeFromUnsigned(unsigned init_value) {
    return FontVariantEastAsian(init_value);
  }

  EastAsianForm Form() const {
    return static_cast<EastAsianForm>(fields_.form_);
  }
  EastAsianWidth Width() const {
    return static_cast<EastAsianWidth>(fields_.width_);
  }
  bool Ruby() const { return fields_.ruby_; }

  void SetForm(EastAsianForm form) { fields_.form_ = form; }
  void SetWidth(EastAsianWidth width) { fields_.width_ = width; }
  void SetRuby(bool ruby) { fields_.ruby_ = ruby; }

  bool IsAllNormal() const { return !fields_as_unsigned_; }

  bool operator==(const FontVariantEastAsian& other) const {
    return fields_as_unsigned_ == other.fields_as_unsigned_;
  }

  WTF::String ToString() const;

 private:
  FontVariantEastAsian(unsigned init_value) : fields_as_unsigned_(init_value) {}

  struct BitFields {
    unsigned form_ : 3;
    unsigned width_ : 2;
    unsigned ruby_ : 1;
    // Ensure |FontDescription| has enough bits when adding values.
  };

  union {
    BitFields fields_;
    unsigned fields_as_unsigned_;
  };
  static_assert(sizeof(BitFields) == sizeof(unsigned),
                "Mapped union types must match in size.");

  // Used in setVariant to store the value in m_fields.m_variantNumeric;
  friend class FontDescription;
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_VARIANT_EAST_ASIAN_H_
