// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_MATH_OPERATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_MATH_OPERATOR_H_

namespace WTF {
class StringView;
}  // namespace WTF

namespace blink {

class CSSParserToken;

enum class CSSMathOperator {
  kAdd,
  kSubtract,
  kMultiply,
  kDivide,
  kMin,
  kMax,
  kClamp,
  kRoundNearest,
  kRoundUp,
  kRoundDown,
  kRoundToZero,
  kMod,
  kRem,
  kHypot,
  kAbs,
  kSign,
  kLog,
  kExp,
  kSqrt,
  kProgress,
  kCalcSize,
  kMediaProgress,
  kContainerProgress,
  kPow,
  kSin,
  kCos,
  kTan,
  kAsin,
  kAcos,
  kAtan,
  kAtan2,
  kInvert,
  kInvalid
};

CSSMathOperator ParseCSSArithmeticOperator(const CSSParserToken& token);
WTF::StringView ToString(CSSMathOperator);
WTF::StringView ToRoundingStrategyString(CSSMathOperator);

bool IsComparison(CSSMathOperator);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_MATH_OPERATOR_H_
