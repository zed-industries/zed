// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_WRITABLE_STREAM_DEFAULT_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_WRITABLE_STREAM_DEFAULT_CONTROLLER_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "v8/include/v8.h"

namespace blink {

class AbortController;
class AbortSignal;
class ExceptionState;
class QueueWithSizes;
class ScriptState;
class ScriptValue;
class StrategySizeAlgorithm;
class StreamAlgorithm;
class StreamStartAlgorithm;
class WritableStreamDefaultWriter;
class WritableStream;

class CORE_EXPORT WritableStreamDefaultController final
    : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static WritableStreamDefaultController* From(ScriptState*, ScriptValue);

  // The JavaScript-exposed constructor throws automatically as no constructor
  // is specified in the IDL. This constructor is used internally during
  // creation of a WritableStream object.
  WritableStreamDefaultController();

  // https://streams.spec.whatwg.org/#ws-default-controller-error
  void error(ScriptState*);
  void error(ScriptState*, ScriptValue e);

  //
  // Methods used by WritableStream
  //

  // https://streams.spec.whatwg.org/#ws-default-controller-private-abort
  ScriptPromise<IDLUndefined> AbortSteps(ScriptState*,
                                         v8::Local<v8::Value> reason);

  // https://streams.spec.whatwg.org/#ws-default-controller-private-error
  void ErrorSteps();

  // https://streams.spec.whatwg.org/#set-up-writable-stream-default-controller
  static void SetUp(ScriptState*,
                    WritableStream*,
                    WritableStreamDefaultController*,
                    StreamStartAlgorithm* start_algorithm,
                    StreamAlgorithm* write_algorithm,
                    StreamAlgorithm* close_algorithm,
                    StreamAlgorithm* abort_algorithm,
                    double high_water_mark,
                    StrategySizeAlgorithm* size_algorithm,
                    ExceptionState&);

  // https://streams.spec.whatwg.org/#set-up-writable-stream-default-controller-from-underlying-sink
  static void SetUpFromUnderlyingSink(ScriptState*,
                                      WritableStream*,
                                      v8::Local<v8::Object> underlying_sink,
                                      double high_water_mark,
                                      StrategySizeAlgorithm* size_algorithm,
                                      ExceptionState&);

  //
  // Methods used by WritableStreamDefaultWriter
  //

  // https://streams.spec.whatwg.org/#writable-stream-default-controller-close
  static void Close(ScriptState*, WritableStreamDefaultController*);

  // https://streams.spec.whatwg.org/#writable-stream-default-controller-get-chunk-size
  // May error the stream as a side-effect.
  static double GetChunkSize(ScriptState*,
                             WritableStreamDefaultController*,
                             v8::Local<v8::Value> chunk);

  // https://streams.spec.whatwg.org/#writable-stream-default-controller-get-desired-size
  static double GetDesiredSize(const WritableStreamDefaultController*);

  // https://streams.spec.whatwg.org/#writable-stream-default-controller-write
  static void Write(ScriptState*,
                    WritableStreamDefaultController*,
                    v8::Local<v8::Value> chunk,
                    double chunk_size,
                    ExceptionState&);

  // https://streams.spec.whatwg.org/#writable-stream-default-controller-error
  // TODO(ricea): Make this private.
  static void Error(ScriptState*,
                    WritableStreamDefaultController*,
                    v8::Local<v8::Value> error);

  // Exposed to WritableStream. Not part of the standard.
  bool Started() const { return started_; }

  //
  // Used by TransformStream
  //
  // https://streams.spec.whatwg.org/#writable-stream-default-controller-error-if-needed
  static void ErrorIfNeeded(ScriptState*,
                            WritableStreamDefaultController*,
                            v8::Local<v8::Value> error);

  // IDL attributes
  AbortSignal* signal() const;

  void Abort(ScriptState*, ScriptValue reason);

  void Trace(Visitor*) const override;

 private:
  class ProcessWriteResolveFunction;
  class ProcessWriteRejectFunction;

  // https://streams.spec.whatwg.org/#writable-stream-default-controller-clear-algorithms
  static void ClearAlgorithms(WritableStreamDefaultController*);

  // https://streams.spec.whatwg.org/#writable-stream-default-controller-advance-queue-if-needed
  static void AdvanceQueueIfNeeded(ScriptState*,
                                   WritableStreamDefaultController*);

  // https://streams.spec.whatwg.org/#writable-stream-default-controller-process-close
  static void ProcessClose(ScriptState*, WritableStreamDefaultController*);

  // https://streams.spec.whatwg.org/#writable-stream-default-controller-process-write
  static void ProcessWrite(ScriptState*,
                           WritableStreamDefaultController*,
                           v8::Local<v8::Value> chunk);

  // https://streams.spec.whatwg.org/#writable-stream-default-controller-get-backpressure
  static bool GetBackpressure(const WritableStreamDefaultController*);

  // Most member variables correspond 1:1 with the internal slots in the
  // standard. See
  // https://streams.spec.whatwg.org/#ws-default-controller-internal-slots.
  Member<StreamAlgorithm> abort_algorithm_;
  Member<StreamAlgorithm> close_algorithm_;
  Member<WritableStream> controlled_writable_stream_;

  // |queue_| covers both the [[queue]] and [[queueTotalSize]] internal slots.
  // Instead of chunks in the queue being wrapped in an object, they are
  // stored-as-is, and the `"close"` marker in the queue is represented by an
  // empty queue together with the |close_queued_| flag being set.
  Member<QueueWithSizes> queue_;
  Member<AbortController> abort_controller_;
  bool close_queued_ = false;
  bool started_ = false;
  double strategy_high_water_mark_ = 0.0;
  Member<StrategySizeAlgorithm> strategy_size_algorithm_;
  Member<StreamAlgorithm> write_algorithm_;
  Member<ProcessWriteResolveFunction> resolve_function_;
  Member<ProcessWriteRejectFunction> reject_function_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_WRITABLE_STREAM_DEFAULT_CONTROLLER_H_
