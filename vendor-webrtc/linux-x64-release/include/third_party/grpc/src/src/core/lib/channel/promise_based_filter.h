// Copyright 2022 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_CHANNEL_PROMISE_BASED_FILTER_H
#define GRPC_SRC_CORE_LIB_CHANNEL_PROMISE_BASED_FILTER_H

// Scaffolding to allow the per-call part of a filter to be authored in a
// promise-style. Most of this will be removed once the promises conversion is
// completed.

#include <grpc/event_engine/event_engine.h>
#include <grpc/grpc.h>
#include <grpc/support/port_platform.h>
#include <stdint.h>
#include <stdlib.h>

#include <atomic>
#include <memory>
#include <new>
#include <optional>
#include <string>
#include <type_traits>
#include <utility>

#include "absl/container/inlined_vector.h"
#include "absl/functional/function_ref.h"
#include "absl/log/check.h"
#include "absl/log/log.h"
#include "absl/meta/type_traits.h"
#include "absl/status/status.h"
#include "absl/strings/string_view.h"
#include "src/core/call/call_filters.h"
#include "src/core/call/call_finalization.h"
#include "src/core/call/message.h"
#include "src/core/call/metadata.h"
#include "src/core/call/metadata_batch.h"
#include "src/core/filter/blackboard.h"
#include "src/core/filter/filter_args.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/channel/channel_fwd.h"
#include "src/core/lib/channel/channel_stack.h"
#include "src/core/lib/event_engine/default_event_engine.h"
#include "src/core/lib/event_engine/event_engine_context.h"  // IWYU pragma: keep
#include "src/core/lib/iomgr/call_combiner.h"
#include "src/core/lib/iomgr/closure.h"
#include "src/core/lib/iomgr/error.h"
#include "src/core/lib/iomgr/exec_ctx.h"
#include "src/core/lib/iomgr/polling_entity.h"
#include "src/core/lib/promise/activity.h"
#include "src/core/lib/promise/arena_promise.h"
#include "src/core/lib/promise/cancel_callback.h"
#include "src/core/lib/promise/context.h"
#include "src/core/lib/promise/detail/promise_like.h"
#include "src/core/lib/promise/pipe.h"
#include "src/core/lib/promise/poll.h"
#include "src/core/lib/promise/promise.h"
#include "src/core/lib/promise/race.h"
#include "src/core/lib/promise/try_seq.h"
#include "src/core/lib/resource_quota/arena.h"
#include "src/core/lib/slice/slice_buffer.h"
#include "src/core/lib/surface/call.h"
#include "src/core/lib/transport/error_utils.h"
#include "src/core/lib/transport/transport.h"
#include "src/core/util/debug_location.h"
#include "src/core/util/match.h"
#include "src/core/util/time.h"

namespace grpc_core {

class ChannelFilter {
 public:
  using Args = FilterArgs;

  // Perform post-initialization step (if any).
  virtual void PostInit() {}

  // Construct a promise for one call.
  virtual ArenaPromise<ServerMetadataHandle> MakeCallPromise(
      CallArgs call_args, NextPromiseFactory next_promise_factory) = 0;

  // Start a legacy transport op
  // Return true if the op was handled, false if it should be passed to the
  // next filter.
  // TODO(ctiller): design a new API for this - we probably don't want big op
  // structures going forward.
  virtual bool StartTransportOp(grpc_transport_op*) { return false; }

  // Perform a legacy get info call
  // Return true if the op was handled, false if it should be passed to the
  // next filter.
  // TODO(ctiller): design a new API for this
  virtual bool GetChannelInfo(const grpc_channel_info*) { return false; }

  virtual ~ChannelFilter() = default;
};

namespace promise_filter_detail {

// Determine if a list of interceptors has any that need to asynchronously error
// the promise. If so, we need to allocate a latch for the generated promise for
// the original promise stack polyfill code that's generated.

template <typename T>
using EnableIfPromise = std::enable_if_t<std::is_invocable_v<T>, void>;

template <typename R, typename Ignored = void>
struct HasAsyncErrorInterceptor;

template <>
struct HasAsyncErrorInterceptor<void> {
  static constexpr bool value = false;
};

template <>
struct HasAsyncErrorInterceptor<const NoInterceptor*> {
  static constexpr bool value = false;
};

template <typename T, typename A0, typename... As>
struct HasAsyncErrorInterceptor<A0 (T::*)(A0, As...)> {
  static constexpr bool value = false;
};

template <typename T, typename... A>
struct HasAsyncErrorInterceptor<absl::Status (T::*)(A...)> {
  static constexpr bool value = true;
};

template <typename R, typename T, typename... A>
struct HasAsyncErrorInterceptor<absl::StatusOr<R> (T::*)(A...)> {
  static constexpr bool value = true;
};

template <typename T, typename... A>
struct HasAsyncErrorInterceptor<ServerMetadataHandle (T::*)(A...)> {
  static constexpr bool value = true;
};

template <typename T, typename... A>
struct HasAsyncErrorInterceptor<void (T::*)(A...)> {
  static constexpr bool value = false;
};

template <typename T, typename Promise, typename... A>
struct HasAsyncErrorInterceptor<Promise (T::*)(A...),
                                EnableIfPromise<Promise>> {
  // TODO(vigneshbabu): - For fused filters, this should be the OR of
  // HasAsyncErrorInterceptor of all constituent filters.
  static constexpr bool value = true;
};

// For the list case we do two interceptors to avoid amiguities with the single
// argument forms above.
template <typename... Interceptors>
inline constexpr bool HasAnyAsyncErrorInterceptor(Interceptors...) {
  return (HasAsyncErrorInterceptor<Interceptors>::value || ...);
}

// value is true if Derived has a member called IsFused.
template <typename Derived>
struct IsFusedFilter {
  template <typename V>
  static std::true_type test(decltype(&V::IsFused));  // SFINAE context
  template <typename V>
  static std::false_type test(...);

