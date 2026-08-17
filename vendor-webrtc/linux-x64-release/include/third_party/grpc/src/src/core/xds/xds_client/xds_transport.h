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

#ifndef GRPC_SRC_CORE_XDS_XDS_CLIENT_XDS_TRANSPORT_H
#define GRPC_SRC_CORE_XDS_XDS_CLIENT_XDS_TRANSPORT_H

#include <memory>
#include <string>

#include "absl/status/status.h"
#include "absl/strings/string_view.h"
#include "src/core/util/dual_ref_counted.h"
#include "src/core/util/orphanable.h"
#include "src/core/xds/xds_client/xds_bootstrap.h"

namespace grpc_core {

// A factory for creating new XdsTransport instances.
class XdsTransportFactory : public DualRefCounted<XdsTransportFactory> {
 public:
  // Represents a transport for xDS communication (e.g., a gRPC channel).
  class XdsTransport : public DualRefCounted<XdsTransport> {
   public:
    // Represents a bidi streaming RPC call.
    class StreamingCall : public InternallyRefCounted<StreamingCall> {
     public:
      // An interface for handling events on a streaming call.
      class EventHandler {
       public:
        virtual ~EventHandler() = default;

        // Called when a SendMessage() operation completes.
        virtual void OnRequestSent(bool ok) = 0;
        // Called when a message is received on the stream.
        virtual void OnRecvMessage(absl::string_view payload) = 0;
        // Called when status is received on the stream.
        virtual void OnStatusReceived(absl::Status status) = 0;
      };

      // Sends a message on the stream.  When the message has been sent,
      // the EventHandler::OnRequestSent() method will be called.
      // Only one message will be in flight at a time; subsequent
      // messages will not be sent until this one is done.
      virtual void SendMessage(std::string payload) = 0;

      // Starts a recv_message operation on the stream.
      virtual void StartRecvMessage() = 0;
    };

    // A watcher for connectivity failures.
    class ConnectivityFailureWatcher
        : public RefCounted<ConnectivityFailureWatcher> {
     public:
      // Will be invoked whenever there is a connectivity failure on the
      // transport.
      virtual void OnConnectivityFailure(absl::Status status) = 0;
    };

    explicit XdsTransport(const char* trace = nullptr)
        : DualRefCounted(trace) {}

    // Starts a connectivity failure watcher on the transport.
    virtual void StartConnectivityFailureWatch(
        RefCountedPtr<ConnectivityFailureWatcher> watcher) = 0;
    // Stops a connectivity failure watcher on the transport.
    virtual void StopConnectivityFailureWatch(
        const RefCountedPtr<ConnectivityFailureWatcher>& watcher) = 0;

    // Create a streaming call on this transport for the specified method.
    // Events on the stream will be reported to event_handler.
    virtual OrphanablePtr<StreamingCall> CreateStreamingCall(
        const char* method,
        std::unique_ptr<StreamingCall::EventHandler> event_handler) = 0;

    // Resets connection backoff for the transport.
    virtual void ResetBackoff() = 0;
  };

  // Returns a transport for the specified server.  If there is already
  // a transport for the server, returns a new ref to that transport;
  // otherwise, creates a new transport.
  //
  // *status will be set if there is an error creating the channel,
  // although the returned channel must still accept calls (which may fail).
  virtual RefCountedPtr<XdsTransport> GetTransport(
      const XdsBootstrap::XdsServerTarget& server, absl::Status* status) = 0;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_XDS_XDS_CLIENT_XDS_TRANSPORT_H
