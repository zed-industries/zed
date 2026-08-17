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

#ifndef GRPC_SRC_CORE_CALL_CALL_FILTERS_H
#define GRPC_SRC_CORE_CALL_CALL_FILTERS_H

#include <grpc/support/port_platform.h>

#include <cstdint>
#include <limits>
#include <memory>
#include <ostream>
#include <type_traits>

#include "absl/log/check.h"
#include "src/core/call/call_state.h"
#include "src/core/call/message.h"
#include "src/core/call/metadata.h"
#include "src/core/lib/promise/for_each.h"
#include "src/core/lib/promise/if.h"
#include "src/core/lib/promise/latch.h"
#include "src/core/lib/promise/map.h"
#include "src/core/lib/promise/promise.h"
#include "src/core/lib/promise/seq.h"
#include "src/core/lib/promise/status_flag.h"
#include "src/core/lib/promise/try_seq.h"
#include "src/core/lib/transport/call_final_info.h"
#include "src/core/util/dump_args.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"

// CallFilters tracks a list of filters that are attached to a call.
// At a high level, a filter (for the purposes of this module) is a class
// that has a Call member class, and a set of methods that are called
// for each major event in the lifetime of a call.
//
// The Call member class must have the following members:
// - OnClientInitialMetadata  - $VALUE_TYPE = ClientMetadata
// - OnServerInitialMetadata  - $VALUE_TYPE = ServerMetadata
// - OnServerToClientMessage  - $VALUE_TYPE = Message
// - OnClientToServerMessage  - $VALUE_TYPE = Message
// - OnClientToServerHalfClose - no value
// - OnServerTrailingMetadata - $VALUE_TYPE = ServerMetadata
// - OnFinalize               - special, see below
// These members define an interception point for a particular event in
// the call lifecycle.
//
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
// - void $INTERCEPTOR_NAME($VALUE_TYPE&, FilterType*):
//   the filter intercepts this event, and can modify the value.
//   it can access the channel via the second argument.
//   it never fails.
// - absl::Status $INTERCEPTOR_NAME($VALUE_TYPE&, FilterType*):
//   the filter intercepts this event, and can modify the value.
//   it can access the channel via the second argument.
//   it can fail, in which case the call will be aborted.
// - ServerMetadataHandle $INTERCEPTOR_NAME($VALUE_TYPE&, FilterType*)
//   the filter intercepts this event, and can modify the value.
//   it can access the channel via the second argument.
//   the filter can return nullptr for success, or a metadata handle for
//   failure (in which case the call will be aborted).
//   useful for cases where the exact metadata returned needs to be customized.
// It's also acceptable to return a promise that resolves to the
// relevant return type listed above.
//
// Finally, OnFinalize is added to intecept call finalization.
// It must have one of the signatures:
// - static inline const NoInterceptor OnFinalize:
//   the filter does not intercept call finalization.
// - void OnFinalize(const grpc_call_final_info*):
//   the filter intercepts call finalization.
// - void OnFinalize(const grpc_call_final_info*, FilterType*):
//   the filter intercepts call finalization.
//
// The constructor of the Call object can either take a pointer to the channel
// object, or not take any arguments.
//
// *THIS MODULE* holds no opinion on what members the channel part of the
// filter should or should not have, but does require that it have a stable
// pointer for the lifetime of a call (ownership is expected to happen
// elsewhere).

namespace grpc_core {

// Tag type to indicate no interception.
// This is used to indicate that a filter does not intercept a particular
// event.
// We declare these as (for example):
//   inline static inline const NoInterceptor OnClientInitialMetadata;
struct NoInterceptor {};

namespace filters_detail {

// Flow control across pipe stages.
// This ends up being exceedingly subtle - essentially we need to ensure that
// across a series of pipes we have no more than one outstanding message at a
// time - but those pipes are for the most part independent.
// How we achieve this is that this NextMessage object holds both the message
// and a completion token - the last owning NextMessage instance will call
// the on_progress method on the referenced CallState - and at that point that
// CallState will allow the next message to be sent through it.
// Next, the ForEach promise combiner explicitly holds onto the wrapper object
// owning the result (this object) and extracts the message from it, but doesn't
// dispose that instance until the action promise for the ForEach iteration
// completes, ensuring most callers need do nothing special to have the
// flow control work correctly.
template <void (CallState::*on_progress)()>
class NextMessage {
 public:
  ~NextMessage() {
    if (message_ != end_of_stream() && message_ != error() &&
        message_ != taken()) {
      delete message_;
    }
    if (call_state_ != nullptr) {
      (call_state_->*on_progress)();
    }
  }

  NextMessage() = default;
  explicit NextMessage(Failure) : message_(error()), call_state_(nullptr) {}
  NextMessage(MessageHandle message, CallState* call_state) {
    DCHECK_NE(call_state, nullptr);
    DCHECK_NE(message.get(), nullptr);
    DCHECK(message.get_deleter().has_freelist());
    message_ = message.release();
    call_state_ = call_state;
  }
  NextMessage(const NextMessage& other) = delete;
  NextMessage& operator=(const NextMessage& other) = delete;
  NextMessage(NextMessage&& other) noexcept
      : message_(std::exchange(other.message_, taken())),
        call_state_(std::exchange(other.call_state_, nullptr)) {}
  NextMessage& operator=(NextMessage&& other) noexcept {
    if (message_ != end_of_stream() && message_ != error() &&
        message_ != taken()) {
      delete message_;
    }
    if (call_state_ != nullptr) {
      (call_state_->*on_progress)();
    }
    message_ = std::exchange(other.message_, taken());
    call_state_ = std::exchange(other.call_state_, nullptr);
    return *this;
  }

  bool ok() const {
    DCHECK_NE(message_, taken());
    return message_ != error();
  }
  bool has_value() const {
    DCHECK_NE(message_, taken());
    DCHECK(ok());
    return message_ != end_of_stream();
  }
  StatusFlag status() const { return StatusFlag(ok()); }
  Message& value() {
    DCHECK_NE(message_, taken());
    DCHECK(ok());
    DCHECK(has_value());
    return *message_;
  }
  const Message& value() const {
    DCHECK_NE(message_, taken());
    DCHECK(ok());
    DCHECK(has_value());
    return *message_;
  }
  MessageHandle TakeValue() {
    DCHECK_NE(message_, taken());
    DCHECK(ok());
    DCHECK(has_value());
    return MessageHandle(std::exchange(message_, taken()),
                         Arena::PooledDeleter());
  }
  bool progressed() const { return call_state_ == nullptr; }
  void Progress() {
    DCHECK(!progressed());
    (call_state_->*on_progress)();
    call_state_ = nullptr;
  }

  template <typename Sink>
  friend void AbslStringify(Sink& sink, const NextMessage& msg) {
    if (!msg.ok()) {
      sink.Append("<failure>");
      return;
    }
    if (!msg.has_value()) {
      sink.Append("<end-of-stream>");
      return;
    }
    AbslStringify(sink, msg.value());
  }

