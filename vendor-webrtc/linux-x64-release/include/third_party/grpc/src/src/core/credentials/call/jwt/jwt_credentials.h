//
//
// Copyright 2016 gRPC authors.
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

#ifndef GRPC_SRC_CORE_CREDENTIALS_CALL_JWT_JWT_CREDENTIALS_H
#define GRPC_SRC_CORE_CREDENTIALS_CALL_JWT_JWT_CREDENTIALS_H

#include <grpc/credentials.h>
#include <grpc/grpc_security.h>
#include <grpc/support/port_platform.h>
#include <grpc/support/sync.h>
#include <grpc/support/time.h>
#include <stdint.h>

#include <optional>
#include <string>

#include "absl/status/statusor.h"
#include "absl/strings/str_format.h"
#include "absl/strings/string_view.h"
#include "absl/time/time.h"
#include "src/core/credentials/call/call_credentials.h"
#include "src/core/credentials/call/jwt/json_token.h"
#include "src/core/lib/promise/arena_promise.h"
#include "src/core/lib/slice/slice.h"
#include "src/core/lib/transport/transport.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/unique_type_name.h"
#include "src/core/util/useful.h"

class grpc_service_account_jwt_access_credentials
    : public grpc_call_credentials {
 public:
  grpc_service_account_jwt_access_credentials(grpc_auth_json_key key,
                                              gpr_timespec token_lifetime);
  ~grpc_service_account_jwt_access_credentials() override;

  void Orphaned() override {}

  grpc_core::ArenaPromise<absl::StatusOr<grpc_core::ClientMetadataHandle>>
  GetRequestMetadata(grpc_core::ClientMetadataHandle initial_metadata,
                     const GetRequestMetadataArgs* args) override;

  const gpr_timespec& jwt_lifetime() const { return jwt_lifetime_; }
  const grpc_auth_json_key& key() const { return key_; }

  std::string debug_string() override {
    return absl::StrFormat(
        "JWTAccessCredentials{ExpirationTime:%s}",
        absl::FormatTime(absl::FromUnixMicros(
            static_cast<int64_t>(gpr_timespec_to_micros(jwt_lifetime_)))));
  };

  static grpc_core::UniqueTypeName Type();

  grpc_core::UniqueTypeName type() const override { return Type(); }

 private:
  int cmp_impl(const grpc_call_credentials* other) const override {
    // TODO(yashykt): Check if we can do something better here
    return grpc_core::QsortCompare(
        static_cast<const grpc_call_credentials*>(this), other);
  }

  // Have a simple cache for now with just 1 entry. We could have a map based on
  // the service_url for a more sophisticated one.
  gpr_mu cache_mu_;
  struct Cache {
    grpc_core::Slice jwt_value;
    std::string service_url;
    gpr_timespec jwt_expiration;
  };
  std::optional<Cache> cached_;

  grpc_auth_json_key key_;
  gpr_timespec jwt_lifetime_;
};

// Private constructor for jwt credentials from an already parsed json key.
// Takes ownership of the key.
grpc_core::RefCountedPtr<grpc_call_credentials>
grpc_service_account_jwt_access_credentials_create_from_auth_json_key(
    grpc_auth_json_key key, gpr_timespec token_lifetime);

namespace grpc_core {

// Exposed for testing purposes only.
absl::StatusOr<std::string> RemoveServiceNameFromJwtUri(absl::string_view uri);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CREDENTIALS_CALL_JWT_JWT_CREDENTIALS_H