  using type = decltype(test<Derived>(nullptr));
  static constexpr bool value = std::is_same_v<type, std::true_type>;
};

template <typename Derived, typename Ignored = void>
struct CallHasAsyncErrorInterceptor;

// Composite for a given channel type to determine if any of its interceptors
// fall into this category: later code should use this.
template <typename Derived>
struct CallHasAsyncErrorInterceptor<
    Derived, std::enable_if_t<!IsFusedFilter<Derived>::value>> {
  static constexpr bool value =
      HasAnyAsyncErrorInterceptor(&Derived::Call::OnClientToServerMessage,
                                  &Derived::Call::OnServerInitialMetadata,
                                  &Derived::Call::OnServerToClientMessage);
};

template <typename Derived>
struct CallHasAsyncErrorInterceptor<
    Derived, std::enable_if_t<IsFusedFilter<Derived>::value>> {
  static constexpr bool value = Derived::FusedFilterHasAsyncErrorInterceptor();
};

// Given a boolean X export a type:
// either T if X is true
// or an empty type if it is false
template <typename T, bool X>
struct TypeIfNeeded;

template <typename T>
struct TypeIfNeeded<T, false> {
  struct Type {
    Type() = default;
    template <typename Whatever>
    explicit Type(Whatever) : Type() {}
  };
};

template <typename T>
struct TypeIfNeeded<T, true> {
  using Type = T;
};

// For the original promise scheme polyfill:
// If a set of interceptors might fail asynchronously, wrap the main
// promise in a race with the cancellation latch.
// If not, just return the main promise.
template <bool X>
struct RaceAsyncCompletion;

template <>
struct RaceAsyncCompletion<false> {
  template <typename Promise>
  static Promise Run(Promise x, void*) {
    return x;
  }
};

template <>
struct RaceAsyncCompletion<true> {
  template <typename Promise>
  static auto Run(Promise x, Latch<ServerMetadataHandle>* latch) {
    return Race(latch->Wait(), std::move(x));
  }
};

// Zero-member wrapper to make sure that Call always has a constructor
// that takes a channel pointer (even if it's thrown away)
template <typename Derived, typename SfinaeVoid = void>
class CallWrapper;

template <typename Derived>
class CallWrapper<Derived, absl::void_t<decltype(typename Derived::Call(
                               std::declval<Derived*>()))>>
    : public Derived::Call {
 public:
  explicit CallWrapper(Derived* channel) : Derived::Call(channel) {}
};

template <typename Derived>
class CallWrapper<Derived, absl::void_t<decltype(typename Derived::Call())>>
    : public Derived::Call {
 public:
  explicit CallWrapper(Derived*) : Derived::Call() {}
};

// For the original promise scheme polyfill: data associated with once call.
template <typename Derived>
struct FilterCallData {
  explicit FilterCallData(Derived* channel) : call(channel), channel(channel) {}
  GPR_NO_UNIQUE_ADDRESS CallWrapper<Derived> call;
  GPR_NO_UNIQUE_ADDRESS
  typename TypeIfNeeded<Latch<ServerMetadataHandle>,
                        CallHasAsyncErrorInterceptor<Derived>::value>::Type
      error_latch;
  GPR_NO_UNIQUE_ADDRESS
  typename TypeIfNeeded<
      Derived*, filters_detail::CallHasChannelAccess<Derived>()>::Type channel;
};

template <typename Promise>
auto MapResult(const NoInterceptor*, Promise x, void*) {
  return x;
}

template <typename Promise, typename Derived>
auto MapResult(absl::Status (Derived::Call::*fn)(ServerMetadata&), Promise x,
               FilterCallData<Derived>* call_data) {
  DCHECK(fn == &Derived::Call::OnServerTrailingMetadata);
  return OnCancel(Map(std::move(x),
                      [call_data](ServerMetadataHandle md) {
                        auto status =
                            call_data->call.OnServerTrailingMetadata(*md);
                        if (!status.ok()) {
                          return ServerMetadataFromStatus(status);
                        }
                        return md;
                      }),
                  [call_data]() {
                    grpc_metadata_batch b;
                    b.Set(GrpcStatusMetadata(), GRPC_STATUS_CANCELLED);
                    b.Set(GrpcCallWasCancelled(), true);
                    call_data->call.OnServerTrailingMetadata(b).IgnoreError();
                  });
}

template <typename Promise, typename Derived>
auto MapResult(void (Derived::Call::*fn)(ServerMetadata&), Promise x,
               FilterCallData<Derived>* call_data) {
  DCHECK(fn == &Derived::Call::OnServerTrailingMetadata);
  return OnCancel(Map(std::move(x),
                      [call_data](ServerMetadataHandle md) {
                        call_data->call.OnServerTrailingMetadata(*md);
                        return md;
                      }),
                  [call_data]() {
                    grpc_metadata_batch b;
                    b.Set(GrpcStatusMetadata(), GRPC_STATUS_CANCELLED);
                    b.Set(GrpcCallWasCancelled(), true);
                    call_data->call.OnServerTrailingMetadata(b);
                  });
}

template <typename Promise, typename Derived>
auto MapResult(void (Derived::Call::*fn)(ServerMetadata&, Derived*), Promise x,
               FilterCallData<Derived>* call_data) {
  DCHECK(fn == &Derived::Call::OnServerTrailingMetadata);
  return OnCancel(
      Map(std::move(x),
          [call_data](ServerMetadataHandle md) {
            call_data->call.OnServerTrailingMetadata(*md, call_data->channel);
            return md;
          }),
      [call_data]() {
        grpc_metadata_batch b;
        b.Set(GrpcStatusMetadata(), GRPC_STATUS_CANCELLED);
        b.Set(GrpcCallWasCancelled(), true);
        call_data->call.OnServerTrailingMetadata(b, call_data->channel);
      });
}

// For fused filters whose OnServerTrailingMetadata takes pointer to the
// channel.
template <typename P, typename Call, typename Derived,
          typename = std::enable_if_t<IsFusedFilter<Derived>::value>>
auto MapResult(void (Call::*fn)(ServerMetadata&, Derived*), P x,
               FilterCallData<Derived>* call_data) {
  DCHECK(fn == &Derived::Call::OnServerTrailingMetadata);
  return OnCancel(
      Map(std::move(x),
          [call_data](ServerMetadataHandle md) {
            call_data->call.OnServerTrailingMetadata(*md, call_data->channel);
            return md;
          }),
      [call_data]() {
        grpc_metadata_batch b;
        b.Set(GrpcStatusMetadata(), GRPC_STATUS_CANCELLED);
        b.Set(GrpcCallWasCancelled(), true);
        call_data->call.OnServerTrailingMetadata(b, call_data->channel);
      });
}

// For fused filters whose OnServerTrailingMetadata does not take pointer to the
// channel.
template <typename P, typename Call, typename Derived,
          typename = std::enable_if_t<IsFusedFilter<Derived>::value>>
auto MapResult(void (Call::*fn)(ServerMetadata&), P x,
               FilterCallData<Derived>* call_data) {
  DCHECK(fn == &Derived::Call::OnServerTrailingMetadata);
  return OnCancel(Map(std::move(x),
                      [call_data](ServerMetadataHandle md) {
                        call_data->call.OnServerTrailingMetadata(*md);
                        return md;
                      }),
                  [call_data]() {
                    grpc_metadata_batch b;
                    b.Set(GrpcStatusMetadata(), GRPC_STATUS_CANCELLED);
                    b.Set(GrpcCallWasCancelled(), true);
                    call_data->call.OnServerTrailingMetadata(b);
                  });
}

template <typename T, typename A>
constexpr bool IsSameExcludingCVRef =
    std::is_same<promise_detail::RemoveCVRef<A>, T>::value;

template <typename T, typename A>
using EnableIfSameExcludingCVRef =
    std::enable_if_t<std::is_same<promise_detail::RemoveCVRef<A>, T>::value,
                     void>;

template <typename T, typename A>
using EnableIfNotSameExcludingCVRef =
    std::enable_if_t<!std::is_same<promise_detail::RemoveCVRef<A>, T>::value,
                     void>;

template <typename Interceptor, typename Derived, typename SfinaeVoid = void>
struct RunCallImpl;

template <typename Derived>
struct RunCallImpl<const NoInterceptor*, Derived> {
  static auto Run(CallArgs call_args, NextPromiseFactory next_promise_factory,
                  void*) {
    return next_promise_factory(std::move(call_args));
  }
};

template <typename Derived, typename A>
struct RunCallImpl<void (Derived::Call::*)(A md), Derived,
                   EnableIfSameExcludingCVRef<ClientMetadata, A>> {
  static auto Run(CallArgs call_args, NextPromiseFactory next_promise_factory,
                  FilterCallData<Derived>* call_data) {
    call_data->call.OnClientInitialMetadata(*call_args.client_initial_metadata);
    return next_promise_factory(std::move(call_args));
  }
};

template <typename Derived, typename A>
struct RunCallImpl<ServerMetadataHandle (Derived::Call::*)(A md), Derived,
                   EnableIfSameExcludingCVRef<ClientMetadata, A>> {
  static auto Run(CallArgs call_args, NextPromiseFactory next_promise_factory,
                  FilterCallData<Derived>* call_data)
      -> ArenaPromise<ServerMetadataHandle> {
    auto return_md = call_data->call.OnClientInitialMetadata(
        *call_args.client_initial_metadata);
    if (return_md == nullptr) return next_promise_factory(std::move(call_args));
    return Immediate(std::move(return_md));
  }
};

template <typename Derived, typename A>
struct RunCallImpl<ServerMetadataHandle (Derived::Call::*)(A md,
                                                           Derived* channel),
                   Derived, EnableIfSameExcludingCVRef<ClientMetadata, A>> {
  static auto Run(CallArgs call_args, NextPromiseFactory next_promise_factory,
                  FilterCallData<Derived>* call_data)
      -> ArenaPromise<ServerMetadataHandle> {
    auto return_md = call_data->call.OnClientInitialMetadata(
        *call_args.client_initial_metadata, call_data->channel);
    if (return_md == nullptr) return next_promise_factory(std::move(call_args));
    return Immediate(std::move(return_md));
  }
};

template <typename Derived, typename A>
struct RunCallImpl<absl::Status (Derived::Call::*)(A md), Derived,
                   EnableIfSameExcludingCVRef<ClientMetadata, A>> {
  static auto Run(CallArgs call_args, NextPromiseFactory next_promise_factory,
                  FilterCallData<Derived>* call_data)
      -> ArenaPromise<ServerMetadataHandle> {
    auto status = call_data->call.OnClientInitialMetadata(
        *call_args.client_initial_metadata);
    if (status.ok()) return next_promise_factory(std::move(call_args));
    return Immediate(ServerMetadataFromStatus(status));
  }
};

template <typename Derived, typename A>
struct RunCallImpl<absl::Status (Derived::Call::*)(A md, Derived* channel),
                   Derived, EnableIfSameExcludingCVRef<ClientMetadata, A>> {
  static auto Run(CallArgs call_args, NextPromiseFactory next_promise_factory,
                  FilterCallData<Derived>* call_data)
      -> ArenaPromise<ServerMetadataHandle> {
    auto status = call_data->call.OnClientInitialMetadata(
        *call_args.client_initial_metadata, call_data->channel);
    if (status.ok()) return next_promise_factory(std::move(call_args));
    return Immediate(ServerMetadataFromStatus(status));
  }
};

template <typename Derived, typename A>
struct RunCallImpl<void (Derived::Call::*)(A md, Derived* channel), Derived,
                   EnableIfSameExcludingCVRef<ClientMetadata, A>> {
  static auto Run(CallArgs call_args, NextPromiseFactory next_promise_factory,
                  FilterCallData<Derived>* call_data) {
    call_data->call.OnClientInitialMetadata(*call_args.client_initial_metadata,
                                            call_data->channel);
    return next_promise_factory(std::move(call_args));
  }
};

template <typename T, typename AnyType = void>
struct StatusCastExistsToServerMetadataHandle {
  static constexpr bool value = false;
};
template <typename T>
struct StatusCastExistsToServerMetadataHandle<
    T,
    absl::void_t<decltype(StatusCast<ServerMetadataHandle>(std::declval<T>))>> {
  static constexpr bool value = true;
};

template <typename Derived, typename A, typename Promise>
struct RunCallImpl<Promise (Derived::Call::*)(A, Derived* channel), Derived,
                   std::enable_if_t<StatusCastExistsToServerMetadataHandle<
                                        PromiseResult<Promise>>::value &&
                                    IsSameExcludingCVRef<ClientMetadata, A>>> {
  static auto Run(CallArgs call_args, NextPromiseFactory next_promise_factory,
                  FilterCallData<Derived>* call_data) {
    ClientMetadata& md_ref = *call_args.client_initial_metadata;
    return TrySeq(
        call_data->call.OnClientInitialMetadata(md_ref, call_data->channel),
        [call_args = std::move(call_args),
         next_promise_factory = std::move(next_promise_factory)]() mutable {
          return next_promise_factory(std::move(call_args));
        });
  }
};

template <typename Derived, typename Promise>
struct RunCallImpl<
    Promise (Derived::Call::*)(ClientMetadataHandle md, Derived* channel),
    Derived,
    std::enable_if_t<std::is_same_v<absl::StatusOr<ClientMetadataHandle>,
                                    PromiseResult<Promise>>>> {
  static auto Run(CallArgs call_args, NextPromiseFactory next_promise_factory,
                  FilterCallData<Derived>* call_data) {
    ClientMetadataHandle md = std::move(call_args.client_initial_metadata);
    return TrySeq(call_data->call.OnClientInitialMetadata(std::move(md),
                                                          call_data->channel),
                  [call_args = std::move(call_args),
                   next_promise_factory = std::move(next_promise_factory)](
                      ClientMetadataHandle md) mutable {
                    call_args.client_initial_metadata = std::move(md);
                    return next_promise_factory(std::move(call_args));
                  });
  }
};

template <typename Derived, typename Call, typename Promise>
struct RunCallImpl<
    Promise (Call::*)(ClientMetadataHandle md, Derived* channel), Derived,
    std::enable_if_t<std::is_same_v<PromiseResult<Promise>,
                                    ServerMetadataOrHandle<ClientMetadata>>,
                     void>> {
  static auto Run(CallArgs call_args, NextPromiseFactory next_promise_factory,
                  FilterCallData<Derived>* call_data) {
    ClientMetadataHandle md = std::move(call_args.client_initial_metadata);
    return TrySeq(call_data->call.OnClientInitialMetadata(std::move(md),
                                                          call_data->channel),
                  [call_args = std::move(call_args),
                   next_promise_factory = std::move(next_promise_factory)](
                      ClientMetadataHandle md) mutable {
                    call_args.client_initial_metadata = std::move(md);
                    return next_promise_factory(std::move(call_args));
                  });
  }
};

template <typename Interceptor, typename Derived>
auto RunCall(Interceptor interceptor, CallArgs call_args,
             NextPromiseFactory next_promise_factory,
             FilterCallData<Derived>* call_data) {
  DCHECK(interceptor == &Derived::Call::OnClientInitialMetadata);
  return RunCallImpl<Interceptor, Derived>::Run(
      std::move(call_args), std::move(next_promise_factory), call_data);
}

template <typename Derived, typename MethodType, typename Ignored = void>
class InterceptClientToServerMessageHandler;

template <typename Derived>
class InterceptClientToServerMessageHandler<Derived, const NoInterceptor*> {
 public:
  explicit InterceptClientToServerMessageHandler(FilterCallData<Derived>*,
                                                 const CallArgs&) {}
  auto operator()() {
    return [](MessageHandle msg) -> std::optional<MessageHandle> {
      return std::move(msg);
    };
  }
};

template <typename Derived, typename MessageType>
class InterceptClientToServerMessageHandler<
    Derived, void (Derived::Call::*)(MessageType),
    EnableIfSameExcludingCVRef<Message, MessageType>> {
 public:
  explicit InterceptClientToServerMessageHandler(
      FilterCallData<Derived>* call_data, const CallArgs&)
      : call_data_(call_data) {}
  auto operator()() {
    return [call_data =
                call_data_](MessageHandle msg) -> std::optional<MessageHandle> {
      call_data->call.OnClientToServerMessage(*msg);
      return std::move(msg);
    };
  }

 private:
  FilterCallData<Derived>* call_data_;
};

template <typename Derived, typename MessageType>
class InterceptClientToServerMessageHandler<
    Derived, ServerMetadataHandle (Derived::Call::*)(MessageType),
    EnableIfSameExcludingCVRef<Message, MessageType>> {
 public:
  explicit InterceptClientToServerMessageHandler(
      FilterCallData<Derived>* call_data, const CallArgs&)
      : call_data_(call_data) {}

  auto operator()() {
    return [call_data =
                call_data_](MessageHandle msg) -> std::optional<MessageHandle> {
      auto return_md = call_data->call.OnClientToServerMessage(*msg);
      if (return_md == nullptr) return std::move(msg);
      if (call_data->error_latch.is_set()) return std::nullopt;
      call_data->error_latch.Set(std::move(return_md));
      return std::nullopt;
    };
  }

 private:
  FilterCallData<Derived>* call_data_;
};

template <typename Derived, typename MessageType>
class InterceptClientToServerMessageHandler<
    Derived, ServerMetadataHandle (Derived::Call::*)(MessageType, Derived*),
    EnableIfSameExcludingCVRef<Message, MessageType>> {
 public:
  explicit InterceptClientToServerMessageHandler(
      FilterCallData<Derived>* call_data, const CallArgs&)
      : call_data_(call_data) {}

  auto operator()() {
    return [call_data =
                call_data_](MessageHandle msg) -> std::optional<MessageHandle> {
      auto return_md =
          call_data->call.OnClientToServerMessage(*msg, call_data->channel);
      if (return_md == nullptr) return std::move(msg);
      if (call_data->error_latch.is_set()) return std::nullopt;
      call_data->error_latch.Set(std::move(return_md));
      return std::nullopt;
    };
  }

 private:
  FilterCallData<Derived>* call_data_;
};

template <typename Derived, typename MessageType>
class InterceptClientToServerMessageHandler<
    Derived, void (Derived::Call::*)(MessageType, Derived*),
    EnableIfSameExcludingCVRef<Message, MessageType>> {
 public:
  explicit InterceptClientToServerMessageHandler(
      FilterCallData<Derived>* call_data, const CallArgs&)
      : call_data_(call_data) {}

  auto operator()() {
    return [call_data =
                call_data_](MessageHandle msg) -> std::optional<MessageHandle> {
      call_data->call.OnClientToServerMessage(*msg, call_data->channel);
      return std::move(msg);
    };
  }

 private:
  FilterCallData<Derived>* call_data_;
};

template <typename Derived>
class InterceptClientToServerMessageHandler<
    Derived, MessageHandle (Derived::Call::*)(MessageHandle, Derived*)> {
 public:
  explicit InterceptClientToServerMessageHandler(
      FilterCallData<Derived>* call_data, const CallArgs&)
      : call_data_(call_data) {}

  auto operator()() {
    return [call_data =
                call_data_](MessageHandle msg) -> std::optional<MessageHandle> {
      return call_data->call.OnClientToServerMessage(std::move(msg),
                                                     call_data->channel);
    };
  }

 private:
  FilterCallData<Derived>* call_data_;
};

template <typename Derived>
class InterceptClientToServerMessageHandler<
    Derived,
    absl::StatusOr<MessageHandle> (Derived::Call::*)(MessageHandle, Derived*)> {
 public:
  explicit InterceptClientToServerMessageHandler(
      FilterCallData<Derived>* call_data, const CallArgs&)
      : call_data_(call_data) {}

  auto operator()() {
    return [call_data =
                call_data_](MessageHandle msg) -> std::optional<MessageHandle> {
      auto r = call_data->call.OnClientToServerMessage(std::move(msg),
                                                       call_data->channel);
      if (r.ok()) return std::move(*r);
      if (call_data->error_latch.is_set()) return std::nullopt;
      call_data->error_latch.Set(ServerMetadataFromStatus(r.status()));
      return std::nullopt;
    };
  }

 private:
  FilterCallData<Derived>* call_data_;
};

// For handling fused filters with a promise return type. This specialization is
// only for fused filters not requiring a channel pointer.
template <typename Derived, typename R, typename Call>
class InterceptClientToServerMessageHandler<Derived, R (Call::*)(MessageHandle),
                                            EnableIfPromise<R>> {
 public:
  explicit InterceptClientToServerMessageHandler(
      FilterCallData<Derived>* call_data, const CallArgs&)
      : call_data_(call_data) {}

  auto operator()() {
    return [call_data = call_data_](MessageHandle msg) {
      return Map(call_data->call.OnClientToServerMessage(std::move(msg)),
                 [call_data](ServerMetadataOrHandle<Message> md)
                     -> std::optional<MessageHandle> {
                   if (!md.ok()) {
                     call_data->error_latch.Set(std::move(md).TakeMetadata());
                     return std::nullopt;
                   }
                   return std::move(md).TakeValue();
                 });
    };
  }

 private:
  FilterCallData<Derived>* call_data_;
};

// For handling fused filters with a promise return type. This specialization is
// only for fused filters requiring a channel pointer.
template <typename Derived, typename Call, typename R>
class InterceptClientToServerMessageHandler<
    Derived, R (Call::*)(MessageHandle, Derived*), EnableIfPromise<R>> {
 public:
  explicit InterceptClientToServerMessageHandler(
      FilterCallData<Derived>* call_data, const CallArgs&)
      : call_data_(call_data) {}

  auto operator()() {
    return [call_data = call_data_](MessageHandle msg) {
      return Map(call_data->call.OnClientToServerMessage(std::move(msg),
                                                         call_data->channel),
                 [call_data](ServerMetadataOrHandle<Message> md)
                     -> std::optional<MessageHandle> {
                   if (!md.ok()) {
                     call_data->error_latch.Set(std::move(md).TakeMetadata());
                     return std::nullopt;
                   }
                   return std::move(md).TakeValue();
                 });
    };
  }

 private:
  FilterCallData<Derived>* call_data_;
};

template <typename HookFn, typename HalfCloseFn, typename Derived,
          typename Ignored = void>
struct InterceptClientToServerMessage;

template <typename Derived>
struct InterceptClientToServerMessage<const NoInterceptor*,
                                      const NoInterceptor*, Derived> {
  static inline void Run(FilterCallData<Derived>*, const CallArgs&) {}
};

template <typename HookFn, typename Call, typename Derived>
struct InterceptClientToServerMessage<
    HookFn, void (Call::*)(), Derived,
    std::enable_if_t<!IsSameExcludingCVRef<HookFn, NoInterceptor*>, void>> {
  static inline void Run(FilterCallData<Derived>* call_data,
                         const CallArgs& call_args) {
    call_args.client_to_server_messages->InterceptAndMapWithHalfClose(
        InterceptClientToServerMessageHandler<Derived, HookFn>(call_data,
                                                               call_args)(),
        [call_data]() { call_data->call.OnClientToServerHalfClose(); });
  }
};

template <typename HookFn, typename Derived>
struct InterceptClientToServerMessage<
    HookFn, const NoInterceptor*, Derived,
    std::enable_if_t<!IsSameExcludingCVRef<HookFn, NoInterceptor*>, void>> {
  static inline void Run(FilterCallData<Derived>* call_data,
                         const CallArgs& call_args) {
    call_args.client_to_server_messages->InterceptAndMap(
        InterceptClientToServerMessageHandler<Derived, HookFn>(call_data,
                                                               call_args)());
  }
};

template <typename Derived, typename MethodType, typename Ignored = void>
struct InterceptServerInitialMetadata;

template <typename Derived>
struct InterceptServerInitialMetadata<Derived, const NoInterceptor*> {
  static inline void Run(FilterCallData<Derived>*, const CallArgs&) {}
};

template <typename Derived>
struct InterceptServerInitialMetadata<Derived, void (Derived::Call::*)(
                                                   ServerMetadata&)> {
  static inline void Run(FilterCallData<Derived>* call_data,
                         const CallArgs& call_args) {
    call_args.server_initial_metadata->InterceptAndMap(
        [call_data](ServerMetadataHandle md) {
          call_data->call.OnServerInitialMetadata(*md);
          return md;
        });
  }
};

template <typename Derived>
struct InterceptServerInitialMetadata<Derived, absl::Status (Derived::Call::*)(
                                                   ServerMetadata&)> {
  static inline void Run(FilterCallData<Derived>* call_data,
                         const CallArgs& call_args) {
    call_args.server_initial_metadata->InterceptAndMap(
        [call_data](
            ServerMetadataHandle md) -> std::optional<ServerMetadataHandle> {
          auto status = call_data->call.OnServerInitialMetadata(*md);
          if (!status.ok() && !call_data->error_latch.is_set()) {
            call_data->error_latch.Set(ServerMetadataFromStatus(status));
            return std::nullopt;
          }
          return std::move(md);
        });
  }
};

template <typename Derived>
struct InterceptServerInitialMetadata<Derived, void (Derived::Call::*)(
                                                   ServerMetadata&, Derived*)> {
  static inline void Run(FilterCallData<Derived>* call_data,
                         const CallArgs& call_args) {
    call_args.server_initial_metadata->InterceptAndMap(
        [call_data](ServerMetadataHandle md) {
          call_data->call.OnServerInitialMetadata(*md, call_data->channel);
          return md;
        });
  }
};

template <typename Derived>
struct InterceptServerInitialMetadata<Derived, absl::Status (Derived::Call::*)(
                                                   ServerMetadata&, Derived*)> {
  static inline void Run(FilterCallData<Derived>* call_data,
                         const CallArgs& call_args) {
    call_args.server_initial_metadata->InterceptAndMap(
        [call_data](
            ServerMetadataHandle md) -> std::optional<ServerMetadataHandle> {
          auto status =
              call_data->call.OnServerInitialMetadata(*md, call_data->channel);
          if (!status.ok() && !call_data->error_latch.is_set()) {
            call_data->error_latch.Set(ServerMetadataFromStatus(status));
            return std::nullopt;
          }
          return std::move(md);
        });
  }
};

// For handling fused filters with a promise return type. This specialization is
// only for fused filters requiring a channel pointer.
template <typename Derived, typename Call, typename R>
struct InterceptServerInitialMetadata<
    Derived, R (Call::*)(ServerMetadataHandle, Derived*), EnableIfPromise<R>> {
  static inline void Run(FilterCallData<Derived>* call_data,
                         const CallArgs& call_args) {
    call_args.server_initial_metadata->InterceptAndMap(
        [call_data](ServerMetadataHandle md) {
          return Map(
              call_data->call.OnServerInitialMetadata(std::move(md),
                                                      call_data->channel),
              [call_data](ServerMetadataOrHandle<ServerMetadata> md)
                  -> std::optional<ServerMetadataHandle> {
                if (!md.ok()) {
                  call_data->error_latch.Set(std::move(md).TakeMetadata());
                  return std::nullopt;
                }
                return std::move(md).TakeValue();
              });
        });
  }
};

// For handling fused filters with a promise return type. This specialization is
// only for fused filters not requiring a channel pointer.
template <typename Derived, typename Call, typename R>
struct InterceptServerInitialMetadata<
    Derived, R (Call::*)(ServerMetadataHandle), EnableIfPromise<R>> {
  static inline void Run(FilterCallData<Derived>* call_data,
                         const CallArgs& call_args) {
    call_args.server_initial_metadata->InterceptAndMap(
        [call_data](ServerMetadataHandle md) {
          return Map(
              call_data->call.OnServerInitialMetadata(std::move(md)),
              [call_data](ServerMetadataOrHandle<ServerMetadata> md)
                  -> std::optional<ServerMetadataHandle> {
                if (!md.ok()) {
                  call_data->error_latch.Set(std::move(md).TakeMetadata());
                  return std::nullopt;
                }
                return std::move(md).TakeValue();
              });
        });
  }
};

template <typename Derived, typename MethodType, typename Ignored = void>
struct InterceptServerToClientMessage;

template <typename Derived>
struct InterceptServerToClientMessage<Derived, const NoInterceptor*> {
  static inline void Run(FilterCallData<Derived>*, const CallArgs&) {}
};

template <typename Derived>
struct InterceptServerToClientMessage<Derived,
                                      void (Derived::Call::*)(const Message&)> {
  static inline void Run(FilterCallData<Derived>* call_data,
                         const CallArgs& call_args) {
    call_args.server_to_client_messages->InterceptAndMap(
        [call_data](MessageHandle msg) -> std::optional<MessageHandle> {
          call_data->call.OnServerToClientMessage(*msg);
          return std::move(msg);
        });
  }
};

template <typename Derived>
struct InterceptServerToClientMessage<Derived,
                                      void (Derived::Call::*)(Message&)> {
  static inline void Run(FilterCallData<Derived>* call_data,
                         const CallArgs& call_args) {
    call_args.server_to_client_messages->InterceptAndMap(
        [call_data](MessageHandle msg) -> std::optional<MessageHandle> {
          call_data->call.OnServerToClientMessage(*msg);
          return std::move(msg);
        });
  }
};

template <typename Derived>
struct InterceptServerToClientMessage<
    Derived, ServerMetadataHandle (Derived::Call::*)(const Message&)> {
  static inline void Run(FilterCallData<Derived>* call_data,
                         const CallArgs& call_args) {
    call_args.server_to_client_messages->InterceptAndMap(
        [call_data](MessageHandle msg) -> std::optional<MessageHandle> {
          auto return_md = call_data->call.OnServerToClientMessage(*msg);
          if (return_md == nullptr) return std::move(msg);
          if (call_data->error_latch.is_set()) return std::nullopt;
          call_data->error_latch.Set(std::move(return_md));
          return std::nullopt;
        });
  }
};

template <typename Derived>
struct InterceptServerToClientMessage<Derived, void (Derived::Call::*)(
                                                   const Message&, Derived*)> {
  static inline void Run(FilterCallData<Derived>* call_data,
                         const CallArgs& call_args) {
    call_args.server_to_client_messages->InterceptAndMap(
        [call_data](MessageHandle msg) -> std::optional<MessageHandle> {
          call_data->call.OnServerToClientMessage(*msg, call_data->channel);
          return std::move(msg);
        });
  }
};

template <typename Derived>
struct InterceptServerToClientMessage<Derived,
                                      ServerMetadataHandle (Derived::Call::*)(
                                          const Message&, Derived*)> {
  static inline void Run(FilterCallData<Derived>* call_data,
                         const CallArgs& call_args) {
    call_args.server_to_client_messages->InterceptAndMap(
        [call_data](MessageHandle msg) -> std::optional<MessageHandle> {
          auto return_md =
              call_data->call.OnServerToClientMessage(*msg, call_data->channel);
          if (return_md == nullptr) return std::move(msg);
          if (call_data->error_latch.is_set()) return std::nullopt;
          call_data->error_latch.Set(std::move(return_md));
          return std::nullopt;
        });
  }
};

template <typename Derived>
struct InterceptServerToClientMessage<Derived, MessageHandle (Derived::Call::*)(
                                                   MessageHandle, Derived*)> {
  static inline void Run(FilterCallData<Derived>* call_data,
                         const CallArgs& call_args) {
    call_args.server_to_client_messages->InterceptAndMap(
        [call_data](MessageHandle msg) -> std::optional<MessageHandle> {
          return call_data->call.OnServerToClientMessage(std::move(msg),
                                                         call_data->channel);
        });
  }
};

template <typename Derived>
struct InterceptServerToClientMessage<Derived, absl::StatusOr<MessageHandle> (
                                                   Derived::Call::*)(
                                                   MessageHandle, Derived*)> {
  static inline void Run(FilterCallData<Derived>* call_data,
                         const CallArgs& call_args) {
    call_args.server_to_client_messages->InterceptAndMap(
        [call_data](MessageHandle msg) -> std::optional<MessageHandle> {
          auto r = call_data->call.OnServerToClientMessage(std::move(msg),
                                                           call_data->channel);
          if (r.ok()) return std::move(*r);
          if (call_data->error_latch.is_set()) return std::nullopt;
          call_data->error_latch.Set(ServerMetadataFromStatus(r.status()));
          return std::nullopt;
        });
  }
};

// For handling fused filters with a promise return type. This specialization is
// only for fused filters not requiring a channel pointer.
template <typename Derived, typename Call, typename R>
struct InterceptServerToClientMessage<Derived, R (Call::*)(MessageHandle),
                                      EnableIfPromise<R>> {
  static inline void Run(FilterCallData<Derived>* call_data,
                         const CallArgs& call_args) {
    call_args.server_to_client_messages->InterceptAndMap(
        [call_data](MessageHandle msg) {
          return Map(
              call_data->call.OnServerToClientMessage(std::move(msg)),
              [call_data](ServerMetadataOrHandle<Message> msg)
                  -> std::optional<MessageHandle> {
                if (!msg.ok()) {
                  call_data->error_latch.Set(std::move(msg).TakeMetadata());
                  return std::nullopt;
                }
                return std::move(msg).TakeValue();
              });
        });
  }
};

// For handling fused filters with a promise return type. This specialization is
// only for fused filters requiring a channel pointer.
template <typename Derived, typename Call, typename R>
struct InterceptServerToClientMessage<
    Derived, R (Call::*)(MessageHandle, Derived*), EnableIfPromise<R>> {
  static inline void Run(FilterCallData<Derived>* call_data,
                         const CallArgs& call_args) {
    call_args.server_to_client_messages->InterceptAndMap(
        [call_data](MessageHandle msg) {
          return Map(
              call_data->call.OnServerToClientMessage(std::move(msg),
                                                      call_data->channel),
              [call_data](ServerMetadataOrHandle<Message> msg)
                  -> std::optional<MessageHandle> {
                if (!msg.ok()) {
                  call_data->error_latch.Set(std::move(msg).TakeMetadata());
                  return std::nullopt;
                }
                return std::move(msg).TakeValue();
              });
        });
  }
};

template <typename Derived, typename MethodType, typename Ignored = void>
struct InterceptFinalize;

template <typename Derived>
struct InterceptFinalize<Derived, const NoInterceptor*> {
  static inline void Run(void*, void*) {}
};

template <typename Derived, typename Call>
struct InterceptFinalize<Derived, void (Call::*)(const grpc_call_final_info*)> {
  static inline void Run(void*, Call* call) {
    GetContext<CallFinalization>()->Add(
        [call](const grpc_call_final_info* final_info) {
          call->OnFinalize(final_info);
        });
  }
};

template <class Derived, typename Call>
struct InterceptFinalize<Derived, void (Call::*)(const grpc_call_final_info*,
                                                 Derived*)> {
  static inline void Run(Derived* channel, Call* call) {
    GetContext<CallFinalization>()->Add(
        [call, channel](const grpc_call_final_info* final_info) {
          call->OnFinalize(final_info, channel);
        });
  }
};

template <typename Derived>
absl::enable_if_t<std::is_empty<FilterCallData<Derived>>::value,
                  FilterCallData<Derived>*>
MakeFilterCall(Derived*) {
  static FilterCallData<Derived> call{nullptr};
  return &call;
}

template <typename Derived>
absl::enable_if_t<!std::is_empty<FilterCallData<Derived>>::value,
                  FilterCallData<Derived>*>
MakeFilterCall(Derived* derived) {
  return GetContext<Arena>()->ManagedNew<FilterCallData<Derived>>(derived);
}

}  // namespace promise_filter_detail

// Base class for promise-based channel filters.
// Eventually this machinery will move elsewhere (the interception logic will
// move directly into the channel stack, and so filters will just directly
// derive from `ChannelFilter`)
//
// Implements new-style call filters, and polyfills them into the previous
// scheme.
//
// Call filters:
// Derived types should declare a class `Call` with the following members:
// - OnClientInitialMetadata  - $VALUE_TYPE = ClientMetadata
// - OnServerInitialMetadata  - $VALUE_TYPE = ServerMetadata
// - OnServerToClientMessage  - $VALUE_TYPE = Message
// - OnClientToServerMessage  - $VALUE_TYPE = Message
// - OnServerTrailingMetadata - $VALUE_TYPE = ServerMetadata
// - OnFinalize               - special, see below
// These members define an interception point for a particular event in
// the call lifecycle.
// The type of these members matters, and is selectable by the class
// author. For $INTERCEPTOR_NAME in the above list:
// - static inline const NoInterceptor $INTERCEPTOR_NAME:
//   defines that this filter does not intercept this event.
//   there is zero runtime cost added to handling that event by this filter.
// - void $INTERCEPTOR_NAME($VALUE_TYPE&):
//   the filter intercepts this event, and can modify the value.
//   it never fails.
// - absl::Status $INTERCEPTOR_NAME($VALUE_TYPE&):
//   the filter intercepts this event, and can modify the value.
//   it can fail, in which case the call will be aborted.
// - ServerMetadataHandle $INTERCEPTOR_NAME($VALUE_TYPE&)
//   the filter intercepts this event, and can modify the value.
//   the filter can return nullptr for success, or a metadata handle for
//   failure (in which case the call will be aborted).
//   useful for cases where the exact metadata returned needs to be customized.
// - void $INTERCEPTOR_NAME($VALUE_TYPE&, Derived*):
//   the filter intercepts this event, and can modify the value.
//   it can access the channel via the second argument.
//   it never fails.
// - absl::Status $INTERCEPTOR_NAME($VALUE_TYPE&, Derived*):
//   the filter intercepts this event, and can modify the value.
//   it can access the channel via the second argument.
//   it can fail, in which case the call will be aborted.
// - ServerMetadataHandle $INTERCEPTOR_NAME($VALUE_TYPE&, Derived*)
//   the filter intercepts this event, and can modify the value.
//   it can access the channel via the second argument.
//   the filter can return nullptr for success, or a metadata handle for
//   failure (in which case the call will be aborted).
//   useful for cases where the exact metadata returned needs to be customized.
// It's also acceptable to return a promise that resolves to the
// relevant return type listed above.
// Finally, OnFinalize can be added to intecept call finalization.
// It must have one of the signatures:
// - static inline const NoInterceptor OnFinalize:
//   the filter does not intercept call finalization.
// - void OnFinalize(const grpc_call_final_info*):
//   the filter intercepts call finalization.
class ImplementChannelFilterTag {};
template <typename Derived>
class ImplementChannelFilter : public ChannelFilter,
                               public ImplementChannelFilterTag {
 public:
  // Polyfill for the original promise scheme.
  // Allows writing v3 filters that work with v2 stacks.
  // (and consequently also v1 stacks since we can polyfill back to that too).
  ArenaPromise<ServerMetadataHandle> MakeCallPromise(
      CallArgs call_args, NextPromiseFactory next_promise_factory) final {
    auto* call = promise_filter_detail::MakeFilterCall<Derived>(
        static_cast<Derived*>(this));
    promise_filter_detail::InterceptClientToServerMessage<
        decltype(&Derived::Call::OnClientToServerMessage),
        decltype(&Derived::Call::OnClientToServerHalfClose),
        Derived>::Run(call, call_args);
    promise_filter_detail::InterceptServerInitialMetadata<
        Derived,
        decltype(&Derived::Call::OnServerInitialMetadata)>::Run(call,
                                                                call_args);
    promise_filter_detail::InterceptServerToClientMessage<
        Derived,
        decltype(&Derived::Call::OnServerToClientMessage)>::Run(call,
                                                                call_args);
    promise_filter_detail::
        InterceptFinalize<Derived, decltype(&Derived::Call::OnFinalize)>::Run(
            static_cast<Derived*>(this),
            static_cast<typename Derived::Call*>(&call->call));
    return promise_filter_detail::MapResult(
        &Derived::Call::OnServerTrailingMetadata,
        promise_filter_detail::RaceAsyncCompletion<
            promise_filter_detail::CallHasAsyncErrorInterceptor<
                Derived>::value>::
            Run(promise_filter_detail::RunCall(
                    &Derived::Call::OnClientInitialMetadata,
                    std::move(call_args), std::move(next_promise_factory),
                    call),
                &call->error_latch),
        call);
  }
};

// Designator for whether a filter is client side or server side.
// Please don't use this outside calls to MakePromiseBasedFilter - it's
// intended to be deleted once the promise conversion is complete.
enum class FilterEndpoint {
  kClient,
  kServer,
};

// Flags for MakePromiseBasedFilter.
static constexpr uint8_t kFilterExaminesServerInitialMetadata = 1;
static constexpr uint8_t kFilterIsLast = 2;
static constexpr uint8_t kFilterExaminesOutboundMessages = 4;
static constexpr uint8_t kFilterExaminesInboundMessages = 8;
static constexpr uint8_t kFilterExaminesCallContext = 16;

namespace promise_filter_detail {

// Proxy channel filter for initialization failure, since we must leave a
// valid filter in place.
class InvalidChannelFilter : public ChannelFilter {
 public:
  ArenaPromise<ServerMetadataHandle> MakeCallPromise(
      CallArgs, NextPromiseFactory) override {
    abort();
  }
};

// Call data shared between all implementations of promise-based filters.
class BaseCallData : public Activity, private Wakeable {
 protected:
  // Hook to allow interception of messages on the send/receive path by
  // PipeSender and PipeReceiver, as appropriate according to whether we're
  // client or server.
  class Interceptor {
   public:
    virtual PipeSender<MessageHandle>* Push() = 0;
    virtual PipeReceiver<MessageHandle>* Pull() = 0;
    virtual PipeReceiver<MessageHandle>* original_receiver() = 0;
    virtual PipeSender<MessageHandle>* original_sender() = 0;
    virtual void GotPipe(PipeSender<MessageHandle>*) = 0;
    virtual void GotPipe(PipeReceiver<MessageHandle>*) = 0;
    virtual ~Interceptor() = default;
  };

