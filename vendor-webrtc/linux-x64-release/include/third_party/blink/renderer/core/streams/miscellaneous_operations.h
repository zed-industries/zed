// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_MISCELLANEOUS_OPERATIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_MISCELLANEOUS_OPERATIONS_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "v8/include/v8.h"

namespace blink {

// Operations from the "Miscellaneous Operations" section of the standard:
// https://streams.spec.whatwg.org/#misc-abstract-ops

class ExceptionState;
class ReadableStream;
class ScriptState;
class StrategySizeAlgorithm;
class StreamAlgorithm;
class StreamStartAlgorithm;
class WritableStream;

// This is slightly different than the version in the standard
// https://streams.spec.whatwg.org/#create-algorithm-from-underlying-method as
// it doesn't require the number of expected arguments, algoArgCount, to be
// specified. Also, it only supports 0 or 1 extraArgs, instead of an arbitrary
// number. Use an empty value for |extra_arg| to mean 0 extraArgs.
// |method_name_for_error| supplies a more helpful name for the method to be
// used in exception messages. For example, if |method_name| is "write",
// |method_name_for_error| might be "underlyingSink.write".
CORE_EXPORT StreamAlgorithm* CreateAlgorithmFromUnderlyingMethod(
    ScriptState*,
    v8::Local<v8::Object> underlying_object,
    const char* method_name,
    const char* method_name_for_error,
    v8::MaybeLocal<v8::Value> extra_arg,
    ExceptionState&);

// Looks up |method_name| on |object|. Will throw an exception if the lookup
// fails or if the resolved method is neither a function nor undefined.
// |name_for_error| is used for the name of the method in exception messages.
CORE_EXPORT v8::MaybeLocal<v8::Value> ResolveMethod(
    ScriptState*,
    v8::Local<v8::Object> object,
    const char* method_name,
    const char* name_for_error,
    ExceptionState&);

// This works like CreateAlgorithmFromUnderlyingMethod() but |method| must
// already have been resolved and verified to be a v8::Function.
CORE_EXPORT StreamAlgorithm* CreateAlgorithmFromResolvedMethod(
    ScriptState*,
    v8::Local<v8::Object> underlying_object,
    v8::Local<v8::Value> method,
    v8::MaybeLocal<v8::Value> extra_arg);

// Create a StreamStartAlgorithm from the "start" method on |underlying_object|.
// Unlike other algorithms, the lookup of the method on the object is done at
// execution time rather than algorithm creation time. |method_name_for_error|
// is used in exception messages. It is not copied so must remain valid until
// the algorithm is run.
CORE_EXPORT StreamStartAlgorithm* CreateStartAlgorithm(
    ScriptState*,
    v8::Local<v8::Object> underlying_object,
    const char* method_name_for_error,
    v8::Local<v8::Value> controller);

// Create a StreamStartAlgorithm from the "start" method on |underlying_object|
// for readable byte streams.
CORE_EXPORT StreamStartAlgorithm* CreateByteStreamStartAlgorithm(
    ScriptState*,
    v8::Local<v8::Object> underlying_object,
    v8::Local<v8::Value> method,
    v8::Local<v8::Value> controller);

// Returns a startAlgorithm that always returns a promise resolved with
// undefined.
CORE_EXPORT StreamStartAlgorithm* CreateTrivialStartAlgorithm();

// Returns a streamAlgorithm that always returns a promise resolved with
// undefined.
CORE_EXPORT StreamAlgorithm* CreateTrivialStreamAlgorithm();

// Returns a strategy object that has no size() function and the supplied
// highWaterMark. It has a null prototype so that it won't be affected by
// changes to the global Object prototype. It behaves the same as a
// CountQueuingStrategy but is faster and safer to use from C++.
CORE_EXPORT ScriptValue CreateTrivialQueuingStrategy(v8::Isolate*,
                                                     size_t high_water_mark);

// Used in place of InvokeOrNoop in spec. Always takes 1 argument.
// https://streams.spec.whatwg.org/#invoke-or-noop
CORE_EXPORT v8::MaybeLocal<v8::Value> CallOrNoop1(ScriptState*,
                                                  v8::Local<v8::Object> object,
                                                  const char* method_name,
                                                  const char* name_for_error,
                                                  v8::Local<v8::Value> arg0,
                                                  ExceptionState&);

// https://streams.spec.whatwg.org/#promise-call
// "PromiseCall(F, V, args)"
// "F" is called |method| here
// "V" is called |recv| here
// "args" becomes |argc| and |argv| here.
CORE_EXPORT ScriptPromise<IDLUndefined> PromiseCall(
    ScriptState*,
    v8::Local<v8::Function> method,
    v8::Local<v8::Object> recv,
    int argc,
    v8::Local<v8::Value> argv[]);

// Unlike in the standard, the caller needs to handle the conversion of the
// value to a Number.
// https://streams.spec.whatwg.org/#validate-and-normalize-high-water-mark
CORE_EXPORT double ValidateAndNormalizeHighWaterMark(double high_water_mark,
                                                     ExceptionState&);

// https://streams.spec.whatwg.org/#make-size-algorithm-from-size-function
CORE_EXPORT StrategySizeAlgorithm* MakeSizeAlgorithmFromSizeFunction(
    ScriptState*,
    v8::Local<v8::Value> size,
    ExceptionState&);

CORE_EXPORT StrategySizeAlgorithm* CreateDefaultSizeAlgorithm();

// Converts |value| to an object. |value| must not be empty. If |value| is
// undefined, an empty object will be returned. If |value| is JavaScript null,
// then an exception will be thrown. In the standard, this is performed as part
// of the GetV() operation, but in this implementation we do it explicitly
// before looking up any properties.
void ScriptValueToObject(ScriptState* script_state,
                         ScriptValue value,
                         v8::Local<v8::Object>* object,
                         ExceptionState& exception_state);

// This class is used for unpacking strategies in the constructors of
// ReadableStream, WritableStream and TransformStream. For example, steps 2.,
// 3., 6., 7. and 8. of https://streams.spec.whatwg.org/#ws-constructor.
class StrategyUnpacker final {
  STACK_ALLOCATED();

 public:
  // Looks up the "size" and "highWaterMark" properties on |strategy|. May run
  // arbitrary user code. The object cannot be used if
  // exception_state.HadException() is true.
  StrategyUnpacker(ScriptState*, ScriptValue strategy, ExceptionState&);
  StrategyUnpacker(const StrategyUnpacker&) = delete;
  StrategyUnpacker& operator=(const StrategyUnpacker&) = delete;
  ~StrategyUnpacker() = default;

  // Performs MakeSizeAlgorithmFromSizeFunction on |size_|. Because this method
  // can throw an exception, the timing when it is called is observable.
  StrategySizeAlgorithm* MakeSizeAlgorithm(ScriptState*, ExceptionState&) const;

  // If |high_water_mark_| is defined, converts it to a number and call
  // ValidateAndNormalizeHighWaterMark on it. Otherwise returns |default_value|.
  // May run arbitrary user code.
  double GetHighWaterMark(ScriptState*,
                          int default_value,
                          ExceptionState&) const;

  bool IsSizeUndefined() const;

 private:
  v8::Local<v8::Value> size_;
  v8::Local<v8::Value> high_water_mark_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_MISCELLANEOUS_OPERATIONS_H_
