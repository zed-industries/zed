// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_DIRECT_SOCKETS_TCP_WRITABLE_STREAM_WRAPPER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_DIRECT_SOCKETS_TCP_WRITABLE_STREAM_WRAPPER_H_

#include "base/notreached.h"
#include "mojo/public/cpp/system/data_pipe.h"
#include "mojo/public/cpp/system/simple_watcher.h"
#include "partition_alloc/partition_root.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_typedefs.h"
#include "third_party/blink/renderer/core/dom/dom_exception.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/direct_sockets/stream_wrapper.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/exception_code.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"

namespace v8 {
class Isolate;
}

namespace blink {

// Helper class to write to a mojo producer handle
class MODULES_EXPORT TCPWritableStreamWrapper
    : public GarbageCollected<TCPWritableStreamWrapper>,
      public WritableStreamWrapper {
  USING_PRE_FINALIZER(TCPWritableStreamWrapper, Dispose);

 public:
  TCPWritableStreamWrapper(ScriptState*,
                           CloseOnceCallback,
                           mojo::ScopedDataPipeProducerHandle,
                           uint64_t inspector_id);

  // WritableStreamWrapper:
  void CloseStream() override;
  void ErrorStream(int32_t error_code) override;
  bool HasPendingWrite() const override;
  void Trace(Visitor*) const override;
  void OnAbortSignal() override;
  ScriptPromise<IDLUndefined> Write(ScriptValue chunk,
                                    ExceptionState&) override;

 private:
  // Called when |data_pipe_| becomes writable or errored.
  void OnHandleReady(MojoResult, const mojo::HandleSignalsState&);

  // Called when |data_pipe_| is closed.
  void OnHandleReset(MojoResult, const mojo::HandleSignalsState&);

  // Writes data contained in |buffer_source_| to |data_pipe_|, possibly in
  // several asynchronous attempts.
  void WriteDataAsynchronously();

  // Writes zero or more bytes of |data| synchronously to |data_pipe_|,
  // returning the number of bytes that were written.
  size_t WriteDataSynchronously(base::span<const uint8_t> data);

  // Resolves |write_promise_resolver_| and resets |buffer_source_| if write
  // operation finished successfully.
  void FinalizeWrite();

  // Errors |writable_|, resolves |writing_aborted_| and resets |data_pipe_|.
  void ErrorStreamAbortAndReset(bool error);

  // Resets |data_pipe_| and clears the watchers. Also discards |cached_data_|.
  void ResetPipe();

  // Prepares the object for destruction.
  void Dispose();

  // Reports write error to Devtools Protocol.
  void ReportWriteError(const WTF::String& message);

  CloseOnceCallback on_close_;

  mojo::ScopedDataPipeProducerHandle data_pipe_;

  // Only armed when we need to write something.
  mojo::SimpleWatcher write_watcher_;

  // Always armed to detect pipe close.
  mojo::SimpleWatcher close_watcher_;

  // Data which has been passed to write() but still needs to be written
  // asynchronously.
  Member<V8BufferSource> buffer_source_;

  // The offset into |cached_data_| of the first byte that still needs to be
  // written.
  size_t offset_ = 0;

  // If an asynchronous write() on the underlying sink object is pending, this
  // will be non-null.
  Member<ScriptPromiseResolver<IDLUndefined>> write_promise_resolver_;

  // Unique id for devtools inspector_network_agent.
  const uint64_t inspector_id_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_DIRECT_SOCKETS_TCP_WRITABLE_STREAM_WRAPPER_H_
