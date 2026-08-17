//
// Copyright 2022 gRPC authors.
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

#ifndef GRPC_SRC_CORE_XDS_GRPC_XDS_TRANSPORT_GRPC_H
#define GRPC_SRC_CORE_XDS_GRPC_XDS_TRANSPORT_GRPC_H

#include <grpc/grpc.h>
#include <grpc/slice.h>
#include <grpc/status.h>
#include <grpc/support/port_platform.h>

#include <functional>
#include <memory>
#include <string>

#include "absl/container/flat_hash_map.h"
#include "absl/status/status.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/iomgr/closure.h"
#include "src/core/lib/iomgr/error.h"
#include "src/core/lib/iomgr/iomgr_fwd.h"
#include "src/core/lib/surface/channel.h"
#include "src/core/util/orphanable.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/sync.h"
#include "src/core/xds/xds_client/xds_bootstrap.h"
#include "src/core/xds/xds_client/xds_transport.h"

namespace grpc_core {

class GrpcXdsTransportFactory final : public XdsTransportFactory {
 public:
  class GrpcXdsTransport;

  explicit GrpcXdsTransportFactory(const ChannelArgs& args);
  ~GrpcXdsTransportFactory() override;

  void Orphaned() override {}

  RefCountedPtr<XdsTransport> GetTransport(
      const XdsBootstrap::XdsServerTarget& server,
      absl::Status* status) override;

  grpc_pollset_set* interested_parties() const { return interested_parties_; }

 private:
  ChannelArgs args_;
  grpc_pollset_set* interested_parties_;

  Mutex mu_;
  absl::flat_hash_map<std::string /*XdsServerTarget key*/, GrpcXdsTransport*>
      transports_ ABSL_GUARDED_BY(&mu_);
};

class GrpcXdsTransportFactory::GrpcXdsTransport final
    : public XdsTransportFactory::XdsTransport {
 public:
  class GrpcStreamingCall;

  GrpcXdsTransport(WeakRefCountedPtr<GrpcXdsTransportFactory> factory,
                   const XdsBootstrap::XdsServerTarget& server,
                   absl::Status* status);
  ~GrpcXdsTransport() override;

  void Orphaned() override;

  void StartConnectivityFailureWatch(
      RefCountedPtr<ConnectivityFailureWatcher> watcher) override;
  void StopConnectivityFailureWatch(
      const RefCountedPtr<ConnectivityFailureWatcher>& watcher) override;

  OrphanablePtr<StreamingCall> CreateStreamingCall(
      const char* method,
      std::unique_ptr<StreamingCall::EventHandler> event_handler) override;

  void ResetBackoff() override;

 private:
  class StateWatcher;

  WeakRefCountedPtr<GrpcXdsTransportFactory> factory_;
  std::string key_;
  RefCountedPtr<Channel> channel_;

  Mutex mu_;
  absl::flat_hash_map<RefCountedPtr<ConnectivityFailureWatcher>, StateWatcher*>
      watchers_ ABSL_GUARDED_BY(&mu_);
};

class GrpcXdsTransportFactory::GrpcXdsTransport::GrpcStreamingCall final
    : public XdsTransportFactory::XdsTransport::StreamingCall {
 public:
  GrpcStreamingCall(WeakRefCountedPtr<GrpcXdsTransportFactory> factory,
                    Channel* channel, const char* method,
                    std::unique_ptr<StreamingCall::EventHandler> event_handler);
  ~GrpcStreamingCall() override;

  void Orphan() override;

  void SendMessage(std::string payload) override;

  void StartRecvMessage() override;

 private:
  static void OnRecvInitialMetadata(void* arg, grpc_error_handle /*error*/);
  static void OnRequestSent(void* arg, grpc_error_handle error);
  static void OnResponseReceived(void* arg, grpc_error_handle /*error*/);
  static void OnStatusReceived(void* arg, grpc_error_handle /*error*/);

  WeakRefCountedPtr<GrpcXdsTransportFactory> factory_;

  std::unique_ptr<StreamingCall::EventHandler> event_handler_;

  // Always non-NULL.
  grpc_call* call_;

  // recv_initial_metadata
  grpc_metadata_array initial_metadata_recv_;
  grpc_closure on_recv_initial_metadata_;

  // send_message
  grpc_byte_buffer* send_message_payload_ = nullptr;
  grpc_closure on_request_sent_;

  // recv_message
  grpc_byte_buffer* recv_message_payload_ = nullptr;
  grpc_closure on_response_received_;

  // recv_trailing_metadata
  grpc_metadata_array trailing_metadata_recv_;
  grpc_status_code status_code_;
  grpc_slice status_details_;
  grpc_closure on_status_received_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_XDS_GRPC_XDS_TRANSPORT_GRPC_H
