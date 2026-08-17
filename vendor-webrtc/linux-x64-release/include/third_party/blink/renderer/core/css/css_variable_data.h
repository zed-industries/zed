// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_VARIABLE_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_VARIABLE_DATA_H_

#include "base/types/pass_key.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/parser/css_parser_token.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/text_encoding.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class CSSSyntaxDefinition;
enum class SecureContextMode;

class CORE_EXPORT CSSVariableData : public GarbageCollected<CSSVariableData> {
 public:
  CSSVariableData()
      : length_(0),
        is_animation_tainted_(false),
        is_attr_tainted_(false),
        needs_variable_resolution_(false),
        is_8bit_(true),
        has_font_units_(false),
        has_root_font_units_(false),
        has_line_height_units_(false),
        has_dashed_functions_(false) {}

  using PassKey = base::PassKey<CSSVariableData>;
  CSSVariableData(PassKey,
                  StringView,
                  bool is_animation_tainted,
                  bool is_attr_tainted,
                  bool needs_variable_resolution,
                  bool has_font_units,
                  bool has_root_font_units,
                  bool has_line_height_units,
                  bool has_dashed_functions);

  // This is the fastest (non-trivial) constructor if you've got the has_* data
  // already, e.g. because you extracted them while tokenizing (see
  // ExtractFeatures()) or got them from another CSSVariableData instance during
  // substitution.
  static CSSVariableData* Create(StringView original_text,
                                 bool is_animation_tainted,
                                 bool is_attr_tainted,
                                 bool needs_variable_resolution,
                                 bool has_font_units,
                                 bool has_root_font_units,
                                 bool has_line_height_units,
                                 bool has_dashed_functions) {
    if (original_text.length() > kMaxVariableBytes) {
      // This should have been blocked off during variable substitution.
      NOTREACHED();
    }

    return MakeGarbageCollected<CSSVariableData>(
        AdditionalBytes(original_text.Is8Bit() ? original_text.length()
                                               : 2 * original_text.length()),
        PassKey(), original_text, is_animation_tainted, is_attr_tainted,
        needs_variable_resolution, has_font_units, has_root_font_units,
        has_line_height_units, has_dashed_functions);
  }

  // This tokenizes the string to determine the has_* data.
  // (The tokens are not used apart from that; only the original string is
  // stored.)
  static CSSVariableData* Create(const String& original_text,
                                 bool is_animation_tainted,
                                 bool is_attr_tainted,
                                 bool needs_variable_resolution);

  void Trace(Visitor*) const {}

  StringView OriginalText() const {
    // SAFETY: See AdditionalBytes() in Create().
    if (is_8bit_) {
      return StringView(UNSAFE_BUFFERS(
          base::span(reinterpret_cast<const LChar*>(this + 1), length_)));
    } else {
      return StringView(UNSAFE_BUFFERS(
          base::span(reinterpret_cast<const UChar*>(this + 1), length_)));
    }
  }

  uint64_t Hash() const {
    return StringHasher::HashMemory(OriginalText().RawByteSpan());
  }

  String Serialize() const;

  bool EqualsIgnoringAttrTainting(const CSSVariableData& other) const;

  bool operator==(const CSSVariableData& other) const;

  bool IsAnimationTainted() const { return is_animation_tainted_; }

  bool IsAttrTainted() const { return is_attr_tainted_; }

  bool NeedsVariableResolution() const { return needs_variable_resolution_; }

  // True if the CSSVariableData has tokens with units that are relative to the
  // font-size of the current element, e.g. 'em'.
  bool HasFontUnits() const { return has_font_units_; }

  // True if the CSSVariableData has tokens with units that are relative to the
  // font-size of the root element, e.g. 'rem'.
  bool HasRootFontUnits() const { return has_root_font_units_; }

  // True if the CSSVariableData has tokens with 'lh' units which are relative
  // to line-height property.
  bool HasLineHeightUnits() const { return has_line_height_units_; }

  // https://drafts.csswg.org/css-mixins-1/#typedef-dashed-function
  bool HasDashedFunctions() const { return has_dashed_functions_; }

  const CSSValue* ParseForSyntax(const CSSSyntaxDefinition&,
                                 SecureContextMode) const;

  CSSVariableData(const CSSVariableData&) = delete;
  CSSVariableData& operator=(const CSSVariableData&) = delete;
  CSSVariableData(CSSVariableData&&) = delete;
  CSSVariableData& operator=(const CSSVariableData&&) = delete;

  // ORs the given flags with those of the given token.
  static void ExtractFeatures(const CSSParserToken& token,
                              bool& has_font_units,
                              bool& has_root_font_units,
                              bool& has_line_height_units,
                              bool& has_dashed_functions);

  // The maximum number of bytes for a CSS variable (including text
  // that comes from var() substitution). This matches Firefox.
  //
  // If you change this, length_ below may need updates.
  //
  // https://drafts.csswg.org/css-variables/#long-variables
  static const size_t kMaxVariableBytes = 2097152;

 private:
  // We'd like to use bool for the booleans, but this causes the struct to
  // balloon in size on Windows:
  // https://randomascii.wordpress.com/2010/06/06/bit-field-packing-with-visual-c/

  // Enough for storing up to 2MB (and then some), cf. kMaxSubstitutionBytes.
  // The remaining 2 bits are kept in reserve for future use.
  const unsigned length_ : 22;
  const unsigned is_animation_tainted_ : 1;       // bool.
  const unsigned is_attr_tainted_ : 1;            // bool.
  const unsigned needs_variable_resolution_ : 1;  // bool.
  const unsigned is_8bit_ : 1;                    // bool.
  unsigned has_font_units_ : 1;                   // bool.
  unsigned has_root_font_units_ : 1;              // bool.
  unsigned has_line_height_units_ : 1;            // bool.
  unsigned has_dashed_functions_ : 1;             // bool.
  unsigned /* unused_ */ : 2;

  // The actual character data is stored after this.
};

#if !DCHECK_IS_ON()
static_assert(sizeof(CSSVariableData) <= 4,
              "CSSVariableData must not grow without thinking");
#endif

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_VARIABLE_DATA_H_