 private:
  static Message* end_of_stream() { return nullptr; }
  static Message* error() { return reinterpret_cast<Message*>(1); }
  static Message* taken() { return reinterpret_cast<Message*>(2); }
  Message* message_ = end_of_stream();
  CallState* call_state_ = nullptr;
};

template <typename T>
struct ArgumentMustBeNextMessage;
template <void (CallState::*on_progress)()>
struct ArgumentMustBeNextMessage<NextMessage<on_progress>> {
  static constexpr bool value() { return true; }
};

inline void* Offset(void* base, size_t amt) {
  return static_cast<char*>(base) + amt;
}

// One call filter constructor
// Contains enough information to allocate and initialize the
// call data for one filter.
struct FilterConstructor {
  // Pointer to corresponding channel data for this filter
  void* channel_data;
  // Offset of the call data for this filter within the call data memory
  // allocation
  size_t call_offset;
  // Initialize the call data for this filter
  void (*call_init)(void* call_data, void* channel_data);
};

// One call filter destructor
struct FilterDestructor {
  // Offset of the call data for this filter within the call data memory
  // allocation
  size_t call_offset;
  // Destroy the call data for this filter
  void (*call_destroy)(void* call_data);
};

template <typename FilterType, typename = void>
struct CallConstructor {
  static void Construct(void* call_data, FilterType*) {
    new (call_data) typename FilterType::Call();
  }
};

template <typename FilterType>
struct CallConstructor<FilterType,
                       absl::void_t<decltype(typename FilterType::Call(
                           static_cast<FilterType*>(nullptr)))>> {
  static void Construct(void* call_data, FilterType* channel) {
    new (call_data) typename FilterType::Call(channel);
  }
};

// Result of a filter operation
// Can be either ok (if ok is non-null) or an error.
// Only one pointer can be set.
template <typename T>
struct ResultOr {
  ResultOr(T ok, ServerMetadataHandle error)
      : ok(std::move(ok)), error(std::move(error)) {
    CHECK((this->ok == nullptr) ^ (this->error == nullptr));
  }
  T ok;
  ServerMetadataHandle error;
};

// One filter operation metadata
// Given a value of type T, produces a promise of type ResultOr<T>.
template <typename T>
struct Operator {
  using Arg = T;
  // Pointer to corresponding channel data for this filter
  void* channel_data;
  // Offset of the call data for this filter within the call data memory
  size_t call_offset;
  // Initialize the promise data for this filter, and poll once.
  // Return the result of the poll.
  // If the promise finishes, also destroy the promise data!
  Poll<ResultOr<T>> (*promise_init)(void* promise_data, void* call_data,
                                    void* channel_data, T value);
  // Poll the promise data for this filter.
  // If the promise finishes, also destroy the promise data!
  // Note that if the promise always finishes on the first poll, then supplying
  // this method is unnecessary (as it will never be called).
  Poll<ResultOr<T>> (*poll)(void* promise_data);
  // Destroy the promise data for this filter for an in-progress operation
  // before the promise finishes.
  // Note that if the promise always finishes on the first poll, then supplying
  // this method is unnecessary (as it will never be called).
  void (*early_destroy)(void* promise_data);
};

struct HalfCloseOperator {
  // Pointer to corresponding channel data for this filter
  void* channel_data;
  // Offset of the call data for this filter within the call data memory
  size_t call_offset;
  void (*half_close)(void* call_data, void* channel_data);
};

struct ServerTrailingMetadataOperator {
  // Pointer to corresponding channel data for this filter
  void* channel_data;
  // Offset of the call data for this filter within the call data memory
  size_t call_offset;
  ServerMetadataHandle (*server_trailing_metadata)(
      void* call_data, void* channel_data, ServerMetadataHandle metadata);
};

GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline void RunHalfClose(
    absl::Span<const HalfCloseOperator> ops, void* call_data) {
  for (const auto& op : ops) {
    op.half_close(Offset(call_data, op.call_offset), op.channel_data);
  }
}

GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline ServerMetadataHandle
RunServerTrailingMetadata(absl::Span<const ServerTrailingMetadataOperator> ops,
                          void* call_data, ServerMetadataHandle md) {
  for (auto& op : ops) {
    md = op.server_trailing_metadata(Offset(call_data, op.call_offset),
                                     op.channel_data, std::move(md));
  }
  return md;
}

// One call finalizer
struct Finalizer {
  void* channel_data;
  size_t call_offset;
  void (*final)(void* call_data, void* channel_data,
                const grpc_call_final_info* final_info);
};

// A layout of operations for a given filter stack
// This includes which operations, how much memory is required, what alignment.
template <typename T>
struct Layout {
  size_t promise_size = 0;
  size_t promise_alignment = 0;
  std::vector<Operator<T>> ops;

  void Add(size_t filter_promise_size, size_t filter_promise_alignment,
           Operator<T> op) {
    promise_size = std::max(promise_size, filter_promise_size);
    promise_alignment = std::max(promise_alignment, filter_promise_alignment);
    ops.push_back(op);
  }