  BaseCallData(grpc_call_element* elem, const grpc_call_element_args* args,
               uint8_t flags,
               absl::FunctionRef<Interceptor*()> make_send_interceptor,
               absl::FunctionRef<Interceptor*()> make_recv_interceptor);

 public:
  ~BaseCallData() override;

  void set_pollent(grpc_polling_entity* pollent) {
    CHECK(nullptr == pollent_.exchange(pollent, std::memory_order_release));
  }

  // Activity implementation (partial).
  void Orphan() final;
  Waker MakeNonOwningWaker() final;
  Waker MakeOwningWaker() final;

  std::string ActivityDebugTag(WakeupMask) const override { return DebugTag(); }

  void Finalize(const grpc_call_final_info* final_info) {
    ScopedContext ctx(this);
    finalization_.Run(final_info);
  }

  virtual void StartBatch(grpc_transport_stream_op_batch* batch) = 0;

  Call* call() { return arena_->GetContext<Call>(); }

 protected:
  class ScopedContext : public promise_detail::Context<Arena>,
                        public promise_detail::Context<grpc_polling_entity>,
                        public promise_detail::Context<CallFinalization>,
                        public promise_detail::Context<
                            grpc_event_engine::experimental::EventEngine> {
   public:
    explicit ScopedContext(BaseCallData* call_data)
        : promise_detail::Context<Arena>(call_data->arena_),
          promise_detail::Context<grpc_polling_entity>(
              call_data->pollent_.load(std::memory_order_acquire)),
          promise_detail::Context<CallFinalization>(&call_data->finalization_) {
    }
  };

