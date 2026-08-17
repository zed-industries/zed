// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_MOCK_FUNCTION_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_MOCK_FUNCTION_SCOPE_H_

#include "testing/gmock/include/gmock/gmock.h"
#include "third_party/blink/renderer/bindings/core/v8/script_function.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class ScriptState;
class ScriptValue;

class MockFunctionScope {
  STACK_ALLOCATED();

 public:
  explicit MockFunctionScope(ScriptState*);
  ~MockFunctionScope();

  ScriptFunction* ExpectCall();
  ScriptFunction* ExpectCall(String* captor);
  ScriptFunction* ExpectNoCall();

 private:
  class MockFunction : public ScriptFunction {
   public:
    MockFunction();
    // TODO(http://crbug.com/1159794): add other convenience methods that allow
    // the test case to capture non-String values.
    MockFunction(ScriptState*, String* captor);
    MOCK_METHOD2(Call, ScriptValue(ScriptState*, ScriptValue));
  };

  ScriptState* script_state_;
  Vector<Persistent<MockFunction>> mock_functions_;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_MOCK_FUNCTION_SCOPE_H_
