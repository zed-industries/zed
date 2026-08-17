//
// Copyright 2021 gRPC authors.
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

#ifndef GRPC_SRC_CORE_SERVER_SERVER_CONFIG_SELECTOR_H
#define GRPC_SRC_CORE_SERVER_SERVER_CONFIG_SELECTOR_H

#include <grpc/support/port_platform.h>

#include <memory>

#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "src/core/lib/transport/metadata_batch.h"
#include "src/core/service_config/service_config.h"
#include "src/core/service_config/service_config_parser.h"
#include "src/core/util/dual_ref_counted.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/useful.h"

namespace grpc_core {

// ServerConfigSelector allows for choosing the service config to apply to a
// server-side call based on the received initial metadata.
class ServerConfigSelector : public RefCounted<ServerConfigSelector> {
 public:
  // Configuration to apply to an incoming call
  struct CallConfig {
    const ServiceConfigParser::ParsedConfigVector* method_configs = nullptr;
    RefCountedPtr<ServiceConfig> service_config;
  };

  ~ServerConfigSelector() override = default;

  // Returns the CallConfig to apply to a call based on the incoming \a metadata
  virtual absl::StatusOr<CallConfig> GetCallConfig(
      grpc_metadata_batch* metadata) = 0;
};

// ServerConfigSelectorProvider allows for subscribers to watch for updates on
// ServerConfigSelector. It is propagated via channel args.
class ServerConfigSelectorProvider
    : public DualRefCounted<ServerConfigSelectorProvider> {
 public:
  class ServerConfigSelectorWatcher {
   public:
    virtual ~ServerConfigSelectorWatcher() = default;
    virtual void OnServerConfigSelectorUpdate(
        absl::StatusOr<RefCountedPtr<ServerConfigSelector>> update) = 0;
  };

  ~ServerConfigSelectorProvider() override = default;
  // Only a single watcher is allowed at present
  virtual absl::StatusOr<RefCountedPtr<ServerConfigSelector>> Watch(
      std::unique_ptr<ServerConfigSelectorWatcher> watcher) = 0;
  virtual void CancelWatch() = 0;

  static absl::string_view ChannelArgName() {
    return "grpc.internal.server_config_selector_provider";
  }
  static int ChannelArgsCompare(const ServerConfigSelectorProvider* a,
                                const ServerConfigSelectorProvider* b) {
    return QsortCompare(a, b);
  }
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_SERVER_SERVER_CONFIG_SELECTOR_H
