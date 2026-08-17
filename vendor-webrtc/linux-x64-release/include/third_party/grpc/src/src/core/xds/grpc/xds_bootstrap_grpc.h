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

#ifndef GRPC_SRC_CORE_XDS_GRPC_XDS_BOOTSTRAP_GRPC_H
#define GRPC_SRC_CORE_XDS_GRPC_XDS_BOOTSTRAP_GRPC_H

#include <grpc/support/port_platform.h>

#include <map>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "src/core/util/json/json.h"
#include "src/core/util/json/json_args.h"
#include "src/core/util/json/json_object_loader.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/validation_errors.h"
#include "src/core/xds/grpc/certificate_provider_store.h"
#include "src/core/xds/grpc/xds_audit_logger_registry.h"
#include "src/core/xds/grpc/xds_cluster_specifier_plugin.h"
#include "src/core/xds/grpc/xds_http_filter_registry.h"
#include "src/core/xds/grpc/xds_lb_policy_registry.h"
#include "src/core/xds/grpc/xds_server_grpc.h"
#include "src/core/xds/xds_client/xds_bootstrap.h"

namespace grpc_core {

class GrpcXdsBootstrap final : public XdsBootstrap {
 public:
  class GrpcNode final : public Node {
   public:
    const std::string& id() const override { return id_; }
    const std::string& cluster() const override { return cluster_; }
    const std::string& locality_region() const override {
      return locality_.region;
    }
    const std::string& locality_zone() const override { return locality_.zone; }
    const std::string& locality_sub_zone() const override {
      return locality_.sub_zone;
    }
    const Json::Object& metadata() const override { return metadata_; }

    static const JsonLoaderInterface* JsonLoader(const JsonArgs&);

   private:
    struct Locality {
      std::string region;
      std::string zone;
      std::string sub_zone;

      static const JsonLoaderInterface* JsonLoader(const JsonArgs&);
    };

    std::string id_;
    std::string cluster_;
    Locality locality_;
    Json::Object metadata_;
  };

  class GrpcAuthority final : public Authority {
   public:
    std::vector<const XdsServer*> servers() const override {
      std::vector<const XdsServer*> servers;
      servers.reserve(servers_.size());
      for (const auto& server : servers_) {
        servers.emplace_back(&server);
      }
      return servers;
    }

    const std::string& client_listener_resource_name_template() const {
      return client_listener_resource_name_template_;
    }

    static const JsonLoaderInterface* JsonLoader(const JsonArgs&);

   private:
    std::vector<GrpcXdsServer> servers_;
    std::string client_listener_resource_name_template_;
  };

  // Creates bootstrap object from json_string.
  static absl::StatusOr<std::unique_ptr<GrpcXdsBootstrap>> Create(
      absl::string_view json_string);

  static const JsonLoaderInterface* JsonLoader(const JsonArgs&);
  void JsonPostLoad(const Json& json, const JsonArgs& args,
                    ValidationErrors* errors);

  std::string ToString() const override;

  std::vector<const XdsServer*> servers() const override {
    std::vector<const XdsServer*> servers;
    servers.reserve(servers_.size());
    for (const auto& server : servers_) {
      servers.emplace_back(&server);
    }
    return servers;
  }

  const Node* node() const override {
    return node_.has_value() ? &*node_ : nullptr;
  }
  const Authority* LookupAuthority(const std::string& name) const override;

  const std::string& client_default_listener_resource_name_template() const {
    return client_default_listener_resource_name_template_;
  }
  const std::string& server_listener_resource_name_template() const {
    return server_listener_resource_name_template_;
  }
  const CertificateProviderStore::PluginDefinitionMap& certificate_providers()
      const {
    return certificate_providers_;
  }
  const XdsHttpFilterRegistry& http_filter_registry() const {
    return http_filter_registry_;
  }
  const XdsClusterSpecifierPluginRegistry& cluster_specifier_plugin_registry()
      const {
    return cluster_specifier_plugin_registry_;
  }
  const XdsLbPolicyRegistry& lb_policy_registry() const {
    return lb_policy_registry_;
  }
  const XdsAuditLoggerRegistry& audit_logger_registry() const {
    return audit_logger_registry_;
  }

  // Exposed for testing purposes only.
  const std::map<std::string, GrpcAuthority>& authorities() const {
    return authorities_;
  }

 private:
  std::vector<GrpcXdsServer> servers_;
  std::optional<GrpcNode> node_;
  std::string client_default_listener_resource_name_template_;
  std::string server_listener_resource_name_template_;
  std::map<std::string, GrpcAuthority> authorities_;
  CertificateProviderStore::PluginDefinitionMap certificate_providers_;
  XdsHttpFilterRegistry http_filter_registry_;
  XdsClusterSpecifierPluginRegistry cluster_specifier_plugin_registry_;
  XdsLbPolicyRegistry lb_policy_registry_;
  XdsAuditLoggerRegistry audit_logger_registry_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_XDS_GRPC_XDS_BOOTSTRAP_GRPC_H
