// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBTRANSPORT_INCOMING_STREAM_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBTRANSPORT_INCOMING_STREAM_H_

#include <stdint.h>

#include <optional>

#include "base/functional/callback.h"
#include "base/logging.h"
#include "base/types/strong_alias.h"
#include "mojo/public/cpp/system/data_pipe.h"
#include "mojo/public/cpp/system/simple_watcher.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"

namespace blink {

class ScriptState;
class ReadableStream;
class ReadableByteStreamController;

// Implementation of the IncomingStream mixin from the standard:
// https://wicg.github.io/web-transport/#incoming-stream. ReceiveStream and
// BidirectionalStream delegate to this to implement the functionality.
class MODULES_EXPORT IncomingStream final
    : public GarbageCollected<IncomingStream> {
  USING_PRE_FINALIZER(IncomingStream, Dispose);

 public:
  enum class State {
    kOpen,
    kAborted,
    kClosed,
  };

  IncomingStream(ScriptState*,
                 base::OnceCallback<void(std::optional<uint8_t>)> on_abort,
                 mojo::ScopedDataPipeConsumerHandle);
  ~IncomingStream();

  // Init() or InitWithExistingReadableStream() must be called before the stream
  // is used.
  void Init(ExceptionState&);

  void InitWithExistingReadableStream(ReadableStream*, ExceptionState&);

  // Methods from the IncomingStream IDL:
  // https://wicg.github.io/web-transport/#incoming-stream
  ReadableStream* Readable() const {
    DVLOG(1) << "IncomingStream::readable() called";

    return readable_.Get();
  }

  // Called from WebTransport via a WebTransportStream class. May execute
  // JavaScript.
  void OnIncomingStreamClosed(bool fin_received);

  // Errors the associated stream with the given reason. Expects a
  // JavaScript scope to have been entered.
  void Error(ScriptValue reason);

  // Called from WebTransport rather than using
  // ExecutionContextLifecycleObserver to ensure correct destruction order.
  // Does not execute JavaScript.
  void ContextDestroyed();

  State GetState() const { return state_; }

  void Trace(Visitor*) const;

 private:
  class UnderlyingByteSource;

  // Called when |data_pipe_| becomes readable, closed or errored.
  void OnHandleReady(MojoResult, const mojo::HandleSignalsState&);

  // Rejects any unfinished read() calls and resets |data_pipe_|.
  void HandlePipeClosed();

  // Handles a remote close appropriately for the value of |fin_received_|.
  void ProcessClose();

  // Reads all the data currently in the pipe and enqueues it. If no data is
  // currently available, triggers the |read_watcher_| and enqueues when data
  // becomes available.
  void ReadFromPipeAndEnqueue(ExceptionState&);

  // Responds current BYOB request or copies a sequence of bytes into an
  // ArrayBuffer and enqueues it if there is no BYOB request. Returns the size
  // of bytes responded or copied.
  size_t RespondBYOBRequestOrEnqueueBytes(base::span<const uint8_t> source,
                                          ExceptionState&);

  // Closes |readable_|, and resets |data_pipe_|.
  void CloseAbortAndReset(ExceptionState&);

  // Errors |readable_|, and resets |data_pipe_|.
  // |exception| will be set as the error on |readable_|.
  void ErrorStreamAbortAndReset(ScriptValue exception);

  // Resets the |data_pipe_|.
  void AbortAndReset(std::optional<uint8_t> code);

  // Resets |data_pipe_| and clears the watchers.
  // If the pipe is open it will be closed as a side-effect.
  void ResetPipe();

  // Prepares the object for destruction.
  void Dispose();

  const Member<ScriptState> script_state_;

  base::OnceCallback<void(std::optional<uint8_t>)> on_abort_;

  mojo::ScopedDataPipeConsumerHandle data_pipe_;

  // Only armed when we need to read something.
  mojo::SimpleWatcher read_watcher_;

  Member<ReadableStream> readable_;
  Member<ReadableByteStreamController> controller_;

  State state_ = State::kOpen;

  // This is set when OnIncomingStreamClosed() is called.
  std::optional<bool> fin_received_;

  // True when |data_pipe_| has been detected to be closed. The close is not
  // processed until |fin_received_| is also set.
  bool is_pipe_closed_ = false;

  // Indicates if we are currently performing a two-phase read from the pipe and
  // so can't start another read.
  bool in_two_phase_read_ = false;

  // Indicates if we need to perform another read after the current one
  // completes.
  bool read_pending_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBTRANSPORT_INCOMING_STREAM_H_
