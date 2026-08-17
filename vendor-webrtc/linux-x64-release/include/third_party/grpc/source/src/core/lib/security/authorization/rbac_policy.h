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

#ifndef GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_RBAC_POLICY_H
#define GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_RBAC_POLICY_H

#include <grpc/grpc_audit_logging.h>
#include <grpc/support/port_platform.h>
#include <stdint.h>

#include <map>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "src/core/util/matchers.h"

namespace grpc_core {

// Represents Envoy RBAC Proto. [See
// https://github.com/envoyproxy/envoy/blob/release/v1.26/api/envoy/config/rbac/v3/rbac.proto]
struct Rbac {
  enum class Action {
    kAllow,
    kDeny,
  };

  enum class AuditCondition {
    kNone,
    kOnDeny,
    kOnAllow,
    kOnDenyAndAllow,
  };

  struct CidrRange {
    CidrRange() = default;
    CidrRange(std::string address_prefix, uint32_t prefix_len);

    CidrRange(CidrRange&& other) noexcept;
    CidrRange& operator=(CidrRange&& other) noexcept;

    std::string ToString() const;

    std::string address_prefix;
    uint32_t prefix_len;
  };

  // TODO(ashithasantosh): Support for destination_port_range.
  struct Permission {
    enum class RuleType {
      kAnd,
      kOr,
      kNot,
      kAny,
      kHeader,
      kPath,
      kDestIp,
      kDestPort,
      kMetadata,
      kReqServerName,
    };

    static Permission MakeAndPermission(
        std::vector<std::unique_ptr<Permission>> permissions);
    static Permission MakeOrPermission(
        std::vector<std::unique_ptr<Permission>> permissions);
    static Permission MakeNotPermission(Permission permission);
    static Permission MakeAnyPermission();
    static Permission MakeHeaderPermission(HeaderMatcher header_matcher);
    static Permission MakePathPermission(StringMatcher string_matcher);
    static Permission MakeDestIpPermission(CidrRange ip);
    static Permission MakeDestPortPermission(int port);
    // All the other fields in MetadataMatcher are ignored except invert.
    static Permission MakeMetadataPermission(bool invert);
    static Permission MakeReqServerNamePermission(StringMatcher string_matcher);

    Permission() = default;

    Permission(Permission&& other) noexcept;
    Permission& operator=(Permission&& other) noexcept;

    std::string ToString() const;

    RuleType type = RuleType::kAnd;
    HeaderMatcher header_matcher;
    StringMatcher string_matcher;
    CidrRange ip;
    int port;
    // For type kAnd/kOr/kNot. For kNot type, the vector will have only one
    // element.
    std::vector<std::unique_ptr<Permission>> permissions;
    // For kMetadata
    bool invert = false;
  };

  struct Principal {
    enum class RuleType {
      kAnd,
      kOr,
      kNot,
      kAny,
      kPrincipalName,
      kSourceIp,
      kDirectRemoteIp,
      kRemoteIp,
      kHeader,
      kPath,
      kMetadata,
    };

    static Principal MakeAndPrincipal(
        std::vector<std::unique_ptr<Principal>> principals);
    static Principal MakeOrPrincipal(
        std::vector<std::unique_ptr<Principal>> principals);
    static Principal MakeNotPrincipal(Principal principal);
    static Principal MakeAnyPrincipal();
    static Principal MakeAuthenticatedPrincipal(
        std::optional<StringMatcher> string_matcher);
    static Principal MakeSourceIpPrincipal(CidrRange ip);
    static Principal MakeDirectRemoteIpPrincipal(CidrRange ip);
    static Principal MakeRemoteIpPrincipal(CidrRange ip);
    static Principal MakeHeaderPrincipal(HeaderMatcher header_matcher);
    static Principal MakePathPrincipal(StringMatcher string_matcher);
    // All the other fields in MetadataMatcher are ignored except invert.
    static Principal MakeMetadataPrincipal(bool invert);

    Principal() = default;

    Principal(Principal&& other) noexcept;
    Principal& operator=(Principal&& other) noexcept;

    std::string ToString() const;

    RuleType type = RuleType::kAnd;
    HeaderMatcher header_matcher;
    std::optional<StringMatcher> string_matcher;
    CidrRange ip;
    // For type kAnd/kOr/kNot. For kNot type, the vector will have only one
    // element.
    std::vector<std::unique_ptr<Principal>> principals;
    // For kMetadata
    bool invert = false;
  };

  struct Policy {
    Policy() = default;
    Policy(Permission permissions, Principal principals);

    Policy(Policy&& other) noexcept;
    Policy& operator=(Policy&& other) noexcept;

    std::string ToString() const;

    Permission permissions;
    Principal principals;
  };

  Rbac() = default;
  Rbac(std::string name, Rbac::Action action,
       std::map<std::string, Policy> policies);

  Rbac(Rbac&& other) noexcept;
  Rbac& operator=(Rbac&& other) noexcept;

  std::string ToString() const;

  // The authorization policy name or empty string in xDS case.
  std::string name;

  Action action;
  std::map<std::string, Policy> policies;

  AuditCondition audit_condition;
  std::vector<std::unique_ptr<experimental::AuditLoggerFactory::Config>>
      logger_configs;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_RBAC_POLICY_H
