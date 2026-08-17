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

#ifndef GRPC_SRC_CORE_CREDENTIALS_CALL_EXTERNAL_EXTERNAL_ACCOUNT_CREDENTIALS_H
#define GRPC_SRC_CORE_CREDENTIALS_CALL_EXTERNAL_EXTERNAL_ACCOUNT_CREDENTIALS_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/support/port_platform.h>
#include <stdint.h>

#include <functional>
#include <memory>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "src/core/credentials/call/oauth2/oauth2_credentials.h"
#include "src/core/credentials/call/token_fetcher/token_fetcher_credentials.h"
#include "src/core/lib/iomgr/closure.h"
#include "src/core/lib/iomgr/error.h"
#include "src/core/lib/iomgr/polling_entity.h"
#include "src/core/util/http_client/httpcli.h"
#include "src/core/util/http_client/parser.h"
#include "src/core/util/json/json.h"
#include "src/core/util/orphanable.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/time.h"

namespace grpc_core {

// Base external account credentials. The base class implements common logic for
// exchanging external account credentials for GCP access token to authorize
// requests to GCP APIs. The specific logic of retrieving subject token is
// implemented in subclasses.
class ExternalAccountCredentials : public TokenFetcherCredentials {
 public:
  struct ServiceAccountImpersonation {
    int32_t token_lifetime_seconds;
  };
  // External account credentials json interface.
  struct Options {
    std::string type;
    std::string audience;
    std::string subject_token_type;
    std::string service_account_impersonation_url;
    ServiceAccountImpersonation service_account_impersonation;
    std::string token_url;
    std::string token_info_url;
    Json credential_source;
    std::string quota_project_id;
    std::string client_id;
    std::string client_secret;
    std::string workforce_pool_user_project;
  };

  static absl::StatusOr<RefCountedPtr<ExternalAccountCredentials>> Create(
      const Json& json, std::vector<std::string> scopes,
      std::shared_ptr<grpc_event_engine::experimental::EventEngine>
          event_engine = nullptr);

  ExternalAccountCredentials(
      Options options, std::vector<std::string> scopes,
      std::shared_ptr<grpc_event_engine::experimental::EventEngine>
          event_engine = nullptr);
  ~ExternalAccountCredentials() override;

 protected:
  // A base class for a cancellable fetch operation.
  class FetchBody : public InternallyRefCounted<FetchBody> {
   public:
    explicit FetchBody(
        absl::AnyInvocable<void(absl::StatusOr<std::string>)> on_done)
        : on_done_(std::move(on_done)) {}

    void Orphan() override {
      Shutdown();
      Unref();
    }

   protected:
    // The subclass must call this when the fetch is complete, even if
    // cancelled.
    void Finish(absl::StatusOr<std::string> result) {
      std::exchange(on_done_, nullptr)(std::move(result));
    }

   private:
    virtual void Shutdown() = 0;

    absl::AnyInvocable<void(absl::StatusOr<std::string>)> on_done_;
  };

  // A simple no-op implementation, used for async execution of the
  // on_done callback.
  class NoOpFetchBody final : public FetchBody {
   public:
    NoOpFetchBody(grpc_event_engine::experimental::EventEngine& event_engine,
                  absl::AnyInvocable<void(absl::StatusOr<std::string>)> on_done,
                  absl::StatusOr<std::string> result);

   private:
    void Shutdown() override {}
  };

  // An implementation for HTTP requests.
  class HttpFetchBody final : public FetchBody {
   public:
    HttpFetchBody(
        absl::FunctionRef<OrphanablePtr<HttpRequest>(grpc_http_response*,
                                                     grpc_closure*)>
            start_http_request,
        absl::AnyInvocable<void(absl::StatusOr<std::string>)> on_done);

    ~HttpFetchBody() override { grpc_http_response_destroy(&response_); }

   private:
    void Shutdown() override { http_request_.reset(); }

    static void OnHttpResponse(void* arg, grpc_error_handle error);

    OrphanablePtr<HttpRequest> http_request_;
    grpc_http_response response_;
    grpc_closure on_http_response_;
  };

  // An implementation of TokenFetcherCredentials::FetchRequest that
  // executes a series of FetchBody operations to ultimately get to a
  // token result.
  class ExternalFetchRequest : public FetchRequest {
   public:
    ExternalFetchRequest(
        ExternalAccountCredentials* creds, Timestamp deadline,
        absl::AnyInvocable<
            void(absl::StatusOr<RefCountedPtr<TokenFetcherCredentials::Token>>)>
            on_done);

    void Orphan() override;

   protected:
    Timestamp deadline() const { return deadline_; }
    grpc_polling_entity* pollent() const { return creds_->pollent(); }
    const Options& options() const { return creds_->options_; }

   private:
    void ExchangeToken(absl::StatusOr<std::string> subject_token);
    void MaybeImpersonateServiceAccount(
        absl::StatusOr<std::string> response_body);
    void OnImpersonateServiceAccount(absl::StatusOr<std::string> response_body);

    void FinishTokenFetch(absl::StatusOr<std::string> response_body);

    // If status is non-OK or we've been shut down, calls FinishTokenFetch()
    // and returns true.
    bool MaybeFailLocked(absl::Status status)
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(&mu_);

    ExternalAccountCredentials* creds_;
    Timestamp deadline_;
    absl::AnyInvocable<void(
        absl::StatusOr<RefCountedPtr<TokenFetcherCredentials::Token>>)>
        on_done_;

    Mutex mu_;
    OrphanablePtr<FetchBody> fetch_body_ ABSL_GUARDED_BY(&mu_);
  };

  virtual absl::string_view CredentialSourceType();

  std::string MetricsHeaderValue();

  absl::string_view audience() const { return options_.audience; }

 private:
  OrphanablePtr<FetchRequest> FetchToken(
      Timestamp deadline,
      absl::AnyInvocable<void(absl::StatusOr<RefCountedPtr<Token>>)> on_done)
      final;

  // Subclasses of ExternalAccountCredentials need to override this
  // method to implement the specific-subject token retrieval logic.
  // The caller will save the resulting FetchBody object, which will
  // be orphaned upon cancellation.  The FetchBody object must
  // eventually invoke on_done.
  virtual OrphanablePtr<FetchBody> RetrieveSubjectToken(
      Timestamp deadline,
      absl::AnyInvocable<void(absl::StatusOr<std::string>)> on_done) = 0;

  Options options_;
  std::vector<std::string> scopes_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CREDENTIALS_CALL_EXTERNAL_EXTERNAL_ACCOUNT_CREDENTIALS_H