  void Reverse() { absl::c_reverse(ops); }
};

// AddOp and friends
// These are helpers to wrap a member function on a class into an operation
// and attach it to a layout.
// There's a generic wrapper function `AddOp` for each of fallible and
// infallible operations.
// There are then specializations of AddOpImpl for each kind of member function
// an operation could have.
// Each specialization has an `Add` member function for the kinds of operations
// it supports: some only support fallible, some only support infallible, some
// support both.

template <typename FilterType, typename T, typename FunctionImpl,
          FunctionImpl impl, typename SfinaeVoid = void>
struct AddOpImpl;

template <typename FunctionImpl, FunctionImpl impl, typename FilterType,
          typename T>
void AddOp(FilterType* channel_data, size_t call_offset, Layout<T>& to) {
  AddOpImpl<FilterType, T, FunctionImpl, impl>::Add(channel_data, call_offset,
                                                    to);
}

template <typename FilterType>
void AddHalfClose(FilterType* channel_data, size_t call_offset,
                  void (FilterType::Call::*)(),
                  std::vector<HalfCloseOperator>& to) {
  to.push_back(
      HalfCloseOperator{channel_data, call_offset, [](void* call_data, void*) {
                          static_cast<typename FilterType::Call*>(call_data)
                              ->OnClientToServerHalfClose();
                        }});
}

template <typename FilterType>
void AddHalfClose(FilterType* channel_data, size_t call_offset,
                  void (FilterType::Call::*)(FilterType*),
                  std::vector<HalfCloseOperator>& to) {
  to.push_back(HalfCloseOperator{
      channel_data, call_offset, [](void* call_data, void* channel_data) {
        static_cast<typename FilterType::Call*>(call_data)
            ->OnClientToServerHalfClose(static_cast<FilterType*>(channel_data));
      }});
}

template <typename FilterType>
void AddHalfClose(FilterType*, size_t, const NoInterceptor*,
                  std::vector<HalfCloseOperator>&) {}

template <typename FilterType>
void AddServerTrailingMetadata(
    FilterType* channel_data, size_t call_offset,
    void (FilterType::Call::*)(ServerMetadata&),
    std::vector<ServerTrailingMetadataOperator>& to) {
  to.push_back(ServerTrailingMetadataOperator{
      channel_data, call_offset,
      [](void* call_data, void*, ServerMetadataHandle metadata) {
        static_cast<typename FilterType::Call*>(call_data)
            ->OnServerTrailingMetadata(*metadata);
        return metadata;
      }});
}

template <typename FilterType>
void AddServerTrailingMetadata(
    FilterType* channel_data, size_t call_offset,
    void (FilterType::Call::*)(ServerMetadata&, FilterType*),
    std::vector<ServerTrailingMetadataOperator>& to) {
  to.push_back(ServerTrailingMetadataOperator{
      channel_data, call_offset,
      [](void* call_data, void* channel_data, ServerMetadataHandle metadata) {
        static_cast<typename FilterType::Call*>(call_data)
            ->OnServerTrailingMetadata(*metadata,
                                       static_cast<FilterType*>(channel_data));
        return metadata;
      }});
}

template <typename FilterType>
void AddServerTrailingMetadata(
    FilterType* channel_data, size_t call_offset,
    absl::Status (FilterType::Call::*)(ServerMetadata&),
    std::vector<ServerTrailingMetadataOperator>& to) {
  to.push_back(ServerTrailingMetadataOperator{
      channel_data, call_offset,
      [](void* call_data, void*, ServerMetadataHandle metadata) {
        auto r = static_cast<typename FilterType::Call*>(call_data)
                     ->OnServerTrailingMetadata(*metadata);
        if (r.ok()) return metadata;
        return CancelledServerMetadataFromStatus(r);
      }});
}

template <typename FilterType>
void AddServerTrailingMetadata(
    FilterType* channel_data, size_t call_offset,
    ServerMetadataHandle (FilterType::Call::*)(ServerMetadataHandle),
    std::vector<ServerTrailingMetadataOperator>& to) {
  to.push_back(ServerTrailingMetadataOperator{
      channel_data, call_offset,
      [](void* call_data, void*, ServerMetadataHandle metadata) {
        return static_cast<typename FilterType::Call*>(call_data)
            ->OnServerTrailingMetadata(std::move(metadata));
      }});
}

template <typename FilterType>
void AddServerTrailingMetadata(FilterType*, size_t, const NoInterceptor*,
                               std::vector<ServerTrailingMetadataOperator>&) {}

// const NoInterceptor $EVENT
// These do nothing, and specifically DO NOT add an operation to the layout.
// Supported for fallible & infallible operations.
template <typename FilterType, typename T, const NoInterceptor* which>
struct AddOpImpl<FilterType, T, const NoInterceptor*, which> {
  static void Add(FilterType*, size_t, Layout<T>&) {}
};

// void $INTERCEPTOR_NAME($VALUE_TYPE&)
template <typename FilterType, typename T,
          void (FilterType::Call::*impl)(typename T::element_type&)>
struct AddOpImpl<FilterType, T,
                 void (FilterType::Call::*)(typename T::element_type&), impl> {
  static void Add(FilterType* channel_data, size_t call_offset, Layout<T>& to) {
    to.Add(0, 0,
           Operator<T>{
               channel_data,
               call_offset,
               [](void*, void* call_data, void*, T value) -> Poll<ResultOr<T>> {
                 (static_cast<typename FilterType::Call*>(call_data)->*impl)(
                     *value);
                 return ResultOr<T>{std::move(value), nullptr};
               },
               nullptr,
               nullptr,
           });
  }
};

// void $INTERCEPTOR_NAME(const $VALUE_TYPE&)
template <typename FilterType, typename T,
          void (FilterType::Call::*impl)(const typename T::element_type&)>
struct AddOpImpl<FilterType, T,
                 void (FilterType::Call::*)(const typename T::element_type&),
                 impl> {
  static void Add(FilterType* channel_data, size_t call_offset, Layout<T>& to) {
    to.Add(0, 0,
           Operator<T>{
               channel_data,
               call_offset,
               [](void*, void* call_data, void*, T value) -> Poll<ResultOr<T>> {
                 (static_cast<typename FilterType::Call*>(call_data)->*impl)(
                     *value);
                 return ResultOr<T>{std::move(value), nullptr};
               },
               nullptr,
               nullptr,
           });
  }
};

// void $INTERCEPTOR_NAME($VALUE_TYPE&, FilterType*)
template <typename FilterType, typename T,
          void (FilterType::Call::*impl)(typename T::element_type&,
                                         FilterType*)>
struct AddOpImpl<
    FilterType, T,
    void (FilterType::Call::*)(typename T::element_type&, FilterType*), impl> {
  static void Add(FilterType* channel_data, size_t call_offset, Layout<T>& to) {
    to.Add(0, 0,
           Operator<T>{
               channel_data,
               call_offset,
               [](void*, void* call_data, void* channel_data,
                  T value) -> Poll<ResultOr<T>> {
                 (static_cast<typename FilterType::Call*>(call_data)->*impl)(
                     *value, static_cast<FilterType*>(channel_data));
                 return ResultOr<T>{std::move(value), nullptr};
               },
               nullptr,
               nullptr,
           });
  }
};

// void $INTERCEPTOR_NAME(const $VALUE_TYPE&, FilterType*)
template <typename FilterType, typename T,
          void (FilterType::Call::*impl)(const typename T::element_type&,
                                         FilterType*)>
struct AddOpImpl<FilterType, T,
                 void (FilterType::Call::*)(const typename T::element_type&,
                                            FilterType*),
                 impl> {
  static void Add(FilterType* channel_data, size_t call_offset, Layout<T>& to) {
    to.Add(0, 0,
           Operator<T>{
               channel_data,
               call_offset,
               [](void*, void* call_data, void* channel_data,
                  T value) -> Poll<ResultOr<T>> {
                 (static_cast<typename FilterType::Call*>(call_data)->*impl)(
                     *value, static_cast<FilterType*>(channel_data));
                 return ResultOr<T>{std::move(value), nullptr};
               },
               nullptr,
               nullptr,
           });
  }
};

// $VALUE_HANDLE $INTERCEPTOR_NAME($VALUE_HANDLE, FilterType*)
template <typename FilterType, typename T,
          T (FilterType::Call::*impl)(T, FilterType*)>
struct AddOpImpl<FilterType, T, T (FilterType::Call::*)(T, FilterType*), impl> {
  static void Add(FilterType* channel_data, size_t call_offset, Layout<T>& to) {
    to.Add(
        0, 0,
        Operator<T>{
            channel_data,
            call_offset,
            [](void*, void* call_data, void* channel_data,
               T value) -> Poll<ResultOr<T>> {
              return ResultOr<T>{
                  (static_cast<typename FilterType::Call*>(call_data)->*impl)(
                      std::move(value), static_cast<FilterType*>(channel_data)),
                  nullptr};
            },
            nullptr,
            nullptr,
        });
  }
};

// absl::Status $INTERCEPTOR_NAME($VALUE_TYPE&)
template <typename FilterType, typename T,
          absl::Status (FilterType::Call::*impl)(typename T::element_type&)>
struct AddOpImpl<FilterType, T,
                 absl::Status (FilterType::Call::*)(typename T::element_type&),
                 impl> {
  static void Add(FilterType* channel_data, size_t call_offset, Layout<T>& to) {
    to.Add(
        0, 0,
        Operator<T>{
            channel_data,
            call_offset,
            [](void*, void* call_data, void*, T value) -> Poll<ResultOr<T>> {
              auto r =
                  (static_cast<typename FilterType::Call*>(call_data)->*impl)(
                      *value);
              if (r.ok()) return ResultOr<T>{std::move(value), nullptr};
              return ResultOr<T>{
                  nullptr, StatusCast<ServerMetadataHandle>(std::move(r))};
            },
            nullptr,
            nullptr,
        });
  }
};

// absl::Status $INTERCEPTOR_NAME(const $VALUE_TYPE&)
template <typename FilterType, typename T,
          absl::Status (FilterType::Call::*impl)(
              const typename T::element_type&)>
struct AddOpImpl<
    FilterType, T,
    absl::Status (FilterType::Call::*)(const typename T::element_type&), impl> {
  static void Add(FilterType* channel_data, size_t call_offset, Layout<T>& to) {
    to.Add(
        0, 0,
        Operator<T>{
            channel_data,
            call_offset,
            [](void*, void* call_data, void*, T value) -> Poll<ResultOr<T>> {
              auto r =
                  (static_cast<typename FilterType::Call*>(call_data)->*impl)(
                      *value);
              if (r.ok()) return ResultOr<T>{std::move(value), nullptr};
              return ResultOr<T>{
                  nullptr, StatusCast<ServerMetadataHandle>(std::move(r))};
            },
            nullptr,
            nullptr,
        });
  }
};

// absl::Status $INTERCEPTOR_NAME($VALUE_TYPE&, FilterType*)
template <typename FilterType, typename T,
          absl::Status (FilterType::Call::*impl)(typename T::element_type&,
                                                 FilterType*)>
struct AddOpImpl<FilterType, T,
                 absl::Status (FilterType::Call::*)(typename T::element_type&,
                                                    FilterType*),
                 impl> {
  static void Add(FilterType* channel_data, size_t call_offset, Layout<T>& to) {
    to.Add(
        0, 0,
        Operator<T>{
            channel_data,
            call_offset,
            [](void*, void* call_data, void* channel_data,
               T value) -> Poll<ResultOr<T>> {
              auto r =
                  (static_cast<typename FilterType::Call*>(call_data)->*impl)(
                      *value, static_cast<FilterType*>(channel_data));
              if (IsStatusOk(r)) return ResultOr<T>{std::move(value), nullptr};
              return ResultOr<T>{
                  nullptr, StatusCast<ServerMetadataHandle>(std::move(r))};
            },
            nullptr,
            nullptr,
        });
  }
};

// absl::Status $INTERCEPTOR_NAME(const $VALUE_TYPE&, FilterType*)
template <typename FilterType, typename T,
          absl::Status (FilterType::Call::*impl)(
              const typename T::element_type&, FilterType*)>
struct AddOpImpl<FilterType, T,
                 absl::Status (FilterType::Call::*)(
                     const typename T::element_type&, FilterType*),
                 impl> {
  static void Add(FilterType* channel_data, size_t call_offset, Layout<T>& to) {
    to.Add(
        0, 0,
        Operator<T>{
            channel_data,
            call_offset,
            [](void*, void* call_data, void* channel_data,
               T value) -> Poll<ResultOr<T>> {
              auto r =
                  (static_cast<typename FilterType::Call*>(call_data)->*impl)(
                      *value, static_cast<FilterType*>(channel_data));
              if (IsStatusOk(r)) return ResultOr<T>{std::move(value), nullptr};
              return ResultOr<T>{
                  nullptr, StatusCast<ServerMetadataHandle>(std::move(r))};
            },
            nullptr,
            nullptr,
        });
  }
};

// absl::StatusOr<$VALUE_HANDLE> $INTERCEPTOR_NAME($VALUE_HANDLE, FilterType*)
template <typename FilterType, typename T,
          absl::StatusOr<T> (FilterType::Call::*impl)(T, FilterType*)>
struct AddOpImpl<FilterType, T,
                 absl::StatusOr<T> (FilterType::Call::*)(T, FilterType*),
                 impl> {
  static void Add(FilterType* channel_data, size_t call_offset, Layout<T>& to) {
    to.Add(
        0, 0,
        Operator<T>{
            channel_data,
            call_offset,
            [](void*, void* call_data, void* channel_data,
               T value) -> Poll<ResultOr<T>> {
              auto r =
                  (static_cast<typename FilterType::Call*>(call_data)->*impl)(
                      std::move(value), static_cast<FilterType*>(channel_data));
              if (IsStatusOk(r)) return ResultOr<T>{std::move(*r), nullptr};
              return ResultOr<T>{
                  nullptr, StatusCast<ServerMetadataHandle>(std::move(r))};
            },
            nullptr,
            nullptr,
        });
  }
};

// ServerMetadataHandle $INTERCEPTOR_NAME($VALUE_TYPE&)
template <typename FilterType, typename T,
          ServerMetadataHandle (FilterType::Call::*impl)(
              typename T::element_type&)>
struct AddOpImpl<FilterType, T,
                 ServerMetadataHandle (FilterType::Call::*)(
                     typename T::element_type&),
                 impl> {
  static void Add(FilterType* channel_data, size_t call_offset, Layout<T>& to) {
    to.Add(
        0, 0,
        Operator<T>{
            channel_data,
            call_offset,
            [](void*, void* call_data, void*, T value) -> Poll<ResultOr<T>> {
              auto r =
                  (static_cast<typename FilterType::Call*>(call_data)->*impl)(
                      *value);
              if (r == nullptr) return ResultOr<T>{std::move(value), nullptr};
              return ResultOr<T>{
                  nullptr, StatusCast<ServerMetadataHandle>(std::move(r))};
            },
            nullptr,
            nullptr,
        });
  }
};

// ServerMetadataHandle $INTERCEPTOR_NAME(const $VALUE_TYPE&)
template <typename FilterType, typename T,
          ServerMetadataHandle (FilterType::Call::*impl)(
              const typename T::element_type&)>
struct AddOpImpl<FilterType, T,
                 ServerMetadataHandle (FilterType::Call::*)(
                     const typename T::element_type&),
                 impl> {
  static void Add(FilterType* channel_data, size_t call_offset, Layout<T>& to) {
    to.Add(
        0, 0,
        Operator<T>{
            channel_data,
            call_offset,
            [](void*, void* call_data, void*, T value) -> Poll<ResultOr<T>> {
              auto r =
                  (static_cast<typename FilterType::Call*>(call_data)->*impl)(
                      *value);
              if (r == nullptr) return ResultOr<T>{std::move(value), nullptr};
              return ResultOr<T>{
                  nullptr, StatusCast<ServerMetadataHandle>(std::move(r))};
            },
            nullptr,
            nullptr,
        });
  }
};

// ServerMetadataHandle $INTERCEPTOR_NAME($VALUE_TYPE&, FilterType*)
template <typename FilterType, typename T,
          ServerMetadataHandle (FilterType::Call::*impl)(
              typename T::element_type&, FilterType*)>
struct AddOpImpl<FilterType, T,
                 ServerMetadataHandle (FilterType::Call::*)(
                     typename T::element_type&, FilterType*),
                 impl> {
  static void Add(FilterType* channel_data, size_t call_offset, Layout<T>& to) {
    to.Add(
        0, 0,
        Operator<T>{
            channel_data,
            call_offset,
            [](void*, void* call_data, void* channel_data,
               T value) -> Poll<ResultOr<T>> {
              auto r =
                  (static_cast<typename FilterType::Call*>(call_data)->*impl)(
                      *value, static_cast<FilterType*>(channel_data));
              if (r == nullptr) return ResultOr<T>{std::move(value), nullptr};
              return ResultOr<T>{
                  nullptr, StatusCast<ServerMetadataHandle>(std::move(r))};
            },
            nullptr,
            nullptr,
        });
  }
};

// ServerMetadataHandle $INTERCEPTOR_NAME(const $VALUE_TYPE&, FilterType*)
template <typename FilterType, typename T,
          ServerMetadataHandle (FilterType::Call::*impl)(
              const typename T::element_type&, FilterType*)>
struct AddOpImpl<FilterType, T,
                 ServerMetadataHandle (FilterType::Call::*)(
                     const typename T::element_type&, FilterType*),
                 impl> {
  static void Add(FilterType* channel_data, size_t call_offset, Layout<T>& to) {
    to.Add(
        0, 0,
        Operator<T>{
            channel_data,
            call_offset,
            [](void*, void* call_data, void* channel_data,
               T value) -> Poll<ResultOr<T>> {
              auto r =
                  (static_cast<typename FilterType::Call*>(call_data)->*impl)(
                      *value, static_cast<FilterType*>(channel_data));
              if (r == nullptr) return ResultOr<T>{std::move(value), nullptr};
              return ResultOr<T>{
                  nullptr, StatusCast<ServerMetadataHandle>(std::move(r))};
            },
            nullptr,
            nullptr,
        });
  }
};

// PROMISE_RETURNING(absl::Status) $INTERCEPTOR_NAME($VALUE_TYPE&)
template <typename FilterType, typename T, typename R,
          R (FilterType::Call::*impl)(typename T::element_type&)>
struct AddOpImpl<
    FilterType, T, R (FilterType::Call::*)(typename T::element_type&), impl,
    absl::enable_if_t<std::is_same<absl::Status, PromiseResult<R>>::value>> {
  static void Add(FilterType* channel_data, size_t call_offset, Layout<T>& to) {
    class Promise {
     public:
      Promise(T value, typename FilterType::Call* call_data, FilterType*)
          : value_(std::move(value)), impl_((call_data->*impl)(*value_)) {}

      Poll<ResultOr<T>> PollOnce() {
        auto p = impl_();
        auto* r = p.value_if_ready();
        if (r == nullptr) return Pending{};
        T value = std::move(value_);
        this->~Promise();
        if (r->ok()) {
          return ResultOr<T>{std::move(value), nullptr};
        }
        return ResultOr<T>{nullptr, CancelledServerMetadataFromStatus(*r)};
      }

     private:
      GPR_NO_UNIQUE_ADDRESS T value_;
      GPR_NO_UNIQUE_ADDRESS R impl_;
    };
    to.Add(sizeof(Promise), alignof(Promise),
           Operator<T>{
               channel_data,
               call_offset,
               [](void* promise_data, void* call_data, void* channel_data,
                  T value) -> Poll<ResultOr<T>> {
                 auto* promise = new (promise_data)
                     Promise(std::move(value),
                             static_cast<typename FilterType::Call*>(call_data),
                             static_cast<FilterType*>(channel_data));
                 return promise->PollOnce();
               },
               [](void* promise_data) {
                 return static_cast<Promise*>(promise_data)->PollOnce();
               },
               [](void* promise_data) {
                 static_cast<Promise*>(promise_data)->~Promise();
               },
           });
  }
};

// PROMISE_RETURNING(absl::Status) $INTERCEPTOR_NAME($VALUE_TYPE&, FilterType*)
template <typename FilterType, typename T, typename R,
          R (FilterType::Call::*impl)(typename T::element_type&, FilterType*)>
struct AddOpImpl<
    FilterType, T,
    R (FilterType::Call::*)(typename T::element_type&, FilterType*), impl,
    absl::enable_if_t<!std::is_same<R, absl::Status>::value &&
                      std::is_same<absl::Status, PromiseResult<R>>::value>> {
  static void Add(FilterType* channel_data, size_t call_offset, Layout<T>& to) {
    class Promise {
     public:
      Promise(T value, typename FilterType::Call* call_data,
              FilterType* channel_data)
          : value_(std::move(value)),
            impl_((call_data->*impl)(*value_, channel_data)) {}

      Poll<ResultOr<T>> PollOnce() {
        auto p = impl_();
        auto* r = p.value_if_ready();
        if (r == nullptr) return Pending{};
        T value = std::move(value_);
        this->~Promise();
        if (r->ok()) {
          return ResultOr<T>{std::move(value), nullptr};
        }
        return ResultOr<T>{nullptr, CancelledServerMetadataFromStatus(*r)};
      }

     private:
      GPR_NO_UNIQUE_ADDRESS T value_;
      GPR_NO_UNIQUE_ADDRESS R impl_;
    };
    to.Add(sizeof(Promise), alignof(Promise),
           Operator<T>{
               channel_data,
               call_offset,
               [](void* promise_data, void* call_data, void* channel_data,
                  T value) -> Poll<ResultOr<T>> {
                 auto* promise = new (promise_data)
                     Promise(std::move(value),
                             static_cast<typename FilterType::Call*>(call_data),
                             static_cast<FilterType*>(channel_data));
                 return promise->PollOnce();
               },
               [](void* promise_data) {
                 return static_cast<Promise*>(promise_data)->PollOnce();
               },
               [](void* promise_data) {
                 static_cast<Promise*>(promise_data)->~Promise();
               },
           });
  }
};

// PROMISE_RETURNING(absl::StatusOr<$VALUE_HANDLE>)
// $INTERCEPTOR_NAME($VALUE_HANDLE, FilterType*)
template <typename FilterType, typename T, typename R,
          R (FilterType::Call::*impl)(T, FilterType*)>
struct AddOpImpl<FilterType, T, R (FilterType::Call::*)(T, FilterType*), impl,
                 absl::enable_if_t<std::is_same<absl::StatusOr<T>,
                                                PromiseResult<R>>::value>> {
  static void Add(FilterType* channel_data, size_t call_offset, Layout<T>& to) {
#if defined(__GNUC__) && __GNUC__ == 9
    // Workaround for a bug in GNU C++ 9 compilers that fail to compile this
    // class.
    class Promise {
     public:
      Promise(T value, typename FilterType::Call* call_data,
              FilterType* channel_data)
          : impl_(std::make_unique<R>(
                (call_data->*impl)(std::move(value), channel_data))) {}

      Poll<ResultOr<T>> PollOnce() {
        auto p = (*impl_)();
        auto* r = p.value_if_ready();
        if (r == nullptr) return Pending{};
        this->~Promise();
        if (r->ok()) return ResultOr<T>{std::move(**r), nullptr};
        return ResultOr<T>{nullptr,
                           CancelledServerMetadataFromStatus(r->status())};
      }

     private:
      std::unique_ptr<R> impl_;
    };
#else
    class Promise {
     public:
      Promise(T value, typename FilterType::Call* call_data,
              FilterType* channel_data)
          : impl_((call_data->*impl)(std::move(value), channel_data)) {}

      Poll<ResultOr<T>> PollOnce() {
        auto p = impl_();
        auto* r = p.value_if_ready();
        if (r == nullptr) return Pending{};
        this->~Promise();
        if (r->ok()) return ResultOr<T>{std::move(**r), nullptr};
        return ResultOr<T>{nullptr,
                           CancelledServerMetadataFromStatus(r->status())};
      }

     private:
      GPR_NO_UNIQUE_ADDRESS R impl_;
    };
#endif
    to.Add(sizeof(Promise), alignof(Promise),
           Operator<T>{
               channel_data,
               call_offset,
               [](void* promise_data, void* call_data, void* channel_data,
                  T value) -> Poll<ResultOr<T>> {
                 auto* promise = new (promise_data)
                     Promise(std::move(value),
                             static_cast<typename FilterType::Call*>(call_data),
                             static_cast<FilterType*>(channel_data));
                 return promise->PollOnce();
               },
               [](void* promise_data) {
                 return static_cast<Promise*>(promise_data)->PollOnce();
               },
               [](void* promise_data) {
                 static_cast<Promise*>(promise_data)->~Promise();
               },
           });
  }
};

struct ChannelDataDestructor {
  void (*destroy)(void* channel_data);
  void* channel_data;
};

// StackData contains the main datastructures built up by this module.
// It's a complete representation of all the code that needs to be invoked
// to execute a call for a given set of filters.
// This structure is held at the channel layer and is shared between many
// in-flight calls.
struct StackData {
  // Overall size and alignment of the call data for this stack.
  size_t call_data_alignment = 1;
  size_t call_data_size = 0;
  // A complete list of filters for this call, so that we can construct the
  // call data for each filter.
  std::vector<FilterConstructor> filter_constructor;
  std::vector<FilterDestructor> filter_destructor;
  // For each kind of operation, a layout of the operations for this call.
  // (there's some duplicate data here, and that's ok: we want to avoid
  // pointer chasing as much as possible when executing a call)
  Layout<ClientMetadataHandle> client_initial_metadata;
  Layout<ServerMetadataHandle> server_initial_metadata;
  Layout<MessageHandle> client_to_server_messages;
  std::vector<HalfCloseOperator> client_to_server_half_close;
  Layout<MessageHandle> server_to_client_messages;
  std::vector<ServerTrailingMetadataOperator> server_trailing_metadata;
  // A list of finalizers for this call.
  // We use a bespoke data structure here because finalizers can never be
  // asynchronous.
  std::vector<Finalizer> finalizers;
  // A list of functions to call when this stack data is destroyed
  // (to capture ownership of channel data)
  std::vector<ChannelDataDestructor> channel_data_destructors;

