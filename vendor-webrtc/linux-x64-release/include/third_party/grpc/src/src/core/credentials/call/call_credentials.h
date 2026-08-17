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

#ifndef GRPC_SRC_CORE_CREDENTIALS_CALL_CALL_CREDENTIALS_H
#define GRPC_SRC_CORE_CREDENTIALS_CALL_CALL_CREDENTIALS_H

#include <grpc/credentials.h>
#include <grpc/grpc.h>
#include <grpc/grpc_security.h>
#include <grpc/grpc_security_constants.h>
#include <grpc/impl/grpc_types.h>
#include <grpc/support/port_platform.h>

#include <string>
#include <utility>
#include <vector>

#include "absl/log/check.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "src/core/credentials/transport/security_connector.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/promise/arena_promise.h"
#include "src/core/lib/slice/slice.h"
#include "src/core/lib/transport/transport.h"
#include "src/core/transport/auth_context.h"
#include "src/core/util/crash.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/unique_type_name.h"

// --- Constants. ---

typedef enum {
  GRPC_CREDENTIALS_OK = 0,
  GRPC_CREDENTIALS_ERROR
} grpc_credentials_status;

#define GRPC_AUTHORIZATION_METADATA_KEY "authorization"
#define GRPC_IAM_AUTHORIZATION_TOKEN_METADATA_KEY \
  "x-goog-iam-authorization-token"
#define GRPC_IAM_AUTHORITY_SELECTOR_METADATA_KEY "x-goog-iam-authority-selector"

#define GRPC_SECURE_TOKEN_REFRESH_THRESHOLD_SECS 60

#define GRPC_COMPUTE_ENGINE_METADATA_HOST "metadata.google.internal."
#define GRPC_COMPUTE_ENGINE_METADATA_TOKEN_PATH \
  "/computeMetadata/v1/instance/service-accounts/default/token"

#define GRPC_GOOGLE_OAUTH2_SERVICE_HOST "oauth2.googleapis.com"
#define GRPC_GOOGLE_OAUTH2_SERVICE_TOKEN_PATH "/token"

#define GRPC_SERVICE_ACCOUNT_POST_BODY_PREFIX                         \
  "grant_type=urn%3Aietf%3Aparams%3Aoauth%3Agrant-type%3Ajwt-bearer&" \
  "assertion="

#define GRPC_REFRESH_TOKEN_POST_BODY_FORMAT_STRING \
  "client_id=%s&client_secret=%s&refresh_token=%s&grant_type=refresh_token"

// --- Google utils ---

// It is the caller's responsibility to gpr_free the result if not NULL.
std::string grpc_get_well_known_google_credentials_file_path(void);

// Implementation function for the different platforms.
std::string grpc_get_well_known_google_credentials_file_path_impl(void);

// Override for testing only. Not thread-safe
typedef std::string (*grpc_well_known_credentials_path_getter)(void);
void grpc_override_well_known_credentials_path_getter(
    grpc_well_known_credentials_path_getter getter);

// --- grpc_core::CredentialsMetadataArray. ---

namespace grpc_core {
using CredentialsMetadataArray = std::vector<std::pair<Slice, Slice>>;
}

// --- grpc_call_credentials. ---

// This type is forward declared as a C struct and we cannot define it as a
// class. Otherwise, compiler will complain about type mismatch due to
// -Wmismatched-tags.
struct grpc_call_credentials
    : public grpc_core::DualRefCounted<grpc_call_credentials> {
 public:
  // TODO(roth): Consider whether security connector actually needs to
  // be part of this interface.  Currently, it is here only for the
  // url_scheme() method, which we might be able to instead add as an
  // auth context property.
  struct GetRequestMetadataArgs {
    grpc_core::RefCountedPtr<grpc_channel_security_connector>
        security_connector;
    grpc_core::RefCountedPtr<grpc_auth_context> auth_context;
  };

  // The pointer value \a type is used to uniquely identify a creds
  // implementation for down-casting purposes. Every creds implementation should
  // use a unique string instance, which should be returned by all instances of
  // that creds implementation.
  explicit grpc_call_credentials(
      grpc_security_level min_security_level = GRPC_PRIVACY_AND_INTEGRITY)
      : min_security_level_(min_security_level) {}

  ~grpc_call_credentials() override = default;

  virtual grpc_core::ArenaPromise<
      absl::StatusOr<grpc_core::ClientMetadataHandle>>
  GetRequestMetadata(grpc_core::ClientMetadataHandle initial_metadata,
                     const GetRequestMetadataArgs* args) = 0;

  virtual grpc_security_level min_security_level() const {
    return min_security_level_;
  }

  // Compares this grpc_call_credentials object with \a other.
  // If this method returns 0, it means that gRPC can treat the two call
  // credentials as effectively the same..
  int cmp(const grpc_call_credentials* other) const {
    CHECK_NE(other, nullptr);
    int r = type().Compare(other->type());
    if (r != 0) return r;
    return cmp_impl(other);
  }

  virtual std::string debug_string() {
    return "grpc_call_credentials did not provide debug string";
  }

  // The pointer value \a type is used to uniquely identify a creds
  // implementation for down-casting purposes. Every creds implementation should
  // use a unique string instance, which should be returned by all instances of
  // that creds implementation.
  virtual grpc_core::UniqueTypeName type() const = 0;

 private:
  // Implementation for `cmp` method intended to be overridden by subclasses.
  // Only invoked if `type()` and `other->type()` point to the same string.
  virtual int cmp_impl(const grpc_call_credentials* other) const = 0;

  const grpc_security_level min_security_level_;
};

#endif  // GRPC_SRC_CORE_CREDENTIALS_CALL_CALL_CREDENTIALS_H
