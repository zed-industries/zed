// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_PIPE_TO_ENGINE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_PIPE_TO_ENGINE_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/core/v8/to_v8_traits.h"
#include "third_party/blink/renderer/core/dom/abort_signal.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "v8/include/v8.h"

namespace blink {

class AbortSignal;
class PipeOptions;
class ReadableStream;
class ReadableStreamDefaultReader;
class ScriptState;
class WritableStream;
class WritableStreamDefaultWriter;

// PipeToEngine implements PipeTo(). All standard steps in this class come from
// https://streams.spec.whatwg.org/#readable-stream-pipe-to
//
// This implementation is simple but suboptimal because it uses V8 promises to
// drive its asynchronous state machine, allocating a lot of temporary V8
// objects as a result.
//
// TODO(ricea): Create internal versions of ReadableStreamDefaultReader::Read()
// and WritableStreamDefaultWriter::Write() to bypass promise creation and so
// reduce the number of allocations on the hot path.
class PipeToEngine final : public GarbageCollected<PipeToEngine> {
 public:
  PipeToEngine(ScriptState* script_state, PipeOptions* pipe_options)
      : script_state_(script_state), pipe_options_(pipe_options) {}
  PipeToEngine(const PipeToEngine&) = delete;
  PipeToEngine& operator=(const PipeToEngine&) = delete;

  // This is the main entrypoint for ReadableStreamPipeTo().
  ScriptPromise<IDLUndefined> Start(ReadableStream* readable,
                                    WritableStream* destination,
                                    ExceptionState&);

  void Trace(Visitor* visitor) const {
    visitor->Trace(script_state_);
    visitor->Trace(pipe_options_);
    visitor->Trace(reader_);
    visitor->Trace(writer_);
    visitor->Trace(promise_);
    visitor->Trace(last_write_);
    visitor->Trace(shutdown_error_);
    visitor->Trace(abort_handle_);
  }

 private:
  // The implementation uses method pointers to maximise code reuse.

  class PipeToAbortAlgorithm;
  class PipeToReadRequest;

  template <typename ReturnType>
  class WrappedPromiseResolve;
  class WrappedPromiseReject;

  // |Action| represents an action that can be passed to the "Shutdown with an
  // action" operation. Each Action is implemented as a method which delegates
  // to some abstract operation, inferring the arguments from the state of
  // |this|.
  using Action = ScriptPromise<IDLUndefined> (PipeToEngine::*)();

  // Checks the state of the streams and executes the shutdown handlers if
  // necessary. Returns true if piping can continue.
  bool CheckInitialState();

  void AbortAlgorithm(AbortSignal* signal);

  ScriptPromise<IDLUndefined> AbortAlgorithmAction();

  // HandleNextEvent() has an unused return value because it is a
  // PromiseReaction. HandleNextEvent() and ReadFulfilled() call each other
  // asynchronously in a loop until the pipe completes.
  void HandleNextEvent();

  void ReadRequestChunkStepsBody(ScriptState* script_state,
                                 v8::Global<v8::Value> chunk);

  // If read() is in progress, then wait for it to tell us that the stream is
  // closed so that we write all the data before shutdown.
  void OnReaderClosed();

  // 1. Errors must be propagated forward: if source.[[state]] is or
  //    becomes "errored", then
  void ReadableError(v8::Local<v8::Value> error);

  // 2. Errors must be propagated backward: if dest.[[state]] is or becomes
  //    "errored", then
  void WritableError(v8::Local<v8::Value> error);

  // 3. Closing must be propagated forward: if source.[[state]] is or
  //    becomes "closed", then
  void ReadableClosed();

  // 4. Closing must be propagated backward: if !
  //    WritableStreamCloseQueuedOrInFlight(dest) is true or dest.[[state]] is
  //    "closed", then
  void WritableStartedClosed();

  // * Shutdown with an action: if any of the above requirements ask to shutdown
  //   with an action |action|, optionally with an error |originalError|, then:
  void ShutdownWithAction(Action action,
                          v8::MaybeLocal<v8::Value> original_error);

  // * Shutdown: if any of the above requirements or steps ask to shutdown,
  //   optionally with an error error, then:
  void Shutdown(v8::MaybeLocal<v8::Value> error_maybe);

  // Calls Finalize(), using the stored shutdown error rather than the value
  // that was passed.
  void FinalizeWithOriginalErrorIfSet();

  // Calls Finalize(), using the value that was passed as the error.
  void FinalizeWithNewError(v8::Local<v8::Value> new_error);

  // * Finalize: both forms of shutdown will eventually ask to finalize,
  //   optionally with an error error, which means to perform the following
  //   steps:
  void Finalize(v8::MaybeLocal<v8::Value> error_maybe);

  bool ShouldWriteQueuedChunks() const;

  ScriptPromise<IDLUndefined> WriteQueuedChunks();

  void IgnoreErrors(v8::Local<v8::Value>) {}

  ScriptPromise<IDLUndefined> InvokeShutdownAction() {
    return (this->*shutdown_action_)();
  }

  v8::Local<v8::Value> ShutdownError() const {
    DCHECK(!shutdown_error_.IsEmpty());
    return shutdown_error_.Get(script_state_->GetIsolate());
  }

  ScriptPromise<IDLUndefined> WritableStreamAbortAction();

  ScriptPromise<IDLUndefined> ReadableStreamCancelAction();

  ScriptPromise<IDLUndefined>
  WritableStreamDefaultWriterCloseWithErrorPropagationAction();

  WritableStream* Destination();

  const WritableStream* Destination() const;

  ReadableStream* Readable();

  Member<ScriptState> script_state_;
  Member<PipeOptions> pipe_options_;
  Member<ReadableStreamDefaultReader> reader_;
  Member<WritableStreamDefaultWriter> writer_;
  Member<ScriptPromiseResolver<IDLUndefined>> promise_;
  Member<AbortSignal::AlgorithmHandle> abort_handle_;
  MemberScriptPromise<IDLUndefined> last_write_;
  Action shutdown_action_;
  TraceWrapperV8Reference<v8::Value> shutdown_error_;
  bool is_shutting_down_ = false;
  bool is_reading_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_PIPE_TO_ENGINE_H_
