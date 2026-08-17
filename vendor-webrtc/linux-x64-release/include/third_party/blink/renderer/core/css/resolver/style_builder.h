/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_STYLE_BUILDER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_STYLE_BUILDER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/core/css/properties/css_property.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class CSSPropertyName;
class StyleResolverState;

class CORE_EXPORT StyleBuilder {
  STATIC_ONLY(StyleBuilder);

 public:
  using ValueMode = CSSProperty::ValueMode;

  // Apply a property/value pair to the ComputedStyle.
  //
  // If the incoming CSSPropertyName is a custom property, a temporary
  // CustomProperty instance is created to carry out the application.
  static void ApplyProperty(const CSSPropertyName&,
                            StyleResolverState&,
                            const CSSValue&,
                            ValueMode = ValueMode::kNormal);

  // Apply a property/value pair to the ComputedStyle.
  //
  // If you are applying a custom property, please ensure that the incoming
  // CSSProperty is an instance of CustomProperty, and not the static Variable
  // instance. See Variable::IsStaticInstance.
  static void ApplyProperty(const CSSProperty&,
                            StyleResolverState&,
                            const CSSValue&,
                            ValueMode = ValueMode::kNormal);

  // Apply a physical property and its value to the ComputedStyle.
  //
  // Physical properties are properties that are not surrogates (see
  // "surrogate_for" in css_properties.json5).
  static void ApplyPhysicalProperty(const CSSProperty&,
                                    StyleResolverState&,
                                    const CSSValue&,
                                    ValueMode = ValueMode::kNormal);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_STYLE_BUILDER_H_