  class Flusher : public latent_see::InnerScope {
   public:
    explicit Flusher(BaseCallData* call,
                     latent_see::Metadata* desc = GRPC_LATENT_SEE_METADATA(
                         "PromiseBasedFilter::Flusher"));
    // Calls closures, schedules batches, relinquishes call combiner.
    ~Flusher();

    void Resume(grpc_transport_stream_op_batch* batch) {
      CHECK(!call_->is_last());
      if (batch->HasOp()) {
        release_.push_back(batch);
      } else if (batch->on_complete != nullptr) {
        Complete(batch);
      }
    }

    void Cancel(grpc_transport_stream_op_batch* batch,
                grpc_error_handle error) {
      grpc_transport_stream_op_batch_queue_finish_with_failure(batch, error,
                                                               &call_closures_);
    }

    void Complete(grpc_transport_stream_op_batch* batch) {
      call_closures_.Add(batch->on_complete, absl::OkStatus(),
                         "Flusher::Complete");
    }

    void AddClosure(grpc_closure* closure, grpc_error_handle error,
                    const char* reason) {
      call_closures_.Add(closure, error, reason);
    }

    BaseCallData* call() const { return call_; }

   private:
    absl::InlinedVector<grpc_transport_stream_op_batch*, 1> release_;
    CallCombinerClosureList call_closures_;
    BaseCallData* const call_;
  };

