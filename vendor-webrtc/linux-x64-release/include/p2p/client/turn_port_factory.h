/*
 *  Copyright 2017 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef P2P_CLIENT_TURN_PORT_FACTORY_H_
#define P2P_CLIENT_TURN_PORT_FACTORY_H_

#include <memory>

#include "p2p/base/port.h"
#include "p2p/client/relay_port_factory_interface.h"
#include "rtc_base/async_packet_socket.h"

namespace webrtc {

// This is a RelayPortFactory that produces TurnPorts.
class TurnPortFactory : public RelayPortFactoryInterface {
 public:
  ~TurnPortFactory() override;

  std::unique_ptr<Port> Create(const CreateRelayPortArgs& args,
                               AsyncPacketSocket* udp_socket) override;

  std::unique_ptr<Port> Create(const CreateRelayPortArgs& args,
                               int min_port,
                               int max_port) override;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::TurnPortFactory;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // P2P_CLIENT_TURN_PORT_FACTORY_H_
