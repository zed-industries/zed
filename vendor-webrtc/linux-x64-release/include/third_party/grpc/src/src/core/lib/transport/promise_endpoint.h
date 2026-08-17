// Copyright 2023 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_TRANSPORT_PROMISE_ENDPOINT_H
#define GRPC_SRC_CORE_LIB_TRANSPORT_PROMISE_ENDPOINT_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/event_engine/slice.h>
#include <grpc/event_engine/slice_buffer.h>
#include <grpc/slice_buffer.h>
#include <grpc/support/port_platform.h>
#include <stddef.h>
#include <stdint.h>

#include <atomic>
#include <cstring>
#include <functional>
#include <memory>
#include <optional>
#include <utility>

#include "absl/base/thread_annotations.h"
#include "absl/log/check.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "src/core/lib/event_engine/extensions/chaotic_good_extension.h"
#include "src/core/lib/event_engine/query_extensions.h"
#include "src/core/lib/iomgr/exec_ctx.h"
#include "src/core/lib/promise/activity.h"
#include "src/core/lib/promise/cancel_callback.h"
#include "src/core/lib/promise/if.h"
#include "src/core/lib/promise/map.h"
#include "src/core/lib/promise/poll.h"
#include "src/core/lib/slice/slice.h"
#include "src/core/lib/slice/slice_buffer.h"
#include "src/core/telemetry/context_list_entry.h"
#include "src/core/util/sync.h"

namespace grpc_core {

// Wrapper around event engine endpoint that provides a promise like API.
class PromiseEndpoint {
 public:
  using WriteArgs =
      grpc_event_engine::experimental::EventEngine::Endpoint::WriteArgs;

  PromiseEndpoint(
      std::unique_ptr<grpc_event_engine::experimental::EventEngine::Endpoint>
          endpoint,
      SliceBuffer already_received);
  PromiseEndpoint() = default;
  ~PromiseEndpoint() = default;
  /// Prevent copying of PromiseEndpoint; moving is fine.
  PromiseEndpoint(const PromiseEndpoint&) = delete;
  PromiseEndpoint& operator=(const PromiseEndpoint&) = delete;
  PromiseEndpoint(PromiseEndpoint&&) = default;
  PromiseEndpoint& operator=(PromiseEndpoint&&) = default;

  // Returns a promise that resolves to a `absl::Status` indicating the result
  // of the write operation.
  //
  // Concurrent writes are not supported, which means callers should not call
  // `Write()` before the previous write finishes. Doing that results in
  // undefined behavior.
  auto Write(SliceBuffer data, WriteArgs write_args) {
    GRPC_LATENT_SEE_PARENT_SCOPE("GRPC:Write");
    // Start write and assert previous write finishes.
    auto prev = write_state_->state.exchange(WriteState::kWriting,
                                             std::memory_order_relaxed);
    CHECK(prev == WriteState::kIdle);
    bool completed;
    if (data.Length() == 0) {
      completed = true;
    } else {
      // TODO(ladynana): Replace this with `SliceBufferCast<>` when it is
      // available.
      grpc_slice_buffer_swap(write_state_->buffer.c_slice_buffer(),
                             data.c_slice_buffer());
      // If `Write()` returns true immediately, the callback will not be called.
      // We still need to call our callback to pick up the result.
      write_state_->waker = GetContext<Activity>()->MakeNonOwningWaker();
      completed = endpoint_->Write(
          [write_state = write_state_](absl::Status status) {
            ExecCtx exec_ctx;
            write_state->Complete(std::move(status));
          },
          &write_state_->buffer, std::move(write_args));
      if (completed) write_state_->waker = Waker();
    }
    return If(
        completed,
        [this]() {
          return [write_state = write_state_]() {
            auto prev = write_state->state.exchange(WriteState::kIdle,
                                                    std::memory_order_relaxed);
            CHECK(prev == WriteState::kWriting);
            return absl::OkStatus();
          };
        },
        GRPC_LATENT_SEE_PROMISE(
            "DelayedWrite", ([this]() {
              return [write_state = write_state_]() -> Poll<absl::Status> {
                // If current write isn't finished return `Pending()`, else
                // return write result.
                WriteState::State expected = WriteState::kWritten;
                if (write_state->state.compare_exchange_strong(
                        expected, WriteState::kIdle, std::memory_order_acquire,
                        std::memory_order_relaxed)) {
                  // State was Written, and we changed it to Idle. We can return
                  // the result.
                  return std::move(write_state->result);
                }
                // State was not Written; since we're polling it must be
                // Writing. Assert that and return Pending.
                CHECK(expected == WriteState::kWriting);
                return Pending();
              };
            })));
  }

