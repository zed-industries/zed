// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_DIRECT_SOCKETS_UDP_SOCKET_MOJO_REMOTE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_DIRECT_SOCKETS_UDP_SOCKET_MOJO_REMOTE_H_

#include "services/network/public/mojom/restricted_udp_socket.mojom-blink.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"

namespace blink {

// A wrapper class of HeapMojoRemote<RestrictedUDPSocket> so that multiple
// owners share the same HeapMojoRemote.
class MODULES_EXPORT UDPSocketMojoRemote
    : public GarbageCollected<UDPSocketMojoRemote> {
 public:
  explicit UDPSocketMojoRemote(ExecutionContext* execution_context);
  ~UDPSocketMojoRemote();

  HeapMojoRemote<network::mojom::blink::RestrictedUDPSocket>& get() {
    return udp_socket_;
  }

  void Close();

  void Trace(Visitor* visitor) const;

 private:
  HeapMojoRemote<network::mojom::blink::RestrictedUDPSocket> udp_socket_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_DIRECT_SOCKETS_UDP_SOCKET_MOJO_REMOTE_H_
