// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_TRANSFORM_STREAM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_TRANSFORM_STREAM_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "v8/include/v8.h"

namespace blink {

class ExceptionState;
class ReadableStream;
class ReadableStreamDefaultController;
class ScriptState;
class StrategySizeAlgorithm;
class StreamAlgorithm;
class StreamStartAlgorithm;
class TransformStreamDefaultController;
class TransformStreamTransformer;
class WritableStream;

// Implementation of TransformStream for Blink.  See
// https://streams.spec.whatwg.org/#ts. The implementation closely follows the
// standard, except where required for performance or integration with Blink.
// In particular, classes, methods and abstract operations are implemented in
// the same order as in the standard, to simplify side-by-side reading.

class CORE_EXPORT TransformStream final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // |Create| functions internally call InitInternal().
  static TransformStream* Create(ScriptState*, ExceptionState&);
  static TransformStream* Create(ScriptState*,
                                 ScriptValue transformer,
                                 ExceptionState&);
  static TransformStream* Create(ScriptState*,
                                 ScriptValue transformer,
                                 ScriptValue writable_strategy,
                                 ExceptionState&);
  static TransformStream* Create(ScriptState*,
                                 ScriptValue transformer,
                                 ScriptValue writable_strategy,
                                 ScriptValue readable_strategy,
                                 ExceptionState&);

  // Creates a TransformStream from a C++ object.
  static TransformStream* Create(ScriptState*,
                                 TransformStreamTransformer*,
                                 ExceptionState&);

  // https://streams.spec.whatwg.org/#create-transform-stream
  static TransformStream* Create(ScriptState*,
                                 StreamStartAlgorithm* start_algorithm,
                                 StreamAlgorithm* transform_algorithm,
                                 StreamAlgorithm* flush_algorithm,
                                 double writable_high_water_mark,
                                 StrategySizeAlgorithm* writable_size_algorithm,
                                 double readable_high_water_mark,
                                 StrategySizeAlgorithm* readable_size_algorithm,
                                 ExceptionState&);

  TransformStream();

  // This constructor produces a TransformStream from an existing {readable,
  // writable} pair. It cannot fail and does not require calling Init().
  TransformStream(ReadableStream*, WritableStream*);

  // IDL attributes
  ReadableStream* readable() const { return readable_.Get(); }
  WritableStream* writable() const { return writable_.Get(); }

  ReadableStream* Readable() const { return readable_.Get(); }
  WritableStream* Writable() const { return writable_.Get(); }

  void Trace(Visitor*) const override;

 private:
  friend class TransformStreamDefaultController;

  class ControllerInterface;
  class FlushAlgorithm;
  class TransformAlgorithm;
  class ReturnStartPromiseAlgorithm;
  class DefaultSinkWriteAlgorithm;
  class DefaultSinkAbortAlgorithm;
  class DefaultSinkCloseAlgorithm;
  class DefaultSourcePullAlgorithm;
  class DefaultSourceCancelAlgorithm;

  // Performs the functions performed by the constructor in the standard.
  // https://streams.spec.whatwg.org/#ts-constructor
  void InitInternal(ScriptState*,
                    ScriptValue raw_transformer,
                    ScriptValue raw_writable_strategy,
                    ScriptValue raw_readable_strategy,
                    ExceptionState&);

  // https://streams.spec.whatwg.org/#initialize-transform-stream
  static void Initialize(ScriptState*,
                         TransformStream*,
                         ScriptPromiseResolver<IDLUndefined>* start_promise,
                         double writable_high_water_mark,
                         StrategySizeAlgorithm* writable_size_algorithm,
                         double readable_high_water_mark,
                         StrategySizeAlgorithm* readable_size_algorithm,
                         ExceptionState&);

  // https://streams.spec.whatwg.org/#transform-stream-error
  static void Error(ScriptState*, TransformStream*, v8::Local<v8::Value> e);

  // https://streams.spec.whatwg.org/#transform-stream-error-writable-and-unblock-write
  static void ErrorWritableAndUnblockWrite(ScriptState*,
                                           TransformStream*,
                                           v8::Local<v8::Value> e);

  // https://streams.spec.whatwg.org/#transform-stream-set-backpressure
  static void SetBackpressure(ScriptState*,
                              TransformStream*,
                              bool backpressure);

  ReadableStreamDefaultController* GetReadableController();

  // The [[backpressure]] internal slot from the standard is here called
  // |had_backpressure_| to conform to Blink style. The initial value is
  // *undefined* in the standard, but it is set to *true* by
  // InitializeTransformStream(), so that is the initial value used here.
  bool had_backpressure_ = true;
  Member<ScriptPromiseResolver<IDLUndefined>> backpressure_change_promise_;
  Member<ReadableStream> readable_;
  Member<TransformStreamDefaultController> transform_stream_controller_;
  Member<WritableStream> writable_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_TRANSFORM_STREAM_H_
