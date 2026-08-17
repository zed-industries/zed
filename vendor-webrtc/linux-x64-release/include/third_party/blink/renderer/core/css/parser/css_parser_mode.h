/*
 * Copyright (C) 2012 Adobe Systems Incorporated. All rights reserved.
 * Copyright (C) 2012 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above
 *    copyright notice, this list of conditions and the following
 *    disclaimer.
 * 2. Redistributions in binary form must reproduce the above
 *    copyright notice, this list of conditions and the following
 *    disclaimer in the documentation and/or other materials
 *    provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDER "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY,
 * OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR
 * TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF
 * THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_PARSER_MODE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_PARSER_MODE_H_

#include <stdint.h>

namespace blink {

// Must not grow beyond 4 bits, due to packing in CSSPropertyValueSet.
enum CSSParserMode : uint8_t {
  kHTMLStandardMode,
  kHTMLQuirksMode,
  // SVG attributes are parsed in quirks mode but rules differ slightly.
  kSVGAttributeMode,
  // @font-face rules are specially tagged in CSSPropertyValueSet so
  // CSSOM modifications don't treat them as style rules.
  kCSSFontFaceRuleMode,
  // @keyframes rules are specially tagged in CSSPropertyValueSet so CSSOM
  // modifications don't allow setting animation-* in their keyframes.
  kCSSKeyframeRuleMode,
  // @property rules are specially tagged so modifications through the
  // inspector don't treat them as style rules.
  kCSSPropertyRuleMode,
  // @font-palette-values rules are specially tagged so modifications through
  // the inspector don't treat them as style rules.
  kCSSFontPaletteValuesRuleMode,
  // @position-try rules have limitations on what they allow, also through
  // mutations in CSSOM.
  // https://drafts.csswg.org/css-anchor-position-1/#om-position-try
  kCSSPositionTryRuleMode,
  // Within @function rules, only the 'result' descriptor and local variables
  // (i.e. "custom properties") are allowed.
  // https://drafts.csswg.org/css-mixins-1/#the-function-declarations-interface
  kCSSFunctionDescriptorsMode,
  // User agent stylesheets are parsed in standards mode but also allows
  // internal properties and values.
  kUASheetMode,
  // This should always be the last entry.
  kNumCSSParserModes
};

inline bool IsQuirksModeBehavior(CSSParserMode mode) {
  return mode == kHTMLQuirksMode;
}

inline bool IsUASheetBehavior(CSSParserMode mode) {
  return mode == kUASheetMode;
}

inline bool IsUseCounterEnabledForMode(CSSParserMode mode) {
  // We don't count the UA style sheet in our statistics.
  return mode != kUASheetMode;
}

// Used in CSSParser APIs to say if we should defer parsing of declaration lists
// in style rules until we need them for CSSOM access, or for applying matched
// rules to computed style.
enum class CSSDeferPropertyParsing { kNo, kYes };

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_PARSER_MODE_H_
