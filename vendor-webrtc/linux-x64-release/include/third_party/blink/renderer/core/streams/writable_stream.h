// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_WRITABLE_STREAM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_WRITABLE_STREAM_H_

#include <memory>

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/bindings/trace_wrapper_v8_reference.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_deque.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "v8/include/v8.h"

namespace blink {

class ExceptionState;
class MessagePort;
class ScriptState;
class StrategySizeAlgorithm;
class StreamAlgorithm;
class StreamStartAlgorithm;
class UnderlyingSinkBase;
class WritableStreamDefaultController;
class WritableStreamDefaultWriter;
class WritableStreamTransferringOptimizer;

class CORE_EXPORT WritableStream : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  enum State : uint8_t {
    kWritable,
    kClosed,
    kErroring,
    kErrored,
  };

  // https://streams.spec.whatwg.org/#ws-constructor
  static WritableStream* Create(ScriptState*, ExceptionState&);
  static WritableStream* Create(ScriptState*,
                                ScriptValue underlying_sink,
                                ExceptionState&);
  static WritableStream* Create(ScriptState*,
                                ScriptValue raw_underlying_sink,
                                ScriptValue raw_strategy,
                                ExceptionState&);

  // https://streams.spec.whatwg.org/#create-writable-stream
  // Unlike in the standard, |high_water_mark| and |size_algorithm| are
  // required parameters.
  static WritableStream* Create(ScriptState*,
                                StreamStartAlgorithm* start_algorithm,
                                StreamAlgorithm* write_algorithm,
                                StreamAlgorithm* close_algorithm,
                                StreamAlgorithm* abort_algorithm,
                                double high_water_mark,
                                StrategySizeAlgorithm* size_algorithm,
                                ExceptionState&);

  static WritableStream* CreateWithCountQueueingStrategy(
      ScriptState*,
      UnderlyingSinkBase*,
      size_t high_water_mark);
  static WritableStream* CreateWithCountQueueingStrategy(
      ScriptState*,
      UnderlyingSinkBase*,
      size_t high_water_mark,
      std::unique_ptr<WritableStreamTransferringOptimizer> optimizer);

  // Called by Create().
  WritableStream();
  ~WritableStream() override;

  // This should only be used with freshly-constructed streams. It expects to be
  // called in a valid microtask scope.
  void InitWithCountQueueingStrategy(
      ScriptState*,
      UnderlyingSinkBase*,
      size_t high_water_mark,
      std::unique_ptr<WritableStreamTransferringOptimizer>,
      ExceptionState&);

  // IDL defined functions

  // https://streams.spec.whatwg.org/#ws-locked
  bool locked() const {
    // https://streams.spec.whatwg.org/#ws-locked
    // 2. Return ! IsWritableStreamLocked(this).
    return IsLocked(this);
  }

  // https://streams.spec.whatwg.org/#ws-abort
  ScriptPromise<IDLUndefined> abort(ScriptState*, ExceptionState&);
  ScriptPromise<IDLUndefined> abort(ScriptState*,
                                    ScriptValue reason,
                                    ExceptionState&);

  // https://streams.spec.whatwg.org/#ws-close
  ScriptPromise<IDLUndefined> close(ScriptState*, ExceptionState&);

  // https://streams.spec.whatwg.org/#ws-get-writer
  WritableStreamDefaultWriter* getWriter(ScriptState*, ExceptionState&);

  // Inherited methods used internally.

  static bool IsLocked(const WritableStream* stream) {
    return stream->writer_ != nullptr;
  }

  void Serialize(ScriptState*, MessagePort*, ExceptionState&);

  static WritableStream* Deserialize(
      ScriptState*,
      MessagePort*,
      std::unique_ptr<WritableStreamTransferringOptimizer> optimizer,
      ExceptionState&);

  //
  // Methods used by ReadableStream::PipeTo
  //

  // https://streams.spec.whatwg.org/#acquire-writable-stream-default-writer
  static WritableStreamDefaultWriter* AcquireDefaultWriter(ScriptState*,
                                                           WritableStream*,
                                                           ExceptionState&);

  //
  // Methods used by WritableStreamDefaultWriter.
  //

  // https://streams.spec.whatwg.org/#writable-stream-abort
  static ScriptPromise<IDLUndefined> Abort(ScriptState*,
                                           WritableStream*,
                                           v8::Local<v8::Value> reason);

  // https://streams.spec.whatwg.org/#writable-stream-add-write-request
  static void AddWriteRequest(WritableStream*,
                              ScriptPromiseResolver<IDLUndefined>*);

  static ScriptPromise<IDLUndefined> Close(ScriptState*, WritableStream*);

  // https://streams.spec.whatwg.org/#writable-stream-close-queued-or-in-flight
  static bool CloseQueuedOrInFlight(const WritableStream*);

  //
  // Methods used by WritableStreamDefaultController.
  //

  // https://streams.spec.whatwg.org/#writable-stream-deal-with-rejection
  static void DealWithRejection(ScriptState*,
                                WritableStream*,
                                v8::Local<v8::Value> error);

  // https://streams.spec.whatwg.org/#writable-stream-start-erroring
  static void StartErroring(ScriptState*,
                            WritableStream*,
                            v8::Local<v8::Value> reason);

