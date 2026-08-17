// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_DIRECT_SOCKETS_UDP_READABLE_STREAM_WRAPPER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_DIRECT_SOCKETS_UDP_READABLE_STREAM_WRAPPER_H_

#include "base/functional/callback_forward.h"
#include "base/time/time.h"
#include "services/network/public/mojom/udp_socket.mojom-blink.h"
#include "third_party/blink/renderer/modules/direct_sockets/stream_wrapper.h"
#include "third_party/blink/renderer/modules/direct_sockets/udp_socket_mojo_remote.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"

namespace blink {

class ScriptState;

class MODULES_EXPORT UDPReadableStreamWrapper
    : public GarbageCollected<UDPReadableStreamWrapper>,
      public ReadableStreamDefaultWrapper,
      public network::mojom::blink::UDPSocketListener {
 public:
  UDPReadableStreamWrapper(
      ScriptState*,
      CloseOnceCallback,
      const Member<UDPSocketMojoRemote> udp_socket,
      mojo::PendingReceiver<network::mojom::blink::UDPSocketListener>
          socket_listener);

  // ReadableStreamWrapper:
  void Pull() override;
  void CloseStream() override;
  void ErrorStream(int32_t error_code) override;
  void Trace(Visitor*) const override;

  // network::mojom::blink::UDPSocketListener:
  void OnReceived(int32_t result,
                  const std::optional<net::IPEndPoint>& src_addr,
                  std::optional<base::span<const uint8_t>> data) override;

 private:
  CloseOnceCallback on_close_;
  const Member<UDPSocketMojoRemote> udp_socket_;
  int32_t pending_receive_requests_ = 0;

  HeapMojoReceiver<network::mojom::blink::UDPSocketListener,
                   UDPReadableStreamWrapper>
      socket_listener_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_DIRECT_SOCKETS_UDP_READABLE_STREAM_WRAPPER_H_
