/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_PEER_NETWORK_DEPENDENCIES_H_
#define API_TEST_PEER_NETWORK_DEPENDENCIES_H_

#include <memory>

#include "absl/base/nullability.h"
#include "rtc_base/network.h"
#include "rtc_base/socket_factory.h"
#include "rtc_base/thread.h"

namespace webrtc {
namespace webrtc_pc_e2e {

// Provides interface to obtain all required objects to inject network layer
// into PeerConnectionFactory.
class PeerNetworkDependencies {
 public:
  virtual ~PeerNetworkDependencies() = default;

  virtual Thread* absl_nonnull network_thread() = 0;
  virtual SocketFactory* absl_nonnull socket_factory() = 0;
  virtual absl_nonnull std::unique_ptr<NetworkManager>
  ReleaseNetworkManager() = 0;
};

}  // namespace webrtc_pc_e2e
}  // namespace webrtc

#endif  // API_TEST_PEER_NETWORK_DEPENDENCIES_H_