  // https://streams.spec.whatwg.org/#writable-stream-finish-erroring
  static void FinishErroring(ScriptState*, WritableStream*);

  // https://streams.spec.whatwg.org/#writable-stream-finish-in-flight-write
  static void FinishInFlightWrite(ScriptState*, WritableStream*);

  // https://streams.spec.whatwg.org/#writable-stream-finish-in-flight-write-with-error
  static void FinishInFlightWriteWithError(ScriptState*,
                                           WritableStream*,
                                           v8::Local<v8::Value> error);

  // https://streams.spec.whatwg.org/#writable-stream-finish-in-flight-close
  static void FinishInFlightClose(ScriptState*, WritableStream*);

  // https://streams.spec.whatwg.org/#writable-stream-finish-in-flight-close-with-error
  static void FinishInFlightCloseWithError(ScriptState*,
                                           WritableStream*,
                                           v8::Local<v8::Value> error);

  // https://streams.spec.whatwg.org/#writable-stream-mark-close-request-in-flight
  static void MarkCloseRequestInFlight(WritableStream*);

  // https://streams.spec.whatwg.org/#writable-stream-mark-first-write-request-in-flight
  static void MarkFirstWriteRequestInFlight(WritableStream*);

  // https://streams.spec.whatwg.org/#writable-stream-update-backpressure
  static void UpdateBackpressure(ScriptState*,
                                 WritableStream*,
                                 bool backpressure);

  // Accessors for use by other stream classes.
  State GetState() const { return state_; }
  bool IsErrored() const { return state_ == kErrored; }
  bool IsErroring() const { return state_ == kErroring; }
  bool IsWritable() const { return state_ == kWritable; }

  bool HasBackpressure() const { return has_backpressure_; }

  bool HasInFlightWriteRequest() const { return in_flight_write_request_; }

  bool IsClosingOrClosed() const {
    return CloseQueuedOrInFlight(this) || state_ == kClosed;
  }

  v8::Local<v8::Value> GetStoredError(v8::Isolate*) const;

  WritableStreamDefaultController* Controller() {
    return writable_stream_controller_.Get();
  }
  const WritableStreamDefaultController* Controller() const {
    return writable_stream_controller_.Get();
  }

  const WritableStreamDefaultWriter* Writer() const { return writer_.Get(); }

  void SetCloseRequest(ScriptPromiseResolver<IDLUndefined>*);
  void SetController(WritableStreamDefaultController*);
  void SetWriter(WritableStreamDefaultWriter*);

  std::unique_ptr<WritableStreamTransferringOptimizer>
  TakeTransferringOptimizer();

  // Utility methods shared with other classes.
  static v8::Local<v8::String> CreateCannotActionOnStateStreamMessage(
      v8::Isolate*,
      const char* action,
      const char* state_name);

  static v8::Local<v8::Value> CreateCannotActionOnStateStreamException(
      v8::Isolate*,
      const char* action,
      State);

  void Trace(Visitor*) const override;

 protected:
  // Used when creating a stream from JavaScript. Called from Create().
  // https://streams.spec.whatwg.org/#ws-constructor
  // TODO(ricea): Port external callers to InitWithCountQueuingStrategy().
  void InitInternal(ScriptState*,
                    ScriptValue raw_underlying_sink,
                    ScriptValue raw_strategy,
                    ExceptionState&);

 private:
  using PromiseQueue = HeapDeque<Member<ScriptPromiseResolver<IDLUndefined>>>;

  class PendingAbortRequest;

  // https://streams.spec.whatwg.org/#writable-stream-has-operation-marked-in-flight
  static bool HasOperationMarkedInFlight(const WritableStream*);

  // https://streams.spec.whatwg.org/#writable-stream-reject-close-and-closed-promise-if-needed
  static void RejectCloseAndClosedPromiseIfNeeded(ScriptState*,
                                                  WritableStream*);

  // Functions that don't appear in the standard.

  // Rejects all the promises in |queue| with value |e|.
  static void RejectPromises(ScriptState*,
                             PromiseQueue* queue,
                             v8::Local<v8::Value> e);

  // Member variables correspond 1:1 to the internal slots in the standard.
  // See https://streams.spec.whatwg.org/#ws-internal-slots

  // |has_backpressure_| corresponds to the [[backpressure]] slot in the
  // |standard.
  bool has_backpressure_ = false;

  // |state_| is here out of order so it doesn't require 7 bytes of padding.
  State state_ = kWritable;

  Member<ScriptPromiseResolver<IDLUndefined>> close_request_;
  Member<ScriptPromiseResolver<IDLUndefined>> in_flight_write_request_;
  Member<ScriptPromiseResolver<IDLUndefined>> in_flight_close_request_;
  Member<PendingAbortRequest> pending_abort_request_;
  TraceWrapperV8Reference<v8::Value> stored_error_;
  Member<WritableStreamDefaultController> writable_stream_controller_;
  Member<WritableStreamDefaultWriter> writer_;
  PromiseQueue write_requests_;
  std::unique_ptr<WritableStreamTransferringOptimizer> transferring_optimizer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_WRITABLE_STREAM_H_