  bool empty() const {
    return filter_constructor.empty() && filter_destructor.empty() &&
           client_initial_metadata.ops.empty() &&
           server_initial_metadata.ops.empty() &&
           client_to_server_messages.ops.empty() &&
           client_to_server_half_close.empty() &&
           server_to_client_messages.ops.empty() &&
           server_trailing_metadata.empty() && finalizers.empty() &&
           channel_data_destructors.empty();
  }

  // Add one filter to the list of filters, and update alignment.
  // Returns the offset of the call data for this filter.
  // Specifically does not update any of the layouts or finalizers.
  // Callers are expected to do that themselves.
  // This separation enables separation of *testing* of filters, and since
  // this is a detail type it's felt that a slightly harder to hold API that
  // we have exactly one caller for is warranted for a more thorough testing
  // story.
  template <typename FilterType>
  absl::enable_if_t<!std::is_empty<typename FilterType::Call>::value, size_t>
  AddFilterConstructor(FilterType* channel_data) {
    const size_t alignment = alignof(typename FilterType::Call);
    call_data_alignment = std::max(call_data_alignment, alignment);
    if (call_data_size % alignment != 0) {
      call_data_size += alignment - call_data_size % alignment;
    }
    const size_t call_offset = call_data_size;
    call_data_size += sizeof(typename FilterType::Call);
    filter_constructor.push_back(FilterConstructor{
        channel_data,
        call_offset,
        [](void* call_data, void* channel_data) {
          CallConstructor<FilterType>::Construct(
              call_data, static_cast<FilterType*>(channel_data));
        },
    });
    return call_offset;
  }

