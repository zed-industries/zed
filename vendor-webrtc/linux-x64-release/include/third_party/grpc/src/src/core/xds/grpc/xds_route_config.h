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

#ifndef GRPC_SRC_CORE_XDS_GRPC_XDS_ROUTE_CONFIG_H
#define GRPC_SRC_CORE_XDS_GRPC_XDS_ROUTE_CONFIG_H

#include <stdint.h>

#include <algorithm>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <variant>
#include <vector>

#include "re2/re2.h"
#include "src/core/call/status_util.h"
#include "src/core/util/matchers.h"
#include "src/core/util/time.h"
#include "src/core/xds/grpc/xds_http_filter.h"
#include "src/core/xds/xds_client/xds_resource_type.h"
#include "src/core/xds/xds_client/xds_resource_type_impl.h"

namespace grpc_core {

struct XdsRouteConfigResource : public XdsResourceType::ResourceData {
  using TypedPerFilterConfig =
      std::map<std::string, XdsHttpFilterImpl::FilterConfig>;

  using ClusterSpecifierPluginMap =
      std::map<std::string /*cluster_specifier_plugin_name*/,
               std::string /*LB policy config*/>;

  struct RetryPolicy {
    internal::StatusCodeSet retry_on;
    uint32_t num_retries;

    struct RetryBackOff {
      Duration base_interval;
      Duration max_interval;

      bool operator==(const RetryBackOff& other) const {
        return base_interval == other.base_interval &&
               max_interval == other.max_interval;
      }
      std::string ToString() const;
    };
    RetryBackOff retry_back_off;

    bool operator==(const RetryPolicy& other) const {
      return (retry_on == other.retry_on && num_retries == other.num_retries &&
              retry_back_off == other.retry_back_off);
    }
    std::string ToString() const;
  };

  struct Route {
    // Matchers for this route.
    struct Matchers {
      StringMatcher path_matcher;
      std::vector<HeaderMatcher> header_matchers;
      std::optional<uint32_t> fraction_per_million;

      bool operator==(const Matchers& other) const {
        return path_matcher == other.path_matcher &&
               header_matchers == other.header_matchers &&
               fraction_per_million == other.fraction_per_million;
      }
      std::string ToString() const;
    };

    Matchers matchers;

    struct UnknownAction {
      bool operator==(const UnknownAction& /* other */) const { return true; }
    };

    struct RouteAction {
      struct HashPolicy {
        struct Header {
          std::string header_name;
          std::unique_ptr<RE2> regex;
          std::string regex_substitution;

          Header() = default;

          // Copyable.
          Header(const Header& other);
          Header& operator=(const Header& other);

          // Movable.
          Header(Header&& other) noexcept;
          Header& operator=(Header&& other) noexcept;

          bool operator==(const Header& other) const;
          std::string ToString() const;
        };

        struct ChannelId {
          bool operator==(const ChannelId&) const { return true; }
        };

        std::variant<Header, ChannelId> policy;
        bool terminal = false;

        bool operator==(const HashPolicy& other) const {
          return policy == other.policy && terminal == other.terminal;
        }
        std::string ToString() const;
      };

      struct ClusterName {
        std::string cluster_name;

        bool operator==(const ClusterName& other) const {
          return cluster_name == other.cluster_name;
        }
      };

      struct ClusterWeight {
        std::string name;
        uint32_t weight;
        TypedPerFilterConfig typed_per_filter_config;

        bool operator==(const ClusterWeight& other) const {
          return name == other.name && weight == other.weight &&
                 typed_per_filter_config == other.typed_per_filter_config;
        }
        std::string ToString() const;
      };

      struct ClusterSpecifierPluginName {
        std::string cluster_specifier_plugin_name;

        bool operator==(const ClusterSpecifierPluginName& other) const {
          return cluster_specifier_plugin_name ==
                 other.cluster_specifier_plugin_name;
        }
      };

      std::vector<HashPolicy> hash_policies;
      std::optional<RetryPolicy> retry_policy;

      // Action for this route.
      std::variant<ClusterName, std::vector<ClusterWeight>,
                   ClusterSpecifierPluginName>
          action;

      // Storing the timeout duration from route action:
      // RouteAction.max_stream_duration.grpc_timeout_header_max or
      // RouteAction.max_stream_duration.max_stream_duration if the former is
      // not set.
      std::optional<Duration> max_stream_duration;

      bool auto_host_rewrite = false;

      bool operator==(const RouteAction& other) const {
        return hash_policies == other.hash_policies &&
               retry_policy == other.retry_policy && action == other.action &&
               max_stream_duration == other.max_stream_duration &&
               auto_host_rewrite == other.auto_host_rewrite;
      }
      std::string ToString() const;
    };

    struct NonForwardingAction {
      bool operator==(const NonForwardingAction& /* other */) const {
        return true;
      }
    };

    std::variant<UnknownAction, RouteAction, NonForwardingAction> action;
    TypedPerFilterConfig typed_per_filter_config;

    bool operator==(const Route& other) const {
      return matchers == other.matchers && action == other.action &&
             typed_per_filter_config == other.typed_per_filter_config;
    }
    std::string ToString() const;
  };

  struct VirtualHost {
    std::vector<std::string> domains;
    std::vector<Route> routes;
    TypedPerFilterConfig typed_per_filter_config;

    bool operator==(const VirtualHost& other) const {
      return domains == other.domains && routes == other.routes &&
             typed_per_filter_config == other.typed_per_filter_config;
    }
    std::string ToString() const;
  };

  std::vector<VirtualHost> virtual_hosts;
  ClusterSpecifierPluginMap cluster_specifier_plugin_map;

  bool operator==(const XdsRouteConfigResource& other) const {
    return virtual_hosts == other.virtual_hosts &&
           cluster_specifier_plugin_map == other.cluster_specifier_plugin_map;
  }
  std::string ToString() const;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_XDS_GRPC_XDS_ROUTE_CONFIG_H
