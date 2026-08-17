//
// Copyright 2018 gRPC authors.
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

#ifndef GRPC_SRC_CORE_XDS_GRPC_XDS_ENDPOINT_H
#define GRPC_SRC_CORE_XDS_GRPC_XDS_ENDPOINT_H

#include <map>
#include <string>
#include <utility>
#include <vector>

#include "absl/base/thread_annotations.h"
#include "absl/random/random.h"
#include "src/core/resolver/endpoint_addresses.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/sync.h"
#include "src/core/xds/xds_client/xds_locality.h"
#include "src/core/xds/xds_client/xds_resource_type.h"
#include "src/core/xds/xds_client/xds_resource_type_impl.h"

// Per-endpoint channel arg key for xDS-configured HTTP CONNECT proxy.
#define GRPC_ARG_XDS_HTTP_PROXY "grpc.internal.xds_http_proxy"

namespace grpc_core {

struct XdsEndpointResource : public XdsResourceType::ResourceData {
  struct Priority {
    struct Locality {
      RefCountedPtr<XdsLocalityName> name;
      uint32_t lb_weight;
      EndpointAddressesList endpoints;

      bool operator==(const Locality& other) const {
        return *name == *other.name && lb_weight == other.lb_weight &&
               endpoints == other.endpoints;
      }
      bool operator!=(const Locality& other) const { return !(*this == other); }
      std::string ToString() const;
    };

    std::map<XdsLocalityName*, Locality, XdsLocalityName::Less> localities;

    bool operator==(const Priority& other) const;
    bool operator!=(const Priority& other) const { return !(*this == other); }
    std::string ToString() const;
  };
  using PriorityList = std::vector<Priority>;

  // There are two phases of accessing this class's content:
  // 1. to initialize in the control plane combiner;
  // 2. to use in the data plane combiner.
  // So no additional synchronization is needed.
  class DropConfig final : public RefCounted<DropConfig> {
   public:
    struct DropCategory {
      bool operator==(const DropCategory& other) const {
        return name == other.name &&
               parts_per_million == other.parts_per_million;
      }

      std::string name;
      const uint32_t parts_per_million;
    };

    using DropCategoryList = std::vector<DropCategory>;

    void AddCategory(std::string name, uint32_t parts_per_million) {
      drop_category_list_.emplace_back(
          DropCategory{std::move(name), parts_per_million});
      if (parts_per_million == 1000000) drop_all_ = true;
    }

    // The only method invoked from outside the WorkSerializer (used in
    // the data plane).
    bool ShouldDrop(const std::string** category_name);

    const DropCategoryList& drop_category_list() const {
      return drop_category_list_;
    }

    bool drop_all() const { return drop_all_; }

    bool operator==(const DropConfig& other) const {
      return drop_category_list_ == other.drop_category_list_;
    }
    bool operator!=(const DropConfig& other) const { return !(*this == other); }

    std::string ToString() const;

   private:
    DropCategoryList drop_category_list_;
    bool drop_all_ = false;

    // TODO(roth): Consider using a separate thread-local BitGen for each CPU
    // to avoid the need for this mutex.
    Mutex mu_;
    absl::BitGen bit_gen_ ABSL_GUARDED_BY(&mu_);
  };

  PriorityList priorities;
  RefCountedPtr<DropConfig> drop_config;

  bool operator==(const XdsEndpointResource& other) const {
    if (priorities != other.priorities) return false;
    if (drop_config == nullptr) return other.drop_config == nullptr;
    if (other.drop_config == nullptr) return false;
    return *drop_config == *other.drop_config;
  }
  std::string ToString() const;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_XDS_GRPC_XDS_ENDPOINT_H
