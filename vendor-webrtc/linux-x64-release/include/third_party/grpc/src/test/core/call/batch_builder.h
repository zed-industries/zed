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

#ifndef GRPC_TEST_CORE_CALL_BATCH_BUILDER_H
#define GRPC_TEST_CORE_CALL_BATCH_BUILDER_H

#include "absl/strings/str_cat.h"
#include "gtest/gtest.h"
#include "src/core/lib/slice/slice.h"
#include "test/core/end2end/cq_verifier.h"

namespace grpc_core {

using ByteBufferUniquePtr =
    std::unique_ptr<grpc_byte_buffer, void (*)(grpc_byte_buffer*)>;
ByteBufferUniquePtr ByteBufferFromSlice(Slice slice);

std::optional<std::string> FindInMetadataArray(const grpc_metadata_array& md,
                                               absl::string_view key);

// Receiving container for incoming metadata.
class IncomingMetadata final : public CqVerifier::SuccessfulStateString {
 public:
  IncomingMetadata() = default;
  ~IncomingMetadata() {
    if (metadata_ != nullptr) grpc_metadata_array_destroy(metadata_.get());
  }

  // Lookup a metadata value by key.
  std::optional<std::string> Get(absl::string_view key) const;

  // Make a GRPC_RECV_INITIAL_METADATA op - intended for the framework, not
  // for tests.
  grpc_op MakeOp();

  std::string GetSuccessfulStateString() override;

 private:
  std::unique_ptr<grpc_metadata_array> metadata_ =
      std::make_unique<grpc_metadata_array>(grpc_metadata_array{0, 0, nullptr});
};

// Receiving container for one incoming message.
class IncomingMessage final : public CqVerifier::SuccessfulStateString {
 public:
  IncomingMessage() = default;
  IncomingMessage(const IncomingMessage&) = delete;
  IncomingMessage& operator=(const IncomingMessage&) = delete;
  ~IncomingMessage() {
    if (payload_ != nullptr) grpc_byte_buffer_destroy(payload_);
  }

  // Get the payload of the message - concatenated together into a string for
  // easy verification.
  std::string payload() const;
  // Check if the message is the end of the stream.
  bool is_end_of_stream() const { return payload_ == nullptr; }
  // Get the type of the message.
  grpc_byte_buffer_type byte_buffer_type() const { return payload_->type; }
  // Get the compression algorithm used for the message.
  grpc_compression_algorithm compression() const {
    return payload_->data.raw.compression;
  }
  std::string GetSuccessfulStateString() override;

  // Make a GRPC_OP_RECV_MESSAGE op - intended for the framework, not for
  // tests.
  grpc_op MakeOp();

  // Accessor for CoreEnd2endTest::IncomingCall - get a pointer to the
  // underlying payload.
  // We don't want to use this in tests directly.
  grpc_byte_buffer** raw_payload_ptr() { return &payload_; }

 private:
  grpc_byte_buffer* payload_ = nullptr;
};

// Receiving container for incoming status on the client from the server.
class IncomingStatusOnClient final : public CqVerifier::SuccessfulStateString {
 public:
  IncomingStatusOnClient() = default;
  IncomingStatusOnClient(const IncomingStatusOnClient&) = delete;
  IncomingStatusOnClient& operator=(const IncomingStatusOnClient&) = delete;
  IncomingStatusOnClient(IncomingStatusOnClient&& other) noexcept = default;
  IncomingStatusOnClient& operator=(IncomingStatusOnClient&& other) noexcept =
      default;
  ~IncomingStatusOnClient() {
    if (data_ != nullptr) {
      grpc_metadata_array_destroy(&data_->trailing_metadata);
      gpr_free(const_cast<char*>(data_->error_string));
    }
  }

  // Get the status code.
  grpc_status_code status() const { return data_->status; }
  // Get the status details.
  std::string message() const {
    return std::string(data_->status_details.as_string_view());
  }
  // Get the error string.
  std::string error_string() const {
    return data_->error_string == nullptr ? "" : data_->error_string;
  }
  // Get a trailing metadata value by key.
  std::optional<std::string> GetTrailingMetadata(absl::string_view key) const;

  std::string GetSuccessfulStateString() override;

  // Make a GRPC_OP_RECV_STATUS_ON_CLIENT op - intended for the framework, not
  // for tests.
  grpc_op MakeOp();

 private:
  struct Data {
    grpc_metadata_array trailing_metadata{0, 0, nullptr};
    grpc_status_code status;
    Slice status_details;
    const char* error_string = nullptr;
  };
  std::unique_ptr<Data> data_ = std::make_unique<Data>();
};

// Receiving container for incoming status on the server from the client.
class IncomingCloseOnServer final : public CqVerifier::SuccessfulStateString {
 public:
  IncomingCloseOnServer() = default;
  IncomingCloseOnServer(const IncomingCloseOnServer&) = delete;
  IncomingCloseOnServer& operator=(const IncomingCloseOnServer&) = delete;