  // Smart pointer like wrapper around a batch.
  // Creation makes a ref count of one capture.
  // Copying increments.
  // Must be moved from or resumed or cancelled before destruction.
  class CapturedBatch final {
   public:
    CapturedBatch();
    explicit CapturedBatch(grpc_transport_stream_op_batch* batch);
    ~CapturedBatch();
    CapturedBatch(const CapturedBatch&);
    CapturedBatch& operator=(const CapturedBatch&);
    CapturedBatch(CapturedBatch&&) noexcept;
    CapturedBatch& operator=(CapturedBatch&&) noexcept;

    grpc_transport_stream_op_batch* operator->() { return batch_; }
    bool is_captured() const { return batch_ != nullptr; }

    // Resume processing this batch (releases one ref, passes it down the
    // stack)
    void ResumeWith(Flusher* releaser);
    // Cancel this batch immediately (releases all refs)
    void CancelWith(grpc_error_handle error, Flusher* releaser);
    // Complete this batch (pass it up) assuming refs drop to zero
    void CompleteWith(Flusher* releaser);

    void Swap(CapturedBatch* other) { std::swap(batch_, other->batch_); }

   private:
    grpc_transport_stream_op_batch* batch_;
  };

  static Arena::PoolPtr<grpc_metadata_batch> WrapMetadata(
      grpc_metadata_batch* p) {
    return Arena::PoolPtr<grpc_metadata_batch>(p,
                                               Arena::PooledDeleter(nullptr));
  }

