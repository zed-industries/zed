//
// Copyright 2019 gRPC authors.
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

#ifndef GRPC_SRC_CORE_XDS_GRPC_XDS_SERVER_GRPC_H
#define GRPC_SRC_CORE_XDS_GRPC_XDS_SERVER_GRPC_H

#include <set>
#include <string>

#include "src/core/credentials/transport/channel_creds_registry.h"
#include "src/core/util/json/json.h"
#include "src/core/util/json/json_args.h"
#include "src/core/util/json/json_object_loader.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/validation_errors.h"
#include "src/core/xds/grpc/xds_server_grpc_interface.h"
#include "src/core/xds/xds_client/xds_bootstrap.h"

namespace grpc_core {

class GrpcXdsServerTarget final : public GrpcXdsServerInterface {
 public:
  explicit GrpcXdsServerTarget(
      std::string server_uri,
      RefCountedPtr<ChannelCredsConfig> channel_creds_config)
      : server_uri_(std::move(server_uri)),
        channel_creds_config_(std::move(channel_creds_config)) {}

  bool Equals(const XdsServerTarget& other) const override;
  std::string Key() const override;
  const std::string& server_uri() const override { return server_uri_; }
  RefCountedPtr<ChannelCredsConfig> channel_creds_config() const override {
    return channel_creds_config_;
  }

 private:
  std::string server_uri_;
  RefCountedPtr<ChannelCredsConfig> channel_creds_config_;
};

class GrpcXdsServer final : public XdsBootstrap::XdsServer {
 public:
  bool IgnoreResourceDeletion() const override;
  bool FailOnDataErrors() const override;
  bool ResourceTimerIsTransientFailure() const override;
  bool TrustedXdsServer() const;
  bool Equals(const XdsServer& other) const override;
  std::string Key() const override;
  std::shared_ptr<const XdsBootstrap::XdsServerTarget> target() const override {
    return server_target_;
  }
  static const JsonLoaderInterface* JsonLoader(const JsonArgs&);
  void JsonPostLoad(const Json& json, const JsonArgs& args,
                    ValidationErrors* errors);

 private:
  std::shared_ptr<GrpcXdsServerTarget> server_target_;
  std::set<std::string> server_features_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_XDS_GRPC_XDS_SERVER_GRPC_H
