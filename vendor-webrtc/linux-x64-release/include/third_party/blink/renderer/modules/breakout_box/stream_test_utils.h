// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_STREAM_TEST_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_STREAM_TEST_UTILS_H_

#include "third_party/blink/renderer/bindings/core/v8/iterable.h"
#include "third_party/blink/renderer/bindings/core/v8/native_value_traits_impl.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_tester.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_binding_for_testing.h"
#include "third_party/blink/renderer/core/streams/readable_stream_default_reader.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"

namespace blink {

class ExecutionContext;
class MediaStreamTrack;
class MediaStreamVideoSource;

template <typename T>
T* ReadObjectFromStream(const V8TestingScope& v8_scope,
                        ReadableStreamDefaultReader* reader) {
  ScriptState* script_state = v8_scope.GetScriptState();
  ScriptPromiseTester read_tester(
      script_state, reader->read(script_state, ASSERT_NO_EXCEPTION));
  read_tester.WaitUntilSettled();
  EXPECT_TRUE(read_tester.IsFulfilled());

  v8::Local<v8::Value> result = read_tester.Value().V8Value();
  EXPECT_TRUE(result->IsObject());
  v8::Local<v8::Value> v8_signal;
  bool done = false;
  EXPECT_TRUE(V8UnpackIterationResult(script_state, result.As<v8::Object>(),
                                      &v8_signal, &done));
  EXPECT_FALSE(done);
  return NativeValueTraits<T>::NativeValue(v8_scope.GetIsolate(), v8_signal,
                                           ASSERT_NO_EXCEPTION);
}

MediaStreamTrack* CreateVideoMediaStreamTrack(ExecutionContext*,
                                              MediaStreamVideoSource*);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_STREAM_TEST_UTILS_H_
