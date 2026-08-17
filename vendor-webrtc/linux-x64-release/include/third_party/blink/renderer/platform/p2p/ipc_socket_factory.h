// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_P2P_IPC_SOCKET_FACTORY_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_P2P_IPC_SOCKET_FACTORY_H_

#include <stdint.h>

#include "base/unguessable_token.h"
#include "net/traffic_annotation/network_traffic_annotation.h"
#include "third_party/blink/renderer/platform/heap/cross_thread_persistent.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"
#include "third_party/webrtc/api/packet_socket_factory.h"

namespace blink {

class P2PSocketDispatcher;

// IpcPacketSocketFactory implements webrtc::PacketSocketFactory
// interface for libjingle using IPC-based P2P sockets. The class must
// be created and used on a thread that is a libjingle thread (implements
// webrtc::Thread) and also has associated base::MessageLoop. Each
// socket created by the factory must be used on the thread it was
// created on.
// The class needs to be destroyed on the libjingle network thread.
class IpcPacketSocketFactory : public webrtc::PacketSocketFactory {
 public:
  PLATFORM_EXPORT explicit IpcPacketSocketFactory(
      WTF::CrossThreadFunction<
          void(base::OnceCallback<void(std::optional<base::UnguessableToken>)>)>
          devtools_token_getter,
      P2PSocketDispatcher* socket_dispatcher,
      const net::NetworkTrafficAnnotationTag& traffic_annotation,
      bool batch_udp_packets);
  IpcPacketSocketFactory(const IpcPacketSocketFactory&) = delete;
  IpcPacketSocketFactory& operator=(const IpcPacketSocketFactory&) = delete;
  ~IpcPacketSocketFactory() override;

  webrtc::AsyncPacketSocket* CreateUdpSocket(
      const webrtc::SocketAddress& local_address,
      uint16_t min_port,
      uint16_t max_port) override;
  webrtc::AsyncListenSocket* CreateServerTcpSocket(
      const webrtc::SocketAddress& local_address,
      uint16_t min_port,
      uint16_t max_port,
      int opts) override;
  webrtc::AsyncPacketSocket* CreateClientTcpSocket(
      const webrtc::SocketAddress& local_address,
      const webrtc::SocketAddress& remote_address,
      const webrtc::PacketSocketTcpOptions& opts) override;
  std::unique_ptr<webrtc::AsyncDnsResolverInterface> CreateAsyncDnsResolver()
      override;

 private:
  WTF::CrossThreadFunction<void(
      base::OnceCallback<void(std::optional<base::UnguessableToken>)>)>
      devtools_token_getter_;
  const bool batch_udp_packets_;

  // `P2PSocketDispatcher` is owned by the main thread, and must be accessed in
  // a thread-safe way. `this` is indirectly owned by
  // `PeerConnectionDependencyFactory`, which holds a hard reference to the
  // dispatcher, so this should be never be null (with the possible exception of
  // the dtor).
  CrossThreadWeakPersistent<P2PSocketDispatcher> socket_dispatcher_;
  const net::NetworkTrafficAnnotationTag traffic_annotation_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_P2P_IPC_SOCKET_FACTORY_H_
