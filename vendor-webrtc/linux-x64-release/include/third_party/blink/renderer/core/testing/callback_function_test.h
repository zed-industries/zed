// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_CALLBACK_FUNCTION_TEST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_CALLBACK_FUNCTION_TEST_H_

#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class ExceptionState;
class HTMLDivElement;
class V8InternalEnum;
class V8TestCallback;
class V8TestEnumCallback;
class V8TestInterfaceCallback;
class V8TestReceiverObjectCallback;
class V8TestSequenceCallback;

class CallbackFunctionTest final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  String testCallback(V8TestCallback*,
                      const String&,
                      const String&,
                      ExceptionState&);
  String testNullableCallback(V8TestCallback*,
                              const String&,
                              const String&,
                              ExceptionState&);
  void testInterfaceCallback(V8TestInterfaceCallback*,
                             HTMLDivElement*,
                             ExceptionState&);
  void testReceiverObjectCallback(V8TestReceiverObjectCallback*,
                                  ExceptionState&);
  Vector<String> testSequenceCallback(V8TestSequenceCallback*,
                                      const Vector<int>& numbers,
                                      ExceptionState&);
  void testEnumCallback(V8TestEnumCallback*,
                        const V8InternalEnum& enum_value,
                        ExceptionState&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_CALLBACK_FUNCTION_TEST_H_
