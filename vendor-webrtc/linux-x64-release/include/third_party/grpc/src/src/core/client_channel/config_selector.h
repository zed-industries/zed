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

#ifndef GRPC_SRC_CORE_CLIENT_CHANNEL_CONFIG_SELECTOR_H
#define GRPC_SRC_CORE_CLIENT_CHANNEL_CONFIG_SELECTOR_H

#include <grpc/grpc.h>
#include <grpc/support/port_platform.h>
#include <string.h>

#include <utility>
#include <vector>

#include "absl/log/check.h"
#include "absl/status/status.h"
#include "absl/strings/string_view.h"
#include "src/core/call/interception_chain.h"
#include "src/core/call/metadata_batch.h"
#include "src/core/client_channel/client_channel_internal.h"
#include "src/core/lib/channel/channel_fwd.h"
#include "src/core/lib/resource_quota/arena.h"
#include "src/core/lib/slice/slice.h"
#include "src/core/service_config/service_config.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/unique_type_name.h"
#include "src/core/util/useful.h"

// Channel arg key for ConfigSelector.
#define GRPC_ARG_CONFIG_SELECTOR "grpc.internal.config_selector"

namespace grpc_core {

// Internal API used to allow resolver implementations to override
// MethodConfig and provide input to LB policies on a per-call basis.
class ConfigSelector : public RefCounted<ConfigSelector> {
 public:
  ~ConfigSelector() override = default;

  virtual UniqueTypeName name() const = 0;

  static bool Equals(const ConfigSelector* cs1, const ConfigSelector* cs2) {
    if (cs1 == nullptr) return cs2 == nullptr;
    if (cs2 == nullptr) return false;
    if (cs1->name() != cs2->name()) return false;
    return cs1->Equals(cs2);
  }

  // The channel will call this when the resolver returns a new ConfigSelector
  // to determine what set of dynamic filters will be configured.
  virtual void AddFilters(InterceptionChainBuilder& /*builder*/) {}
  // TODO(roth): Remove this once the legacy filter stack goes away.
  virtual std::vector<const grpc_channel_filter*> GetFilters() { return {}; }

  // Gets the configuration for the call and stores it in service config
  // call data.
  struct GetCallConfigArgs {
    grpc_metadata_batch* initial_metadata;
    Arena* arena;
    ClientChannelServiceConfigCallData* service_config_call_data;
  };
  virtual absl::Status GetCallConfig(GetCallConfigArgs args) = 0;

  static absl::string_view ChannelArgName() { return GRPC_ARG_CONFIG_SELECTOR; }
  static int ChannelArgsCompare(const ConfigSelector* a,
                                const ConfigSelector* b) {
    return QsortCompare(a, b);
  }

 private:
  // Will be called only if the two objects have the same name, so
  // subclasses can be free to safely down-cast the argument.
  virtual bool Equals(const ConfigSelector* other) const = 0;
};

// Default ConfigSelector that gets the MethodConfig from the service config.
class DefaultConfigSelector final : public ConfigSelector {
 public:
  explicit DefaultConfigSelector(RefCountedPtr<ServiceConfig> service_config)
      : service_config_(std::move(service_config)) {
    // The client channel code ensures that this will never be null.
    // If neither the resolver nor the client application provide a
    // config, a default empty config will be used.
    DCHECK(service_config_ != nullptr);
  }

  UniqueTypeName name() const override {
    static UniqueTypeName::Factory kFactory("default");
    return kFactory.Create();
  }

  absl::Status GetCallConfig(GetCallConfigArgs args) override {
    Slice* path = args.initial_metadata->get_pointer(HttpPathMetadata());
    CHECK_NE(path, nullptr);
    auto* parsed_method_configs =
        service_config_->GetMethodParsedConfigVector(path->c_slice());
    args.service_config_call_data->SetServiceConfig(service_config_,
                                                    parsed_method_configs);
    return absl::OkStatus();
  }

  // Only comparing the ConfigSelector itself, not the underlying
  // service config, so we always return true.
  bool Equals(const ConfigSelector* /*other*/) const override { return true; }

 private:
  RefCountedPtr<ServiceConfig> service_config_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CLIENT_CHANNEL_CONFIG_SELECTOR_H
