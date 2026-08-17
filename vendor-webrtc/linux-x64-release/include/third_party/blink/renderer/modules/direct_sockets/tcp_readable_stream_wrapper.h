// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_DIRECT_SOCKETS_TCP_READABLE_STREAM_WRAPPER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_DIRECT_SOCKETS_TCP_READABLE_STREAM_WRAPPER_H_

#include "base/functional/callback.h"
#include "mojo/public/cpp/system/data_pipe.h"
#include "mojo/public/cpp/system/handle_signals_state.h"
#include "mojo/public/cpp/system/simple_watcher.h"
#include "net/base/net_errors.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/dom/dom_exception.h"
#include "third_party/blink/renderer/modules/direct_sockets/stream_wrapper.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/exception_code.h"
#include "third_party/blink/renderer/platform/bindings/trace_wrapper_v8_reference.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"

namespace blink {

// Helper class to read from a mojo consumer handle
class MODULES_EXPORT TCPReadableStreamWrapper
    : public GarbageCollected<TCPReadableStreamWrapper>,
      public ReadableByteStreamWrapper {
  USING_PRE_FINALIZER(TCPReadableStreamWrapper, Dispose);

 public:
  TCPReadableStreamWrapper(ScriptState*,
                           CloseOnceCallback,
                           mojo::ScopedDataPipeConsumerHandle,
                           uint64_t inspector_id);

  // ReadableStreamWrapper:
  void Pull() override;
  void CloseStream() override;
  void ErrorStream(int32_t error_code) override;
  void Trace(Visitor*) const override;

 private:
  // Called when |data_pipe_| becomes readable or errored.
  void OnHandleReady(MojoResult, const mojo::HandleSignalsState&);

  // Called when |data_pipe_| gets reset.
  void OnHandleReset(MojoResult, const mojo::HandleSignalsState&);

  // Resets |data_pipe_| and clears the watcher.
  void ResetPipe();

  // Prepares the object for destruction.
  void Dispose();

  CloseOnceCallback on_close_;

  mojo::ScopedDataPipeConsumerHandle data_pipe_;

  // Only armed when we need to read something.
  mojo::SimpleWatcher read_watcher_;

  // Always armed to detect pipe close.
  mojo::SimpleWatcher close_watcher_;

  // Indicates whether peer closed gracefully (EOF).
  bool graceful_peer_shutdown_ = false;

  // Stores a v8::Local<v8::Value> V8DOMException inside.
  TraceWrapperV8Reference<v8::Value> pending_exception_;
  int pending_net_error_ = net::OK;

  // Unique id for devtools inspector_network_agent.
  const uint64_t inspector_id_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_DIRECT_SOCKETS_TCP_READABLE_STREAM_WRAPPER_H_
