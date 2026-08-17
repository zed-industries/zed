// Copyright 2024 gRPC authors.
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

#ifndef GRPC_SRC_CORE_CALL_CALL_SPINE_H
#define GRPC_SRC_CORE_CALL_CALL_SPINE_H

#include <grpc/support/port_platform.h>

#include "absl/log/check.h"
#include "src/core/call/call_arena_allocator.h"
#include "src/core/call/call_filters.h"
#include "src/core/call/message.h"
#include "src/core/call/metadata.h"
#include "src/core/lib/promise/detail/status.h"
#include "src/core/lib/promise/if.h"
#include "src/core/lib/promise/latch.h"
#include "src/core/lib/promise/party.h"
#include "src/core/lib/promise/pipe.h"
#include "src/core/lib/promise/prioritized_race.h"
#include "src/core/lib/promise/promise.h"
#include "src/core/lib/promise/status_flag.h"
#include "src/core/lib/promise/try_seq.h"
#include "src/core/util/dual_ref_counted.h"

namespace grpc_core {

// The common middle part of a call - a reference is held by each of
// CallInitiator and CallHandler - which provide interfaces that are appropriate
// for each side of a call.
// Hosts context, call filters, and the arena.
class CallSpine final : public Party {
 public:
  static RefCountedPtr<CallSpine> Create(
      ClientMetadataHandle client_initial_metadata,
      RefCountedPtr<Arena> arena) {
    Arena* arena_ptr = arena.get();
    return RefCountedPtr<CallSpine>(arena_ptr->New<CallSpine>(
        std::move(client_initial_metadata), std::move(arena)));
  }

  ~CallSpine() override { CallOnDone(true); }

  CallFilters& call_filters() { return call_filters_; }

  // Add a callback to be called when server trailing metadata is received and
  // return true.
  // If CallOnDone has already been invoked, does nothing and returns false.
  GRPC_MUST_USE_RESULT bool OnDone(absl::AnyInvocable<void(bool)> fn) {
    if (call_filters().WasServerTrailingMetadataPulled()) {
      return false;
    }
    if (on_done_ == nullptr) {
      on_done_ = std::move(fn);
      return true;
    }
    on_done_ = [first = std::move(fn),
                next = std::move(on_done_)](bool cancelled) mutable {
      first(cancelled);
      next(cancelled);
    };
    return true;
  }
  void CallOnDone(bool cancelled) {
    if (on_done_ != nullptr) std::exchange(on_done_, nullptr)(cancelled);
  }

  auto PullServerInitialMetadata() {
    return call_filters().PullServerInitialMetadata();
  }

  auto PullServerTrailingMetadata() {
    return Map(
        call_filters().PullServerTrailingMetadata(),
        [this](ServerMetadataHandle result) {
          CallOnDone(result->get(GrpcCallWasCancelled()).value_or(false));
          return result;
        });
  }

  auto PushClientToServerMessage(MessageHandle message) {
    return call_filters().PushClientToServerMessage(std::move(message));
  }

  auto PullClientToServerMessage() {
    return call_filters().PullClientToServerMessage();
  }

  auto PushServerToClientMessage(MessageHandle message) {
    return call_filters().PushServerToClientMessage(std::move(message));
  }

  auto PullServerToClientMessage() {
    return call_filters().PullServerToClientMessage();
  }

  void PushServerTrailingMetadata(ServerMetadataHandle md) {
    GRPC_TRACE_LOG(call_state, INFO)
        << "[call_state] PushServerTrailingMetadata: " << this << " "
        << md->DebugString();
    call_filters().PushServerTrailingMetadata(std::move(md));
  }

  void FinishSends() { call_filters().FinishClientToServerSends(); }

  auto PullClientInitialMetadata() {
    return call_filters().PullClientInitialMetadata();
  }

  StatusFlag PushServerInitialMetadata(ServerMetadataHandle md) {
    return call_filters().PushServerInitialMetadata(std::move(md));
  }

  auto WasCancelled() { return call_filters().WasCancelled(); }

  ClientMetadata& UnprocessedClientInitialMetadata() {
    return *call_filters().unprocessed_client_initial_metadata();
  }

  // Wrap a promise so that if it returns failure it automatically cancels
  // the rest of the call.
  // The resulting (returned) promise will resolve to Empty.
  template <typename Promise>
  auto CancelIfFails(Promise promise) {
    DCHECK(GetContext<Activity>() == this);
    using P = promise_detail::PromiseLike<Promise>;
    using ResultType = typename P::Result;
    return Map(std::move(promise),
               [self = RefAsSubclass<CallSpine>()](ResultType r) {
                 self->CancelIfFailed(r);
               });
  }

