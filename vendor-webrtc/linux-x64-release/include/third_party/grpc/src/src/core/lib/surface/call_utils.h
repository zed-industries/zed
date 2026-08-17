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

#ifndef GRPC_SRC_CORE_LIB_SURFACE_CALL_UTILS_H
#define GRPC_SRC_CORE_LIB_SURFACE_CALL_UTILS_H

#include <grpc/byte_buffer.h>
#include <grpc/compression.h>
#include <grpc/event_engine/event_engine.h>
#include <grpc/grpc.h>
#include <grpc/impl/call.h>
#include <grpc/impl/propagation_bits.h>
#include <grpc/slice.h>
#include <grpc/slice_buffer.h>
#include <grpc/status.h>
#include <grpc/support/alloc.h>
#include <grpc/support/atm.h>
#include <grpc/support/port_platform.h>
#include <grpc/support/string_util.h>
#include <inttypes.h>
#include <limits.h>
#include <stdlib.h>
#include <string.h>

#include <algorithm>
#include <atomic>
#include <cstdint>
#include <string>
#include <type_traits>
#include <utility>

#include "absl/log/check.h"
#include "absl/status/status.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "src/core/call/message.h"
#include "src/core/call/metadata.h"
#include "src/core/call/metadata_batch.h"
#include "src/core/lib/promise/activity.h"
#include "src/core/lib/promise/cancel_callback.h"
#include "src/core/lib/promise/map.h"
#include "src/core/lib/promise/poll.h"
#include "src/core/lib/promise/seq.h"
#include "src/core/lib/promise/status_flag.h"
#include "src/core/lib/surface/completion_queue.h"
#include "src/core/util/crash.h"

namespace grpc_core {

class PublishToAppEncoder {
 public:
  explicit PublishToAppEncoder(grpc_metadata_array* dest,
                               const grpc_metadata_batch* encoding,
                               bool is_client)
      : dest_(dest), encoding_(encoding), is_client_(is_client) {}

  void Encode(const Slice& key, const Slice& value) {
    Append(key.c_slice(), value.c_slice());
  }

  // Catch anything that is not explicitly handled, and do not publish it to the
  // application. If new metadata is added to a batch that needs to be
  // published, it should be called out here.
  template <typename Which>
  void Encode(Which, const typename Which::ValueType&) {}

  void Encode(UserAgentMetadata, const Slice& slice) {
    Append(UserAgentMetadata::key(), slice);
  }

  void Encode(HostMetadata, const Slice& slice) {
    Append(HostMetadata::key(), slice);
  }

  void Encode(GrpcPreviousRpcAttemptsMetadata, uint32_t count) {
    Append(GrpcPreviousRpcAttemptsMetadata::key(), count);
  }

  void Encode(GrpcRetryPushbackMsMetadata, Duration count) {
    Append(GrpcRetryPushbackMsMetadata::key(), count.millis());
  }

  void Encode(LbTokenMetadata, const Slice& slice) {
    Append(LbTokenMetadata::key(), slice);
  }

  void Encode(W3CTraceParentMetadata, const Slice& slice) {
    Append(W3CTraceParentMetadata::key(), slice);
  }

 private:
  void Append(absl::string_view key, int64_t value) {
    Append(StaticSlice::FromStaticString(key).c_slice(),
           Slice::FromInt64(value).c_slice());
  }

  void Append(absl::string_view key, const Slice& value) {
    Append(StaticSlice::FromStaticString(key).c_slice(), value.c_slice());
  }

  void Append(grpc_slice key, grpc_slice value) {
    if (dest_->count == dest_->capacity) {
      Crash(absl::StrCat(
          "Too many metadata entries: capacity=", dest_->capacity, " on ",
          is_client_ ? "client" : "server", " encoding ", encoding_->count(),
          " elements: ", encoding_->DebugString().c_str()));
    }
    auto* mdusr = &dest_->metadata[dest_->count++];
    mdusr->key = key;
    mdusr->value = value;
  }

