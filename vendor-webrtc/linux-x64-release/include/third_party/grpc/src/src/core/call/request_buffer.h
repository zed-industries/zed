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

#ifndef GRPC_SRC_CORE_CALL_REQUEST_BUFFER_H
#define GRPC_SRC_CORE_CALL_REQUEST_BUFFER_H

#include <utility>

#include "src/core/call/call_spine.h"
#include "src/core/call/message.h"
#include "src/core/call/metadata.h"

namespace grpc_core {

// Outbound request buffer.
// Collects client->server metadata and messages whilst in its initial buffering
// mode. In buffering mode it can have zero or more Reader objects attached to
// it.
// The buffer can later be switched to committed mode, at which point it
// will have exactly one Reader object attached to it.
// Callers can choose to switch to committed mode based upon policy of their
// choice.
class RequestBuffer {
 public:
  // One reader of the request buffer.
  class Reader {
   public:
    explicit Reader(RequestBuffer* buffer) ABSL_LOCKS_EXCLUDED(buffer->mu_)
        : buffer_(buffer) {
      buffer->AddReader(this);
    }
    ~Reader() ABSL_LOCKS_EXCLUDED(buffer_->mu_) { buffer_->RemoveReader(this); }

    Reader(const Reader&) = delete;
    Reader& operator=(const Reader&) = delete;

    // Pull client initial metadata. Returns a promise that resolves to
    // ValueOrFailure<ClientMetadataHandle>.
    GRPC_MUST_USE_RESULT auto PullClientInitialMetadata() {
      return [this]() { return PollPullClientInitialMetadata(); };
    }
    // Pull a message. Returns a promise that resolves to a
    // ValueOrFailure<std::optional<MessageHandle>>.
    GRPC_MUST_USE_RESULT auto PullMessage() {
      return [this]() { return PollPullMessage(); };
    }

    absl::Status TakeError() { return std::move(error_); }

   private:
    friend class RequestBuffer;

    Poll<ValueOrFailure<ClientMetadataHandle>> PollPullClientInitialMetadata();
    Poll<ValueOrFailure<std::optional<MessageHandle>>> PollPullMessage();

    template <typename T>
    T ClaimObject(T& object) ABSL_EXCLUSIVE_LOCKS_REQUIRED(buffer_->mu_) {
      if (buffer_->winner_ == this) return std::move(object);
      return CopyObject(object);
    }

    ClientMetadataHandle CopyObject(const ClientMetadataHandle& md) {
      return Arena::MakePooled<ClientMetadata>(md->Copy());
    }

    MessageHandle CopyObject(const MessageHandle& msg) {
      return Arena::MakePooled<Message>(msg->payload()->Copy(), msg->flags());
    }

    RequestBuffer* const buffer_;
    bool pulled_client_initial_metadata_ = false;
    size_t message_index_ = 0;
    absl::Status error_;
    Waker pull_waker_;
  };

  RequestBuffer();

  // Push ClientInitialMetadata into the buffer.
  // This is instantaneous, and returns success with the amount of data
  // buffered, or failure.
  ValueOrFailure<size_t> PushClientInitialMetadata(ClientMetadataHandle md);
  // Resolves to a ValueOrFailure<size_t> where the size_t is the amount of data
  // buffered (or 0 if we're in committed mode).
  GRPC_MUST_USE_RESULT auto PushMessage(MessageHandle message) {
    return [this, message = std::move(message)]() mutable {
      return PollPushMessage(message);
    };
  }
  // Push end of stream (client half-closure).
  StatusFlag FinishSends();
  // Cancel the request, propagate failure to all readers.
  void Cancel(absl::Status error = absl::CancelledError());

  // Switch to committed mode - needs to be called exactly once with the winning
  // reader. All other readers will see failure.
  void Commit(Reader* winner);

  bool committed() const {
    MutexLock lock(&mu_);
    return winner_ != nullptr;
  }

 private:
  // Buffering state: we're collecting metadata and messages.
  struct Buffering {
    Buffering();
    // Initial metadata, or nullptr if not yet received.
    ClientMetadataHandle initial_metadata;
    // Buffered messages.
    absl::InlinedVector<MessageHandle, 1> messages;
    // Amount of data buffered.
    size_t buffered = 0;
  };
  // Buffered state: all messages have been collected (the client has finished
  // sending).
  struct Buffered {
    Buffered(ClientMetadataHandle md,
             absl::InlinedVector<MessageHandle, 1> msgs)
        : initial_metadata(std::move(md)), messages(std::move(msgs)) {}
    ClientMetadataHandle initial_metadata;
    absl::InlinedVector<MessageHandle, 1> messages;
  };
  // Streaming state: we're streaming messages to the server.
  // This implies winner_ is set.
  struct Streaming {
    MessageHandle message;
    bool end_of_stream = false;
  };
  // Cancelled state: the request has been cancelled.
  struct Cancelled {
    explicit Cancelled(absl::Status error) : error(std::move(error)) {}
    absl::Status error;
  };
  using State = std::variant<Buffering, Buffered, Streaming, Cancelled>;

  Poll<ValueOrFailure<size_t>> PollPushMessage(MessageHandle& message);
  Pending PendingPull(Reader* reader) ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_) {
    reader->pull_waker_ = Activity::current()->MakeOwningWaker();
    return Pending{};
  }
  Pending PendingPush() ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_) {
    push_waker_ = Activity::current()->MakeOwningWaker();
    return Pending{};
  }
  void MaybeSwitchToStreaming() ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_) {
    auto& buffering = std::get<Buffering>(state_);
    if (winner_ == nullptr) return;
    if (winner_->message_index_ < buffering.messages.size()) return;
    state_.emplace<Streaming>();
    push_waker_.Wakeup();
  }

  void WakeupAsyncAllPullers() ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_) {
    WakeupAsyncAllPullersExcept(nullptr);
  }
  void WakeupAsyncAllPullersExcept(Reader* except_reader)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_);

  void AddReader(Reader* reader) ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_) {
    readers_.insert(reader);
  }

  void RemoveReader(Reader* reader) ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_) {
    readers_.erase(reader);
  }

  std::string DebugString(Reader* caller) ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_);

  mutable Mutex mu_;
  Reader* winner_ ABSL_GUARDED_BY(mu_){nullptr};
  State state_ ABSL_GUARDED_BY(mu_);
  // TODO(ctiller): change this to an intrusively linked list to avoid
  // allocations.
  absl::flat_hash_set<Reader*> readers_ ABSL_GUARDED_BY(mu_);
  Waker push_waker_ ABSL_GUARDED_BY(mu_);
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CALL_REQUEST_BUFFER_H
