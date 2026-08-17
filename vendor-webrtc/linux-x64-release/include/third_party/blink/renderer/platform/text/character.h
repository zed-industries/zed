/*
 * Copyright (C) 2014 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_CHARACTER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_CHARACTER_H_

#include <unicode/uchar.h>
#include <unicode/uniset.h>

#include "base/containers/span.h"
#include "base/gtest_prod_util.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/text/character_property.h"
#include "third_party/blink/renderer/platform/text/east_asian_spacing_type.h"
#include "third_party/blink/renderer/platform/text/han_kerning_char_type.h"
#include "third_party/blink/renderer/platform/text/text_direction.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/ascii_ctype.h"
#include "third_party/blink/renderer/platform/wtf/text/character_names.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class PLATFORM_EXPORT Character {
  STATIC_ONLY(Character);

 public:
  static inline bool IsInRange(UChar32 character,
                               UChar32 lower_bound,
                               UChar32 upper_bound) {
    return character >= lower_bound && character <= upper_bound;
  }

  // Commonly used Unicode Blocks in CSS specs.
  // https://www.unicode.org/Public/UNIDATA/Blocks.txt
  static bool IsBlockCjkSymbolsAndPunctuation(UChar32 ch) {
    return IsInRange(ch, 0x3000, 0x303F);
  }
  static bool IsBlockHalfwidthAndFullwidthForms(UChar32 ch) {
    return IsInRange(ch, 0xFF00, 0xFFEF);
  }

  // East Asian Width: https://unicode.org/reports/tr11/
  static UEastAsianWidth EastAsianWidth(UChar32 ch) {
    return static_cast<UEastAsianWidth>(
        u_getIntPropertyValue(ch, UCHAR_EAST_ASIAN_WIDTH));
  }
  static bool IsEastAsianWidthFullwidth(UChar32 ch);

  static inline bool IsUnicodeVariationSelector(UChar32 character) {
    // http://www.unicode.org/Public/UCD/latest/ucd/StandardizedVariants.html
    return IsInRange(character, 0x180B,
                     0x180D)  // MONGOLIAN FREE VARIATION SELECTOR ONE to THREE
           ||
           IsInRange(character, 0xFE00, 0xFE0F)  // VARIATION SELECTOR-1 to 16
           || IsInRange(character, 0xE0100,
                        0xE01EF);  // VARIATION SELECTOR-17 to 256
  }

  static inline bool IsUnicodeEmojiVariationSelector(UChar32 character) {
    // https://www.unicode.org/Public/emoji/5.0/emoji-variation-sequences.txt
    return character == 0xFE0E ||
           character == 0xFE0F;  // VARIATION SELECTOR-15 to 16
  }

  static bool IsCJKIdeographOrSymbol(UChar32 c) {
    // Below U+02C7 is likely a common case.
    return c < 0x2C7 ? false : IsCJKIdeographOrSymbolSlow(c);
  }
  static bool IsCJKIdeographOrSymbolBase(UChar32 c) {
    return IsCJKIdeographOrSymbol(c) &&
           !(U_GET_GC_MASK(c) & (U_GC_M_MASK | U_GC_LM_MASK | U_GC_SK_MASK));
  }

  static bool IsHangul(UChar32 c) {
    // Below U+1100 is likely a common case.
    return c < 0x1100 ? false : IsHangulSlow(c);
  }

  static unsigned ExpansionOpportunityCount(base::span<const LChar>,
                                            TextDirection,
                                            bool& is_after_expansion);
  static unsigned ExpansionOpportunityCount(base::span<const UChar>,
                                            TextDirection,
                                            bool& is_after_expansion);

  static bool IsUprightInMixedVertical(UChar32 character);

  // https://html.spec.whatwg.org/C/#prod-potentialcustomelementname
  static bool IsPotentialCustomElementName8BitChar(LChar ch) {
    return IsASCIILower(ch) || IsASCIIDigit(ch) || ch == '-' || ch == '.' ||
           ch == '_' || ch == 0xb7 || (0xc0 <= ch && ch != 0xd7 && ch != 0xf7);
  }
  static bool IsPotentialCustomElementNameChar(UChar32 character);

  // http://unicode.org/reports/tr9/#Directional_Formatting_Characters
  static bool IsBidiControl(UChar32 character);
  static bool MaybeBidiRtlUtf16(base::StrictNumeric<UChar> ch);
  static bool MaybeBidiRtl(UChar32 ch);
  static bool MaybeBidiRtl(const String&);

  static HanKerningCharType GetHanKerningCharType(UChar32 character);
  static EastAsianSpacingType GetEastAsianSpacingType(UChar32 character);
  // Check the `HanKerningCharType` of a character without knowing the font.
  // It depends on fonts, so it may not be `kOpen` or `kClose` even when this
  // function returns `true`. See `HanKerning::GetCharType`.
  static bool MaybeHanKerningOpen(UChar32 ch) {
    return MaybeHanKerningOpenOrCloseFast(ch) && MaybeHanKerningOpenSlow(ch);
  }
  static bool MaybeHanKerningClose(UChar32 ch) {
    return MaybeHanKerningOpenOrCloseFast(ch) && MaybeHanKerningCloseSlow(ch);
  }
  // Check if the character may be `kOpen` or `kClose` only by ranges, without
  // getting the Unicode property. Faster than `MaybeHanKerningOpen` and
  // `MaybeHanKerningClose` but has more cases where it returns `true` for other
  // characters.
  static bool MaybeHanKerningOpenOrCloseFast(UChar32 character) {
    return IsInRange(character, kLeftSingleQuotationMarkCharacter, 0x301F) ||
           IsInRange(character, 0xFF08, 0xFF60);
  }

  // Collapsible white space characters defined in CSS:
  // https://drafts.csswg.org/css-text-3/#collapsible-white-space
  static bool IsCollapsibleSpace(UChar c) {
    return c == kSpaceCharacter || c == kNewlineCharacter ||
           c == kTabulationCharacter || c == kCarriageReturnCharacter;
  }
  static bool IsLineFeed(UChar c) { return c == kNewlineCharacter; }
  template <typename CharacterType>
  static bool IsOtherSpaceSeparator(CharacterType c) {
    return c == kIdeographicSpaceCharacter;
  }
  static bool TreatAsSpace(UChar32 c) {
    return c == kSpaceCharacter || c == kTabulationCharacter ||
           c == kNewlineCharacter || c == kNoBreakSpaceCharacter;
  }
  static bool TreatAsZeroWidthSpace(UChar32 c) {
    return TreatAsZeroWidthSpaceInComplexScript(c) ||
           c == kZeroWidthNonJoinerCharacter || c == kZeroWidthJoinerCharacter;
  }
  static bool TreatAsZeroWidthSpaceInComplexScriptLegacy(UChar32 c) {
    return c == kFormFeedCharacter || c == kCarriageReturnCharacter ||
           c == kSoftHyphenCharacter || c == kZeroWidthSpaceCharacter ||
           (c >= kLeftToRightMarkCharacter && c <= kRightToLeftMarkCharacter) ||
           (c >= kLeftToRightEmbedCharacter &&
            c <= kRightToLeftOverrideCharacter) ||
           c == kZeroWidthNoBreakSpaceCharacter ||
           c == kObjectReplacementCharacter;
  }
  static bool TreatAsZeroWidthSpaceInComplexScript(UChar32 c) {
    if (c == kFormFeedCharacter || c == kCarriageReturnCharacter ||
        c == kObjectReplacementCharacter) {
      return true;
    }
    return IsDefaultIgnorable(c);
  }
  // https://unicode.org/reports/tr44/#Default_Ignorable_Code_Point
  static bool IsDefaultIgnorable(UChar32 c) {
    if (c < 0x0100) {
      return c == kSoftHyphenCharacter;
    }
    return u_hasBinaryProperty(c, UCHAR_DEFAULT_IGNORABLE_CODE_POINT);
  }
  static bool CanTextDecorationSkipInk(UChar32);
  static bool CanReceiveTextEmphasis(UChar32);

  static bool IsGraphemeExtended(UChar32 c) {
    // http://unicode.org/reports/tr29/#Extend
    return u_hasBinaryProperty(c, UCHAR_GRAPHEME_EXTEND);
  }

  // Returns true if the character has a Emoji property.
  // See http://www.unicode.org/Public/emoji/3.0/emoji-data.txt
  static bool IsEmoji(UChar32);
  // Default presentation style according to:
  // http://www.unicode.org/reports/tr51/#Presentation_Style
  static bool IsEmojiTextDefault(UChar32);
  static bool IsEmojiEmojiDefault(UChar32);
  static bool IsEmojiModifierBase(UChar32);
  static inline bool IsEmojiKeycapBase(UChar32 ch) {
    return (ch >= '0' && ch <= '9') || ch == '#' || ch == '*';
  }
  static bool IsRegionalIndicator(UChar32);
  static bool IsModifier(UChar32 c) { return c >= 0x1F3FB && c <= 0x1F3FF; }
  // http://www.unicode.org/reports/tr51/proposed.html#flag-emoji-tag-sequences
  static bool IsEmojiTagSequence(UChar32);
  static bool IsEmojiComponent(UChar32);
  static bool IsExtendedPictographic(UChar32);
  static bool MaybeEmojiPresentation(UChar32);

  static bool IsStandardizedVariationSequence(UChar32, UChar32);
  static bool IsEmojiVariationSequence(UChar32, UChar32);
  static bool IsIdeographicVariationSequence(UChar32 ch, UChar32 vs);
  static bool IsVariationSequence(UChar32, UChar32);

  static inline bool IsNormalizedCanvasSpaceCharacter(UChar32 c) {
    // According to specification all space characters should be replaced with
    // 0x0020 space character.
    // http://www.whatwg.org/specs/web-apps/current-work/multipage/the-canvas-element.html#text-preparation-algorithm
    // The space characters according to specification are : U+0020, U+0009,
    // U+000A, U+000C, and U+000D.
    // http://www.whatwg.org/specs/web-apps/current-work/multipage/common-microsyntaxes.html#space-character
    // This function returns true for 0x000B also, so that this is backward
    // compatible.  Otherwise, the test
    // web_tests/canvas/philip/tests/2d.text.draw.space.collapse.space.html
    // will fail
    return c == 0x0009 || (c >= 0x000A && c <= 0x000D);
  }

  static bool IsCommonOrInheritedScript(UChar32);
  static bool IsPrivateUse(UChar32);
  static bool IsNonCharacter(UChar32);

  // Returns whether a script code could be determined for the given character
  // and that script code is not USCRIPT_COMMON or USCRIPT_INHERITED.
  static bool HasDefiniteScript(UChar32);

  static bool IsModernGeorgianUppercase(UChar32 c) {
    return IsInRange(c, 0x1C90, 0x1CBF);
  }

  // Map modern secular Georgian uppercase letters added in Unicode
  // 11.0 to their corresponding lowercase letters.
  // https://www.unicode.org/charts/PDF/U10A0.pdf
  // https://www.unicode.org/charts/PDF/U1C90.pdf
  static UChar32 LowercaseModernGeorgianUppercase(UChar32 c) {
    return (c - (0x1C90 - 0x10D0));
  }

  static bool IsVerticalMathCharacter(UChar32);

 private:
  FRIEND_TEST_ALL_PREFIXES(CharacterTest, Derived);

  static bool IsCJKIdeographOrSymbolSlow(UChar32);
  static bool IsHangulSlow(UChar32);
  static bool MaybeHanKerningOpenSlow(UChar32);
  static bool MaybeHanKerningCloseSlow(UChar32);
  static void ApplyPatternAndFreezeIfEmpty(icu::UnicodeSet* unicodeSet,
                                           const char* pattern);
};

// True if the character may need the Bidi reordering. If false, the
// `Bidi_Class` of `ch` isn't `R`, `AL`, nor Bidi controls.
// https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5B%5B%3Abc%3DR%3A%5D%5B%3Abc%3DAL%3A%5D%5D&g=bc
// https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=[:Bidi_C:]
//
// This function assumes all non-BMP characters may be Bidi.
inline bool Character::MaybeBidiRtlUtf16(base::StrictNumeric<UChar> ch) {
  return ch >= 0x0590 &&
         // `InlineItemsBuilder` may emit U+200B Zero Width Space.
         ch != kZeroWidthSpaceCharacter &&
         // General Punctuation such as curly quotes.
         !IsInRange(ch, 0x2010, 0x2029) &&
         // CJK etc., up to Surrogate Pairs.
         !IsInRange(ch, 0x206A, 0xD7FF) &&
         // Common in CJK.
         !IsInRange(ch, 0xFF00, 0xFFFF);
}

inline bool Character::MaybeBidiRtl(UChar32 ch) {
  return ch >= 0x0590 &&
         // `InlineItemsBuilder` may emit U+200B Zero Width Space.
         ch != kZeroWidthSpaceCharacter &&
         // General Punctuation such as curly quotes.
         !IsInRange(ch, 0x2010, 0x2029) &&
         // CJK etc., up to Surrogate Pairs.
         !IsInRange(ch, 0x206A, 0xD7FF) &&
         // Common in CJK.
         !IsInRange(ch, 0xFF00, 0xFFFF) &&
         // Kana Extended-B, Kana Supplement, Kana Extended-A, Small Kana
         // Extension
         !IsInRange(ch, 0x1AFF0, 0x1B16F) &&
         // CJK Ideographs Extensions
         !IsInRange(ch, 0x20000, 0x323AF);
}

inline bool Character::MaybeBidiRtl(const String& text) {
  return !text.Is8Bit() && !text.IsAllSpecialCharacters<[](UChar c) {
    return !MaybeBidiRtlUtf16(c);
  }>();
}

inline bool Character::IsEastAsianWidthFullwidth(UChar32 ch) {
  // All EAW=F characters are in the "Halfwidth and Fullwidth forms" block,
  // except U+3000 IDEOGRAPHIC SPACE.
  // https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=[:ea=F:]
  return ch == kIdeographicSpaceCharacter ||
         (IsBlockHalfwidthAndFullwidthForms(ch) &&
          EastAsianWidth(ch) == UEastAsianWidth::U_EA_FULLWIDTH);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_CHARACTER_H_