  grpc_metadata_array* const dest_;
  const grpc_metadata_batch* const encoding_;
  const bool is_client_;
};

void PublishMetadataArray(grpc_metadata_batch* md, grpc_metadata_array* array,
                          bool is_client);
void CToMetadata(grpc_metadata* metadata, size_t count, grpc_metadata_batch* b);
const char* GrpcOpTypeName(grpc_op_type op);

bool ValidateMetadata(size_t count, grpc_metadata* metadata);
void EndOpImmediately(grpc_completion_queue* cq, void* notify_tag,
                      bool is_notify_tag_closure);

inline bool AreWriteFlagsValid(uint32_t flags) {
  // check that only bits in GRPC_WRITE_(INTERNAL?)_USED_MASK are set
  const uint32_t allowed_write_positions =
      (GRPC_WRITE_USED_MASK | GRPC_WRITE_INTERNAL_USED_MASK);
  const uint32_t invalid_positions = ~allowed_write_positions;
  return !(flags & invalid_positions);
}

inline bool AreInitialMetadataFlagsValid(uint32_t flags) {
  // check that only bits in GRPC_WRITE_(INTERNAL?)_USED_MASK are set
  uint32_t invalid_positions = ~GRPC_INITIAL_METADATA_USED_MASK;
  return !(flags & invalid_positions);
}

// One batch operation
// Wrapper around promise steps to perform once of the batch operations for the
// legacy grpc surface api.
template <typename SetupResult, grpc_op_type kOp>
class OpHandlerImpl {
 public:
  using PromiseFactory = promise_detail::OncePromiseFactory<void, SetupResult>;
  using Promise = typename PromiseFactory::Promise;
  static_assert(!std::is_same<Promise, void>::value,
                "PromiseFactory must return a promise");

  OpHandlerImpl() : state_(State::kDismissed) {}
  explicit OpHandlerImpl(SetupResult result) : state_(State::kPromiseFactory) {
    Construct(&promise_factory_, std::move(result));
  }

  ~OpHandlerImpl() {
    switch (state_) {
      case State::kDismissed:
        break;
      case State::kPromiseFactory:
        Destruct(&promise_factory_);
        break;
      case State::kPromise:
        Destruct(&promise_);
        break;
    }
  }

  OpHandlerImpl(const OpHandlerImpl&) = delete;
  OpHandlerImpl& operator=(const OpHandlerImpl&) = delete;
  OpHandlerImpl(OpHandlerImpl&& other) noexcept : state_(other.state_) {
    switch (state_) {
      case State::kDismissed:
        break;
      case State::kPromiseFactory:
        Construct(&promise_factory_, std::move(other.promise_factory_));
        break;
      case State::kPromise:
        Construct(&promise_, std::move(other.promise_));
        break;
    }
  }
  OpHandlerImpl& operator=(OpHandlerImpl&& other) noexcept = delete;

  Poll<StatusFlag> operator()() {
    switch (state_) {
      case State::kDismissed:
        GRPC_TRACE_LOG(call, INFO)
            << Activity::current()->DebugTag() << "Dismissed " << OpName();
        return Success{};
      case State::kPromiseFactory: {
        GRPC_TRACE_LOG(call, INFO)
            << Activity::current()->DebugTag() << "Construct " << OpName();
        auto promise = promise_factory_.Make();
        Destruct(&promise_factory_);
        Construct(&promise_, std::move(promise));
        state_ = State::kPromise;
      }
        [[fallthrough]];
      case State::kPromise: {
        GRPC_TRACE_LOG(call, INFO)
            << Activity::current()->DebugTag() << "BeginPoll " << OpName();
        auto r = poll_cast<StatusFlag>(promise_());
        GRPC_TRACE_LOG(call, INFO)
            << Activity::current()->DebugTag() << "EndPoll " << OpName()
            << " --> "
            << (r.pending() ? "PENDING" : (r.value().ok() ? "OK" : "FAILURE"));
        return r;
      }
    }
    GPR_UNREACHABLE_CODE(return Pending{});
  }

 private:
  enum class State {
    kDismissed,
    kPromiseFactory,
    kPromise,
  };

  static const char* OpName() { return GrpcOpTypeName(kOp); }

  // gcc-12 has problems with this being a variant
  GPR_NO_UNIQUE_ADDRESS State state_;
  union {
    PromiseFactory promise_factory_;
    Promise promise_;
  };
};

template <grpc_op_type op_type, typename PromiseFactory>
auto OpHandler(PromiseFactory setup) {
  return OpHandlerImpl<PromiseFactory, op_type>(std::move(setup));
}

class BatchOpIndex {
 public:
  BatchOpIndex(const grpc_op* ops, size_t nops) : ops_(ops) {
    for (size_t i = 0; i < nops; i++) {
      idxs_[ops[i].op] = static_cast<uint8_t>(i);
    }
  }

