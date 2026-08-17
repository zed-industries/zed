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

#ifndef GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_MATCHERS_H
#define GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_MATCHERS_H

#include <grpc/support/port_platform.h>
#include <stdint.h>

#include <memory>
#include <optional>
#include <utility>
#include <vector>

#include "src/core/lib/iomgr/resolved_address.h"
#include "src/core/lib/security/authorization/evaluate_args.h"
#include "src/core/lib/security/authorization/rbac_policy.h"
#include "src/core/util/matchers.h"

namespace grpc_core {

// Describes the rules for matching permission or principal.
class AuthorizationMatcher {
 public:
  virtual ~AuthorizationMatcher() = default;

  // Returns whether or not the permission/principal matches the rules of the
  // matcher.
  virtual bool Matches(const EvaluateArgs& args) const = 0;

  // Creates an instance of a matcher based off the rules defined in Permission
  // config.
  static std::unique_ptr<AuthorizationMatcher> Create(
      Rbac::Permission permission);

  // Creates an instance of a matcher based off the rules defined in Principal
  // config.
  static std::unique_ptr<AuthorizationMatcher> Create(
      Rbac::Principal principal);
};

class AlwaysAuthorizationMatcher : public AuthorizationMatcher {
 public:
  explicit AlwaysAuthorizationMatcher() = default;

  bool Matches(const EvaluateArgs&) const override { return true; }
};

class AndAuthorizationMatcher : public AuthorizationMatcher {
 public:
  explicit AndAuthorizationMatcher(
      std::vector<std::unique_ptr<AuthorizationMatcher>> matchers)
      : matchers_(std::move(matchers)) {}

  bool Matches(const EvaluateArgs& args) const override;

 private:
  std::vector<std::unique_ptr<AuthorizationMatcher>> matchers_;
};

class OrAuthorizationMatcher : public AuthorizationMatcher {
 public:
  explicit OrAuthorizationMatcher(
      std::vector<std::unique_ptr<AuthorizationMatcher>> matchers)
      : matchers_(std::move(matchers)) {}

  bool Matches(const EvaluateArgs& args) const override;

 private:
  std::vector<std::unique_ptr<AuthorizationMatcher>> matchers_;
};

// Negates matching the provided permission/principal.
class NotAuthorizationMatcher : public AuthorizationMatcher {
 public:
  explicit NotAuthorizationMatcher(
      std::unique_ptr<AuthorizationMatcher> matcher)
      : matcher_(std::move(matcher)) {}

  bool Matches(const EvaluateArgs& args) const override;

 private:
  std::unique_ptr<AuthorizationMatcher> matcher_;
};

class MetadataAuthorizationMatcher : public AuthorizationMatcher {
 public:
  explicit MetadataAuthorizationMatcher(bool invert) : invert_(invert) {}

  // In RBAC, metadata refers to the Envoy metadata which has no relation to
  // gRPC metadata. Envoy metadata is a generic state shared between filters,
  // which has no gRPC equivalent. RBAC implementations in gRPC will treat Envoy
  // metadata as an empty map. Since ValueMatcher can only match if a value is
  // present (even NullMatch), the metadata matcher will not match unless invert
  // is set to true.
  bool Matches(const EvaluateArgs&) const override { return invert_; }

 private:
  const bool invert_;
};

// Perform a match against HTTP headers.
class HeaderAuthorizationMatcher : public AuthorizationMatcher {
 public:
  explicit HeaderAuthorizationMatcher(HeaderMatcher matcher)
      : matcher_(std::move(matcher)) {}

  bool Matches(const EvaluateArgs& args) const override;

 private:
  const HeaderMatcher matcher_;
};

// Perform a match against IP Cidr Range.
class IpAuthorizationMatcher : public AuthorizationMatcher {
 public:
  enum class Type {
    kDestIp,
    kSourceIp,
    kDirectRemoteIp,
    kRemoteIp,
  };

  IpAuthorizationMatcher(Type type, Rbac::CidrRange range);

  bool Matches(const EvaluateArgs& args) const override;

 private:
  const Type type_;
  // Subnet masked address.
  grpc_resolved_address subnet_address_;
  const uint32_t prefix_len_;
};

// Perform a match against port number of the destination (local) address.
class PortAuthorizationMatcher : public AuthorizationMatcher {
 public:
  explicit PortAuthorizationMatcher(int port) : port_(port) {}

  bool Matches(const EvaluateArgs& args) const override;

 private:
  const int port_;
};

// Matches the principal name as described in the peer certificate. Uses URI SAN
// or DNS SAN in that order, otherwise uses subject field.
class AuthenticatedAuthorizationMatcher : public AuthorizationMatcher {
 public:
  explicit AuthenticatedAuthorizationMatcher(std::optional<StringMatcher> auth)
      : matcher_(std::move(auth)) {}

  bool Matches(const EvaluateArgs& args) const override;

 private:
  const std::optional<StringMatcher> matcher_;
};

// Perform a match against the request server from the client's connection
// request. This is typically TLS SNI. Currently unsupported.
class ReqServerNameAuthorizationMatcher : public AuthorizationMatcher {
 public:
  explicit ReqServerNameAuthorizationMatcher(
      StringMatcher requested_server_name)
      : matcher_(std::move(requested_server_name)) {}

  bool Matches(const EvaluateArgs&) const override;

 private:
  const StringMatcher matcher_;
};

// Perform a match against the path header of HTTP request.
class PathAuthorizationMatcher : public AuthorizationMatcher {
 public:
  explicit PathAuthorizationMatcher(StringMatcher path)
      : matcher_(std::move(path)) {}

  bool Matches(const EvaluateArgs& args) const override;

 private:
  const StringMatcher matcher_;
};

// Performs a match for policy field in RBAC, which is a collection of
// permission and principal matchers. Policy matches iff, we find a match in one
// of its permissions and a match in one of its principals.
class PolicyAuthorizationMatcher : public AuthorizationMatcher {
 public:
  explicit PolicyAuthorizationMatcher(Rbac::Policy policy)
      : permissions_(
            AuthorizationMatcher::Create(std::move(policy.permissions))),
        principals_(
            AuthorizationMatcher::Create(std::move(policy.principals))) {}

  bool Matches(const EvaluateArgs& args) const override;

 private:
  std::unique_ptr<AuthorizationMatcher> permissions_;
  std::unique_ptr<AuthorizationMatcher> principals_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_MATCHERS_H
