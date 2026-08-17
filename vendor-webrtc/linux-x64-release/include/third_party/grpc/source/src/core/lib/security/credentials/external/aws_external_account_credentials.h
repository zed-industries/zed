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

#ifndef GRPC_SRC_CORE_LIB_SECURITY_CREDENTIALS_EXTERNAL_AWS_EXTERNAL_ACCOUNT_CREDENTIALS_H
#define GRPC_SRC_CORE_LIB_SECURITY_CREDENTIALS_EXTERNAL_AWS_EXTERNAL_ACCOUNT_CREDENTIALS_H

#include <grpc/support/port_platform.h>

#include <functional>
#include <memory>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "src/core/lib/iomgr/error.h"
#include "src/core/lib/security/credentials/external/aws_request_signer.h"
#include "src/core/lib/security/credentials/external/external_account_credentials.h"
#include "src/core/util/http_client/httpcli.h"
#include "src/core/util/http_client/parser.h"
#include "src/core/util/orphanable.h"
#include "src/core/util/ref_counted_ptr.h"

namespace grpc_core {

class AwsExternalAccountCredentials final : public ExternalAccountCredentials {
 public:
  static absl::StatusOr<RefCountedPtr<AwsExternalAccountCredentials>> Create(
      Options options, std::vector<std::string> scopes,
      std::shared_ptr<grpc_event_engine::experimental::EventEngine>
          event_engine = nullptr);

  AwsExternalAccountCredentials(
      Options options, std::vector<std::string> scopes,
      std::shared_ptr<grpc_event_engine::experimental::EventEngine>
          event_engine,
      grpc_error_handle* error);

  std::string debug_string() override;

  static UniqueTypeName Type();

  UniqueTypeName type() const override { return Type(); }

 private:
  // A FetchBody impl that itself performs a sequence of FetchBody operations.
  class AwsFetchBody : public FetchBody {
   public:
    AwsFetchBody(absl::AnyInvocable<void(absl::StatusOr<std::string>)> on_done,
                 AwsExternalAccountCredentials* creds, Timestamp deadline);

   private:
    void Shutdown() override;

    void AsyncFinish(absl::StatusOr<std::string> result);
    bool MaybeFail(absl::Status status) ABSL_EXCLUSIVE_LOCKS_REQUIRED(&mu_);

    void Start();
    void RetrieveImdsV2SessionToken() ABSL_EXCLUSIVE_LOCKS_REQUIRED(&mu_);
    void RetrieveRegion() ABSL_EXCLUSIVE_LOCKS_REQUIRED(&mu_);
    void RetrieveRoleName() ABSL_EXCLUSIVE_LOCKS_REQUIRED(&mu_);
    void RetrieveSigningKeys() ABSL_EXCLUSIVE_LOCKS_REQUIRED(&mu_);
    void OnRetrieveSigningKeys(std::string result)
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(&mu_);
    void BuildSubjectToken() ABSL_EXCLUSIVE_LOCKS_REQUIRED(&mu_);

    void AddMetadataRequestHeaders(grpc_http_request* request);

    AwsExternalAccountCredentials* creds_;
    Timestamp deadline_;

    Mutex mu_;
    OrphanablePtr<FetchBody> fetch_body_ ABSL_GUARDED_BY(&mu_);

    // Information required by request signer
    std::string region_;
    std::string role_name_;
    std::string access_key_id_;
    std::string secret_access_key_;
    std::string token_;
    std::string imdsv2_session_token_;
  };

  OrphanablePtr<FetchBody> RetrieveSubjectToken(
      Timestamp deadline,
      absl::AnyInvocable<void(absl::StatusOr<std::string>)> on_done) override;

  absl::string_view CredentialSourceType() override;

  std::string audience_;

  // Fields of credential source
  std::string region_url_;
  std::string url_;
  std::string regional_cred_verification_url_;
  std::string imdsv2_session_token_url_;

  // These fields are set on the first fetch attempt and cached after that.
  std::unique_ptr<AwsRequestSigner> signer_;
  std::string cred_verification_url_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_SECURITY_CREDENTIALS_EXTERNAL_AWS_EXTERNAL_ACCOUNT_CREDENTIALS_H