  class ReceiveInterceptor final : public Interceptor {
   public:
    explicit ReceiveInterceptor(Arena* arena) : pipe_{arena} {}

    PipeReceiver<MessageHandle>* original_receiver() override {
      return &pipe_.receiver;
    }
    PipeSender<MessageHandle>* original_sender() override { abort(); }

    void GotPipe(PipeReceiver<MessageHandle>* receiver) override {
      CHECK_EQ(receiver_, nullptr);
      receiver_ = receiver;
    }

    void GotPipe(PipeSender<MessageHandle>*) override { abort(); }

    PipeSender<MessageHandle>* Push() override { return &pipe_.sender; }
    PipeReceiver<MessageHandle>* Pull() override {
      CHECK_NE(receiver_, nullptr);
      return receiver_;
    }

   private:
    Pipe<MessageHandle> pipe_;
    PipeReceiver<MessageHandle>* receiver_ = nullptr;
  };

  class SendInterceptor final : public Interceptor {
   public:
    explicit SendInterceptor(Arena* arena) : pipe_{arena} {}

    PipeReceiver<MessageHandle>* original_receiver() override { abort(); }
    PipeSender<MessageHandle>* original_sender() override {
      return &pipe_.sender;
    }

    void GotPipe(PipeReceiver<MessageHandle>*) override { abort(); }

    void GotPipe(PipeSender<MessageHandle>* sender) override {
      CHECK_EQ(sender_, nullptr);
      sender_ = sender;
    }

    PipeSender<MessageHandle>* Push() override {
      CHECK_NE(sender_, nullptr);
      return sender_;
    }
    PipeReceiver<MessageHandle>* Pull() override { return &pipe_.receiver; }

   private:
    Pipe<MessageHandle> pipe_;
    PipeSender<MessageHandle>* sender_ = nullptr;
  };

  // State machine for sending messages: handles intercepting send_message ops
  // and forwarding them through pipes to the promise, then getting the result
  // down the stack.
  // Split into its own class so that we don't spend the memory instantiating
  // these members for filters that don't need to intercept sent messages.
  class SendMessage {
   public:
    SendMessage(BaseCallData* base, Interceptor* interceptor)
        : base_(base), interceptor_(interceptor) {}
    ~SendMessage() { interceptor_->~Interceptor(); }

    Interceptor* interceptor() { return interceptor_; }

    // Start a send_message op.
    void StartOp(CapturedBatch batch);
    // Publish the outbound pipe to the filter.
    // This happens when the promise requests to call the next filter: until
    // this occurs messages can't be sent as we don't know the pipe that the
    // promise expects to send on.
    template <typename T>
    void GotPipe(T* pipe);
    // Called from client/server polling to do the send message part of the
    // work.
    void WakeInsideCombiner(Flusher* flusher, bool allow_push_to_pipe);
    // Call is completed, we have trailing metadata. Close things out.
    void Done(const ServerMetadata& metadata, Flusher* flusher);
    // Return true if we have a batch captured (for debug logs)
    bool HaveCapturedBatch() const { return batch_.is_captured(); }
    // Return true if we're not actively sending a message.
    bool IsIdle() const;
    // Return true if we've released the message for forwarding down the stack.
    bool IsForwarded() const { return state_ == State::kForwardedBatch; }

   private:
    enum class State : uint8_t {
      // Starting state: no batch started, no outgoing pipe configured.
      kInitial,
      // We have an outgoing pipe, but no batch started.
      // (this is the steady state).
      kIdle,
      // We have a batch started, but no outgoing pipe configured.
      // Stall until we have one.
      kGotBatchNoPipe,
      // We have a batch, and an outgoing pipe. On the next poll we'll push the
      // message into the pipe to the promise.
      kGotBatch,
      // We've pushed a message into the promise, and we're now waiting for it
      // to pop out the other end so we can forward it down the stack.
      kPushedToPipe,
      // We've forwarded a message down the stack, and now we're waiting for
      // completion.
      kForwardedBatch,
      // We've got the completion callback, we'll close things out during poll
      // and then forward completion callbacks up and transition back to idle.
      kBatchCompleted,
      // We're almost done, but need to poll first.
      kCancelledButNotYetPolled,
      // We're done.
      kCancelled,
      // We're done, but we haven't gotten a status yet
      kCancelledButNoStatus,
    };
    static const char* StateString(State);

    void OnComplete(absl::Status status);

    BaseCallData* const base_;
    State state_ = State::kInitial;
    Interceptor* const interceptor_;
    std::optional<PipeSender<MessageHandle>::PushType> push_;
    std::optional<PipeReceiverNextType<MessageHandle>> next_;
    CapturedBatch batch_;
    grpc_closure* intercepted_on_complete_;
    grpc_closure on_complete_ =
        MakeMemberClosure<SendMessage, &SendMessage::OnComplete>(this);
    absl::Status completed_status_;
  };

  // State machine for receiving messages: handles intercepting recv_message
  // ops, forwarding them down the stack, and then publishing the result via
  // pipes to the promise (and ultimately calling the right callbacks for the
  // batch when our promise has completed processing of them).
  // Split into its own class so that we don't spend the memory instantiating
  // these members for filters that don't need to intercept sent messages.
  class ReceiveMessage {
   public:
    ReceiveMessage(BaseCallData* base, Interceptor* interceptor)
        : base_(base), interceptor_(interceptor) {}
    ~ReceiveMessage() { interceptor_->~Interceptor(); }

    Interceptor* interceptor() { return interceptor_; }

    // Start a recv_message op.
    void StartOp(CapturedBatch& batch);
    // Publish the inbound pipe to the filter.
    // This happens when the promise requests to call the next filter: until
    // this occurs messages can't be received as we don't know the pipe that the
    // promise expects to forward them with.
    template <typename T>
    void GotPipe(T* pipe);
    // Called from client/server polling to do the receive message part of the
    // work.
    void WakeInsideCombiner(Flusher* flusher, bool allow_push_to_pipe);
    // Call is completed, we have trailing metadata. Close things out.
    void Done(const ServerMetadata& metadata, Flusher* flusher);

   private:
    enum class State : uint8_t {
      // Starting state: no batch started, no incoming pipe configured.
      kInitial,
      // We have an incoming pipe, but no batch started.
      // (this is the steady state).
      kIdle,
      // We received a batch and forwarded it on, but have not got an incoming
      // pipe configured.
      kForwardedBatchNoPipe,
      // We received a batch and forwarded it on.
      kForwardedBatch,
      // We got the completion for the recv_message, but we don't yet have a
      // pipe configured. Stall until this changes.
      kBatchCompletedNoPipe,
      // We got the completion for the recv_message, and we have a pipe
      // configured: next poll will push the message into the pipe for the
      // filter to process.
      kBatchCompleted,
      // We've pushed a message into the promise, and we're now waiting for it
      // to pop out the other end so we can forward it up the stack.
      kPushedToPipe,
      // We've got a message out of the pipe, now we need to wait for processing
      // to completely quiesce in the promise prior to forwarding the completion
      // up the stack.
      kPulledFromPipe,
      // We're done.
      kCancelled,
      // Call got terminated whilst we were idle: we need to close the sender
      // pipe next poll.
      kCancelledWhilstIdle,
      // Call got terminated whilst we had forwarded a recv_message down the
      // stack: we need to keep track of that until we get the completion so
      // that we do the right thing in OnComplete.
      kCancelledWhilstForwarding,
      // The same, but before we got the pipe
      kCancelledWhilstForwardingNoPipe,
      // Call got terminated whilst we had a recv_message batch completed, and
      // we've now received the completion.
      // On the next poll we'll close things out and forward on completions,
      // then transition to cancelled.
      kBatchCompletedButCancelled,
      // The same, but before we got the pipe
      kBatchCompletedButCancelledNoPipe,
      // Completed successfully while we're processing a recv message - see
      // kPushedToPipe.
      kCompletedWhilePushedToPipe,
      // Completed successfully while we're processing a recv message - see
      // kPulledFromPipe.
      kCompletedWhilePulledFromPipe,
      // Completed successfully while we were waiting to process
      // kBatchCompleted.
      kCompletedWhileBatchCompleted,
    };
    static const char* StateString(State);