  template <typename FilterType>
  absl::enable_if_t<
      std::is_empty<typename FilterType::Call>::value &&
          !std::is_trivially_constructible<typename FilterType::Call>::value,
      size_t>
  AddFilterConstructor(FilterType* channel_data) {
    const size_t alignment = alignof(typename FilterType::Call);
    call_data_alignment = std::max(call_data_alignment, alignment);
    filter_constructor.push_back(FilterConstructor{
        channel_data,
        0,
        [](void* call_data, void* channel_data) {
          CallConstructor<FilterType>::Construct(
              call_data, static_cast<FilterType*>(channel_data));
        },
    });
    return 0;
  }

  template <typename FilterType>
  absl::enable_if_t<
      std::is_empty<typename FilterType::Call>::value &&
          std::is_trivially_constructible<typename FilterType::Call>::value,
      size_t>
  AddFilterConstructor(FilterType*) {
    const size_t alignment = alignof(typename FilterType::Call);
    call_data_alignment = std::max(call_data_alignment, alignment);
    return 0;
  }

  template <typename FilterType>
  absl::enable_if_t<
      !std::is_trivially_destructible<typename FilterType::Call>::value>
  AddFilterDestructor(size_t call_offset) {
    filter_destructor.push_back(FilterDestructor{
        call_offset,
        [](void* call_data) {
          Destruct(static_cast<typename FilterType::Call*>(call_data));
        },
    });
  }

  template <typename FilterType>
  absl::enable_if_t<
      std::is_trivially_destructible<typename FilterType::Call>::value>
  AddFilterDestructor(size_t) {}

  template <typename FilterType>
  size_t AddFilter(FilterType* filter) {
    const size_t call_offset = AddFilterConstructor(filter);
    AddFilterDestructor<FilterType>(call_offset);
    return call_offset;
  }

  // Per operation adders - one for each interception point.
  // Delegate to AddOp() above.

  template <typename FilterType>
  void AddClientInitialMetadataOp(FilterType* channel_data,
                                  size_t call_offset) {
    AddOp<decltype(&FilterType::Call::OnClientInitialMetadata),
          &FilterType::Call::OnClientInitialMetadata>(channel_data, call_offset,
                                                      client_initial_metadata);
  }

  template <typename FilterType>
  void AddServerInitialMetadataOp(FilterType* channel_data,
                                  size_t call_offset) {
    AddOp<decltype(&FilterType::Call::OnServerInitialMetadata),
          &FilterType::Call::OnServerInitialMetadata>(channel_data, call_offset,
                                                      server_initial_metadata);
  }

  template <typename FilterType>
  void AddClientToServerMessageOp(FilterType* channel_data,
                                  size_t call_offset) {
    AddOp<decltype(&FilterType::Call::OnClientToServerMessage),
          &FilterType::Call::OnClientToServerMessage>(
        channel_data, call_offset, client_to_server_messages);
  }

  template <typename FilterType>
  void AddClientToServerHalfClose(FilterType* channel_data,
                                  size_t call_offset) {
    AddHalfClose(channel_data, call_offset,
                 &FilterType::Call::OnClientToServerHalfClose,
                 client_to_server_half_close);
  }

  template <typename FilterType>
  void AddServerToClientMessageOp(FilterType* channel_data,
                                  size_t call_offset) {
    AddOp<decltype(&FilterType::Call::OnServerToClientMessage),
          &FilterType::Call::OnServerToClientMessage>(
        channel_data, call_offset, server_to_client_messages);
  }

  template <typename FilterType>
  void AddServerTrailingMetadataOp(FilterType* channel_data,
                                   size_t call_offset) {
    AddServerTrailingMetadata(channel_data, call_offset,
                              &FilterType::Call::OnServerTrailingMetadata,
                              server_trailing_metadata);
  }

  // Finalizer interception adders

  template <typename FilterType>
  void AddFinalizer(FilterType*, size_t, const NoInterceptor* p) {
    DCHECK(p == &FilterType::Call::OnFinalize);
  }

  template <typename FilterType>
  void AddFinalizer(FilterType* channel_data, size_t call_offset,
                    void (FilterType::Call::*p)(const grpc_call_final_info*)) {
    DCHECK(p == &FilterType::Call::OnFinalize);
    finalizers.push_back(Finalizer{
        channel_data,
        call_offset,
        [](void* call_data, void*, const grpc_call_final_info* final_info) {
          static_cast<typename FilterType::Call*>(call_data)->OnFinalize(
              final_info);
        },
    });
  }