  // 1. Check if op_type is in the batch
  // 2. If it is, run the setup function in the context of the API call (NOT in
  //    the call party).
  // 3. This setup function returns a promise factory which we'll then run *in*
  //    the party to do initial setup, and have it return the promise that we'll
  //    ultimately poll on til completion.
  // Once we express our surface API in terms of core internal types this whole
  // dance will go away.
  template <grpc_op_type op_type, typename SetupFn>
  auto OpHandler(SetupFn setup) {
    using SetupResult = decltype(std::declval<SetupFn>()(grpc_op()));
    using Impl = OpHandlerImpl<SetupResult, op_type>;
    if (const grpc_op* op = this->op(op_type)) {
      auto r = setup(*op);
      return Impl(std::move(r));
    } else {
      return Impl();
    }
  }

  const grpc_op* op(grpc_op_type op_type) const {
    return idxs_[op_type] == 255 ? nullptr : &ops_[idxs_[op_type]];
  }

  bool has_op(grpc_op_type op_type) const { return idxs_[op_type] != 255; }

 private:
  const grpc_op* const ops_;
  std::array<uint8_t, 8> idxs_{255, 255, 255, 255, 255, 255, 255, 255};
};

// Defines a promise that calls grpc_cq_end_op() (on first poll) and then waits
// for the callback supplied to grpc_cq_end_op() to be called, before resolving
// to Empty{}
class WaitForCqEndOp {
 public:
  WaitForCqEndOp(bool is_closure, void* tag, grpc_error_handle error,
                 grpc_completion_queue* cq)
      : state_{NotStarted{is_closure, tag, std::move(error), cq}} {}

  Poll<Empty> operator()();

  WaitForCqEndOp(const WaitForCqEndOp&) = delete;
  WaitForCqEndOp& operator=(const WaitForCqEndOp&) = delete;
  WaitForCqEndOp(WaitForCqEndOp&& other) noexcept
      : state_(std::move(std::get<NotStarted>(other.state_))) {
    other.state_.emplace<Invalid>();
  }
  WaitForCqEndOp& operator=(WaitForCqEndOp&& other) noexcept {
    state_ = std::move(std::get<NotStarted>(other.state_));
    other.state_.emplace<Invalid>();
    return *this;
  }

 private:
  struct NotStarted {
    bool is_closure;
    void* tag;
    grpc_error_handle error;
    grpc_completion_queue* cq;
  };
  struct Started {
    explicit Started(Waker waker) : waker(std::move(waker)) {}
    Waker waker;
    grpc_cq_completion completion;
    std::atomic<bool> done{false};
  };
  struct Invalid {};
  using State = std::variant<NotStarted, Started, Invalid>;

  static std::string StateString(const State& state);

  State state_{Invalid{}};
};

template <typename FalliblePart, typename FinalPart>
auto InfallibleBatch(FalliblePart fallible_part, FinalPart final_part,
                     bool is_notify_tag_closure, void* notify_tag,
                     grpc_completion_queue* cq) {
  // Perform fallible_part, then final_part, then wait for the
  // completion queue to be done.
  // If cancelled, we'll ensure the completion queue is notified.
  // There's a slight bug here in that if we cancel this promise after
  // the WaitForCqEndOp we'll double post -- but we don't currently do that.
  return OnCancelFactory(
      [fallible_part = std::move(fallible_part),
       final_part = std::move(final_part), is_notify_tag_closure, notify_tag,
       cq]() mutable {
        return LogPollBatch(notify_tag,
                            Seq(std::move(fallible_part), std::move(final_part),
                                [is_notify_tag_closure, notify_tag, cq]() {
                                  return WaitForCqEndOp(is_notify_tag_closure,
                                                        notify_tag,
                                                        absl::OkStatus(), cq);
                                }));
      },
      [cq, notify_tag]() {
        grpc_cq_end_op(
            cq, notify_tag, absl::OkStatus(),
            [](void*, grpc_cq_completion* completion) { delete completion; },
            nullptr, new grpc_cq_completion);
      });
}

template <typename FalliblePart>
auto FallibleBatch(FalliblePart fallible_part, bool is_notify_tag_closure,
                   void* notify_tag, grpc_completion_queue* cq) {
  // Perform fallible_part, then wait for the completion queue to be done.
  // If cancelled, we'll ensure the completion queue is notified.
  // There's a slight bug here in that if we cancel this promise after
  // the WaitForCqEndOp we'll double post -- but we don't currently do that.
  return OnCancelFactory(
      [fallible_part = std::move(fallible_part), is_notify_tag_closure,
       notify_tag, cq]() mutable {
        return LogPollBatch(
            notify_tag,
            Seq(std::move(fallible_part),
                [is_notify_tag_closure, notify_tag, cq](StatusFlag r) {
                  return WaitForCqEndOp(is_notify_tag_closure, notify_tag,
                                        StatusCast<absl::Status>(r), cq);
                }));
      },
      [cq]() {
        grpc_cq_end_op(
            cq, nullptr, absl::CancelledError(),
            [](void*, grpc_cq_completion* completion) { delete completion; },
            nullptr, new grpc_cq_completion);
      });
}

template <typename F>
class PollBatchLogger {
 public:
  PollBatchLogger(void* tag, F f) : tag_(tag), f_(std::move(f)) {}

