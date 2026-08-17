/*
 *  Copyright 2004 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef P2P_TEST_NAT_SOCKET_FACTORY_H_
#define P2P_TEST_NAT_SOCKET_FACTORY_H_

#include <stddef.h>

#include <cstdint>
#include <map>
#include <memory>
#include <set>

#include "api/array_view.h"
#include "api/units/time_delta.h"
#include "p2p/test/nat_server.h"
#include "p2p/test/nat_types.h"
#include "rtc_base/socket.h"
#include "rtc_base/socket_address.h"
#include "rtc_base/socket_factory.h"
#include "rtc_base/socket_server.h"
#include "rtc_base/thread.h"

namespace webrtc {

const size_t kNATEncodedIPv4AddressSize = 8U;
const size_t kNATEncodedIPv6AddressSize = 20U;

// Used by the NAT socket implementation.
class NATInternalSocketFactory {
 public:
  virtual ~NATInternalSocketFactory() {}
  virtual Socket* CreateInternalSocket(int family,
                                       int type,
                                       const SocketAddress& local_addr,
                                       SocketAddress* nat_addr) = 0;
};

// Creates sockets that will send all traffic through a NAT, using an existing
// NATServer instance running at nat_addr. The actual data is sent using sockets
// from a socket factory, given to the constructor.
class NATSocketFactory : public SocketFactory, public NATInternalSocketFactory {
 public:
  NATSocketFactory(SocketFactory* factory,
                   const SocketAddress& nat_udp_addr,
                   const SocketAddress& nat_tcp_addr);

  NATSocketFactory(const NATSocketFactory&) = delete;
  NATSocketFactory& operator=(const NATSocketFactory&) = delete;

  // SocketFactory implementation
  Socket* CreateSocket(int family, int type) override;

  // NATInternalSocketFactory implementation
  Socket* CreateInternalSocket(int family,
                               int type,
                               const SocketAddress& local_addr,
                               SocketAddress* nat_addr) override;

 private:
  SocketFactory* factory_;
  SocketAddress nat_udp_addr_;
  SocketAddress nat_tcp_addr_;
};

// Creates sockets that will send traffic through a NAT depending on what
// address they bind to. This can be used to simulate a client on a NAT sending
// to a client that is not behind a NAT.
// Note that the internal addresses of clients must be unique. This is because
// there is only one socketserver per thread, and the Bind() address is used to
// figure out which NAT (if any) the socket should talk to.
//
// Example with 3 NATs (2 cascaded), and 3 clients.
// ss->AddTranslator("1.2.3.4", "192.168.0.1", NAT_ADDR_RESTRICTED);
// ss->AddTranslator("99.99.99.99", "10.0.0.1", NAT_SYMMETRIC)->
//     AddTranslator("10.0.0.2", "192.168.1.1", NAT_OPEN_CONE);
// ss->GetTranslator("1.2.3.4")->AddClient("1.2.3.4", "192.168.0.2");
// ss->GetTranslator("99.99.99.99")->AddClient("10.0.0.3");
// ss->GetTranslator("99.99.99.99")->GetTranslator("10.0.0.2")->
//     AddClient("192.168.1.2");
class NATSocketServer : public SocketServer, public NATInternalSocketFactory {
 public:
  class Translator;

  // holds a list of NATs
  class TranslatorMap : private std::map<SocketAddress, Translator*> {
   public:
    ~TranslatorMap();
    Translator* Get(const SocketAddress& ext_ip);
    Translator* Add(const SocketAddress& ext_ip, Translator*);
    void Remove(const SocketAddress& ext_ip);
    Translator* FindClient(const SocketAddress& int_ip);
  };

  // a specific NAT
  class Translator {
   public:
    Translator(NATSocketServer* server,
               NATType type,
               const SocketAddress& int_addr,
               Thread& external_socket_thread,
               SocketFactory* ext_factory,
               const SocketAddress& ext_addr);
    ~Translator();

    SocketFactory* internal_factory() { return internal_server_.get(); }
    SocketAddress internal_udp_address() const {
      return nat_server_->internal_udp_address();
    }
    SocketAddress internal_tcp_address() const {
      return SocketAddress();  // nat_server_->internal_tcp_address();
    }

    Translator* GetTranslator(const SocketAddress& ext_ip);
    Translator* AddTranslator(const SocketAddress& ext_ip,
                              const SocketAddress& int_ip,
                              NATType type);
    void RemoveTranslator(const SocketAddress& ext_ip);

    bool AddClient(const SocketAddress& int_ip);
    void RemoveClient(const SocketAddress& int_ip);

    // Looks for the specified client in this or a child NAT.
    Translator* FindClient(const SocketAddress& int_ip);

   private:
    NATSocketServer* server_;
    std::unique_ptr<SocketServer> internal_server_;
    std::unique_ptr<NATServer> nat_server_;
    TranslatorMap nats_;
    std::set<SocketAddress> clients_;
  };

  explicit NATSocketServer(SocketServer* ss);

  NATSocketServer(const NATSocketServer&) = delete;
  NATSocketServer& operator=(const NATSocketServer&) = delete;

  SocketServer* socketserver() { return server_; }
  Thread* queue() { return msg_queue_; }

  Translator* GetTranslator(const SocketAddress& ext_ip);
  Translator* AddTranslator(const SocketAddress& ext_ip,
                            const SocketAddress& int_ip,
                            NATType type);
  void RemoveTranslator(const SocketAddress& ext_ip);

  // SocketServer implementation
  Socket* CreateSocket(int family, int type) override;

  void SetMessageQueue(Thread* queue) override;
  bool Wait(TimeDelta max_wait_duration, bool process_io) override;
  void WakeUp() override;

  // NATInternalSocketFactory implementation
  Socket* CreateInternalSocket(int family,
                               int type,
                               const SocketAddress& local_addr,
                               SocketAddress* nat_addr) override;

 private:
  SocketServer* server_;
  Thread* msg_queue_;
  TranslatorMap nats_;
};

// Free-standing NAT helper functions.
size_t PackAddressForNAT(char* buf,
                         size_t buf_size,
                         const SocketAddress& remote_addr);
size_t UnpackAddressFromNAT(ArrayView<const uint8_t> buf,
                            SocketAddress* remote_addr);
}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::kNATEncodedIPv4AddressSize;
using ::webrtc::kNATEncodedIPv6AddressSize;
using ::webrtc::NATInternalSocketFactory;
using ::webrtc::NATSocketFactory;
using ::webrtc::NATSocketServer;
using ::webrtc::PackAddressForNAT;
using ::webrtc::UnpackAddressFromNAT;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // P2P_TEST_NAT_SOCKET_FACTORY_H_