  // Returns a promise that resolves to `SliceBuffer` with
  // `num_bytes` bytes.
  //
  // Concurrent reads are not supported, which means callers should not call
  // `Read()` before the previous read finishes. Doing that results in
  // undefined behavior.
  auto Read(size_t num_bytes) {
    GRPC_LATENT_SEE_PARENT_SCOPE("GRPC:Read");
    // Assert previous read finishes.
    CHECK(!read_state_->complete.load(std::memory_order_relaxed));
    // Should not have pending reads.
    CHECK(read_state_->pending_buffer.Count() == 0u);
    bool complete = true;
    while (read_state_->buffer.Length() < num_bytes) {
      // Set read args with hinted bytes.
      grpc_event_engine::experimental::EventEngine::Endpoint::ReadArgs
          read_args;
      read_args.set_read_hint_bytes(
          static_cast<int64_t>(num_bytes - read_state_->buffer.Length()));
      // If `Read()` returns true immediately, the callback will not be
      // called.
      read_state_->waker = GetContext<Activity>()->MakeNonOwningWaker();
      if (endpoint_->Read(
              [read_state = read_state_, num_bytes](absl::Status status) {
                ExecCtx exec_ctx;
                read_state->Complete(std::move(status), num_bytes);
              },
              &read_state_->pending_buffer, std::move(read_args))) {
        read_state_->waker = Waker();
        read_state_->pending_buffer.MoveFirstNBytesIntoSliceBuffer(
            read_state_->pending_buffer.Length(), read_state_->buffer);
        DCHECK(read_state_->pending_buffer.Count() == 0u);
      } else {
        complete = false;
        break;
      }
    }
    return If(
        complete,
        [this, num_bytes]() {
          SliceBuffer ret;
          grpc_slice_buffer_move_first_no_inline(
              read_state_->buffer.c_slice_buffer(), num_bytes,
              ret.c_slice_buffer());
          return [ret = std::move(
                      ret)]() mutable -> Poll<absl::StatusOr<SliceBuffer>> {
            return std::move(ret);
          };
        },
        GRPC_LATENT_SEE_PROMISE(
            "DelayedRead", ([this, num_bytes]() {
              return [read_state = read_state_,
                      num_bytes]() -> Poll<absl::StatusOr<SliceBuffer>> {
                if (!read_state->complete.load(std::memory_order_acquire)) {
                  return Pending();
                }
                // If read succeeds, return `SliceBuffer` with `num_bytes`
                // bytes.
                if (read_state->result.ok()) {
                  SliceBuffer ret;
                  grpc_slice_buffer_move_first_no_inline(
                      read_state->buffer.c_slice_buffer(), num_bytes,
                      ret.c_slice_buffer());
                  read_state->complete.store(false, std::memory_order_relaxed);
                  return std::move(ret);
                }
                read_state->complete.store(false, std::memory_order_relaxed);
                return std::move(read_state->result);
              };
            })));
  }

  // Returns a promise that resolves to `Slice` with at least
  // `num_bytes` bytes which should be less than INT64_MAX bytes.
  //
  // Concurrent reads are not supported, which means callers should not call
  // `ReadSlice()` before the previous read finishes. Doing that results in
  // undefined behavior.
  auto ReadSlice(size_t num_bytes) {
    return Map(Read(num_bytes),
               [](absl::StatusOr<SliceBuffer> buffer) -> absl::StatusOr<Slice> {
                 if (!buffer.ok()) return buffer.status();
                 return buffer->JoinIntoSlice();
               });
  }

  // Returns a promise that resolves to a byte with type `uint8_t`.
  auto ReadByte() {
    return Map(ReadSlice(1),
               [](absl::StatusOr<Slice> slice) -> absl::StatusOr<uint8_t> {
                 if (!slice.ok()) return slice.status();
                 return (*slice)[0];
               });
  }

