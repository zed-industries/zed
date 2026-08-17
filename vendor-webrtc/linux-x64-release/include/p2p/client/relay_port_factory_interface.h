/*
 *  Copyright 2017 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef P2P_CLIENT_RELAY_PORT_FACTORY_INTERFACE_H_
#define P2P_CLIENT_RELAY_PORT_FACTORY_INTERFACE_H_

#include <memory>
#include <string>

#include "api/environment/environment.h"
#include "api/packet_socket_factory.h"
#include "p2p/base/port.h"
#include "p2p/base/port_allocator.h"
#include "rtc_base/async_packet_socket.h"
#include "rtc_base/network.h"
#include "rtc_base/thread.h"

namespace webrtc {
class TurnCustomizer;
class FieldTrialsView;
}  // namespace webrtc

namespace webrtc {

// A struct containing arguments to RelayPortFactory::Create()
struct CreateRelayPortArgs {
  Environment env;
  Thread* network_thread;
  PacketSocketFactory* socket_factory;
  const Network* network;
  const ProtocolAddress* server_address;
  const RelayServerConfig* config;
  std::string username;
  std::string password;
  TurnCustomizer* turn_customizer = nullptr;
  // Relative priority of candidates from this TURN server in relation
  // to the candidates from other servers. Required because ICE priorities
  // need to be unique.
  int relative_priority = 0;
};

// A factory for creating RelayPort's.
class RelayPortFactoryInterface {
 public:
  virtual ~RelayPortFactoryInterface() {}

  // This variant is used for UDP connection to the relay server
  // using a already existing shared socket.
  virtual std::unique_ptr<Port> Create(const CreateRelayPortArgs& args,
                                       AsyncPacketSocket* udp_socket) = 0;

  // This variant is used for the other cases.
  virtual std::unique_ptr<Port> Create(const CreateRelayPortArgs& args,
                                       int min_port,
                                       int max_port) = 0;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::CreateRelayPortArgs;
using ::webrtc::RelayPortFactoryInterface;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // P2P_CLIENT_RELAY_PORT_FACTORY_INTERFACE_H_