  template <typename StatusType>
  void CancelIfFailed(const StatusType& r) {
    if (!IsStatusOk(r)) {
      GRPC_TRACE_LOG(call_state, INFO)
          << "[call_state] spine " << this << " fails: " << r;
      Cancel();
    }
  }

  void Cancel() { call_filters().Cancel(); }

  // Spawn a promise that returns Empty{} and save some boilerplate handling
  // that detail.
  template <typename PromiseFactory>
  void SpawnInfallible(absl::string_view name, PromiseFactory promise_factory) {
    Spawn(name, std::move(promise_factory), [](Empty) {});
  }

  // Spawn a promise that returns some status-like type; if the status
  // represents failure automatically cancel the rest of the call.
  template <typename PromiseFactory>
  void SpawnGuarded(absl::string_view name, PromiseFactory promise_factory,
                    DebugLocation whence = {}) {
    using FactoryType =
        promise_detail::OncePromiseFactory<void, PromiseFactory>;
    using PromiseType = typename FactoryType::Promise;
    using ResultType = typename PromiseType::Result;
    static_assert(
        std::is_same<bool,
                     decltype(IsStatusOk(std::declval<ResultType>()))>::value,
        "SpawnGuarded promise must return a status-like object");
    Spawn(name, std::move(promise_factory), [this, whence](ResultType r) {
      if (!IsStatusOk(r)) {
        GRPC_TRACE_LOG(promise_primitives, INFO)
            << "SpawnGuarded sees failure: " << r
            << " (source: " << whence.file() << ":" << whence.line() << ")";
        auto status = StatusCast<ServerMetadataHandle>(std::move(r));
        status->Set(GrpcCallWasCancelled(), true);
        PushServerTrailingMetadata(std::move(status));
      }
    });
  }

  // Wrap a promise so that if the call completes that promise is cancelled.
  template <typename Promise>
  auto UntilCallCompletes(Promise promise) {
    using Result = PromiseResult<Promise>;
    return PrioritizedRace(std::move(promise), Map(WasCancelled(), [](bool) {
                             return FailureStatusCast<Result>(Failure{});
                           }));
  }

  template <typename PromiseFactory>
  void SpawnGuardedUntilCallCompletes(absl::string_view name,
                                      PromiseFactory promise_factory) {
    SpawnGuarded(name, [this, promise_factory]() mutable {
      return UntilCallCompletes(promise_factory());
    });
  }

  // Spawned operations: these are callable from /outside/ the call; they spawn
  // an operation into the call and execute that operation.
  //
  // Server -> client operations are serialized in the order they are spawned.
  // Client -> server operations are serialized in the order they are spawned.
  //
  // It's required that at most one thread call a server->client operation at a
  // time, and likewise for client->server operations. There is no requirement
  // that there be synchronization between the two directionalities.
  //
  // No ordering is given between the `Spawn` and the basic operations.

  void SpawnPushServerInitialMetadata(ServerMetadataHandle md) {
    server_to_client_serializer()->Spawn(
        [md = std::move(md), self = RefAsSubclass<CallSpine>()]() mutable {
          self->CancelIfFailed(self->PushServerInitialMetadata(std::move(md)));
        });
  }

  void SpawnPushServerToClientMessage(MessageHandle msg) {
    server_to_client_serializer()->Spawn(
        [msg = std::move(msg), self = RefAsSubclass<CallSpine>()]() mutable {
          return self->CancelIfFails(
              self->PushServerToClientMessage(std::move(msg)));
        });
  }

  void SpawnPushClientToServerMessage(MessageHandle msg) {
    client_to_server_serializer()->Spawn(
        [msg = std::move(msg), self = RefAsSubclass<CallSpine>()]() mutable {
          return self->CancelIfFails(
              self->PushClientToServerMessage(std::move(msg)));
        });
  }

  void SpawnFinishSends() {
    client_to_server_serializer()->Spawn([self = RefAsSubclass<CallSpine>()]() {
      self->FinishSends();
      return Empty{};
    });
  }

