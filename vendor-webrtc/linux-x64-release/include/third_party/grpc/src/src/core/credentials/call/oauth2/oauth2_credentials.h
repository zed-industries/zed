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

#ifndef GRPC_SRC_CORE_CREDENTIALS_CALL_OAUTH2_OAUTH2_CREDENTIALS_H
#define GRPC_SRC_CORE_CREDENTIALS_CALL_OAUTH2_OAUTH2_CREDENTIALS_H

#include <grpc/credentials.h>
#include <grpc/grpc_security.h>
#include <grpc/support/port_platform.h>
#include <grpc/support/sync.h>
#include <grpc/support/time.h>

#include <atomic>
#include <optional>
#include <string>
#include <utility>

#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "src/core/credentials/call/call_credentials.h"
#include "src/core/credentials/call/token_fetcher/token_fetcher_credentials.h"
#include "src/core/lib/iomgr/closure.h"
#include "src/core/lib/iomgr/error.h"
#include "src/core/lib/iomgr/polling_entity.h"
#include "src/core/lib/promise/activity.h"
#include "src/core/lib/promise/arena_promise.h"
#include "src/core/lib/slice/slice.h"
#include "src/core/lib/transport/transport.h"
#include "src/core/util/http_client/httpcli.h"
#include "src/core/util/http_client/parser.h"
#include "src/core/util/json/json.h"
#include "src/core/util/orphanable.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/time.h"
#include "src/core/util/unique_type_name.h"
#include "src/core/util/uri.h"
#include "src/core/util/useful.h"

// Constants.
#define GRPC_STS_POST_MINIMAL_BODY_FORMAT_STRING                               \
  "grant_type=urn:ietf:params:oauth:grant-type:token-exchange&subject_token=%" \
  "s&subject_token_type=%s"

// auth_refresh_token parsing.
struct grpc_auth_refresh_token {
  const char* type;
  char* client_id;
  char* client_secret;
  char* refresh_token;
};
/// Returns 1 if the object is valid, 0 otherwise.
int grpc_auth_refresh_token_is_valid(
    const grpc_auth_refresh_token* refresh_token);

/// Creates a refresh token object from string. Returns an invalid object if a
/// parsing error has been encountered.
grpc_auth_refresh_token grpc_auth_refresh_token_create_from_string(
    const char* json_string);

/// Creates a refresh token object from parsed json. Returns an invalid object
/// if a parsing error has been encountered.
grpc_auth_refresh_token grpc_auth_refresh_token_create_from_json(
    const grpc_core::Json& json);

/// Destructs the object.
void grpc_auth_refresh_token_destruct(grpc_auth_refresh_token* refresh_token);

// -- Oauth2 Token Fetcher credentials --
//
//  This object is a base for credentials that need to acquire an oauth2 token
//  from an http service.

namespace grpc_core {

// A base class for oauth2 token fetching credentials.
// Subclasses must implement StartHttpRequest().
class Oauth2TokenFetcherCredentials : public TokenFetcherCredentials {
 public:
  std::string debug_string() override;

  UniqueTypeName type() const override;

  OrphanablePtr<FetchRequest> FetchToken(
      Timestamp deadline,
      absl::AnyInvocable<
          void(absl::StatusOr<RefCountedPtr<TokenFetcherCredentials::Token>>)>
          on_done) final;

  virtual OrphanablePtr<HttpRequest> StartHttpRequest(
      grpc_polling_entity* pollent, Timestamp deadline,
      grpc_http_response* response, grpc_closure* on_complete) = 0;

 private:
  class HttpFetchRequest;

  int cmp_impl(const grpc_call_credentials* other) const override {
    // TODO(yashykt): Check if we can do something better here
    return QsortCompare(static_cast<const grpc_call_credentials*>(this), other);
  }
};

}  // namespace grpc_core

// Google refresh token credentials.
class grpc_google_refresh_token_credentials final
    : public grpc_core::Oauth2TokenFetcherCredentials {
 public:
  explicit grpc_google_refresh_token_credentials(
      grpc_auth_refresh_token refresh_token);
  ~grpc_google_refresh_token_credentials() override;

  const grpc_auth_refresh_token& refresh_token() const {
    return refresh_token_;
  }

  std::string debug_string() override;

  grpc_core::UniqueTypeName type() const override;

 private:
  grpc_core::OrphanablePtr<grpc_core::HttpRequest> StartHttpRequest(
      grpc_polling_entity* pollent, grpc_core::Timestamp deadline,
      grpc_http_response* response, grpc_closure* on_complete) override;

  grpc_auth_refresh_token refresh_token_;
};

// Access token credentials.
class grpc_access_token_credentials final : public grpc_call_credentials {
 public:
  explicit grpc_access_token_credentials(const char* access_token);

  void Orphaned() override {}

  grpc_core::ArenaPromise<absl::StatusOr<grpc_core::ClientMetadataHandle>>
  GetRequestMetadata(grpc_core::ClientMetadataHandle initial_metadata,
                     const GetRequestMetadataArgs* args) override;

  std::string debug_string() override;

  static grpc_core::UniqueTypeName Type();

  grpc_core::UniqueTypeName type() const override { return Type(); }

 private:
  int cmp_impl(const grpc_call_credentials* other) const override {
    // TODO(yashykt): Check if we can do something better here
    return grpc_core::QsortCompare(
        static_cast<const grpc_call_credentials*>(this), other);
  }

  const grpc_core::Slice access_token_value_;
};

// Private constructor for refresh token credentials from an already parsed
// refresh token. Takes ownership of the refresh token.
grpc_core::RefCountedPtr<grpc_call_credentials>
grpc_refresh_token_credentials_create_from_auth_refresh_token(
    grpc_auth_refresh_token token);

grpc_credentials_status
grpc_oauth2_token_fetcher_credentials_parse_server_response_body(
    absl::string_view body, std::optional<grpc_core::Slice>* token_value,
    grpc_core::Duration* token_lifetime);

// Exposed for testing only.
grpc_credentials_status
grpc_oauth2_token_fetcher_credentials_parse_server_response(
    const struct grpc_http_response* response,
    std::optional<grpc_core::Slice>* token_value,
    grpc_core::Duration* token_lifetime);

namespace grpc_core {
// Exposed for testing only. This function validates the options, ensuring that
// the required fields are set, and outputs the parsed URL of the STS token
// exchanged service.
absl::StatusOr<URI> ValidateStsCredentialsOptions(
    const grpc_sts_credentials_options* options);
}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CREDENTIALS_CALL_OAUTH2_OAUTH2_CREDENTIALS_H
