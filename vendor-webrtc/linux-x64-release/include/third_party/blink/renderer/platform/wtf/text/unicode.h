/*
 *  Copyright (C) 2006 George Staikos <staikos@kde.org>
 *  Copyright (C) 2006, 2008, 2009 Apple Inc. All rights reserved.
 *  Copyright (C) 2007-2009 Torch Mobile, Inc.
 *
 *  This library is free software; you can redistribute it and/or
 *  modify it under the terms of the GNU Library General Public
 *  License as published by the Free Software Foundation; either
 *  version 2 of the License, or (at your option) any later version.
 *
 *  This library is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 *  Library General Public License for more details.
 *
 *  You should have received a copy of the GNU Library General Public License
 *  along with this library; see the file COPYING.LIB.  If not, write to
 *  the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 *  Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_UNICODE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_UNICODE_H_

#include <unicode/uchar.h>

#include "third_party/blink/renderer/platform/wtf/text/ascii_ctype.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_uchar.h"

namespace WTF {
namespace unicode {

enum CharDirection {
  kLeftToRight = U_LEFT_TO_RIGHT,
  kRightToLeft = U_RIGHT_TO_LEFT,
  kEuropeanNumber = U_EUROPEAN_NUMBER,
  kEuropeanNumberSeparator = U_EUROPEAN_NUMBER_SEPARATOR,
  kEuropeanNumberTerminator = U_EUROPEAN_NUMBER_TERMINATOR,
  kArabicNumber = U_ARABIC_NUMBER,
  kCommonNumberSeparator = U_COMMON_NUMBER_SEPARATOR,
  kBlockSeparator = U_BLOCK_SEPARATOR,
  kSegmentSeparator = U_SEGMENT_SEPARATOR,
  kWhiteSpaceNeutral = U_WHITE_SPACE_NEUTRAL,
  kOtherNeutral = U_OTHER_NEUTRAL,
  kLeftToRightEmbedding = U_LEFT_TO_RIGHT_EMBEDDING,
  kLeftToRightOverride = U_LEFT_TO_RIGHT_OVERRIDE,
  kRightToLeftArabic = U_RIGHT_TO_LEFT_ARABIC,
  kRightToLeftEmbedding = U_RIGHT_TO_LEFT_EMBEDDING,
  kRightToLeftOverride = U_RIGHT_TO_LEFT_OVERRIDE,
  kPopDirectionalFormat = U_POP_DIRECTIONAL_FORMAT,
  kNonSpacingMark = U_DIR_NON_SPACING_MARK,
  kBoundaryNeutral = U_BOUNDARY_NEUTRAL
};

enum CharDecompositionType {
  kDecompositionNone = U_DT_NONE,
  kDecompositionCanonical = U_DT_CANONICAL,
  kDecompositionCompat = U_DT_COMPAT,
  kDecompositionCircle = U_DT_CIRCLE,
  kDecompositionFinal = U_DT_FINAL,
  kDecompositionFont = U_DT_FONT,
  kDecompositionFraction = U_DT_FRACTION,
  kDecompositionInitial = U_DT_INITIAL,
  kDecompositionIsolated = U_DT_ISOLATED,
  kDecompositionMedial = U_DT_MEDIAL,
  kDecompositionNarrow = U_DT_NARROW,
  kDecompositionNoBreak = U_DT_NOBREAK,
  kDecompositionSmall = U_DT_SMALL,
  kDecompositionSquare = U_DT_SQUARE,
  kDecompositionSub = U_DT_SUB,
  kDecompositionSuper = U_DT_SUPER,
  kDecompositionVertical = U_DT_VERTICAL,
  kDecompositionWide = U_DT_WIDE,
};

enum CharCategory {
  kNoCategory = 0,
  kOther_NotAssigned = U_MASK(U_GENERAL_OTHER_TYPES),
  kLetter_Uppercase = U_MASK(U_UPPERCASE_LETTER),
  kLetter_Lowercase = U_MASK(U_LOWERCASE_LETTER),
  kLetter_Titlecase = U_MASK(U_TITLECASE_LETTER),
  kLetter_Modifier = U_MASK(U_MODIFIER_LETTER),
  kLetter_Other = U_MASK(U_OTHER_LETTER),

  kMark_NonSpacing = U_MASK(U_NON_SPACING_MARK),
  kMark_Enclosing = U_MASK(U_ENCLOSING_MARK),
  kMark_SpacingCombining = U_MASK(U_COMBINING_SPACING_MARK),

  kNumber_DecimalDigit = U_MASK(U_DECIMAL_DIGIT_NUMBER),
  kNumber_Letter = U_MASK(U_LETTER_NUMBER),
  kNumber_Other = U_MASK(U_OTHER_NUMBER),

  kSeparator_Space = U_MASK(U_SPACE_SEPARATOR),
  kSeparator_Line = U_MASK(U_LINE_SEPARATOR),
  kSeparator_Paragraph = U_MASK(U_PARAGRAPH_SEPARATOR),

  kOther_Control = U_MASK(U_CONTROL_CHAR),
  kOther_Format = U_MASK(U_FORMAT_CHAR),
  kOther_PrivateUse = U_MASK(U_PRIVATE_USE_CHAR),
  kOther_Surrogate = U_MASK(U_SURROGATE),

  kPunctuation_Dash = U_MASK(U_DASH_PUNCTUATION),
  kPunctuation_Open = U_MASK(U_START_PUNCTUATION),
  kPunctuation_Close = U_MASK(U_END_PUNCTUATION),
  kPunctuation_Connector = U_MASK(U_CONNECTOR_PUNCTUATION),
  kPunctuation_Other = U_MASK(U_OTHER_PUNCTUATION),

  kSymbol_Math = U_MASK(U_MATH_SYMBOL),
  kSymbol_Currency = U_MASK(U_CURRENCY_SYMBOL),
  kSymbol_Modifier = U_MASK(U_MODIFIER_SYMBOL),
  kSymbol_Other = U_MASK(U_OTHER_SYMBOL),

  kPunctuation_InitialQuote = U_MASK(U_INITIAL_PUNCTUATION),
  kPunctuation_FinalQuote = U_MASK(U_FINAL_PUNCTUATION)
};

inline UChar32 FoldCase(UChar32 c) {
  return u_foldCase(c, U_FOLD_CASE_DEFAULT);
}

inline UChar32 ToLower(UChar32 c) {
  return u_tolower(c);
}

inline UChar32 ToUpper(UChar32 c) {
  return u_toupper(c);
}

inline UChar32 ToTitleCase(UChar32 c) {
  return u_totitle(c);
}

inline bool IsAlphanumeric(UChar32 c) {
  return !!u_isalnum(c);
}

inline bool IsPrintableChar(UChar32 c) {
  return !!u_isprint(c);
}

inline bool IsPunct(UChar32 c) {
  return !!u_ispunct(c);
}

inline bool HasLineBreakingPropertyComplexContext(UChar32 c) {
  return u_getIntPropertyValue(c, UCHAR_LINE_BREAK) == U_LB_COMPLEX_CONTEXT;
}

inline CharCategory Category(UChar32 c) {
  return static_cast<CharCategory>(U_GET_GC_MASK(c));
}

inline bool IsSymbol(UChar32 c) {
  CharCategory char_category = Category(c);
  return char_category == kSymbol_Math || char_category == kSymbol_Currency ||
         char_category == kSymbol_Modifier || char_category == kSymbol_Other;
}

inline CharDirection Direction(UChar32 c) {
  return static_cast<CharDirection>(u_charDirection(c));
}

inline bool IsLower(UChar32 c) {
  return !!u_islower(c);
}

inline bool IsUpper(UChar32 c) {
  return !!u_isupper(c);
}

inline CharDecompositionType DecompositionType(UChar32 c) {
  return static_cast<CharDecompositionType>(
      u_getIntPropertyValue(c, UCHAR_DECOMPOSITION_TYPE));
}

inline bool IsSpaceOrNewline(UChar c) {
  // Use IsASCIISpace() for basic Latin-1.
  // This will include newlines, which aren't included in Unicode DirWS.
  return c <= 0x7F
             ? WTF::IsASCIISpace(c)
             : WTF::unicode::Direction(c) == WTF::unicode::kWhiteSpaceNeutral;
}

}  // namespace unicode
}  // namespace WTF

using WTF::unicode::IsSpaceOrNewline;

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_UNICODE_H_