  void SpawnPushServerTrailingMetadata(ServerMetadataHandle md) {
    if (md->get(GrpcCallWasCancelled()).value_or(false)) {
      // Cancellation doesn't serialize with the rest of ops
      SpawnInfallible(
          "push-server-trailing-metadata",
          [md = std::move(md), self = RefAsSubclass<CallSpine>()]() mutable {
            self->PushServerTrailingMetadata(std::move(md));
            return Empty{};
          });
    } else {
      server_to_client_serializer()->Spawn(
          [md = std::move(md), self = RefAsSubclass<CallSpine>()]() mutable {
            self->PushServerTrailingMetadata(std::move(md));
            return Empty{};
          });
    }
  }

  void SpawnCancel() {
    SpawnInfallible("cancel", [self = RefAsSubclass<CallSpine>()]() {
      self->call_filters().Cancel();
    });
  }

  void AddChildCall(RefCountedPtr<CallSpine> child_call) {
    child_calls_.push_back(std::move(child_call));
    if (child_calls_.size() == 1) {
      SpawnInfallible(
          "check_cancellation", [self = RefAsSubclass<CallSpine>()]() mutable {
            auto was_completed =
                self->call_filters().ServerTrailingMetadataWasPushed();
            return Map(std::move(was_completed),
                       [self = std::move(self)](Empty) {
                         for (auto& child : self->child_calls_) {
                           child->SpawnCancel();
                         }
                         return Empty{};
                       });
          });
    }
  }

 private:
  friend class Arena;
  CallSpine(ClientMetadataHandle client_initial_metadata,
            RefCountedPtr<Arena> arena)
      : Party(std::move(arena)),
        call_filters_(std::move(client_initial_metadata)) {}

  SpawnSerializer* client_to_server_serializer() {
    if (client_to_server_serializer_ == nullptr) {
      client_to_server_serializer_ = MakeSpawnSerializer();
    }
    return client_to_server_serializer_;
  }

  SpawnSerializer* server_to_client_serializer() {
    if (server_to_client_serializer_ == nullptr) {
      server_to_client_serializer_ = MakeSpawnSerializer();
    }
    return server_to_client_serializer_;
  }

  // Call filters/pipes part of the spine
  CallFilters call_filters_;
  absl::AnyInvocable<void(bool)> on_done_{nullptr};
  // Call spines that should be cancelled if this spine is cancelled
  absl::InlinedVector<RefCountedPtr<CallSpine>, 3> child_calls_;
  SpawnSerializer* client_to_server_serializer_ = nullptr;
  SpawnSerializer* server_to_client_serializer_ = nullptr;
};

class CallHandler;

class CallInitiator {
 public:
  using NextMessage = ServerToClientNextMessage;

  CallInitiator() = default;
  explicit CallInitiator(RefCountedPtr<CallSpine> spine)
      : spine_(std::move(spine)) {
    DCHECK_NE(spine_.get(), nullptr);
  }

  // Wrap a promise so that if it returns failure it automatically cancels
  // the rest of the call.
  // The resulting (returned) promise will resolve to Empty.
  template <typename Promise>
  auto CancelIfFails(Promise promise) {
    DCHECK_NE(spine_.get(), nullptr);
    return spine_->CancelIfFails(std::move(promise));
  }

  auto PullServerInitialMetadata() {
    DCHECK_NE(spine_.get(), nullptr);
    return spine_->PullServerInitialMetadata();
  }

  auto PushMessage(MessageHandle message) {
    DCHECK_NE(spine_.get(), nullptr);
    return spine_->PushClientToServerMessage(std::move(message));
  }

  void SpawnPushMessage(MessageHandle message) {
    DCHECK_NE(spine_.get(), nullptr);
    spine_->SpawnPushClientToServerMessage(std::move(message));
  }

  void FinishSends() {
    DCHECK_NE(spine_.get(), nullptr);
    spine_->FinishSends();
  }

  void SpawnFinishSends() {
    DCHECK_NE(spine_.get(), nullptr);
    spine_->SpawnFinishSends();
  }

  auto PullMessage() {
    DCHECK_NE(spine_.get(), nullptr);
    return spine_->PullServerToClientMessage();
  }

  auto PullServerTrailingMetadata() {
    DCHECK_NE(spine_.get(), nullptr);
    return spine_->PullServerTrailingMetadata();
  }

  void Cancel(absl::Status error) {
    DCHECK_NE(spine_.get(), nullptr);
    CHECK(!error.ok());
    auto status = ServerMetadataFromStatus(error);
    status->Set(GrpcCallWasCancelled(), true);
    spine_->PushServerTrailingMetadata(std::move(status));
  }

