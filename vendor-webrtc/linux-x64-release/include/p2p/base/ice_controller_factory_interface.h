/*
 *  Copyright 2019 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef P2P_BASE_ICE_CONTROLLER_FACTORY_INTERFACE_H_
#define P2P_BASE_ICE_CONTROLLER_FACTORY_INTERFACE_H_

#include <functional>
#include <memory>
#include <string>

#include "p2p/base/connection.h"
#include "p2p/base/ice_controller_interface.h"
#include "p2p/base/ice_transport_internal.h"
#include "p2p/base/transport_description.h"

namespace webrtc {

// struct with arguments to IceControllerFactoryInterface::Create
struct IceControllerFactoryArgs {
  std::function<IceTransportStateInternal()> ice_transport_state_func;
  std::function<IceRole()> ice_role_func;
  std::function<bool(const webrtc::Connection*)> is_connection_pruned_func;
  const IceFieldTrials* ice_field_trials;
  std::string ice_controller_field_trials;
};

class IceControllerFactoryInterface {
 public:
  virtual ~IceControllerFactoryInterface() = default;
  virtual std::unique_ptr<IceControllerInterface> Create(
      const IceControllerFactoryArgs& args) = 0;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::IceControllerFactoryArgs;
using ::webrtc::IceControllerFactoryInterface;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // P2P_BASE_ICE_CONTROLLER_FACTORY_INTERFACE_H_