  // Get the cancellation bit.
  bool was_cancelled() const { return cancelled_ != 0; }

  // Make a GRPC_OP_RECV_CLOSE_ON_SERVER op - intended for the framework, not
  // for tests.
  grpc_op MakeOp();

  std::string GetSuccessfulStateString() override {
    return absl::StrCat("close_on_server: cancelled=", cancelled_);
  }

 private:
  int cancelled_;
};

// Build one batch. Returned from NewBatch (use that to instantiate this!)
// Upon destruction of the BatchBuilder, the batch will be executed with any
// added batches.
class BatchBuilder {
 public:
  BatchBuilder(grpc_call* call, CqVerifier* cq_verifier, int tag)
      : call_(call), tag_(tag), cq_verifier_(cq_verifier) {
    cq_verifier_->ClearSuccessfulStateStrings(CqVerifier::tag(tag_));
  }
  ~BatchBuilder();

  BatchBuilder(const BatchBuilder&) = delete;
  BatchBuilder& operator=(const BatchBuilder&) = delete;
  BatchBuilder(BatchBuilder&&) noexcept = default;

  // Add a GRPC_OP_SEND_INITIAL_METADATA op.
  // Optionally specify flags, compression level.
  BatchBuilder& SendInitialMetadata(
      std::initializer_list<std::pair<absl::string_view, absl::string_view>> md,
      uint32_t flags = 0,
      std::optional<grpc_compression_level> compression_level = std::nullopt);

  // Add a GRPC_OP_SEND_MESSAGE op.
  BatchBuilder& SendMessage(Slice payload, uint32_t flags = 0);
  BatchBuilder& SendMessage(absl::string_view payload, uint32_t flags = 0) {
    return SendMessage(Slice::FromCopiedString(payload), flags);
  }

  // Add a GRPC_OP_SEND_CLOSE_FROM_CLIENT op.
  BatchBuilder& SendCloseFromClient();

  // Add a GRPC_OP_SEND_STATUS_FROM_SERVER op.
  BatchBuilder& SendStatusFromServer(
      grpc_status_code status, absl::string_view message,
      std::initializer_list<std::pair<absl::string_view, absl::string_view>>
          md);

  // Add a GRPC_OP_RECV_INITIAL_METADATA op.
  BatchBuilder& RecvInitialMetadata(IncomingMetadata& md) {
    cq_verifier_->AddSuccessfulStateString(CqVerifier::tag(tag_), &md);
    ops_.emplace_back(md.MakeOp());
    return *this;
  }

  // Add a GRPC_OP_RECV_MESSAGE op.
  BatchBuilder& RecvMessage(IncomingMessage& msg) {
    cq_verifier_->AddSuccessfulStateString(CqVerifier::tag(tag_), &msg);
    ops_.emplace_back(msg.MakeOp());
    return *this;
  }

  // Add a GRPC_OP_RECV_STATUS_ON_CLIENT op.
  BatchBuilder& RecvStatusOnClient(IncomingStatusOnClient& status) {
    cq_verifier_->AddSuccessfulStateString(CqVerifier::tag(tag_), &status);
    ops_.emplace_back(status.MakeOp());
    return *this;
  }

  // Add a GRPC_OP_RECV_CLOSE_ON_SERVER op.
  BatchBuilder& RecvCloseOnServer(IncomingCloseOnServer& close) {
    cq_verifier_->AddSuccessfulStateString(CqVerifier::tag(tag_), &close);
    ops_.emplace_back(close.MakeOp());
    return *this;
  }

 private:
  // We need to track little bits of memory up until the batch is executed.
  // One Thing is one such block of memory.
  // We specialize it with SpecificThing to track a specific type of memory.
  // These get placed on things_ and deleted when the batch is executed.
  class Thing {
   public:
    virtual ~Thing() = default;
  };
  template <typename T>
  class SpecificThing final : public Thing {
   public:
    template <typename... Args>
    explicit SpecificThing(Args&&... args) : t_(std::forward<Args>(args)...) {}
    SpecificThing() = default;

    T& get() { return t_; }

   private:
    T t_;
  };

  // Make a thing of type T, and return a reference to it.
  template <typename T, typename... Args>
  T& Make(Args&&... args) {
    things_.emplace_back(new SpecificThing<T>(std::forward<Args>(args)...));
    return static_cast<SpecificThing<T>*>(things_.back().get())->get();
  }

  grpc_call* call_;
  const int tag_;
  std::vector<grpc_op> ops_;
  std::vector<std::unique_ptr<Thing>> things_;
  CqVerifier* const cq_verifier_;
};

}  // namespace grpc_core

#endif  // GRPC_TEST_CORE_CALL_BATCH_BUILDER_H
