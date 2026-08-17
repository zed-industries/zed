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

#ifndef GRPC_SRC_CORE_CREDENTIALS_CALL_TOKEN_FETCHER_TOKEN_FETCHER_CREDENTIALS_H
#define GRPC_SRC_CORE_CREDENTIALS_CALL_TOKEN_FETCHER_TOKEN_FETCHER_CREDENTIALS_H

#include <grpc/event_engine/event_engine.h>

#include <atomic>
#include <memory>
#include <utility>
#include <variant>

#include "absl/container/flat_hash_set.h"
#include "absl/functional/any_invocable.h"
#include "absl/status/statusor.h"
#include "src/core/call/metadata.h"
#include "src/core/credentials/call/call_credentials.h"
#include "src/core/lib/iomgr/polling_entity.h"
#include "src/core/lib/promise/arena_promise.h"
#include "src/core/util/backoff.h"
#include "src/core/util/http_client/httpcli.h"
#include "src/core/util/orphanable.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/sync.h"
#include "src/core/util/time.h"
#include "src/core/util/useful.h"

namespace grpc_core {

// A base class for credentials that fetch tokens via an HTTP request.
// Subclasses must implement FetchToken().
class TokenFetcherCredentials : public grpc_call_credentials {
 public:
  // Represents a token.
  class Token : public RefCounted<Token> {
   public:
    Token(Slice token, Timestamp expiration);

    // Returns the token's expiration time.
    Timestamp ExpirationTime() const { return expiration_; }

    // Adds the token to the call's client initial metadata.
    void AddTokenToClientInitialMetadata(ClientMetadata& metadata) const;

   private:
    Slice token_;
    Timestamp expiration_;
  };

  ~TokenFetcherCredentials() override;

  void Orphaned() override;

  ArenaPromise<absl::StatusOr<ClientMetadataHandle>> GetRequestMetadata(
      ClientMetadataHandle initial_metadata,
      const GetRequestMetadataArgs* args) override;

 protected:
  // Base class for fetch requests.
  class FetchRequest : public InternallyRefCounted<FetchRequest> {};

  explicit TokenFetcherCredentials(
      std::shared_ptr<grpc_event_engine::experimental::EventEngine>
          event_engine = nullptr,
      bool test_only_use_backoff_jitter = true);

  // Fetches a token.  The on_done callback will be invoked when complete.
  virtual OrphanablePtr<FetchRequest> FetchToken(
      Timestamp deadline,
      absl::AnyInvocable<void(absl::StatusOr<RefCountedPtr<Token>>)>
          on_done) = 0;

  grpc_event_engine::experimental::EventEngine& event_engine() const {
    return *event_engine_;
  }

  grpc_polling_entity* pollent() { return &pollent_; }

 private:
  // A call that is waiting for a token fetch request to complete.
  struct QueuedCall : public RefCounted<QueuedCall> {
    std::atomic<bool> done{false};
    Waker waker;
    grpc_polling_entity* pollent;
    ClientMetadataHandle md;
    absl::StatusOr<RefCountedPtr<Token>> result;
  };

  class FetchState : public InternallyRefCounted<FetchState> {
   public:
    explicit FetchState(WeakRefCountedPtr<TokenFetcherCredentials> creds)
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(&TokenFetcherCredentials::mu_);

    // Disabling thread safety annotations, since Orphan() is called
    // by OrpahanablePtr<>, which does not have the right lock
    // annotations.
    void Orphan() override ABSL_NO_THREAD_SAFETY_ANALYSIS;

    // Returns non-OK when we're in backoff.
    absl::Status status() const;

    RefCountedPtr<QueuedCall> QueueCall(ClientMetadataHandle initial_metadata)
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(&TokenFetcherCredentials::mu_);

   private:
    class BackoffTimer : public InternallyRefCounted<BackoffTimer> {
     public:
      BackoffTimer(RefCountedPtr<FetchState> fetch_state, absl::Status status)
          ABSL_EXCLUSIVE_LOCKS_REQUIRED(&TokenFetcherCredentials::mu_);

      // Disabling thread safety annotations, since Orphan() is called
      // by OrpahanablePtr<>, which does not have the right lock
      // annotations.
      void Orphan() override ABSL_NO_THREAD_SAFETY_ANALYSIS;

      absl::Status status() const { return status_; }

     private:
      void OnTimer();

      RefCountedPtr<FetchState> fetch_state_;
      const absl::Status status_;
      std::optional<grpc_event_engine::experimental::EventEngine::TaskHandle>
          timer_handle_ ABSL_GUARDED_BY(&TokenFetcherCredentials::mu_);
    };

    struct Shutdown {};

    void StartFetchAttempt()
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(&TokenFetcherCredentials::mu_);
    void TokenFetchComplete(absl::StatusOr<RefCountedPtr<Token>> token);
    void ResumeQueuedCalls(absl::StatusOr<RefCountedPtr<Token>> token)
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(&TokenFetcherCredentials::mu_);

    WeakRefCountedPtr<TokenFetcherCredentials> creds_;
    // Pending token-fetch request or backoff timer, if any.
    std::variant<OrphanablePtr<FetchRequest>, OrphanablePtr<BackoffTimer>,
                 Shutdown>
        state_ ABSL_GUARDED_BY(&TokenFetcherCredentials::mu_);
    // Calls that are queued up waiting for the token.
    absl::flat_hash_set<RefCountedPtr<QueuedCall>> queued_calls_
        ABSL_GUARDED_BY(&TokenFetcherCredentials::mu_);
    // Backoff state.
    BackOff backoff_ ABSL_GUARDED_BY(&TokenFetcherCredentials::mu_);
  };

  int cmp_impl(const grpc_call_credentials* other) const override {
    // TODO(yashykt): Check if we can do something better here
    return QsortCompare(static_cast<const grpc_call_credentials*>(this), other);
  }

  std::shared_ptr<grpc_event_engine::experimental::EventEngine> event_engine_;
  const bool test_only_use_backoff_jitter_;

  Mutex mu_;
  // Cached token, if any.
  RefCountedPtr<Token> token_ ABSL_GUARDED_BY(&mu_);
  // Fetch state, if any.
  OrphanablePtr<FetchState> fetch_state_ ABSL_GUARDED_BY(&mu_);

  grpc_polling_entity pollent_ ABSL_GUARDED_BY(&mu_);
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CREDENTIALS_CALL_TOKEN_FETCHER_TOKEN_FETCHER_CREDENTIALS_H
