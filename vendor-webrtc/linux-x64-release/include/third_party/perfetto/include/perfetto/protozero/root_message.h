/*
 * Copyright (C) 2020 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_PROTOZERO_ROOT_MESSAGE_H_
#define INCLUDE_PERFETTO_PROTOZERO_ROOT_MESSAGE_H_

#include "perfetto/protozero/message.h"
#include "perfetto/protozero/message_arena.h"

namespace protozero {

// Helper class to hand out messages using the default MessageArena.
// Usage:
// RootMessage<perfetto::protos::zero::MyMessage> msg;
// msg.Reset(stream_writer);
// msg.set_foo(...);
// auto* nested = msg.set_nested();
template <typename T = Message>
class RootMessage : public T {
 public:
  RootMessage() { T::Reset(nullptr, &root_arena_); }

  // Disallow copy and move.
  RootMessage(const RootMessage&) = delete;
  RootMessage& operator=(const RootMessage&) = delete;
  RootMessage(RootMessage&&) = delete;
  RootMessage& operator=(RootMessage&&) = delete;

  void Reset(ScatteredStreamWriter* writer) {
    root_arena_.Reset();
    Message::Reset(writer, &root_arena_);
  }

 private:
  MessageArena root_arena_;
};

}  // namespace protozero

#endif  // INCLUDE_PERFETTO_PROTOZERO_ROOT_MESSAGE_H_
