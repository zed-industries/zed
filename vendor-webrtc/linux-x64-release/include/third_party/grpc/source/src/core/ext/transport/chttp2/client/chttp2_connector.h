//
//
// Copyright 2015 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//

#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_CLIENT_CHTTP2_CONNECTOR_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_CLIENT_CHTTP2_CONNECTOR_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/support/port_platform.h>

#include <optional>

#include "absl/base/thread_annotations.h"
#include "src/core/client_channel/connector.h"
#include "src/core/handshaker/handshaker.h"
#include "src/core/lib/iomgr/closure.h"
#include "src/core/lib/iomgr/endpoint.h"
#include "src/core/lib/iomgr/error.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/sync.h"

namespace grpc_core {

class Chttp2Connector : public SubchannelConnector {
 public:
  void Connect(const Args& args, Result* result, grpc_closure* notify) override;
  void Shutdown(grpc_error_handle error) override;

 private:
  void OnHandshakeDone(absl::StatusOr<HandshakerArgs*> result);
  static void OnReceiveSettings(void* arg, grpc_error_handle error);
  void OnTimeout() ABSL_LOCKS_EXCLUDED(mu_);

  // We cannot invoke notify_ until both OnTimeout() and OnReceiveSettings()
  // have been called since that is an indicator to the upper layer that we are
  // done with the connection attempt. So, the notification process is broken
  // into two steps. 1) Either OnTimeout() or OnReceiveSettings() gets invoked
  // first. Whichever gets invoked, calls MaybeNotify() to set the result and
  // triggers the other callback to be invoked. 2) When the other callback is
  // invoked, we call MaybeNotify() again to actually invoke the notify_
  // callback. Note that this only happens if the handshake is done and the
  // connector is waiting on the SETTINGS frame.
  void MaybeNotify(grpc_error_handle error);

  Mutex mu_;
  Args args_;
  Result* result_ = nullptr;
  grpc_closure* notify_ = nullptr;
  bool shutdown_ = false;
  grpc_closure on_receive_settings_;
  std::optional<grpc_event_engine::experimental::EventEngine::TaskHandle>
      timer_handle_ ABSL_GUARDED_BY(mu_);
  // A raw pointer will suffice since args_ holds a copy of the ChannelArgs
  // which holds an std::shared_ptr of the EventEngine.
  grpc_event_engine::experimental::EventEngine* event_engine_
      ABSL_GUARDED_BY(mu_);
  std::optional<grpc_error_handle> notify_error_;
  RefCountedPtr<HandshakeManager> handshake_mgr_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_CLIENT_CHTTP2_CONNECTOR_H
