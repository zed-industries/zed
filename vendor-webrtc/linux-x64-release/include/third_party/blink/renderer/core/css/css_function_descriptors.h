// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FUNCTION_DESCRIPTORS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FUNCTION_DESCRIPTORS_H_

#include "third_party/blink/renderer/core/css/style_rule_css_style_declaration.h"

namespace blink {

// https://drafts.csswg.org/css-mixins-1/#cssfunctiondescriptors
class CORE_EXPORT CSSFunctionDescriptors : public StyleRuleCSSStyleDeclaration {
  DEFINE_WRAPPERTYPEINFO();

 public:
  CSSFunctionDescriptors(MutableCSSPropertyValueSet&, CSSRule*);

  // Within @function rules, only the 'result' descriptor and local variables
  // are valid. This function imposes that restriction.
  bool IsPropertyValid(CSSPropertyID) const override;

  String result();
  void setResult(const ExecutionContext* execution_context,
                 const String& value,
                 ExceptionState& exception_state);

  void Trace(Visitor*) const override;

 private:
  String Get(CSSPropertyID);
  void Set(const ExecutionContext* execution_context,
           CSSPropertyID,
           const String& value,
           ExceptionState&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FUNCTION_DESCRIPTORS_H_
