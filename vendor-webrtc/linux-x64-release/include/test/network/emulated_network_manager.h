/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_NETWORK_EMULATED_NETWORK_MANAGER_H_
#define TEST_NETWORK_EMULATED_NETWORK_MANAGER_H_

#include <functional>
#include <memory>
#include <vector>

#include "absl/base/nullability.h"
#include "api/task_queue/task_queue_base.h"
#include "api/test/network_emulation/network_emulation_interfaces.h"
#include "api/test/network_emulation_manager.h"
#include "api/test/time_controller.h"
#include "rtc_base/network.h"
#include "rtc_base/socket_factory.h"
#include "rtc_base/socket_server.h"
#include "rtc_base/thread.h"
#include "test/network/network_emulation.h"

namespace webrtc {
namespace test {

class EmulatedNetworkManager : public EmulatedNetworkManagerInterface {
 public:
  EmulatedNetworkManager(TimeController* absl_nonnull time_controller,
                         TaskQueueBase* absl_nonnull task_queue,
                         EndpointsContainer* absl_nonnull endpoints_container);
  ~EmulatedNetworkManager() override;

  void UpdateNetworks();

  Thread* absl_nonnull network_thread() override {
    return network_thread_.get();
  }
  SocketFactory* absl_nonnull socket_factory() override {
    return socket_server_;
  }
  absl_nonnull std::unique_ptr<NetworkManager> ReleaseNetworkManager() override;

  std::vector<EmulatedEndpoint*> endpoints() const override {
    return endpoints_container_->GetEndpoints();
  }
  void GetStats(
      std::function<void(EmulatedNetworkStats)> stats_callback) const override;

 private:
  class NetworkManagerImpl;

  TaskQueueBase* absl_nonnull const task_queue_;
  const EndpointsContainer* absl_nonnull const endpoints_container_;

  // Socket server is owned by the `network_thread_'
  SocketServer* absl_nonnull const socket_server_;

  const absl_nonnull std::unique_ptr<Thread> network_thread_;
  absl_nullable std::unique_ptr<NetworkManagerImpl> network_manager_;

  // Keep pointer to the network manager when it is extracted to be injected
  // into PeerConnectionFactory. That is brittle and may crash if a test would
  // try to use emulated network after related PeerConnectionFactory is deleted.
  NetworkManagerImpl* absl_nonnull const network_manager_ptr_;
};

}  // namespace test
}  // namespace webrtc

#endif  // TEST_NETWORK_EMULATED_NETWORK_MANAGER_H_
