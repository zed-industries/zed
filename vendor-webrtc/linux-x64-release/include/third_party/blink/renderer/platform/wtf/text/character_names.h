/*
 * Copyright (C) 2007, 2009, 2010 Apple Inc. All rights reserved.
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
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_CHARACTER_NAMES_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_CHARACTER_NAMES_H_

#include "third_party/blink/renderer/platform/wtf/text/wtf_uchar.h"

namespace WTF {
namespace unicode {

// Names here are taken from the Unicode standard.

// Most of these are UChar constants, not UChar32, which makes them
// more convenient for WebCore code that mostly uses UTF-16.

const UChar kActivateArabicFormShapingCharacter = 0x206D;
const UChar kActivateSymmetricSwappingCharacter = 0x206B;
const UChar32 kAegeanWordSeparatorLineCharacter = 0x10100;
const UChar32 kAegeanWordSeparatorDotCharacter = 0x10101;
const UChar kArabicLetterMarkCharacter = 0x061C;
const UChar32 kArabicMathematicalOperatorMeemWithHahWithTatweel = 0x1EEF0;
const UChar32 kArabicMathematicalOperatorHahWithDal = 0x1EEF1;
const UChar kBlackCircleCharacter = 0x25CF;
const UChar kBlackDownPointingSmallTriangle = 0x25BE;
const UChar kBlackRightPointingSmallTriangle = 0x25B8;
const UChar kBlackSquareCharacter = 0x25A0;
const UChar kBlackUpPointingTriangleCharacter = 0x25B2;
const UChar kBulletCharacter = 0x2022;
const UChar kBullseyeCharacter = 0x25CE;
const UChar32 kCancelTag = 0xE007F;
const UChar kCarriageReturnCharacter = 0x000D;
const UChar kCjkWaterCharacter = 0x6C34;
const UChar kColon = 0x3A;
const UChar kCombiningAcuteAccentCharacter = 0x0301;
const UChar kCombiningEnclosingCircleBackslashCharacter = 0x20E0;
const UChar kCombiningEnclosingKeycapCharacter = 0x20E3;
const UChar kCombiningLongSolidusOverlay = 0x0338;
const UChar kCombiningLongVerticalLineOverlay = 0x20D2;
const UChar kCombiningMinusSignBelow = 0x0320;
const UChar kComma = 0x2C;
const UChar kDeleteCharacter = 0x007F;
const UChar kDoubleStruckItalicCapitalDCharacter = 0x2145;
const UChar kDoubleStruckItalicSmallDCharacter = 0x2146;
const UChar kEnQuadCharacter = 0x2000;
const UChar kEthiopicNumberHundredCharacter = 0x137B;
const UChar kEthiopicNumberTenThousandCharacter = 0x137C;
const UChar kEthiopicPrefaceColonCharacter = 0x1366;
const UChar kEthiopicWordspaceCharacter = 0x1361;
const UChar kHeavyBlackHeartCharacter = 0x2764;
const UChar32 kEyeCharacter = 0x1F441;
const UChar32 kBoyCharacter = 0x1F466;
const UChar32 kGirlCharacter = 0x1F467;
const UChar32 kManCharacter = 0x1F468;
const UChar32 kWomanCharacter = 0x1F469;
const UChar32 kKissMarkCharacter = 0x1F48B;
const UChar32 kFamilyCharacter = 0x1F46A;
const UChar kFemaleSignCharacter = 0x2640;
const UChar kFirstStrongIsolateCharacter = 0x2068;
const UChar kFisheyeCharacter = 0x25C9;
const UChar kFourthRootCharacter = 0x221C;
const UChar kFullstopCharacter = 0x002E;
const UChar kFullwidthColon = 0xFF1A;
const UChar kFullwidthComma = 0xFF0C;
const UChar kFullwidthExclamationMark = 0xFF01;
const UChar kFullwidthFullStop = 0xFF0E;
const UChar kFullwidthSemicolon = 0xFF1B;
const UChar kGreekCapitalReversedDottedLunateSigmaSymbol = 0x03FF;
const UChar kGreekKappaSymbol = 0x03F0;
const UChar kGreekLetterDigamma = 0x03DC;
const UChar kGreekLowerAlpha = 0x03B1;
const UChar kGreekLowerOmega = 0x03C9;
const UChar kGreekLunateEpsilonSymbol = 0x03F5;
const UChar kGreekPhiSymbol = 0x03D5;
const UChar kGreekPiSymbol = 0x03D6;
const UChar kGreekRhoSymbol = 0x03F1;
const UChar kGreekSmallLetterDigamma = 0x03DD;
const UChar kGreekThetaSymbol = 0x03D1;
const UChar kGreekUpperAlpha = 0x0391;
const UChar kGreekUpperOmega = 0x03A9;
const UChar kGreekUpperTheta = 0x03F4;
const UChar kHebrewPunctuationGereshCharacter = 0x05F3;
const UChar kHebrewPunctuationGershayimCharacter = 0x05F4;
const UChar kHellschreiberPauseSymbol = 0x2BFF;
const UChar kHiraganaLetterACharacter = 0x3042;
const UChar kHiraganaLetterSmallACharacter = 0x3041;
const UChar kHoleGreekUpperTheta = 0x03A2;
const UChar kHorizontalEllipsisCharacter = 0x2026;
const UChar kHyphenationPointCharacter = 0x2027;
const UChar kHyphenCharacter = 0x2010;
const UChar kHyphenMinusCharacter = 0x002D;
const UChar kIdeographicCommaCharacter = 0x3001;
const UChar kIdeographicFullStopCharacter = 0x3002;
const UChar kIdeographicSpaceCharacter = 0x3000;
const UChar kInhibitArabicFormShapingCharacter = 0x206C;
const UChar kInhibitSymmetricSwappingCharacter = 0x206A;
const UChar kKatakanaMiddleDot = 0x30FB;
const UChar kLatinCapitalLetterIWithDotAbove = 0x0130;
const UChar kLatinSmallLetterDotlessI = 0x0131;
const UChar kLatinSmallLetterDotlessJ = 0x0237;
const UChar kLeftCornerBracket = 0x300C;
const UChar kLeftDoubleQuotationMarkCharacter = 0x201C;
const UChar kLeftSingleQuotationMarkCharacter = 0x2018;
const UChar32 kLeftSpeechBubbleCharacter = 0x1F5E8;
const UChar kLeftToRightEmbedCharacter = 0x202A;
const UChar kLeftToRightIsolateCharacter = 0x2066;
const UChar kLeftToRightMarkCharacter = 0x200E;
const UChar kLeftToRightOverrideCharacter = 0x202D;
const UChar kLineSeparator = 0x2028;
const UChar kLineTabulationCharacter = 0x000B;
const UChar kLowLineCharacter = 0x005F;
const UChar kMaleSignCharacter = 0x2642;
const UChar32 kMathItalicSmallDotlessI = 0x1D6A4;
const UChar32 kMathItalicSmallDotlessJ = 0x1D6A5;
const UChar32 kMathBoldEpsilonSymbol = 0x1D6DC;
const UChar32 kMathBoldKappaSymbol = 0x1D6DE;
const UChar32 kMathBoldNabla = 0x1D6C1;
const UChar32 kMathBoldPartialDifferential = 0x1D6DB;
const UChar32 kMathBoldPhiSymbol = 0x1D6DF;
const UChar32 kMathBoldPiSymbol = 0x1D6E1;
const UChar32 kMathBoldRhoSymbol = 0x1D6E0;
const UChar32 kMathBoldSmallA = 0x1D41A;
const UChar32 kMathBoldSmallAlpha = 0x1D6C2;
const UChar32 kMathBoldSmallDigamma = 0x1D7CB;
const UChar32 kMathBoldUpperA = 0x1D400;
const UChar32 kMathBoldUpperAlpha = 0x1D6A8;
const UChar32 kMathBoldUpperTheta = 0x1D6B9;
const UChar32 kMathBoldThetaSymbol = 0x1D6DD;
const UChar32 kMathItalicUpperA = 0x1D434;
const UChar32 kMathItalicUpperAlpha = 0x1D6E2;
const UChar kMiddleDotCharacter = 0x00B7;
const UChar kMinusSignCharacter = 0x2212;
const UChar kMongolianFreeVariationSelectorTwo = 0x180C;
const UChar kMongolianLetterA = 0x1820;
const UChar kNewlineCharacter = 0x000A;
const UChar kNonBreakingHyphen = 0x2011;
const UChar32 kNonCharacter = 0xFFFF;
const UChar kFormFeedCharacter = 0x000C;
const UChar32 kNabla = 0x2207;
const UChar kNationalDigitShapesCharacter = 0x206E;
const UChar kNominalDigitShapesCharacter = 0x206F;
const UChar kNoBreakSpaceCharacter = 0x00A0;
const UChar kObjectReplacementCharacter = 0xFFFC;
const UChar kOverlineCharacter = 0x203E;
const UChar kParagraphSeparator = 0x2029;
const UChar32 kPartialDifferential = 0x2202;
const UChar kPopDirectionalFormattingCharacter = 0x202C;
const UChar kPopDirectionalIsolateCharacter = 0x2069;
const UChar32 kRainbowCharacter = 0x1F308;
const UChar kReplacementCharacter = 0xFFFD;
const UChar kReverseSolidusCharacter = 0x005C;
const UChar kRightDoubleQuotationMarkCharacter = 0x201D;
const UChar kRightSingleQuotationMarkCharacter = 0x2019;
const UChar kRightToLeftEmbedCharacter = 0x202B;
const UChar kRightToLeftIsolateCharacter = 0x2067;
const UChar kRightToLeftMarkCharacter = 0x200F;
const UChar kRightToLeftOverrideCharacter = 0x202E;
const UChar kSemiColon = 0x3B;
const UChar kSesameDotCharacter = 0xFE45;
const UChar32 kShakingFaceEmoji = 0x1FAE8;
const UChar kSmallLetterSharpSCharacter = 0x00DF;
const UChar kSolidusCharacter = 0x002F;
const UChar kSoftHyphenCharacter = 0x00AD;
const UChar kSpaceCharacter = 0x0020;
const UChar kDigitZeroCharacter = 0x0030;
const UChar32 kSquareRootCharacter = 0x221A;
const UChar kStaffOfAesculapiusCharacter = 0x2695;
const UChar kTabulationCharacter = 0x0009;
const UChar32 kTagDigitZero = 0xE0030;
const UChar32 kTagDigitNine = 0xE0039;
const UChar32 kTagLatinSmallLetterA = 0xE0061;
const UChar32 kTagLatinSmallLetterZ = 0xE007A;
const UChar kTibetanMarkIntersyllabicTshegCharacter = 0x0F0B;
const UChar kTibetanMarkDelimiterTshegBstarCharacter = 0x0F0C;
const UChar kTildeOperatorCharacter = 0x223C;
const UChar32 kUgariticWordDividerCharacter = 0x1039F;
const UChar kVariationSelector15Character = 0xFE0E;
const UChar kVariationSelector16Character = 0xFE0F;
const UChar kVariationSelector2Character = 0xFE01;
const UChar kVerticalLineCharacter = 0x7C;
const UChar32 kWavingWhiteFlagCharacter = 0x1F3F3;
const UChar kWhiteBulletCharacter = 0x25E6;
const UChar kWhiteCircleCharacter = 0x25CB;
const UChar kWhiteSesameDotCharacter = 0xFE46;
const UChar kWhiteUpPointingTriangleCharacter = 0x25B3;
const UChar kYenSignCharacter = 0x00A5;
const UChar kZeroWidthJoinerCharacter = 0x200D;
const UChar kZeroWidthNonJoinerCharacter = 0x200C;
const UChar kZeroWidthSpaceCharacter = 0x200B;
const UChar kZeroWidthNoBreakSpaceCharacter = 0xFEFF;
const UChar kPrivateUseFirstCharacter = 0xE000;
const UChar kPrivateUseLastCharacter = 0xF8FF;
const UChar32 kMaxCodepoint = 0x10ffff;

}  // namespace unicode
}  // namespace WTF

using WTF::unicode::kActivateArabicFormShapingCharacter;
using WTF::unicode::kActivateSymmetricSwappingCharacter;
using WTF::unicode::kAegeanWordSeparatorDotCharacter;
using WTF::unicode::kAegeanWordSeparatorLineCharacter;
using WTF::unicode::kArabicLetterMarkCharacter;
using WTF::unicode::kArabicMathematicalOperatorHahWithDal;
using WTF::unicode::kArabicMathematicalOperatorMeemWithHahWithTatweel;
using WTF::unicode::kBlackCircleCharacter;
using WTF::unicode::kBlackDownPointingSmallTriangle;
using WTF::unicode::kBlackRightPointingSmallTriangle;
using WTF::unicode::kBlackSquareCharacter;
using WTF::unicode::kBlackUpPointingTriangleCharacter;
using WTF::unicode::kBulletCharacter;
using WTF::unicode::kBullseyeCharacter;
using WTF::unicode::kCancelTag;
using WTF::unicode::kCarriageReturnCharacter;
using WTF::unicode::kCjkWaterCharacter;
using WTF::unicode::kColon;
using WTF::unicode::kCombiningAcuteAccentCharacter;
using WTF::unicode::kCombiningEnclosingCircleBackslashCharacter;
using WTF::unicode::kCombiningEnclosingKeycapCharacter;
using WTF::unicode::kCombiningLongSolidusOverlay;
using WTF::unicode::kCombiningLongVerticalLineOverlay;
using WTF::unicode::kCombiningMinusSignBelow;
using WTF::unicode::kComma;
using WTF::unicode::kDigitZeroCharacter;
using WTF::unicode::kDoubleStruckItalicCapitalDCharacter;
using WTF::unicode::kDoubleStruckItalicSmallDCharacter;
using WTF::unicode::kEnQuadCharacter;
using WTF::unicode::kEthiopicNumberHundredCharacter;
using WTF::unicode::kEthiopicNumberTenThousandCharacter;
using WTF::unicode::kEthiopicPrefaceColonCharacter;
using WTF::unicode::kEthiopicWordspaceCharacter;
using WTF::unicode::kEyeCharacter;
using WTF::unicode::kFamilyCharacter;
using WTF::unicode::kFemaleSignCharacter;
using WTF::unicode::kFirstStrongIsolateCharacter;
using WTF::unicode::kFisheyeCharacter;
using WTF::unicode::kFormFeedCharacter;
using WTF::unicode::kFourthRootCharacter;
using WTF::unicode::kFullstopCharacter;
using WTF::unicode::kFullwidthColon;
using WTF::unicode::kFullwidthComma;
using WTF::unicode::kFullwidthExclamationMark;
using WTF::unicode::kFullwidthFullStop;
using WTF::unicode::kFullwidthSemicolon;
using WTF::unicode::kGreekCapitalReversedDottedLunateSigmaSymbol;
using WTF::unicode::kGreekKappaSymbol;
using WTF::unicode::kGreekLetterDigamma;
using WTF::unicode::kGreekLowerAlpha;
using WTF::unicode::kGreekLowerOmega;
using WTF::unicode::kGreekLunateEpsilonSymbol;
using WTF::unicode::kGreekPhiSymbol;
using WTF::unicode::kGreekPiSymbol;
using WTF::unicode::kGreekRhoSymbol;
using WTF::unicode::kGreekSmallLetterDigamma;
using WTF::unicode::kGreekThetaSymbol;
using WTF::unicode::kGreekUpperAlpha;
using WTF::unicode::kGreekUpperOmega;
using WTF::unicode::kGreekUpperTheta;
using WTF::unicode::kHebrewPunctuationGereshCharacter;
using WTF::unicode::kHebrewPunctuationGershayimCharacter;
using WTF::unicode::kHellschreiberPauseSymbol;
using WTF::unicode::kHiraganaLetterSmallACharacter;
using WTF::unicode::kHoleGreekUpperTheta;
using WTF::unicode::kHorizontalEllipsisCharacter;
using WTF::unicode::kHyphenationPointCharacter;
using WTF::unicode::kHyphenCharacter;
using WTF::unicode::kHyphenMinusCharacter;
using WTF::unicode::kIdeographicCommaCharacter;
using WTF::unicode::kIdeographicFullStopCharacter;
using WTF::unicode::kIdeographicSpaceCharacter;
using WTF::unicode::kInhibitArabicFormShapingCharacter;
using WTF::unicode::kInhibitSymmetricSwappingCharacter;
using WTF::unicode::kKatakanaMiddleDot;
using WTF::unicode::kLatinCapitalLetterIWithDotAbove;
using WTF::unicode::kLatinSmallLetterDotlessI;
using WTF::unicode::kLatinSmallLetterDotlessJ;
using WTF::unicode::kLeftCornerBracket;
using WTF::unicode::kLeftDoubleQuotationMarkCharacter;
using WTF::unicode::kLeftSingleQuotationMarkCharacter;
using WTF::unicode::kLeftSpeechBubbleCharacter;
using WTF::unicode::kLeftToRightEmbedCharacter;
using WTF::unicode::kLeftToRightIsolateCharacter;
using WTF::unicode::kLeftToRightMarkCharacter;
using WTF::unicode::kLeftToRightOverrideCharacter;
using WTF::unicode::kLineSeparator;
using WTF::unicode::kLowLineCharacter;
using WTF::unicode::kMaleSignCharacter;
using WTF::unicode::kMathBoldEpsilonSymbol;
using WTF::unicode::kMathBoldKappaSymbol;
using WTF::unicode::kMathBoldNabla;
using WTF::unicode::kMathBoldPartialDifferential;
using WTF::unicode::kMathBoldPhiSymbol;
using WTF::unicode::kMathBoldPiSymbol;
using WTF::unicode::kMathBoldRhoSymbol;
using WTF::unicode::kMathBoldSmallA;
using WTF::unicode::kMathBoldSmallAlpha;
using WTF::unicode::kMathBoldSmallDigamma;
using WTF::unicode::kMathBoldThetaSymbol;
using WTF::unicode::kMathBoldUpperA;
using WTF::unicode::kMathBoldUpperAlpha;
using WTF::unicode::kMathBoldUpperTheta;
using WTF::unicode::kMathItalicSmallDotlessI;
using WTF::unicode::kMathItalicSmallDotlessJ;
using WTF::unicode::kMathItalicUpperA;
using WTF::unicode::kMathItalicUpperAlpha;
using WTF::unicode::kMaxCodepoint;
using WTF::unicode::kMiddleDotCharacter;
using WTF::unicode::kMinusSignCharacter;
using WTF::unicode::kMongolianFreeVariationSelectorTwo;
using WTF::unicode::kMongolianLetterA;
using WTF::unicode::kNabla;
using WTF::unicode::kNationalDigitShapesCharacter;
using WTF::unicode::kNewlineCharacter;
using WTF::unicode::kNoBreakSpaceCharacter;
using WTF::unicode::kNominalDigitShapesCharacter;
using WTF::unicode::kNonBreakingHyphen;
using WTF::unicode::kNonCharacter;
using WTF::unicode::kObjectReplacementCharacter;
using WTF::unicode::kOverlineCharacter;
using WTF::unicode::kParagraphSeparator;
using WTF::unicode::kPartialDifferential;
using WTF::unicode::kPopDirectionalFormattingCharacter;
using WTF::unicode::kPopDirectionalIsolateCharacter;
using WTF::unicode::kPrivateUseFirstCharacter;
using WTF::unicode::kPrivateUseLastCharacter;
using WTF::unicode::kRainbowCharacter;
using WTF::unicode::kReplacementCharacter;
using WTF::unicode::kReverseSolidusCharacter;
using WTF::unicode::kRightDoubleQuotationMarkCharacter;
using WTF::unicode::kRightSingleQuotationMarkCharacter;
using WTF::unicode::kRightToLeftEmbedCharacter;
using WTF::unicode::kRightToLeftIsolateCharacter;
using WTF::unicode::kRightToLeftMarkCharacter;
using WTF::unicode::kRightToLeftOverrideCharacter;
using WTF::unicode::kSemiColon;
using WTF::unicode::kSesameDotCharacter;
using WTF::unicode::kShakingFaceEmoji;
using WTF::unicode::kSmallLetterSharpSCharacter;
using WTF::unicode::kSoftHyphenCharacter;
using WTF::unicode::kSolidusCharacter;
using WTF::unicode::kSpaceCharacter;
using WTF::unicode::kSquareRootCharacter;
using WTF::unicode::kStaffOfAesculapiusCharacter;
using WTF::unicode::kTabulationCharacter;
using WTF::unicode::kTagDigitNine;
using WTF::unicode::kTagDigitZero;
using WTF::unicode::kTagLatinSmallLetterA;
using WTF::unicode::kTagLatinSmallLetterZ;
using WTF::unicode::kTibetanMarkDelimiterTshegBstarCharacter;
using WTF::unicode::kTibetanMarkIntersyllabicTshegCharacter;
using WTF::unicode::kTildeOperatorCharacter;
using WTF::unicode::kUgariticWordDividerCharacter;
using WTF::unicode::kVariationSelector15Character;
using WTF::unicode::kVariationSelector16Character;
using WTF::unicode::kVariationSelector2Character;
using WTF::unicode::kVerticalLineCharacter;
using WTF::unicode::kWavingWhiteFlagCharacter;
using WTF::unicode::kWhiteBulletCharacter;
using WTF::unicode::kWhiteCircleCharacter;
using WTF::unicode::kWhiteSesameDotCharacter;
using WTF::unicode::kWhiteUpPointingTriangleCharacter;
using WTF::unicode::kYenSignCharacter;
using WTF::unicode::kZeroWidthJoinerCharacter;
using WTF::unicode::kZeroWidthNoBreakSpaceCharacter;
using WTF::unicode::kZeroWidthNonJoinerCharacter;
using WTF::unicode::kZeroWidthSpaceCharacter;

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_CHARACTER_NAMES_H_
