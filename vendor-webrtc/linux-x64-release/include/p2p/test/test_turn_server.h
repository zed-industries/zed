/*
 *  Copyright 2012 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef P2P_TEST_TEST_TURN_SERVER_H_
#define P2P_TEST_TEST_TURN_SERVER_H_

#include <cstddef>
#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/sequence_checker.h"
#include "api/transport/stun.h"
#include "p2p/base/basic_packet_socket_factory.h"
#include "p2p/base/port_interface.h"
#include "p2p/test/turn_server.h"
#include "rtc_base/async_udp_socket.h"
#include "rtc_base/checks.h"
#include "rtc_base/net_helpers.h"
#include "rtc_base/socket.h"
#include "rtc_base/socket_address.h"
#include "rtc_base/socket_factory.h"
#include "rtc_base/ssl_adapter.h"
#include "rtc_base/ssl_identity.h"
#include "rtc_base/ssl_stream_adapter.h"
#include "rtc_base/thread.h"

namespace webrtc {

static const char kTestRealm[] = "example.org";
static const char kTestSoftware[] = "TestTurnServer";

class TestTurnRedirector : public TurnRedirectInterface {
 public:
  explicit TestTurnRedirector(const std::vector<SocketAddress>& addresses)
      : alternate_server_addresses_(addresses),
        iter_(alternate_server_addresses_.begin()) {}

  virtual bool ShouldRedirect(const SocketAddress&, SocketAddress* out) {
    if (!out || iter_ == alternate_server_addresses_.end()) {
      return false;
    }
    *out = *iter_++;
    return true;
  }

 private:
  const std::vector<SocketAddress>& alternate_server_addresses_;
  std::vector<SocketAddress>::const_iterator iter_;
};

class TestTurnServer : public TurnAuthInterface {
 public:
  TestTurnServer(Thread* thread,
                 SocketFactory* socket_factory,
                 const SocketAddress& int_addr,
                 const SocketAddress& udp_ext_addr,
                 ProtocolType int_protocol = webrtc::PROTO_UDP,
                 bool ignore_bad_cert = true,
                 absl::string_view common_name = "test turn server")
      : server_(thread), socket_factory_(socket_factory) {
    AddInternalSocket(int_addr, int_protocol, ignore_bad_cert, common_name);
    server_.SetExternalSocketFactory(
        new BasicPacketSocketFactory(socket_factory), udp_ext_addr);
    server_.set_realm(kTestRealm);
    server_.set_software(kTestSoftware);
    server_.set_auth_hook(this);
  }

  ~TestTurnServer() { RTC_DCHECK(thread_checker_.IsCurrent()); }

  void set_enable_otu_nonce(bool enable) {
    RTC_DCHECK(thread_checker_.IsCurrent());
    server_.set_enable_otu_nonce(enable);
  }

  TurnServer* server() {
    RTC_DCHECK(thread_checker_.IsCurrent());
    return &server_;
  }

  void set_redirect_hook(TurnRedirectInterface* redirect_hook) {
    RTC_DCHECK(thread_checker_.IsCurrent());
    server_.set_redirect_hook(redirect_hook);
  }

  void set_enable_permission_checks(bool enable) {
    RTC_DCHECK(thread_checker_.IsCurrent());
    server_.set_enable_permission_checks(enable);
  }

  void AddInternalSocket(const SocketAddress& int_addr,
                         ProtocolType proto,
                         bool ignore_bad_cert = true,
                         absl::string_view common_name = "test turn server") {
    RTC_DCHECK(thread_checker_.IsCurrent());
    if (proto == webrtc::PROTO_UDP) {
      server_.AddInternalSocket(
          AsyncUDPSocket::Create(socket_factory_, int_addr), proto);
    } else if (proto == webrtc::PROTO_TCP || proto == webrtc::PROTO_TLS) {
      // For TCP we need to create a server socket which can listen for incoming
      // new connections.
      Socket* socket = socket_factory_->CreateSocket(AF_INET, SOCK_STREAM);
      socket->Bind(int_addr);
      socket->Listen(5);
      if (proto == webrtc::PROTO_TLS) {
        // For TLS, wrap the TCP socket with an SSL adapter. The adapter must
        // be configured with a self-signed certificate for testing.
        // Additionally, the client will not present a valid certificate, so we
        // must not fail when checking the peer's identity.
        std::unique_ptr<SSLAdapterFactory> ssl_adapter_factory =
            SSLAdapterFactory::Create();
        ssl_adapter_factory->SetRole(webrtc::SSL_SERVER);
        ssl_adapter_factory->SetIdentity(
            SSLIdentity::Create(common_name, KeyParams()));
        ssl_adapter_factory->SetIgnoreBadCert(ignore_bad_cert);
        server_.AddInternalServerSocket(socket, proto,
                                        std::move(ssl_adapter_factory));
      } else {
        server_.AddInternalServerSocket(socket, proto);
      }
    } else {
      RTC_DCHECK_NOTREACHED() << "Unknown protocol type: " << proto;
    }
  }

  // Finds the first allocation in the server allocation map with a source
  // ip and port matching the socket address provided.
  TurnServerAllocation* FindAllocation(const SocketAddress& src) {
    RTC_DCHECK(thread_checker_.IsCurrent());
    const TurnServer::AllocationMap& map = server_.allocations();
    for (TurnServer::AllocationMap::const_iterator it = map.begin();
         it != map.end(); ++it) {
      if (src == it->first.src()) {
        return it->second.get();
      }
    }
    return NULL;
  }

 private:
  // For this test server, succeed if the password is the same as the username.
  // Obviously, do not use this in a production environment.
  virtual bool GetKey(absl::string_view username,
                      absl::string_view realm,
                      std::string* key) {
    RTC_DCHECK(thread_checker_.IsCurrent());
    return ComputeStunCredentialHash(std::string(username), std::string(realm),
                                     std::string(username), key);
  }

  TurnServer server_;
  SocketFactory* socket_factory_;
  SequenceChecker thread_checker_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::kTestRealm;
using ::webrtc::kTestSoftware;
using ::webrtc::TestTurnRedirector;
using ::webrtc::TestTurnServer;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // P2P_TEST_TEST_TURN_SERVER_H_
