// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_OBJECT_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_OBJECT_PARSER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "v8/include/v8.h"

namespace blink {

class ExceptionState;
class ExecutionContext;

class CORE_EXPORT V8ObjectParser final {
  STATIC_ONLY(V8ObjectParser);

 public:
  // Retrieves and parses a CSS property list off the class constructor
  // function. Returns true if the list exists, and successfully converted to a
  // Vector<String> type. It does not fail if the list contains invalid CSS
  // properties, to ensure forward compatibility.
  static bool ParseCSSPropertyList(v8::Local<v8::Context>,
                                   const ExecutionContext*,
                                   v8::Local<v8::Object> constructor,
                                   const AtomicString list_name,
                                   Vector<CSSPropertyID>* native_properties,
                                   Vector<AtomicString>* custom_properties,
                                   ExceptionState&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_OBJECT_PARSER_H_