  template <typename FilterType>
  void AddFinalizer(FilterType* channel_data, size_t call_offset,
                    void (FilterType::Call::*p)(const grpc_call_final_info*,
                                                FilterType*)) {
    DCHECK(p == &FilterType::Call::OnFinalize);
    finalizers.push_back(Finalizer{
        channel_data,
        call_offset,
        [](void* call_data, void* channel_data,
           const grpc_call_final_info* final_info) {
          static_cast<typename FilterType::Call*>(call_data)->OnFinalize(
              final_info, static_cast<FilterType*>(channel_data));
        },
    });
  }
};

// OperationExecutor is a helper class to execute a sequence of operations
// from a layout on one value.
// We instantiate one of these during the *Pull* promise for each operation
// and wait for it to resolve.
// At this layer the filters look like a list of transformations on the
// value pushed.
// An early-failing filter will cause subsequent filters to not execute.
template <typename T>
class OperationExecutor {
 public:
  OperationExecutor() = default;
  ~OperationExecutor();
  OperationExecutor(const OperationExecutor&) = delete;
  OperationExecutor& operator=(const OperationExecutor&) = delete;
  OperationExecutor(OperationExecutor&& other) noexcept
      : ops_(other.ops_), end_ops_(other.end_ops_) {
    // Movable iff we're not running.
    DCHECK_EQ(other.promise_data_, nullptr);
  }
  OperationExecutor& operator=(OperationExecutor&& other) noexcept {
    DCHECK_EQ(other.promise_data_, nullptr);
    DCHECK_EQ(promise_data_, nullptr);
    ops_ = other.ops_;
    end_ops_ = other.end_ops_;
    return *this;
  }
  // IsRunning() is true if we're currently executing a sequence of operations.
  bool IsRunning() const { return promise_data_ != nullptr; }
  // Start executing a layout. May allocate space to store the relevant promise.
  // Returns the result of the first poll.
  // If the promise finishes, also destroy the promise data.
  Poll<ResultOr<T>> Start(const Layout<T>* layout, T input, void* call_data);
  // Continue executing a layout. Returns the result of the next poll.
  // If the promise finishes, also destroy the promise data.
  Poll<ResultOr<T>> Step(void* call_data);

 private:
  // Start polling on the current step of the layout.
  // `input` is the current value (either the input to the first step, or the
  // so far transformed value)
  // `call_data` is the call data for the filter stack.
  // If this op finishes immediately then we iterative move to the next step.
  // If we reach the end up the ops, we return the overall poll result,
  // otherwise we return Pending.
  Poll<ResultOr<T>> InitStep(T input, void* call_data);
  // Continue polling on the current step of the layout.
  // Called on the next poll after InitStep returns pending.
  // If the promise is still pending, returns this.
  // If the promise completes we call into InitStep to continue execution
  // through the filters.
  Poll<ResultOr<T>> ContinueStep(void* call_data);

  void* promise_data_ = nullptr;
  const Operator<T>* ops_;
  const Operator<T>* end_ops_;
};

template <typename T>
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline OperationExecutor<
    T>::~OperationExecutor() {
  if (promise_data_ != nullptr) {
    if (ops_ != end_ops_) ops_->early_destroy(promise_data_);
    gpr_free_aligned(promise_data_);
  }
}

template <typename T>
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline Poll<ResultOr<T>>
OperationExecutor<T>::Start(const Layout<T>* layout, T input, void* call_data) {
  ops_ = layout->ops.data();
  end_ops_ = ops_ + layout->ops.size();
  if (layout->promise_size == 0) {
    // No call state ==> instantaneously ready
    auto r = InitStep(std::move(input), call_data);
    CHECK(r.ready());
    return r;
  }
  promise_data_ =
      gpr_malloc_aligned(layout->promise_size, layout->promise_alignment);
  return InitStep(std::move(input), call_data);
}

template <typename T>
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline Poll<ResultOr<T>>
OperationExecutor<T>::InitStep(T input, void* call_data) {
  CHECK(input != nullptr);
  while (true) {
    if (ops_ == end_ops_) {
      return ResultOr<T>{std::move(input), nullptr};
    }
    auto p =
        ops_->promise_init(promise_data_, Offset(call_data, ops_->call_offset),
                           ops_->channel_data, std::move(input));
    if (auto* r = p.value_if_ready()) {
      if (r->ok == nullptr) {
        ops_ = end_ops_;
        return std::move(*r);
      }
      input = std::move(r->ok);
      ++ops_;
      continue;
    }
    return Pending{};
  }
}

template <typename T>
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline Poll<ResultOr<T>>
OperationExecutor<T>::Step(void* call_data) {
  DCHECK_NE(promise_data_, nullptr);
  auto p = ContinueStep(call_data);
  if (p.ready()) {
    gpr_free_aligned(promise_data_);
    promise_data_ = nullptr;
  }
  return p;
}

template <typename T>
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline Poll<ResultOr<T>>
OperationExecutor<T>::ContinueStep(void* call_data) {
  auto p = ops_->poll(promise_data_);
  if (auto* r = p.value_if_ready()) {
    if (r->ok == nullptr) return std::move(*r);
    ++ops_;
    return InitStep(std::move(r->ok), call_data);
  }
  return Pending{};
}

template <typename Fn>
class ServerTrailingMetadataInterceptor {
 public:
  class Call {
   public:
    static const inline NoInterceptor OnClientInitialMetadata;
    static const inline NoInterceptor OnServerInitialMetadata;
    static const inline NoInterceptor OnClientToServerMessage;
    static const inline NoInterceptor OnClientToServerHalfClose;
    static const inline NoInterceptor OnServerToClientMessage;
    static const inline NoInterceptor OnFinalize;
    void OnServerTrailingMetadata(ServerMetadata& md,
                                  ServerTrailingMetadataInterceptor* filter) {
      filter->fn_(md);
    }
  };

  explicit ServerTrailingMetadataInterceptor(Fn fn) : fn_(std::move(fn)) {}

 private:
  GPR_NO_UNIQUE_ADDRESS Fn fn_;
};

template <typename Fn>
class ClientInitialMetadataInterceptor {
 public:
  class Call {
   public:
    auto OnClientInitialMetadata(ClientMetadata& md,
                                 ClientInitialMetadataInterceptor* filter) {
      return filter->fn_(md);
    }
    static const inline NoInterceptor OnServerInitialMetadata;
    static const inline NoInterceptor OnClientToServerMessage;
    static const inline NoInterceptor OnClientToServerHalfClose;
    static const inline NoInterceptor OnServerToClientMessage;
    static const inline NoInterceptor OnServerTrailingMetadata;
    static const inline NoInterceptor OnFinalize;
  };

  explicit ClientInitialMetadataInterceptor(Fn fn) : fn_(std::move(fn)) {}

 private:
  GPR_NO_UNIQUE_ADDRESS Fn fn_;
};

// Determine if an interceptor needs to access the channel via one of its
// arguments.

template <typename T>
constexpr bool MethodHasChannelAccess = false;

template <typename T, typename R, typename A>
constexpr bool MethodHasChannelAccess<R (T::*)(A)> = false;

template <typename T, typename R>
constexpr bool MethodHasChannelAccess<R (T::*)()> = false;

template <typename T, typename R, typename A, typename C>
constexpr bool MethodHasChannelAccess<R (T::*)(A, C)> = true;

template <typename... Ts>
constexpr bool AnyMethodHasChannelAccess = (MethodHasChannelAccess<Ts> || ...);

// Composite for a given channel type to determine if any of its interceptors
// fall into this category: later code should use this.
template <typename Derived>
inline constexpr bool CallHasChannelAccess() {
  return AnyMethodHasChannelAccess<
      decltype(&Derived::Call::OnClientInitialMetadata),
      decltype(&Derived::Call::OnClientToServerMessage),
      decltype(&Derived::Call::OnServerInitialMetadata),
      decltype(&Derived::Call::OnServerToClientMessage),
      decltype(&Derived::Call::OnServerTrailingMetadata),
      decltype(&Derived::Call::OnFinalize)>;
}
}  // namespace filters_detail

namespace for_each_detail {
template <void (CallState::*on_progress)()>
struct NextValueTraits<filters_detail::NextMessage<on_progress>> {
  using NextMsg = filters_detail::NextMessage<on_progress>;
  using Value = MessageHandle;

  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static NextValueType Type(
      const NextMsg& t) {
    if (!t.ok()) return NextValueType::kError;
    if (t.has_value()) return NextValueType::kValue;
    return NextValueType::kEndOfStream;
  }

  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static MessageHandle TakeValue(
      NextMsg& t) {
    return t.TakeValue();
  }
};
}  // namespace for_each_detail

template <void (CallState::*on_progress)()>
struct FailureStatusCastImpl<filters_detail::NextMessage<on_progress>,
                             StatusFlag> {
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static filters_detail::NextMessage<
      on_progress>
  Cast(StatusFlag flag) {
    DCHECK_EQ(flag, Failure{});
    return filters_detail::NextMessage<on_progress>(Failure{});
  }
};

namespace promise_detail {
template <void (CallState::*on_progress)()>
struct TrySeqTraitsWithSfinae<filters_detail::NextMessage<on_progress>> {
  using UnwrappedType = MessageHandle;
  using WrappedType = filters_detail::NextMessage<on_progress>;
  template <typename Next>
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static auto CallFactory(
      Next* next, WrappedType&& value) {
    return next->Make(value.TakeValue());
  }
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static bool IsOk(
      const WrappedType& value) {
    return value.ok();
  }
  static const char* ErrorString(const WrappedType& status) {
    DCHECK(!status.ok());
    return "failed";
  }
  template <typename R>
  static R ReturnValue(WrappedType&& status) {
    DCHECK(!status.ok());
    return WrappedType(Failure{});
  }
  template <typename F, typename Elem>
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static auto CallSeqFactory(
      F& f, Elem&& elem, WrappedType value)
      -> decltype(f(std::forward<Elem>(elem), std::declval<MessageHandle>())) {
    return f(std::forward<Elem>(elem), value.TakeValue());
  }
  template <typename Result, typename RunNext>
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static Poll<Result>
  CheckResultAndRunNext(WrappedType prior, RunNext run_next) {
    if (!prior.ok()) return WrappedType(prior.status());
    return run_next(std::move(prior));
  }
};
}  // namespace promise_detail

using ServerToClientNextMessage =
    filters_detail::NextMessage<&CallState::FinishPullServerToClientMessage>;
using ClientToServerNextMessage =
    filters_detail::NextMessage<&CallState::FinishPullClientToServerMessage>;

// Execution environment for a stack of filters.
// This is a per-call object.
class CallFilters {
 public:
  class StackBuilder;
  class StackTestSpouse;

