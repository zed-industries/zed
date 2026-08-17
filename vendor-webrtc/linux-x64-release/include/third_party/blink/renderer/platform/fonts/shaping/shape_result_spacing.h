// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_SHAPE_RESULT_SPACING_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_SHAPE_RESULT_SPACING_H_

#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/text/character.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class FontDescription;
class TextRun;

// A context object to apply letter-spacing, word-spacing, and justification to
// ShapeResult.
template <typename TextContainerType>
class PLATFORM_EXPORT ShapeResultSpacing final {
  STACK_ALLOCATED();

 public:
  explicit ShapeResultSpacing(const TextContainerType& text,
                              bool allow_word_spacing_anywhere = false)
      : text_(text),
        allow_word_spacing_anywhere_(allow_word_spacing_anywhere) {}

  const TextContainerType& Text() const { return text_; }
  TextRunLayoutUnit LetterSpacing() const {
    return has_spacing_ ? letter_spacing_ : TextRunLayoutUnit();
  }
  TextRunLayoutUnit WordSpacing() const {
    return has_spacing_ ? word_spacing_ : TextRunLayoutUnit();
  }
  bool HasSpacing() const { return has_spacing_; }
  bool IsLetterSpacingAppliedForTesting() const {
    return is_letter_spacing_applied_;
  }
  bool IsWordSpacingAppliedForTesting() const {
    return is_word_spacing_applied_;
  }
  bool HasExpansion() const { return expansion_opportunity_count_; }
  unsigned ExpansionOppotunityCount() const {
    return expansion_opportunity_count_;
  }

  // Set letter-spacing and word-spacing.
  bool SetSpacing(const FontDescription&);
  bool SetSpacing(TextRunLayoutUnit letter_spacing,
                  TextRunLayoutUnit word_spacing);

  // Set the expansion for the justification.
  void SetExpansion(InlineLayoutUnit expansion,
                    TextDirection,
                    bool allows_leading_expansion = false,
                    bool allows_trailing_expansion = false);

  // Set letter-spacing, word-spacing, and
  // justification. Available only for TextRun.
  void SetSpacingAndExpansion(const FontDescription&);
  // This one is not only for TextRun.
  void SetSpacingAndExpansion(const FontDescription&, bool normalize_space);

  // Compute the sum of all spacings for the specified |index|.
  // The |index| is for the |TextContainerType| given in the constructor.
  // For justification, this function must be called incrementally since it
  // keeps states and counts consumed justification opportunities.
  struct ComputeSpacingParameters {
    unsigned index;
    float original_advance = 0.0;
  };
  TextRunLayoutUnit ComputeSpacing(unsigned index,
                                   float& offset,
                                   bool is_cursive_script = false) {
    return ComputeSpacing(ComputeSpacingParameters{.index = index}, offset,
                          is_cursive_script);
  }
  TextRunLayoutUnit ComputeSpacing(const ComputeSpacingParameters& parameters,
                                   float& offset,
                                   bool is_cursive_script);

 private:
  bool IsAfterExpansion() const { return is_after_expansion_; }

  void ComputeExpansion(bool allows_leading_expansion,
                        bool allows_trailing_expansion,
                        TextDirection);

  TextRunLayoutUnit NextExpansion();

  const TextContainerType& text_;
  TextRunLayoutUnit letter_spacing_;
  TextRunLayoutUnit word_spacing_;
  InlineLayoutUnit expansion_;
  TextRunLayoutUnit expansion_per_opportunity_;
  unsigned expansion_opportunity_count_ = 0;
  bool has_spacing_ = false;
  bool is_letter_spacing_applied_ = false;
  bool is_word_spacing_applied_ = false;
  bool normalize_space_ = false;
  bool allow_tabs_ = false;
  bool is_after_expansion_ = false;
  bool allow_word_spacing_anywhere_ = false;
};

// Forward declare so no implicit instantiations happen before the
// first explicit instantiation (which would be a C++ violation).
template <>
void ShapeResultSpacing<TextRun>::SetSpacingAndExpansion(
    const FontDescription&);
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_SHAPE_RESULT_SPACING_H_
