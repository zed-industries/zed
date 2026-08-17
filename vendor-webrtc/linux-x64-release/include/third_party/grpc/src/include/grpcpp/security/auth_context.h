//
//
// Copyright 2015 gRPC authors.
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

#ifndef GRPCPP_SECURITY_AUTH_CONTEXT_H
#define GRPCPP_SECURITY_AUTH_CONTEXT_H

#include <grpcpp/support/config.h>
#include <grpcpp/support/string_ref.h>

#include <iterator>
#include <vector>

struct grpc_auth_context;
struct grpc_auth_property;
struct grpc_auth_property_iterator;

namespace grpc {
class SecureAuthContext;

typedef std::pair<string_ref, string_ref> AuthProperty;

class AuthPropertyIterator {
 public:
  using iterator_category = std::forward_iterator_tag;
  using value_type = const AuthProperty;
  using pointer = void;
  using reference = void;
  using difference_type = std::ptrdiff_t;

  ~AuthPropertyIterator();
  AuthPropertyIterator& operator++();
  AuthPropertyIterator operator++(int);
  bool operator==(const AuthPropertyIterator& rhs) const;
  bool operator!=(const AuthPropertyIterator& rhs) const;
  AuthProperty operator*();

 protected:
  AuthPropertyIterator();
  AuthPropertyIterator(const grpc_auth_property* property,
                       const grpc_auth_property_iterator* iter);

 private:
  friend class SecureAuthContext;
  const grpc_auth_property* property_;
  // The following items form a grpc_auth_property_iterator.
  const grpc_auth_context* ctx_;
  size_t index_;
  const char* name_;
};

/// Class encapsulating the Authentication Information.
///
/// It includes the secure identity of the peer, the type of secure transport
/// used as well as any other properties required by the authorization layer.
class AuthContext {
 public:
  virtual ~AuthContext() {}

  /// Returns true if the peer is authenticated.
  virtual bool IsPeerAuthenticated() const = 0;

  /// A peer identity.
  ///
  /// It is, in general, comprised of one or more properties (in which case they
  /// have the same name).
  virtual std::vector<grpc::string_ref> GetPeerIdentity() const = 0;
  virtual std::string GetPeerIdentityPropertyName() const = 0;

  /// Returns all the property values with the given name.
  virtual std::vector<grpc::string_ref> FindPropertyValues(
      const std::string& name) const = 0;

  /// Iteration over all the properties.
  virtual AuthPropertyIterator begin() const = 0;
  virtual AuthPropertyIterator end() const = 0;

  /// Mutation functions: should only be used by an AuthMetadataProcessor.
  virtual void AddProperty(const std::string& key, const string_ref& value) = 0;
  virtual bool SetPeerIdentityPropertyName(const std::string& name) = 0;
};

}  // namespace grpc

#endif  // GRPCPP_SECURITY_AUTH_CONTEXT_H