  // A stack is an opaque, immutable type that contains the data necessary to
  // execute a call through a given set of filters.
  // It's reference counted so that it can be shared between many calls.
  // It contains pointers to the individual filters, yet it does not own those
  // pointers: it's expected that some other object will track that ownership.
  class Stack : public RefCounted<Stack> {
   public:
    ~Stack() override;

   private:
    friend class CallFilters;
    friend class StackBuilder;
    friend class StackTestSpouse;
    explicit Stack(filters_detail::StackData data) : data_(std::move(data)) {}
    const filters_detail::StackData data_;
  };

  // Build stacks... repeatedly call Add with each filter that contributes to
  // the stack, then call Build() to generate a ref counted Stack object.
  class StackBuilder {
   public:
    ~StackBuilder();

    template <typename FilterType>
    void Add(FilterType* filter) {
      const size_t call_offset = data_.AddFilter<FilterType>(filter);
      data_.AddClientInitialMetadataOp(filter, call_offset);
      data_.AddServerInitialMetadataOp(filter, call_offset);
      data_.AddClientToServerMessageOp(filter, call_offset);
      data_.AddClientToServerHalfClose(filter, call_offset);
      data_.AddServerToClientMessageOp(filter, call_offset);
      data_.AddServerTrailingMetadataOp(filter, call_offset);
      data_.AddFinalizer(filter, call_offset, &FilterType::Call::OnFinalize);
    }

    void AddOwnedObject(void (*destroy)(void* p), void* p) {
      data_.channel_data_destructors.push_back({destroy, p});
    }

    template <typename T>
    void AddOwnedObject(RefCountedPtr<T> p) {
      AddOwnedObject([](void* p) { static_cast<T*>(p)->Unref(); }, p.release());
    }

    template <typename T>
    void AddOwnedObject(std::unique_ptr<T> p) {
      AddOwnedObject([](void* p) { delete static_cast<T*>(p); }, p.release());
    }

    template <typename Fn>
    void AddOnClientInitialMetadata(Fn fn) {
      auto filter = std::make_unique<
          filters_detail::ClientInitialMetadataInterceptor<Fn>>(std::move(fn));
      Add(filter.get());
      AddOwnedObject(std::move(filter));
    }

    template <typename Fn>
    void AddOnServerTrailingMetadata(Fn fn) {
      auto filter = std::make_unique<
          filters_detail::ServerTrailingMetadataInterceptor<Fn>>(std::move(fn));
      Add(filter.get());
      AddOwnedObject(std::move(filter));
    }

    RefCountedPtr<Stack> Build();

   private:
    filters_detail::StackData data_;
  };

  explicit CallFilters(ClientMetadataHandle client_initial_metadata)
      : call_data_(nullptr),
        push_client_initial_metadata_(std::move(client_initial_metadata)) {}
  ~CallFilters() {
    if (call_data_ != nullptr && call_data_ != &g_empty_call_data_) {
      for (const auto& stack : stacks_) {
        for (const auto& destructor : stack.stack->data_.filter_destructor) {
          destructor.call_destroy(filters_detail::Offset(
              call_data_, stack.call_data_offset + destructor.call_offset));
        }
      }
      gpr_free_aligned(call_data_);
    }
  };

  CallFilters(const CallFilters&) = delete;
  CallFilters& operator=(const CallFilters&) = delete;
  CallFilters(CallFilters&&) = delete;
  CallFilters& operator=(CallFilters&&) = delete;

  void AddStack(RefCountedPtr<Stack> stack) {
    if (stack->data_.empty()) return;
    stacks_.emplace_back(std::move(stack));
  }
  void Start();

  // Access client initial metadata before it's processed
  ClientMetadata* unprocessed_client_initial_metadata() {
    return push_client_initial_metadata_.get();
  }

 private:
  template <typename Output, typename Input,
            Input(CallFilters::* input_location),
            filters_detail::Layout<Input>(filters_detail::StackData::* layout),
            void (CallState::*on_done)(), typename StackIterator>
  class MetadataExecutor {
   public:
    MetadataExecutor(CallFilters* filters, StackIterator stack_begin,
                     StackIterator stack_end)
        : stack_current_(stack_begin),
          stack_end_(stack_end),
          filters_(filters) {
      DCHECK_NE((filters_->*input_location).get(), nullptr);
    }

    Poll<ValueOrFailure<Output>> operator()() {
      if ((filters_->*input_location) != nullptr) {
        if (stack_current_ == stack_end_) {
          DCHECK_NE((filters_->*input_location).get(), nullptr);
          (filters_->call_state_.*on_done)();
          return Output(std::move(filters_->*input_location));
        }
        return FinishStep(executor_.Start(
            &(stack_current_->stack->data_.*layout),
            std::move(filters_->*input_location), filters_->call_data_));
      } else {
        return FinishStep(executor_.Step(filters_->call_data_));
      }
    }

   private:
    Poll<ValueOrFailure<Output>> FinishStep(
        Poll<filters_detail::ResultOr<Input>> p) {
      auto* r = p.value_if_ready();
      if (r == nullptr) return Pending{};
      if (r->ok != nullptr) {
        ++stack_current_;
        if (stack_current_ == stack_end_) {
          (filters_->call_state_.*on_done)();
          return ValueOrFailure<Output>{std::move(r->ok)};
        }
        return FinishStep(
            executor_.Start(&(stack_current_->stack->data_.*layout),
                            std::move(r->ok), filters_->call_data_));
      }
      (filters_->call_state_.*on_done)();
      filters_->PushServerTrailingMetadata(std::move(r->error));
      return Failure{};
    }

    StackIterator stack_current_;
    StackIterator stack_end_;
    CallFilters* filters_;
    filters_detail::OperationExecutor<Input> executor_;
  };

  template <MessageHandle(CallFilters::* input_location),
            filters_detail::Layout<MessageHandle>(
                filters_detail::StackData::* layout),
            void (CallState::*on_done)(), typename StackIterator>
  class MessageExecutor {
   public:
    using NextMsg = filters_detail::NextMessage<on_done>;

    MessageExecutor(CallFilters* filters, StackIterator stack_begin,
                    StackIterator stack_end)
        : stack_current_(stack_begin),
          stack_end_(stack_end),
          filters_(filters) {
      DCHECK_NE((filters_->*input_location).get(), nullptr);
    }

    Poll<NextMsg> operator()() {
      if ((filters_->*input_location) != nullptr) {
        if (stack_current_ == stack_end_) {
          DCHECK_NE((filters_->*input_location).get(), nullptr);
          return NextMsg(std::move(filters_->*input_location),
                         &filters_->call_state_);
        }
        return FinishStep(executor_.Start(
            &(stack_current_->stack->data_.*layout),
            std::move(filters_->*input_location), filters_->call_data_));
      } else {
        return FinishStep(executor_.Step(filters_->call_data_));
      }
    }

