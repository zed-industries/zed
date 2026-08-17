// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_LENGTH_PROPERTY_FUNCTIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_LENGTH_PROPERTY_FUNCTIONS_H_

#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/core/css_value_keywords.h"
#include "third_party/blink/renderer/platform/geometry/length.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ComputedStyle;
class ComputedStyleBuilder;
class CSSProperty;

class LengthPropertyFunctions {
  STATIC_ONLY(LengthPropertyFunctions);

 public:
  static Length::ValueRange GetValueRange(const CSSProperty&);
  static bool IsZoomedLength(const CSSProperty&);
  static bool CanAnimateKeyword(const CSSProperty&, CSSValueID);
  static bool GetPixelsForKeyword(const CSSProperty&,
                                  CSSValueID,
                                  double& result_pixels);
  static bool GetInitialLength(const CSSProperty&,
                               const ComputedStyle& initial_style,
                               Length& result);
  static bool GetLength(const CSSProperty&,
                        const ComputedStyle&,
                        Length& result);
  static bool SetLength(const CSSProperty&,
                        ComputedStyleBuilder&,
                        const Length&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_LENGTH_PROPERTY_FUNCTIONS_H_
