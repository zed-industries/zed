// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_DIRECT_SOCKETS_TCP_SERVER_SOCKET_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_DIRECT_SOCKETS_TCP_SERVER_SOCKET_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise_property.h"
#include "third_party/blink/renderer/modules/direct_sockets/socket.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class ExceptionState;
class ScriptState;
class TCPServerReadableStreamWrapper;
class TCPServerSocketOpenInfo;
class TCPServerSocketOptions;

class MODULES_EXPORT TCPServerSocket final : public ScriptWrappable,
                                             public Socket {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // IDL definitions
  static TCPServerSocket* Create(ScriptState*,
                                 const String& local_address,
                                 const TCPServerSocketOptions*,
                                 ExceptionState&);

  // Socket:
  ScriptPromise<TCPServerSocketOpenInfo> opened(ScriptState*) const;
  ScriptPromise<IDLUndefined> close(ScriptState*, ExceptionState&) override;

  explicit TCPServerSocket(ScriptState*);
  ~TCPServerSocket() override;

  // Validates options and calls OpenTCPServerSocket(...).
  bool Open(const String& local_address,
            const TCPServerSocketOptions*,
            ExceptionState&);

  // On net::OK initializes readable stream and resolves opened promise.
  // Otherwise rejects the opened promise.
  void OnTCPServerSocketOpened(
      mojo::PendingRemote<network::mojom::blink::TCPServerSocket>,
      int32_t result,
      const std::optional<net::IPEndPoint>& local_addr);

  void Trace(Visitor*) const override;

  // Socket:
  void ContextDestroyed() override;

 private:
  // Resets mojo resources held by this class.
  void ReleaseResources();

  void OnReadableStreamClosed(v8::Local<v8::Value> exception, int net_error);

  Member<ScriptPromiseProperty<TCPServerSocketOpenInfo, DOMException>> opened_;

  Member<TCPServerReadableStreamWrapper> readable_stream_wrapper_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_DIRECT_SOCKETS_TCP_SERVER_SOCKET_H_