    void OnComplete(absl::Status status);

    BaseCallData* const base_;
    Interceptor* const interceptor_;
    State state_ = State::kInitial;
    uint32_t scratch_flags_;
    std::optional<SliceBuffer>* intercepted_slice_buffer_;
    uint32_t* intercepted_flags_;
    std::optional<PipeSender<MessageHandle>::PushType> push_;
    std::optional<PipeReceiverNextType<MessageHandle>> next_;
    absl::Status completed_status_;
    grpc_closure* intercepted_on_complete_;
    grpc_closure on_complete_ =
        MakeMemberClosure<ReceiveMessage, &ReceiveMessage::OnComplete>(this);
  };

  Arena* arena() { return arena_; }
  grpc_call_element* elem() const { return elem_; }
  CallCombiner* call_combiner() const { return call_combiner_; }
  Timestamp deadline() const { return deadline_; }
  grpc_call_stack* call_stack() const { return call_stack_; }
  Pipe<ServerMetadataHandle>* server_initial_metadata_pipe() const {
    return server_initial_metadata_pipe_;
  }
  SendMessage* send_message() const { return send_message_; }
  ReceiveMessage* receive_message() const { return receive_message_; }

  bool is_last() const {
    return grpc_call_stack_element(call_stack_, call_stack_->count - 1) ==
           elem_;
  }

  virtual void WakeInsideCombiner(Flusher* flusher) = 0;

  virtual absl::string_view ClientOrServerString() const = 0;
  std::string LogTag() const;

 private:
  // Wakeable implementation.
  void Wakeup(WakeupMask) final;
  void WakeupAsync(WakeupMask) final { Crash("not implemented"); }
  void Drop(WakeupMask) final;

  virtual void OnWakeup() = 0;

  grpc_call_stack* const call_stack_;
  grpc_call_element* const elem_;
  Arena* const arena_;
  CallCombiner* const call_combiner_;
  const Timestamp deadline_;
  CallFinalization finalization_;
  std::atomic<grpc_polling_entity*> pollent_{nullptr};
  Pipe<ServerMetadataHandle>* const server_initial_metadata_pipe_;
  SendMessage* const send_message_;
  ReceiveMessage* const receive_message_;
};

class ClientCallData : public BaseCallData {
 public:
  ClientCallData(grpc_call_element* elem, const grpc_call_element_args* args,
                 uint8_t flags);
  ~ClientCallData() override;

  // Activity implementation.
  void ForceImmediateRepoll(WakeupMask) final;
  // Handle one grpc_transport_stream_op_batch
  void StartBatch(grpc_transport_stream_op_batch* batch) override;

  std::string DebugTag() const override;

 private:
  // At what stage is our handling of send initial metadata?
  enum class SendInitialState {
    // Start state: no op seen
    kInitial,
    // We've seen the op, and started the promise in response to it, but have
    // not yet sent the op to the next filter.
    kQueued,
    // We've sent the op to the next filter.
    kForwarded,
    // We were cancelled.
    kCancelled
  };
  // At what stage is our handling of recv trailing metadata?
  enum class RecvTrailingState {
    // Start state: no op seen
    kInitial,
    // We saw the op, and since it was bundled with send initial metadata, we
    // queued it until the send initial metadata can be sent to the next
    // filter.
    kQueued,
    // We've forwarded the op to the next filter.
    kForwarded,
    // The op has completed from below, but we haven't yet forwarded it up
    // (the promise gets to interject and mutate it).
    kComplete,
    // We've called the recv_metadata_ready callback from the original
    // recv_trailing_metadata op that was presented to us.
    kResponded,
    // We've been cancelled and handled that locally.
    // (i.e. whilst the recv_trailing_metadata op is queued in this filter).
    kCancelled
  };

  static const char* StateString(SendInitialState);
  static const char* StateString(RecvTrailingState);
  std::string DebugString() const;

  struct RecvInitialMetadata;
  class PollContext;

  // Handle cancellation.
  void Cancel(grpc_error_handle error, Flusher* flusher);
  // Begin running the promise - which will ultimately take some initial
  // metadata and return some trailing metadata.
  void StartPromise(Flusher* flusher);
  // Interject our callback into the op batch for recv trailing metadata
  // ready. Stash a pointer to the trailing metadata that will be filled in,
  // so we can manipulate it later.
  void HookRecvTrailingMetadata(CapturedBatch batch);
  // Construct a promise that will "call" the next filter.
  // Effectively:
  //   - put the modified initial metadata into the batch to be sent down.
  //   - return a wrapper around PollTrailingMetadata as the promise.
  ArenaPromise<ServerMetadataHandle> MakeNextPromise(CallArgs call_args);
  // Wrapper to make it look like we're calling the next filter as a promise.
  // First poll: send the send_initial_metadata op down the stack.
  // All polls: await receiving the trailing metadata, then return it to the
  // application.
  Poll<ServerMetadataHandle> PollTrailingMetadata();
  static void RecvTrailingMetadataReadyCallback(void* arg,
                                                grpc_error_handle error);
  void RecvTrailingMetadataReady(grpc_error_handle error);
  void RecvInitialMetadataReady(grpc_error_handle error);
  // Given an error, fill in ServerMetadataHandle to represent that error.
  void SetStatusFromError(grpc_metadata_batch* metadata,
                          grpc_error_handle error);
  // Wakeup and poll the promise if appropriate.
  void WakeInsideCombiner(Flusher* flusher) override;
  void OnWakeup() override;

  absl::string_view ClientOrServerString() const override { return "CLI"; }

  // Contained promise
  ArenaPromise<ServerMetadataHandle> promise_;
  // Queued batch containing at least a send_initial_metadata op.
  CapturedBatch send_initial_metadata_batch_;
  // Pointer to where trailing metadata will be stored.
  grpc_metadata_batch* recv_trailing_metadata_ = nullptr;
  // Trailing metadata as returned by the promise, if we hadn't received
  // trailing metadata from below yet (so we can substitute it in).
  ServerMetadataHandle cancelling_metadata_;
  // State tracking recv initial metadata for filters that care about it.
  RecvInitialMetadata* recv_initial_metadata_ = nullptr;
  // Closure to call when we're done with the trailing metadata.
  grpc_closure* original_recv_trailing_metadata_ready_ = nullptr;
  // Our closure pointing to RecvTrailingMetadataReadyCallback.
  grpc_closure recv_trailing_metadata_ready_;
  // Error received during cancellation.
  grpc_error_handle cancelled_error_;
  // State of the send_initial_metadata op.
  SendInitialState send_initial_state_ = SendInitialState::kInitial;
  // State of the recv_trailing_metadata op.
  RecvTrailingState recv_trailing_state_ = RecvTrailingState::kInitial;
  // Polling related data. Non-null if we're actively polling
  PollContext* poll_ctx_ = nullptr;
  // Initial metadata outstanding token
  ClientInitialMetadataOutstandingToken initial_metadata_outstanding_token_;
};

class ServerCallData : public BaseCallData {
 public:
  ServerCallData(grpc_call_element* elem, const grpc_call_element_args* args,
                 uint8_t flags);
  ~ServerCallData() override;

  // Activity implementation.
  void ForceImmediateRepoll(WakeupMask) final;
  // Handle one grpc_transport_stream_op_batch
  void StartBatch(grpc_transport_stream_op_batch* batch) override;

  std::string DebugTag() const override;

 protected:
  absl::string_view ClientOrServerString() const override { return "SVR"; }

 private:
  // At what stage is our handling of recv initial metadata?
  enum class RecvInitialState {
    // Start state: no op seen
    kInitial,
    // Op seen, and forwarded to the next filter.
    // Now waiting for the callback.
    kForwarded,
    // The op has completed from below, but we haven't yet forwarded it up
    // (the promise gets to interject and mutate it).
    kComplete,
    // We've sent the response to the next filter up.
    kResponded,
  };
  // At what stage is our handling of send trailing metadata?
  enum class SendTrailingState {
    // Start state: no op seen
    kInitial,
    // We saw the op, but it was with a send message op (or one was in progress)
    // - so we'll wait for that to complete before processing the trailing
    // metadata.
    kQueuedBehindSendMessage,
    // We saw the op, and are waiting for the promise to complete
    // to forward it. First however we need to close sends.
    kQueuedButHaventClosedSends,
    // We saw the op, and are waiting for the promise to complete
    // to forward it.
    kQueued,
    // We've forwarded the op to the next filter.
    kForwarded,
    // We were cancelled.
    kCancelled
  };

  static const char* StateString(RecvInitialState state);
  static const char* StateString(SendTrailingState state);
  std::string DebugString() const;

  class PollContext;
  struct SendInitialMetadata;

  // Shut things down when the call completes.
  void Completed(grpc_error_handle error, bool tarpit_cancellation,
                 Flusher* flusher);
  // Construct a promise that will "call" the next filter.
  // Effectively:
  //   - put the modified initial metadata into the batch being sent up.
  //   - return a wrapper around PollTrailingMetadata as the promise.
  ArenaPromise<ServerMetadataHandle> MakeNextPromise(CallArgs call_args);
  // Wrapper to make it look like we're calling the next filter as a promise.
  // All polls: await sending the trailing metadata, then forward it down the
  // stack.
  Poll<ServerMetadataHandle> PollTrailingMetadata();
  static void RecvInitialMetadataReadyCallback(void* arg,
                                               grpc_error_handle error);
  void RecvInitialMetadataReady(grpc_error_handle error);
  static void RecvTrailingMetadataReadyCallback(void* arg,
                                                grpc_error_handle error);
  void RecvTrailingMetadataReady(grpc_error_handle error);
  // Wakeup and poll the promise if appropriate.
  void WakeInsideCombiner(Flusher* flusher) override;
  void OnWakeup() override;

