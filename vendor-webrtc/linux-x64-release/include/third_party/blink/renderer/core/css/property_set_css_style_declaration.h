/*
 * Copyright (C) 2012 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTY_SET_CSS_STYLE_DECLARATION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTY_SET_CSS_STYLE_DECLARATION_H_

#include "third_party/blink/renderer/core/css/abstract_property_set_css_style_declaration.h"

namespace blink {

class MutableCSSPropertyValueSet;

class CORE_EXPORT PropertySetCSSStyleDeclaration
    : public AbstractPropertySetCSSStyleDeclaration {
 public:
  PropertySetCSSStyleDeclaration(ExecutionContext* execution_context,
                                 MutableCSSPropertyValueSet& property_set)
      : AbstractPropertySetCSSStyleDeclaration(execution_context),
        property_set_(&property_set) {}

  bool IsPropertyValid(CSSPropertyID) const override { return true; }
  void Trace(Visitor*) const override;

 protected:
  MutableCSSPropertyValueSet& PropertySet() const final {
    DCHECK(property_set_);
    return *property_set_;
  }

  Member<MutableCSSPropertyValueSet> property_set_;  // Cannot be null
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTY_SET_CSS_STYLE_DECLARATION_H_
