/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
 * Copyright (C) 2013 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_EDITING_STYLE_UTILITIES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_EDITING_STYLE_UTILITIES_H_

#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/core/css_value_keywords.h"
#include "third_party/blink/renderer/core/dom/container_node.h"
#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class CSSStyleDeclaration;
class EditingStyle;
class MutableCSSPropertyValueSet;
class Node;
class CSSPropertyValueSet;

class EditingStyleUtilities {
  STATIC_ONLY(EditingStyleUtilities);

 public:
  static EditingStyle* CreateWrappingStyleForAnnotatedSerialization(
      Element* context);
  static EditingStyle* CreateWrappingStyleForSerialization(Element* context);
  static EditingStyle* CreateStyleAtSelectionStart(
      const VisibleSelection&,
      bool should_use_background_color_in_effect = false,
      MutableCSSPropertyValueSet* style_to_check = nullptr);
  static bool IsEmbedOrIsolate(CSSValueID unicode_bidi) {
    return unicode_bidi == CSSValueID::kIsolate ||
           unicode_bidi == CSSValueID::kWebkitIsolate ||
           unicode_bidi == CSSValueID::kEmbed;
  }

  static void StripUAStyleRulesForMarkupSanitization(EditingStyle* style);

  static bool IsTransparentColorValue(const CSSValue*);
  static bool HasTransparentBackgroundColor(CSSStyleDeclaration*);
  static bool HasTransparentBackgroundColor(CSSPropertyValueSet*);
  static const CSSValue* BackgroundColorValueInEffect(Node*);
  static bool HasAncestorVerticalAlignStyle(Node&, CSSValueID);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_EDITING_STYLE_UTILITIES_H_