  // Enables RPC receive coalescing and alignment of memory holding received
  // RPCs.
  void EnforceRxMemoryAlignmentAndCoalescing() {
    auto* chaotic_good_ext = grpc_event_engine::experimental::QueryExtension<
        grpc_event_engine::experimental::ChaoticGoodExtension>(endpoint_.get());
    if (chaotic_good_ext != nullptr) {
      chaotic_good_ext->EnforceRxMemoryAlignment();
      chaotic_good_ext->EnableRpcReceiveCoalescing();
      if (read_state_->buffer.Length() == 0) {
        return;
      }

      // Copy everything from read_state_->buffer into a single slice and
      // replace the contents of read_state_->buffer with that slice.
      grpc_slice slice = grpc_slice_malloc_large(read_state_->buffer.Length());
      CHECK(reinterpret_cast<uintptr_t>(GRPC_SLICE_START_PTR(slice)) % 64 == 0);
      size_t ofs = 0;
      for (size_t i = 0; i < read_state_->buffer.Count(); i++) {
        memcpy(
            GRPC_SLICE_START_PTR(slice) + ofs,
            GRPC_SLICE_START_PTR(
                read_state_->buffer.c_slice_buffer()->slices[i]),
            GRPC_SLICE_LENGTH(read_state_->buffer.c_slice_buffer()->slices[i]));
        ofs +=
            GRPC_SLICE_LENGTH(read_state_->buffer.c_slice_buffer()->slices[i]);
      }

      read_state_->buffer.Clear();
      read_state_->buffer.AppendIndexed(
          grpc_event_engine::experimental::Slice(slice));
      DCHECK(read_state_->buffer.Length() == ofs);
    }
  }

  const grpc_event_engine::experimental::EventEngine::ResolvedAddress&
  GetPeerAddress() const;
  const grpc_event_engine::experimental::EventEngine::ResolvedAddress&
  GetLocalAddress() const;

  std::shared_ptr<grpc_event_engine::experimental::EventEngine::Endpoint>
  GetEventEngineEndpoint() const {
    return endpoint_;
  }

 private:
  std::shared_ptr<grpc_event_engine::experimental::EventEngine::Endpoint>
      endpoint_;

  struct ReadState : public RefCounted<ReadState> {
    std::atomic<bool> complete{false};
    // Read buffer used for storing successful reads given by
    // `EventEngine::Endpoint` but not yet requested by the caller.
    grpc_event_engine::experimental::SliceBuffer buffer;
    // Buffer used to accept data from `EventEngine::Endpoint`.
    // Every time after a successful read from `EventEngine::Endpoint`, the data
    // in this buffer should be appended to `buffer`.
    grpc_event_engine::experimental::SliceBuffer pending_buffer;
    // Used for store the result from `EventEngine::Endpoint::Read()`.
    absl::Status result;
    Waker waker;
    // Backing endpoint: we keep this on ReadState as reads will need to
    // repeatedly read until the target size is hit, and we don't want to access
    // the main object during this dance (indeed the main object may be
    // deleted).
    std::weak_ptr<grpc_event_engine::experimental::EventEngine::Endpoint>
        endpoint;

    void Complete(absl::Status status, size_t num_bytes_requested);
  };

  struct WriteState : public RefCounted<WriteState> {
    enum State : uint8_t {
      kIdle,     // Not writing.
      kWriting,  // Write started, but not completed.
      kWritten,  // Write completed.
    };

    std::atomic<State> state{kIdle};
    // Write buffer used for `EventEngine::Endpoint::Write()` to ensure the
    // memory behind the buffer is not lost.
    grpc_event_engine::experimental::SliceBuffer buffer;
    // Used for store the result from `EventEngine::Endpoint::Write()`.
    absl::Status result;
    Waker waker;

    void Complete(absl::Status status) {
      result = std::move(status);
      auto w = std::move(waker);
      auto prev = state.exchange(kWritten, std::memory_order_release);
      // Previous state should be Writing. If we got anything else we've entered
      // the callback path twice.
      CHECK(prev == kWriting);
      w.Wakeup();
    }
  };

  RefCountedPtr<WriteState> write_state_ = MakeRefCounted<WriteState>();
  RefCountedPtr<ReadState> read_state_ = MakeRefCounted<ReadState>();
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_TRANSPORT_PROMISE_ENDPOINT_H
