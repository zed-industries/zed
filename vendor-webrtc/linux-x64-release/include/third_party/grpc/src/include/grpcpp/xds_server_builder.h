//
//
// Copyright 2020 gRPC authors.
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

#ifndef GRPCPP_XDS_SERVER_BUILDER_H
#define GRPCPP_XDS_SERVER_BUILDER_H

#include <grpc/support/port_platform.h>
#include <grpcpp/server_builder.h>

namespace grpc {

class XdsServerServingStatusNotifierInterface {
 public:
  struct ServingStatusUpdate {
    grpc::Status status;
  };

  virtual ~XdsServerServingStatusNotifierInterface() = default;

  // \a uri contains the listening target associated with the notification. Note
  // that a single target provided to XdsServerBuilder can get resolved to
  // multiple listening addresses.
  // The callback is invoked each time there is an update to the serving status.
  // The API does not provide any guarantees around duplicate updates.
  // Status::OK signifies that the server is serving, while a non-OK status
  // signifies that the server is not serving.
  virtual void OnServingStatusUpdate(std::string uri,
                                     ServingStatusUpdate update) = 0;
};

class XdsServerBuilder : public grpc::ServerBuilder {
 public:
  // NOTE: class experimental_type is not part of the public API of this class
  // TODO(yashykt): Integrate into public API when this is no longer
  // experimental.
  class experimental_type : public grpc::ServerBuilder::experimental_type {
   public:
    explicit experimental_type(XdsServerBuilder* builder)
        : ServerBuilder::experimental_type(builder), builder_(builder) {}

    // EXPERIMENTAL: Sets the drain grace period in ms for older connections
    // when updates to a Listener is received.
    void set_drain_grace_time(int drain_grace_time_ms) {
      builder_->drain_grace_time_ms_ = drain_grace_time_ms;
    }

   private:
    XdsServerBuilder* builder_;
  };

  // It is the responsibility of the application to make sure that \a notifier
  // outlasts the life of the server. Notifications will start being made
  // asynchronously once `BuildAndStart()` has been called. Note that it is
  // possible for notifications to be made before `BuildAndStart()` returns.
  void set_status_notifier(XdsServerServingStatusNotifierInterface* notifier) {
    notifier_ = notifier;
  }

  /// NOTE: The function experimental() is not stable public API. It is a view
  /// to the experimental components of this class. It may be changed or removed
  /// at any time.
  experimental_type experimental() { return experimental_type(this); }

 private:
  // Called at the beginning of BuildAndStart().
  ChannelArguments BuildChannelArgs() override;

  static void OnServingStatusUpdate(void* user_data, const char* uri,
                                    grpc_serving_status_update update) {
    if (user_data == nullptr) return;
    XdsServerServingStatusNotifierInterface* notifier =
        static_cast<XdsServerServingStatusNotifierInterface*>(user_data);
    notifier->OnServingStatusUpdate(
        uri, {grpc::Status(static_cast<StatusCode>(update.code),
                           update.error_message)});
  }

  XdsServerServingStatusNotifierInterface* notifier_ = nullptr;
  int drain_grace_time_ms_ = -1;
};

}  // namespace grpc

#endif  // GRPCPP_XDS_SERVER_BUILDER_H