  void SpawnCancel(absl::Status error) {
    DCHECK_NE(spine_.get(), nullptr);
    CHECK(!error.ok());
    auto status = ServerMetadataFromStatus(error);
    status->Set(GrpcCallWasCancelled(), true);
    spine_->SpawnPushServerTrailingMetadata(std::move(status));
  }

  void Cancel() {
    DCHECK_NE(spine_.get(), nullptr);
    spine_->Cancel();
  }

  void SpawnCancel() {
    DCHECK_NE(spine_.get(), nullptr);
    spine_->SpawnCancel();
  }

  GRPC_MUST_USE_RESULT bool OnDone(absl::AnyInvocable<void(bool)> fn) {
    DCHECK_NE(spine_.get(), nullptr);
    return spine_->OnDone(std::move(fn));
  }

  template <typename Promise>
  auto UntilCallCompletes(Promise promise) {
    DCHECK_NE(spine_.get(), nullptr);
    return spine_->UntilCallCompletes(std::move(promise));
  }

  template <typename PromiseFactory>
  void SpawnGuarded(absl::string_view name, PromiseFactory promise_factory) {
    DCHECK_NE(spine_.get(), nullptr);
    spine_->SpawnGuarded(name, std::move(promise_factory));
  }

  template <typename PromiseFactory>
  void SpawnGuardedUntilCallCompletes(absl::string_view name,
                                      PromiseFactory promise_factory) {
    DCHECK_NE(spine_.get(), nullptr);
    spine_->SpawnGuardedUntilCallCompletes(name, std::move(promise_factory));
  }

  template <typename PromiseFactory>
  void SpawnInfallible(absl::string_view name, PromiseFactory promise_factory) {
    DCHECK_NE(spine_.get(), nullptr);
    spine_->SpawnInfallible(name, std::move(promise_factory));
  }

  template <typename PromiseFactory>
  auto SpawnWaitable(absl::string_view name, PromiseFactory promise_factory) {
    DCHECK_NE(spine_.get(), nullptr);
    return spine_->SpawnWaitable(name, std::move(promise_factory));
  }

  bool WasCancelledPushed() const {
    DCHECK_NE(spine_.get(), nullptr);
    return spine_->call_filters().WasCancelledPushed();
  }

  Arena* arena() {
    DCHECK_NE(spine_.get(), nullptr);
    return spine_->arena();
  }
  Party* party() {
    DCHECK_NE(spine_.get(), nullptr);
    return spine_.get();
  }

 private:
  friend class CallHandler;
  RefCountedPtr<CallSpine> spine_;
};

class CallHandler {
 public:
  using NextMessage = ClientToServerNextMessage;

  explicit CallHandler(RefCountedPtr<CallSpine> spine)
      : spine_(std::move(spine)) {}

  auto PullClientInitialMetadata() {
    return spine_->PullClientInitialMetadata();
  }

  auto PushServerInitialMetadata(ServerMetadataHandle md) {
    return spine_->PushServerInitialMetadata(std::move(md));
  }

  void SpawnPushServerInitialMetadata(ServerMetadataHandle md) {
    return spine_->SpawnPushServerInitialMetadata(std::move(md));
  }

  void PushServerTrailingMetadata(ServerMetadataHandle status) {
    spine_->PushServerTrailingMetadata(std::move(status));
  }

  void SpawnPushServerTrailingMetadata(ServerMetadataHandle status) {
    spine_->SpawnPushServerTrailingMetadata(std::move(status));
  }

  GRPC_MUST_USE_RESULT bool OnDone(absl::AnyInvocable<void(bool)> fn) {
    return spine_->OnDone(std::move(fn));
  }

  // Wrap a promise so that if it returns failure it automatically cancels
  // the rest of the call.
  // The resulting (returned) promise will resolve to Empty.
  template <typename Promise>
  auto CancelIfFails(Promise promise) {
    return spine_->CancelIfFails(std::move(promise));
  }

  auto PushMessage(MessageHandle message) {
    return spine_->PushServerToClientMessage(std::move(message));
  }

  void SpawnPushMessage(MessageHandle message) {
    spine_->SpawnPushServerToClientMessage(std::move(message));
  }

  auto PullMessage() { return spine_->PullClientToServerMessage(); }

  auto WasCancelled() { return spine_->WasCancelled(); }

  bool WasCancelledPushed() const {
    return spine_->call_filters().WasCancelledPushed();
  }

  template <typename Promise>
  auto UntilCallCompletes(Promise promise) {
    return spine_->UntilCallCompletes(std::move(promise));
  }

