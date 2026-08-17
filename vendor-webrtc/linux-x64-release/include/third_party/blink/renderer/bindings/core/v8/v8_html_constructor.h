// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_HTML_CONSTRUCTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_HTML_CONSTRUCTOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html_element_type_helpers.h"
#include "third_party/blink/renderer/platform/bindings/wrapper_type_info.h"
#include "v8/include/v8.h"

namespace blink {

// https://html.spec.whatwg.org/C/#html-element-constructors
class CORE_EXPORT V8HTMLConstructor {
  STATIC_ONLY(V8HTMLConstructor);

 public:
  static void HtmlConstructor(const v8::FunctionCallbackInfo<v8::Value>&,
                              const WrapperTypeInfo&,
                              const HTMLElementType);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_HTML_CONSTRUCTOR_H_
