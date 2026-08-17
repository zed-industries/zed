/*
 * Copyright (C) 2017 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef INCLUDE_PERFETTO_PROTOZERO_MESSAGE_HANDLE_H_
#define INCLUDE_PERFETTO_PROTOZERO_MESSAGE_HANDLE_H_

#include <functional>

#include "perfetto/base/export.h"
#include "perfetto/protozero/message.h"
#include "perfetto/protozero/scattered_stream_writer.h"

namespace protozero {

class Message;

class PERFETTO_EXPORT_COMPONENT MessageFinalizationListener {
 public:
  virtual ~MessageFinalizationListener();
  virtual void OnMessageFinalized(Message* message) = 0;
};

// MessageHandle allows to decouple the lifetime of a proto message
// from the underlying storage. It gives the following guarantees:
// - The underlying message is finalized (if still alive) if the handle goes
//   out of scope.
// - In Debug / DCHECK_ALWAYS_ON builds, the handle becomes null once the
//   message is finalized. This is to enforce the append-only API. For instance
//   when adding two repeated messages, the addition of the 2nd one forces
//   the finalization of the first.
// Think about this as a WeakPtr<Message> which calls
// Message::Finalize() when going out of scope.

class PERFETTO_EXPORT_COMPONENT MessageHandleBase {
 public:
  ~MessageHandleBase() {
    if (message_) {
#if PERFETTO_DCHECK_IS_ON()
      PERFETTO_DCHECK(generation_ == message_->generation_);
#endif
      FinalizeMessage();
    }
  }

  // Move-only type.
  MessageHandleBase(MessageHandleBase&& other) noexcept {
    Move(std::move(other));
  }

  MessageHandleBase& operator=(MessageHandleBase&& other) noexcept {
    // If the current handle was pointing to a message and is being reset to a
    // new one, finalize the old message. However, if the other message is the
    // same as the one we point to, don't finalize.
    if (message_ && message_ != other.message_)
      FinalizeMessage();
    Move(std::move(other));
    return *this;
  }

  explicit operator bool() const {
#if PERFETTO_DCHECK_IS_ON()
    PERFETTO_DCHECK(!message_ || generation_ == message_->generation_);
#endif
    return !!message_;
  }

  void set_finalization_listener(MessageFinalizationListener* listener) {
    listener_ = listener;
  }

  // Returns a (non-owned, it should not be deleted) pointer to the
  // ScatteredStreamWriter used to write the message data. The Message becomes
  // unusable after this point.
  //
  // The caller can now write directly, without using protozero::Message.
  ScatteredStreamWriter* TakeStreamWriter() {
    ScatteredStreamWriter* stream_writer = message_->stream_writer_;
#if PERFETTO_DCHECK_IS_ON()
    message_->set_handle(nullptr);
#endif
    message_ = nullptr;
    listener_ = nullptr;
    return stream_writer;
  }

 protected:
  explicit MessageHandleBase(Message* message = nullptr) : message_(message) {
#if PERFETTO_DCHECK_IS_ON()
    generation_ = message_ ? message->generation_ : 0;
    if (message_)
      message_->set_handle(this);
#endif
  }

  Message* operator->() const {
#if PERFETTO_DCHECK_IS_ON()
    PERFETTO_DCHECK(!message_ || generation_ == message_->generation_);
#endif
    return message_;
  }
  Message& operator*() const { return *(operator->()); }

 private:
  friend class Message;
  MessageHandleBase(const MessageHandleBase&) = delete;
  MessageHandleBase& operator=(const MessageHandleBase&) = delete;

  void reset_message() {
    // This is called by Message::Finalize().
    PERFETTO_DCHECK(message_->is_finalized());
    message_ = nullptr;
    listener_ = nullptr;
  }

  void Move(MessageHandleBase&& other) {
    message_ = other.message_;
    other.message_ = nullptr;
    listener_ = other.listener_;
    other.listener_ = nullptr;
#if PERFETTO_DCHECK_IS_ON()
    if (message_) {
      generation_ = message_->generation_;
      message_->set_handle(this);
    }
#endif
  }

  void FinalizeMessage() {
    // |message_| and |listener_| may be cleared by reset_message() during
    // Message::Finalize().
    auto* listener = listener_;
    auto* message = message_;
    message->Finalize();
    if (listener)
      listener->OnMessageFinalized(message);
  }

  Message* message_;
  MessageFinalizationListener* listener_ = nullptr;
#if PERFETTO_DCHECK_IS_ON()
  uint32_t generation_;
#endif
};

template <typename T>
class MessageHandle : public MessageHandleBase {
 public:
  MessageHandle() : MessageHandle(nullptr) {}
  explicit MessageHandle(T* message) : MessageHandleBase(message) {}

  explicit operator bool() const { return MessageHandleBase::operator bool(); }

  T& operator*() const {
    return static_cast<T&>(MessageHandleBase::operator*());
  }

  T* operator->() const {
    return static_cast<T*>(MessageHandleBase::operator->());
  }

  T* get() const { return static_cast<T*>(MessageHandleBase::operator->()); }
};

}  // namespace protozero

#endif  // INCLUDE_PERFETTO_PROTOZERO_MESSAGE_HANDLE_H_
