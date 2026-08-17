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

#ifndef INCLUDE_PERFETTO_PROTOZERO_MESSAGE_ARENA_H_
#define INCLUDE_PERFETTO_PROTOZERO_MESSAGE_ARENA_H_

#include <stdint.h>

#include <forward_list>
#include <type_traits>

#include "perfetto/base/export.h"
#include "perfetto/base/logging.h"
#include "perfetto/protozero/message.h"

namespace protozero {

class Message;

// Object allocator for fixed-sized protozero::Message objects.
// It's a simple bump-pointer allocator which leverages the stack-alike
// usage pattern of protozero nested messages. It avoids hitting the system
// allocator in most cases, by reusing the same block, and falls back on
// allocating new blocks only when using deeply nested messages (which are
// extremely rare).
// This is used by RootMessage<T> to handle the storage for root-level messages.
class PERFETTO_EXPORT_COMPONENT MessageArena {
 public:
  MessageArena();
  ~MessageArena();

  // Strictly no copies or moves as this is used to hand out pointers.
  MessageArena(const MessageArena&) = delete;
  MessageArena& operator=(const MessageArena&) = delete;
  MessageArena(MessageArena&&) = delete;
  MessageArena& operator=(MessageArena&&) = delete;

  // Allocates a new Message object.
  Message* NewMessage();

  // Deletes the last message allocated. The |msg| argument is used only for
  // DCHECKs, it MUST be the pointer obtained by the last NewMessage() call.
  void DeleteLastMessage(Message* msg) {
    PERFETTO_DCHECK(!blocks_.empty() && blocks_.front().entries > 0);
    PERFETTO_DCHECK(blocks_.front().storage[blocks_.front().entries - 1] ==
                    static_cast<void*>(msg));
    DeleteLastMessageInternal();
  }

  // Resets the state of the arena, clearing up all but one block. This is used
  // to avoid leaking outstanding unfinished sub-messages while recycling the
  // RootMessage object (this is extremely rare due to the RAII scoped handles
  // but could happen if some client does some overly clever std::move() trick).
  void Reset() {
    PERFETTO_DCHECK(!blocks_.empty());
    blocks_.resize(1);
    auto& block = blocks_.front();
    block.entries = 0;
    PERFETTO_ASAN_POISON(block.storage, sizeof(block.storage));
  }

 private:
  void DeleteLastMessageInternal();

  struct Block {
    static constexpr size_t kCapacity = 16;

    Block() { PERFETTO_ASAN_POISON(storage, sizeof(storage)); }

    alignas(Message) char storage[kCapacity][sizeof(Message)];
    uint32_t entries = 0;  // # Message entries used (<= kCapacity).
  };

  // blocks are used to hand out pointers and must not be moved. Hence why
  // std::list rather than std::vector.
  std::forward_list<Block> blocks_;
};

}  // namespace protozero

#endif  // INCLUDE_PERFETTO_PROTOZERO_MESSAGE_ARENA_H_
