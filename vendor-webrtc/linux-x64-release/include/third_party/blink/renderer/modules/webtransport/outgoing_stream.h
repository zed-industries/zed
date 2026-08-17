// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBTRANSPORT_OUTGOING_STREAM_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBTRANSPORT_OUTGOING_STREAM_H_

#include <cstddef>
#include <cstdint>

#include "base/containers/span.h"
#include "base/memory/raw_ptr.h"
#include "base/types/strong_alias.h"
#include "mojo/public/cpp/system/data_pipe.h"
#include "mojo/public/cpp/system/simple_watcher.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/dom/abort_signal.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "v8/include/v8.h"

namespace blink {

class ExceptionState;
class ScriptState;
class WritableStream;
class WritableStreamDefaultController;

// Implementation of the OutgoingStream mixin from the standard. SendStream and
// BidirectionalStream delegate to this.
class MODULES_EXPORT OutgoingStream final
    : public GarbageCollected<OutgoingStream> {
  USING_PRE_FINALIZER(OutgoingStream, Dispose);

 public:
  // An interface for SendStream and BidirectionalStream to implement when using
  // this class. At most one of these methods will be called.
  class Client : public GarbageCollectedMixin {
   public:
    virtual ~Client() = default;

    // Request that a Fin message for this stream be sent to the server.
    virtual void SendFin() = 0;

    // Notify that the stream is either closed or errored and WebTransport
    // should drop its reference to the stream.
    virtual void ForgetStream() = 0;

    // Send RESET_STREAM with `code`. This does not imply ForgetStream().
    virtual void Reset(uint8_t code) = 0;
  };

  enum class State {
    kOpen,
    kSentFin,
    kAborted,
  };

  OutgoingStream(ScriptState*, Client*, mojo::ScopedDataPipeProducerHandle);
  ~OutgoingStream();

  // Init() or InitWithExistingWritableStream() must be called before the stream
  // is used.
  void Init(ExceptionState&);

  void InitWithExistingWritableStream(WritableStream*, ExceptionState&);

  void AbortAlgorithm(OutgoingStream*);

  // Implementation of OutgoingStream IDL, used by client classes to implement
  // it. https://wicg.github.io/web-transport/#outgoing-stream
  WritableStream* Writable() const {
    DVLOG(1) << "OutgoingStream::writable() called";

    return writable_.Get();
  }

  ScriptState* GetScriptState() { return script_state_.Get(); }

  // Called from WebTransport via a WebTransportStream.
  void OnOutgoingStreamClosed();

  // Errors the associated stream with the given reason. Expects a JavaScript
  // scope to be entered.
  void Error(ScriptValue reason);

  State GetState() const { return state_; }

  // Called from WebTransport rather than using
  // ExecutionContextLifecycleObserver to ensure correct destruction order.
  // Does not execute JavaScript.
  void ContextDestroyed();

  void Trace(Visitor*) const;

 private:
  class UnderlyingSink;

  using IsLocalAbort = base::StrongAlias<class IsLocalAbortTag, bool>;

  // Called when |data_pipe_| becomes writable or errored.
  void OnHandleReady(MojoResult, const mojo::HandleSignalsState&);

  // Called when |data_pipe_| is closed.
  void OnPeerClosed(MojoResult, const mojo::HandleSignalsState&);

  // Rejects any unfinished write() calls and resets |data_pipe_|.
  void HandlePipeClosed();

  // Implements UnderlyingSink::write().
  ScriptPromise<IDLUndefined> SinkWrite(ScriptState*,
                                        ScriptValue chunk,
                                        ExceptionState&);

  // Writes |data| to |data_pipe_|, possible saving unwritten data to
  // |cached_data_|.
  ScriptPromise<IDLUndefined> WriteOrCacheData(ScriptState*,
                                               base::span<const uint8_t> data);

  // Attempts to write some more of |cached_data_| to |data_pipe_|.
  void WriteCachedData();

  // Writes zero or more bytes of |data| synchronously to |data_pipe_|,
  // returning the number of bytes that were written.
  size_t WriteDataSynchronously(base::span<const uint8_t> data);

  // Creates a DOMException indicating that the stream has been aborted.
  // If IsLocalAbort it true it will indicate a locally-initiated abort,
  // otherwise it will indicate a remote-initiated abort.
  ScriptValue CreateAbortException(IsLocalAbort);

  // Errors |writable_|, and resets |data_pipe_|.
  void ErrorStreamAbortAndReset(ScriptValue reason);

  // Reset the |data_pipe_|.
  void AbortAndReset();

  // Resets |data_pipe_| and clears the watchers. Also discards |cached_data_|.
  // If the pipe is open it will be closed as a side-effect.
  void ResetPipe();

  // Prepares the object for destruction.
  void Dispose();

  const Member<ScriptState> script_state_;
  Member<Client> client_;
  mojo::ScopedDataPipeProducerHandle data_pipe_;

  // Only armed when we need to write something.
  mojo::SimpleWatcher write_watcher_;

  // Always armed to detect close.
  mojo::SimpleWatcher close_watcher_;

  // Data which has been passed to write() but still needs to be written
  // asynchronously.
  // Uses a custom CachedDataBuffer rather than a Vector because
  // WTF::Vector is currently limited to 2GB.
  // TODO(ricea): Change this to a Vector when it becomes 64-bit safe.
  class CachedDataBuffer;
  std::unique_ptr<CachedDataBuffer> cached_data_;

  // The offset into |cached_data_| of the first byte that still needs to be
  // written.
  size_t offset_ = 0;

  Member<WritableStream> writable_;
  Member<AbortSignal::AlgorithmHandle> send_stream_abort_handle_;
  Member<WritableStreamDefaultController> controller_;

  // If an asynchronous write() on the underlying sink object is pending, this
  // will be non-null.
  Member<ScriptPromiseResolver<IDLUndefined>> write_promise_resolver_;

  // If a close() on the underlying sink object is pending, this will be
  // non-null.
  Member<ScriptPromiseResolver<IDLUndefined>> close_promise_resolver_;

  Member<ScriptPromiseResolver<IDLUndefined>> pending_operation_;

  State state_ = State::kOpen;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBTRANSPORT_OUTGOING_STREAM_H_