   private:
    Poll<NextMsg> FinishStep(Poll<filters_detail::ResultOr<MessageHandle>> p) {
      auto* r = p.value_if_ready();
      if (r == nullptr) return Pending{};
      if (r->ok != nullptr) {
        ++stack_current_;
        if (stack_current_ == stack_end_) {
          return NextMsg{std::move(r->ok), &filters_->call_state_};
        }
        return FinishStep(
            executor_.Start(&(stack_current_->stack->data_.*layout),
                            std::move(r->ok), filters_->call_data_));
      }
      (filters_->call_state_.*on_done)();
      filters_->PushServerTrailingMetadata(std::move(r->error));
      return Failure{};
    }

    StackIterator stack_current_;
    StackIterator stack_end_;
    CallFilters* filters_;
    filters_detail::OperationExecutor<MessageHandle> executor_;
  };

 public:
  // Client: Fetch client initial metadata
  // Returns a promise that resolves to ValueOrFailure<ClientMetadataHandle>
  GRPC_MUST_USE_RESULT auto PullClientInitialMetadata() {
    call_state_.BeginPullClientInitialMetadata();
    return MetadataExecutor<ClientMetadataHandle, ClientMetadataHandle,
                            &CallFilters::push_client_initial_metadata_,
                            &filters_detail::StackData::client_initial_metadata,
                            &CallState::FinishPullClientInitialMetadata,
                            StacksVector::const_iterator>(
        this, stacks_.cbegin(), stacks_.cend());
  }
  // Server: Push server initial metadata
  // Returns a promise that resolves to a StatusFlag indicating success
  StatusFlag PushServerInitialMetadata(ServerMetadataHandle md) {
    push_server_initial_metadata_ = std::move(md);
    return call_state_.PushServerInitialMetadata();
  }
  // Client: Fetch server initial metadata
  // Returns a promise that resolves to ValueOrFailure<ServerMetadataHandle>
  GRPC_MUST_USE_RESULT auto PullServerInitialMetadata() {
    return Seq(
        [this]() {
          return call_state_.PollPullServerInitialMetadataAvailable();
        },
        [this](bool has_server_initial_metadata) {
          return If(
              has_server_initial_metadata,
              [this]() {
                return Map(
                    MetadataExecutor<
                        std::optional<ServerMetadataHandle>,
                        ServerMetadataHandle,
                        &CallFilters::push_server_initial_metadata_,
                        &filters_detail::StackData::server_initial_metadata,
                        &CallState::FinishPullServerInitialMetadata,
                        StacksVector::const_reverse_iterator>(
                        this, stacks_.crbegin(), stacks_.crend()),
                    [](ValueOrFailure<std::optional<ServerMetadataHandle>> r) {
                      if (r.ok()) return std::move(*r);
                      return std::optional<ServerMetadataHandle>{};
                    });
              },
              []() {
                return Immediate(std::optional<ServerMetadataHandle>{});
              });
        });
  }
  // Client: Push client to server message
  // Returns a promise that resolves to a StatusFlag indicating success
  GRPC_MUST_USE_RESULT auto PushClientToServerMessage(MessageHandle message) {
    call_state_.BeginPushClientToServerMessage();
    DCHECK_NE(message.get(), nullptr);
    DCHECK_EQ(push_client_to_server_message_.get(), nullptr);
    push_client_to_server_message_ = std::move(message);
    return [this]() { return call_state_.PollPushClientToServerMessage(); };
  }
  // Client: Indicate that no more messages will be sent
  void FinishClientToServerSends() { call_state_.ClientToServerHalfClose(); }
  // Server: Fetch client to server message
  // Returns a promise that resolves to ClientToServerNextMessage
  GRPC_MUST_USE_RESULT auto PullClientToServerMessage() {
    return TrySeq(
        [this]() {
          return call_state_.PollPullClientToServerMessageAvailable();
        },
        [this](bool message_available) {
          return If(
              message_available,
              [this]() {
                return MessageExecutor<
                    &CallFilters::push_client_to_server_message_,
                    &filters_detail::StackData::client_to_server_messages,
                    &CallState::FinishPullClientToServerMessage,
                    StacksVector::const_iterator>(this, stacks_.cbegin(),
                                                  stacks_.cend());
              },
              []() -> ClientToServerNextMessage {
                return ClientToServerNextMessage();
              });
        });
  }
  // Server: Push server to client message
  // Returns a promise that resolves to a StatusFlag indicating success
  GRPC_MUST_USE_RESULT auto PushServerToClientMessage(MessageHandle message) {
    call_state_.BeginPushServerToClientMessage();
    push_server_to_client_message_ = std::move(message);
    return [this]() { return call_state_.PollPushServerToClientMessage(); };
  }
  // Server: Fetch server to client message
  // Returns a promise that resolves to ServerToClientNextMessage
  GRPC_MUST_USE_RESULT auto PullServerToClientMessage() {
    return TrySeq(
        [this]() {
          return call_state_.PollPullServerToClientMessageAvailable();
        },
        [this](bool message_available) {
          return If(
              message_available,
              [this]() {
                return MessageExecutor<
                    &CallFilters::push_server_to_client_message_,
                    &filters_detail::StackData::server_to_client_messages,
                    &CallState::FinishPullServerToClientMessage,
                    StacksVector::const_reverse_iterator>(
                    this, stacks_.crbegin(), stacks_.crend());
              },
              []() -> ServerToClientNextMessage {
                return ServerToClientNextMessage();
              });
        });
  }
  // Server: Indicate end of response
  // Closes the request entirely - no messages can be sent/received
  // If no server initial metadata has been sent, implies
  // NoServerInitialMetadata() called.
  void PushServerTrailingMetadata(ServerMetadataHandle md);
  // Optimized path to push cancellation onto the call
  void Cancel();
  // Client: Fetch server trailing metadata
  // Returns a promise that resolves to ServerMetadataHandle
  GRPC_MUST_USE_RESULT auto PullServerTrailingMetadata() {
    return Map(
        [this]() { return call_state_.PollServerTrailingMetadataAvailable(); },
        [this](Empty) {
          auto value = std::move(push_server_trailing_metadata_);
          if (call_data_ != nullptr) {
            for (auto it = stacks_.crbegin(); it != stacks_.crend(); ++it) {
              value = filters_detail::RunServerTrailingMetadata(
                  it->stack->data_.server_trailing_metadata,
                  filters_detail::Offset(call_data_, it->call_data_offset),
                  std::move(value));
            }
          }
          return value;
        });
  }
  // Server: Wait for server trailing metadata to have been sent
  // Returns a promise that resolves to a StatusFlag indicating whether the
  // request was cancelled or not -- failure to send trailing metadata is
  // considered a cancellation, as is actual cancellation -- but not application
  // errors.
  GRPC_MUST_USE_RESULT auto WasCancelled() {
    return [this]() { return call_state_.PollWasCancelled(); };
  }
  // Client & server: wait for server trailing metadata to be available, and
  // resolve to empty when it is.
  GRPC_MUST_USE_RESULT auto ServerTrailingMetadataWasPushed() {
    return
        [this]() { return call_state_.PollServerTrailingMetadataWasPushed(); };
  }
  // Client & server: returns true if server trailing metadata has been pushed
  // *and* contained a cancellation, false otherwise.
  GRPC_MUST_USE_RESULT bool WasCancelledPushed() const {
    return call_state_.WasCancelledPushed();
  }

  // Returns true if server trailing metadata has been pulled
  bool WasServerTrailingMetadataPulled() const {
    return call_state_.WasServerTrailingMetadataPulled();
  }

  // Client & server: fill in final_info with the final status of the call.
  void Finalize(const grpc_call_final_info* final_info);

  std::string DebugString() const;

 private:
  void CancelDueToFailedPipeOperation(SourceLocation but_where = {});

  struct AddedStack {
    explicit AddedStack(RefCountedPtr<Stack> stack)
        : call_data_offset(std::numeric_limits<size_t>::max()),
          stack(std::move(stack)) {}
    size_t call_data_offset;
    RefCountedPtr<Stack> stack;
  };

  using StacksVector = absl::InlinedVector<AddedStack, 2>;

  StacksVector stacks_;

  CallState call_state_;

  void* call_data_;
  ClientMetadataHandle push_client_initial_metadata_;
  ServerMetadataHandle push_server_initial_metadata_;
  MessageHandle push_client_to_server_message_;
  MessageHandle push_server_to_client_message_;
  ServerMetadataHandle push_server_trailing_metadata_;

  static char g_empty_call_data_;
};

static_assert(
    filters_detail::ArgumentMustBeNextMessage<
        absl::remove_cvref_t<decltype(std::declval<CallFilters*>()
                                          ->PullServerToClientMessage()()
                                          .value())>>::value(),
    "PullServerToClientMessage must return a NextMessage");

static_assert(
    filters_detail::ArgumentMustBeNextMessage<
        absl::remove_cvref_t<decltype(std::declval<CallFilters*>()
                                          ->PullClientToServerMessage()()
                                          .value())>>::value(),
    "PullServerToClientMessage must return a NextMessage");

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CALL_CALL_FILTERS_H