  // Contained promise
  ArenaPromise<ServerMetadataHandle> promise_;
  // Pointer to where initial metadata will be stored.
  grpc_metadata_batch* recv_initial_metadata_ = nullptr;
  // Pointer to where trailing metadata will be stored.
  grpc_metadata_batch* recv_trailing_metadata_ = nullptr;
  // State for sending initial metadata.
  SendInitialMetadata* send_initial_metadata_ = nullptr;
  // Closure to call when we're done with the initial metadata.
  grpc_closure* original_recv_initial_metadata_ready_ = nullptr;
  // Our closure pointing to RecvInitialMetadataReadyCallback.
  grpc_closure recv_initial_metadata_ready_;
  // Closure to call when we're done with the trailing metadata.
  grpc_closure* original_recv_trailing_metadata_ready_ = nullptr;
  // Our closure pointing to RecvTrailingMetadataReadyCallback.
  grpc_closure recv_trailing_metadata_ready_;
  // Error received during cancellation.
  grpc_error_handle cancelled_error_;
  // Trailing metadata batch
  CapturedBatch send_trailing_metadata_batch_;
  // State of the send_initial_metadata op.
  RecvInitialState recv_initial_state_ = RecvInitialState::kInitial;
  // State of the recv_trailing_metadata op.
  SendTrailingState send_trailing_state_ = SendTrailingState::kInitial;
  // Current poll context (or nullptr if not polling).
  PollContext* poll_ctx_ = nullptr;
  // Whether to forward the recv_initial_metadata op at the end of promise
  // wakeup.
  bool forward_recv_initial_metadata_callback_ = false;
};

// Specific call data per channel filter.
// Note that we further specialize for clients and servers since their
// implementations are very different.
template <FilterEndpoint endpoint>
class CallData;

// Client implementation of call data.
template <>
class CallData<FilterEndpoint::kClient> : public ClientCallData {
 public:
  using ClientCallData::ClientCallData;
};

// Server implementation of call data.
template <>
class CallData<FilterEndpoint::kServer> : public ServerCallData {
 public:
  using ServerCallData::ServerCallData;
};

struct BaseCallDataMethods {
  static void SetPollsetOrPollsetSet(grpc_call_element* elem,
                                     grpc_polling_entity* pollent) {
    static_cast<BaseCallData*>(elem->call_data)->set_pollent(pollent);
  }

  static void DestructCallData(grpc_call_element* elem,
                               const grpc_call_final_info* final_info) {
    auto* cd = static_cast<BaseCallData*>(elem->call_data);
    cd->Finalize(final_info);
    cd->~BaseCallData();
  }

  static void StartTransportStreamOpBatch(
      grpc_call_element* elem, grpc_transport_stream_op_batch* batch) {
    static_cast<BaseCallData*>(elem->call_data)->StartBatch(batch);
  }
};

// The type of object returned by a filter's Create method.
template <typename T>
using CreatedType = typename decltype(T::Create(ChannelArgs(), {}))::value_type;

template <typename GrpcChannelOrCallElement>
inline ChannelFilter* ChannelFilterFromElem(GrpcChannelOrCallElement* elem) {
  return *static_cast<ChannelFilter**>(elem->channel_data);
}

template <typename CallData, uint8_t kFlags>
struct CallDataFilterWithFlagsMethods {
  static absl::Status InitCallElem(grpc_call_element* elem,
                                   const grpc_call_element_args* args) {
    new (elem->call_data) CallData(elem, args, kFlags);
    return absl::OkStatus();
  }

  static void DestroyCallElem(grpc_call_element* elem,
                              const grpc_call_final_info* final_info,
                              grpc_closure* then_schedule_closure) {
    BaseCallDataMethods::DestructCallData(elem, final_info);
    if ((kFlags & kFilterIsLast) != 0) {
      ExecCtx::Run(DEBUG_LOCATION, then_schedule_closure, absl::OkStatus());
    } else {
      CHECK_EQ(then_schedule_closure, nullptr);
    }
  }
};

struct ChannelFilterMethods {
  static ArenaPromise<ServerMetadataHandle> MakeCallPromise(
      grpc_channel_element* elem, CallArgs call_args,
      NextPromiseFactory next_promise_factory) {
    return ChannelFilterFromElem(elem)->MakeCallPromise(
        std::move(call_args), std::move(next_promise_factory));
  }

  static void StartTransportOp(grpc_channel_element* elem,
                               grpc_transport_op* op) {
    if (!ChannelFilterFromElem(elem)->StartTransportOp(op)) {
      grpc_channel_next_op(elem, op);
    }
  }

  static void PostInitChannelElem(grpc_channel_stack*,
                                  grpc_channel_element* elem) {
    ChannelFilterFromElem(elem)->PostInit();
  }

  static void GetChannelInfo(grpc_channel_element* elem,
                             const grpc_channel_info* info) {
    if (!ChannelFilterFromElem(elem)->GetChannelInfo(info)) {
      grpc_channel_next_get_info(elem, info);
    }
  }
};

template <typename F, uint8_t kFlags>
struct ChannelFilterWithFlagsMethods {
  static absl::Status InitChannelElem(grpc_channel_element* elem,
                                      grpc_channel_element_args* args) {
    CHECK(args->is_last == ((kFlags & kFilterIsLast) != 0));
    auto status = F::Create(
        args->channel_args,
        ChannelFilter::Args(args->channel_stack, elem,
                            grpc_channel_stack_filter_instance_number,
                            args->old_blackboard, args->new_blackboard));
    if (!status.ok()) {
      new (elem->channel_data) F*(nullptr);
      return absl_status_to_grpc_error(status.status());
    }
    new (elem->channel_data) F*(status->release());
    return absl::OkStatus();
  }

  static void DestroyChannelElem(grpc_channel_element* elem) {
    CreatedType<F> channel_elem(DownCast<F*>(ChannelFilterFromElem(elem)));
  }
};

}  // namespace promise_filter_detail

// F implements ChannelFilter and :
// class SomeChannelFilter : public ChannelFilter {
//  public:
//   static absl::StatusOr<SomeChannelFilter> Create(
//       ChannelArgs channel_args, ChannelFilter::Args filter_args);
// };
template <typename F, FilterEndpoint kEndpoint, uint8_t kFlags = 0>
absl::enable_if_t<std::is_base_of<ChannelFilter, F>::value &&
                      !std::is_base_of<ImplementChannelFilterTag, F>::value,
                  grpc_channel_filter>
MakePromiseBasedFilter() {
  using CallData = promise_filter_detail::CallData<kEndpoint>;

  return grpc_channel_filter{
      // start_transport_stream_op_batch
      promise_filter_detail::BaseCallDataMethods::StartTransportStreamOpBatch,
      // start_transport_op
      promise_filter_detail::ChannelFilterMethods::StartTransportOp,
      // sizeof_call_data
      sizeof(CallData),
      // init_call_elem
      promise_filter_detail::CallDataFilterWithFlagsMethods<
          CallData, kFlags>::InitCallElem,
      // set_pollset_or_pollset_set
      promise_filter_detail::BaseCallDataMethods::SetPollsetOrPollsetSet,
      // destroy_call_elem
      promise_filter_detail::CallDataFilterWithFlagsMethods<
          CallData, kFlags>::DestroyCallElem,
      // sizeof_channel_data
      sizeof(F),
      // init_channel_elem
      promise_filter_detail::ChannelFilterWithFlagsMethods<
          F, kFlags>::InitChannelElem,
      // post_init_channel_elem
      promise_filter_detail::ChannelFilterMethods::PostInitChannelElem,
      // destroy_channel_elem
      promise_filter_detail::ChannelFilterWithFlagsMethods<
          F, kFlags>::DestroyChannelElem,
      // get_channel_info
      promise_filter_detail::ChannelFilterMethods::GetChannelInfo,
      // name
      UniqueTypeNameFor<F>(),
  };
}

template <typename F, FilterEndpoint kEndpoint, uint8_t kFlags = 0>
absl::enable_if_t<std::is_base_of<ImplementChannelFilterTag, F>::value,
                  grpc_channel_filter>
MakePromiseBasedFilter() {
  using CallData = promise_filter_detail::CallData<kEndpoint>;

  return grpc_channel_filter{
      // start_transport_stream_op_batch
      promise_filter_detail::BaseCallDataMethods::StartTransportStreamOpBatch,
      // start_transport_op
      promise_filter_detail::ChannelFilterMethods::StartTransportOp,
      // sizeof_call_data
      sizeof(CallData),
      // init_call_elem
      promise_filter_detail::CallDataFilterWithFlagsMethods<
          CallData, kFlags>::InitCallElem,
      // set_pollset_or_pollset_set
      promise_filter_detail::BaseCallDataMethods::SetPollsetOrPollsetSet,
      // destroy_call_elem
      promise_filter_detail::CallDataFilterWithFlagsMethods<
          CallData, kFlags>::DestroyCallElem,
      // sizeof_channel_data
      sizeof(F),
      // init_channel_elem
      promise_filter_detail::ChannelFilterWithFlagsMethods<
          F, kFlags>::InitChannelElem,
      // post_init_channel_elem
      promise_filter_detail::ChannelFilterMethods::PostInitChannelElem,
      // destroy_channel_elem
      promise_filter_detail::ChannelFilterWithFlagsMethods<
          F, kFlags>::DestroyChannelElem,
      // get_channel_info
      promise_filter_detail::ChannelFilterMethods::GetChannelInfo,
      // name
      UniqueTypeNameFor<F>(),
  };
}

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_CHANNEL_PROMISE_BASED_FILTER_H
