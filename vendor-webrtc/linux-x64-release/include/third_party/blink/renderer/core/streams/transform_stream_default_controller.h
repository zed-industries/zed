// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_TRANSFORM_STREAM_DEFAULT_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_TRANSFORM_STREAM_DEFAULT_CONTROLLER_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "v8/include/v8.h"

namespace blink {

class ExceptionState;
class ReadableStreamDefaultController;
class ScriptState;
class StreamAlgorithm;
class TransformStream;

class CORE_EXPORT TransformStreamDefaultController : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  TransformStreamDefaultController();
  ~TransformStreamDefaultController() override;

  // https://streams.spec.whatwg.org/#ts-default-controller-desired-size
  std::optional<double> desiredSize() const;

  // https://streams.spec.whatwg.org/#ts-default-controller-enqueue
  void enqueue(ScriptState*, ExceptionState&);
  void enqueue(ScriptState*, ScriptValue chunk, ExceptionState&);

  // https://streams.spec.whatwg.org/#ts-default-controller-error
  void error(ScriptState*);
  void error(ScriptState*, ScriptValue reason);

  // https://streams.spec.whatwg.org/#ts-default-controller-terminate
  void terminate(ScriptState*);

  void Trace(Visitor*) const override;

 private:
  friend class TransformStream;

  class DefaultTransformAlgorithm;
  class PerformTransformRejectFunction;

  // https://streams.spec.whatwg.org/#set-up-transform-stream-default-controller
  static void SetUp(ScriptState*,
                    TransformStream*,
                    TransformStreamDefaultController*,
                    StreamAlgorithm* transform_algorithm,
                    StreamAlgorithm* flush_algorithm);

  // https://streams.spec.whatwg.org/#set-up-transform-stream-default-controller-from-transformer
  static v8::Local<v8::Value> SetUpFromTransformer(
      ScriptState*,
      TransformStream*,
      v8::Local<v8::Object> transformer,
      ExceptionState&);

  // https://streams.spec.whatwg.org/#transform-stream-default-controller-clear-algorithms
  static void ClearAlgorithms(TransformStreamDefaultController*);

  // https://streams.spec.whatwg.org/#transform-stream-default-controller-enqueue
  static void Enqueue(ScriptState*,
                      TransformStreamDefaultController*,
                      v8::Local<v8::Value> chunk,
                      ExceptionState&);

  // https://streams.spec.whatwg.org/#transform-stream-default-controller-error
  static void Error(ScriptState*,
                    TransformStreamDefaultController*,
                    v8::Local<v8::Value> e);

  // https://streams.spec.whatwg.org/#transform-stream-default-controller-perform-transform
  static ScriptPromise<IDLUndefined> PerformTransform(
      ScriptState*,
      TransformStreamDefaultController*,
      v8::Local<v8::Value> chunk);

  // https://streams.spec.whatwg.org/#transform-stream-default-controller-terminate
  static void Terminate(ScriptState*, TransformStreamDefaultController*);

  static ReadableStreamDefaultController* GetDefaultController(
      TransformStream*);

  Member<TransformStream> controlled_transform_stream_;
  Member<StreamAlgorithm> flush_algorithm_;
  Member<StreamAlgorithm> transform_algorithm_;
  Member<PerformTransformRejectFunction> reject_function_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_TRANSFORM_STREAM_DEFAULT_CONTROLLER_H_