  template <typename PromiseFactory>
  void SpawnGuarded(absl::string_view name, PromiseFactory promise_factory,
                    DebugLocation whence = {}) {
    spine_->SpawnGuarded(name, std::move(promise_factory), whence);
  }

  template <typename PromiseFactory>
  void SpawnGuardedUntilCallCompletes(absl::string_view name,
                                      PromiseFactory promise_factory) {
    spine_->SpawnGuardedUntilCallCompletes(name, std::move(promise_factory));
  }

  template <typename PromiseFactory>
  void SpawnInfallible(absl::string_view name, PromiseFactory promise_factory) {
    spine_->SpawnInfallible(name, std::move(promise_factory));
  }

  template <typename PromiseFactory>
  auto SpawnWaitable(absl::string_view name, PromiseFactory promise_factory) {
    return spine_->SpawnWaitable(name, std::move(promise_factory));
  }

  void AddChildCall(const CallInitiator& initiator) {
    CHECK(initiator.spine_ != nullptr);
    spine_->AddChildCall(initiator.spine_);
  }

  Arena* arena() { return spine_->arena(); }
  Party* party() { return spine_.get(); }

 private:
  RefCountedPtr<CallSpine> spine_;
};

class UnstartedCallHandler {
 public:
  explicit UnstartedCallHandler(RefCountedPtr<CallSpine> spine)
      : spine_(std::move(spine)) {}

  void PushServerTrailingMetadata(ServerMetadataHandle status) {
    spine_->PushServerTrailingMetadata(std::move(status));
  }

  GRPC_MUST_USE_RESULT bool OnDone(absl::AnyInvocable<void(bool)> fn) {
    return spine_->OnDone(std::move(fn));
  }

  template <typename Promise>
  auto CancelIfFails(Promise promise) {
    return spine_->CancelIfFails(std::move(promise));
  }

  template <typename PromiseFactory>
  void SpawnGuarded(absl::string_view name, PromiseFactory promise_factory,
                    DebugLocation whence = {}) {
    spine_->SpawnGuarded(name, std::move(promise_factory), whence);
  }

  template <typename PromiseFactory>
  void SpawnGuardedUntilCallCompletes(absl::string_view name,
                                      PromiseFactory promise_factory) {
    spine_->SpawnGuardedUntilCallCompletes(name, std::move(promise_factory));
  }

  template <typename PromiseFactory>
  void SpawnInfallible(absl::string_view name, PromiseFactory promise_factory) {
    spine_->SpawnInfallible(name, std::move(promise_factory));
  }

  template <typename PromiseFactory>
  auto SpawnWaitable(absl::string_view name, PromiseFactory promise_factory) {
    return spine_->SpawnWaitable(name, std::move(promise_factory));
  }

  ClientMetadata& UnprocessedClientInitialMetadata() {
    return spine_->UnprocessedClientInitialMetadata();
  }

  void AddCallStack(RefCountedPtr<CallFilters::Stack> call_filters) {
    spine_->call_filters().AddStack(std::move(call_filters));
  }

  CallHandler StartCall() {
    spine_->call_filters().Start();
    return CallHandler(std::move(spine_));
  }

  Arena* arena() { return spine_->arena(); }

 private:
  RefCountedPtr<CallSpine> spine_;
};

struct CallInitiatorAndHandler {
  CallInitiator initiator;
  UnstartedCallHandler handler;
};

CallInitiatorAndHandler MakeCallPair(
    ClientMetadataHandle client_initial_metadata, RefCountedPtr<Arena> arena);

template <typename CallHalf>
auto MessagesFrom(CallHalf h) {
  struct Wrapper {
    CallHalf h;
    auto Next() { return h.PullMessage(); }
  };
  return Wrapper{std::move(h)};
}

template <typename CallHalf>
auto MessagesFrom(CallHalf* h) {
  struct Wrapper {
    CallHalf* h;
    auto Next() { return h->PullMessage(); }
  };
  return Wrapper{h};
}

// Forward a call from `call_handler` to `call_initiator` (with initial metadata
// `client_initial_metadata`)
// `on_server_trailing_metadata_from_initiator` is a callback that will be
// called with the server trailing metadata received by the initiator, and can
// be used to mutate that metadata if desired.
void ForwardCall(
    CallHandler call_handler, CallInitiator call_initiator,
    absl::AnyInvocable<void(ServerMetadata&)>
        on_server_trailing_metadata_from_initiator = [](ServerMetadata&) {});

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CALL_CALL_SPINE_H