  auto operator()() {
    GRPC_TRACE_LOG(call, INFO) << "Poll batch " << tag_;
    auto r = f_();
    GRPC_TRACE_LOG(call, INFO)
        << "Poll batch " << tag_ << " --> " << ResultString(r);
    return r;
  }

 private:
  template <typename T>
  static std::string ResultString(Poll<T> r) {
    if (r.pending()) return "PENDING";
    return ResultString(r.value());
  }
  static std::string ResultString(Empty) { return "DONE"; }

  void* tag_;
  F f_;
};

template <typename F>
PollBatchLogger<F> LogPollBatch(void* tag, F f) {
  return PollBatchLogger<F>(tag, std::move(f));
}

class MessageReceiver {
 public:
  grpc_compression_algorithm incoming_compression_algorithm() const {
    return incoming_compression_algorithm_;
  }

  void SetIncomingCompressionAlgorithm(
      grpc_compression_algorithm incoming_compression_algorithm) {
    incoming_compression_algorithm_ = incoming_compression_algorithm;
  }

  uint32_t last_message_flags() const { return test_only_last_message_flags_; }

  template <typename Puller>
  auto MakeBatchOp(const grpc_op& op, Puller* puller) {
    CHECK_EQ(recv_message_, nullptr);
    recv_message_ = op.data.recv_message.recv_message;
    return [this, puller]() mutable {
      return Map(puller->PullMessage(),
                 [this](typename Puller::NextMessage msg) {
                   return FinishRecvMessage(std::move(msg));
                 });
    };
  }

 private:
  template <typename NextMessage>
  StatusFlag FinishRecvMessage(NextMessage result) {
    if (!result.ok()) {
      GRPC_TRACE_LOG(call, INFO)
          << Activity::current()->DebugTag()
          << "[call] RecvMessage: outstanding_recv "
             "finishes: received end-of-stream with error";
      *recv_message_ = nullptr;
      recv_message_ = nullptr;
      return Failure{};
    }
    if (!result.has_value()) {
      GRPC_TRACE_LOG(call, INFO) << Activity::current()->DebugTag()
                                 << "[call] RecvMessage: outstanding_recv "
                                    "finishes: received end-of-stream";
      *recv_message_ = nullptr;
      recv_message_ = nullptr;
      return Success{};
    }
    MessageHandle message = result.TakeValue();
    test_only_last_message_flags_ = message->flags();
    if ((message->flags() & GRPC_WRITE_INTERNAL_COMPRESS) &&
        (incoming_compression_algorithm_ != GRPC_COMPRESS_NONE)) {
      *recv_message_ = grpc_raw_compressed_byte_buffer_create(
          nullptr, 0, incoming_compression_algorithm_);
    } else {
      *recv_message_ = grpc_raw_byte_buffer_create(nullptr, 0);
    }
    grpc_slice_buffer_move_into(message->payload()->c_slice_buffer(),
                                &(*recv_message_)->data.raw.slice_buffer);
    GRPC_TRACE_LOG(call, INFO)
        << Activity::current()->DebugTag()
        << "[call] RecvMessage: outstanding_recv "
           "finishes: received "
        << (*recv_message_)->data.raw.slice_buffer.length << " byte message";
    recv_message_ = nullptr;
    return Success{};
  }

  grpc_byte_buffer** recv_message_ = nullptr;
  uint32_t test_only_last_message_flags_ = 0;
  // Compression algorithm for incoming data
  grpc_compression_algorithm incoming_compression_algorithm_ =
      GRPC_COMPRESS_NONE;
};

std::string MakeErrorString(const ServerMetadata* trailing_metadata);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_SURFACE_CALL_UTILS_H
