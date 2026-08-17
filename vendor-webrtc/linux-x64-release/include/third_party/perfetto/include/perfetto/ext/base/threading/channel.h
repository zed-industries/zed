/*
 * Copyright (C) 2022 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_EXT_BASE_THREADING_CHANNEL_H_
#define INCLUDE_PERFETTO_EXT_BASE_THREADING_CHANNEL_H_

#include <mutex>
#include <optional>

#include "perfetto/base/compiler.h"
#include "perfetto/base/platform_handle.h"
#include "perfetto/base/thread_annotations.h"
#include "perfetto/ext/base/circular_queue.h"
#include "perfetto/ext/base/event_fd.h"

namespace perfetto {
namespace base {

// Unidirectional conduit used to send values between threads with a fixed-sized
// buffer in-between.
//
// When a channel is read from when empty or written to when full, the operation
// will not succeed and the caller can choose to a) abandon the operation,
// or b) use |read_fd| or |write_fd| (as appropriate) which will be become
// "ready" (i.e. base::TaskRunner watches will fire) when the operation would
// succeed.
//
// A channel is very similar to a Unix pipe except with the values being sent
// a) not needing to be serializable b) data does not go through the kernel.
template <typename T>
class Channel {
 public:
  struct ReadResult {
    ReadResult(std::optional<T> _item, bool _is_closed)
        : item(std::move(_item)), is_closed(_is_closed) {}

    bool operator==(const ReadResult& res) const {
      return item == res.item && is_closed == res.is_closed;
    }

    // The item read from the channel or std::nullopt if the channel is empty.
    // If so, callers can use |read_fd| to be notified when a read operation
    // would succeed.
    std::optional<T> item;

    // Indicates the channel is closed. Readers can continue to read from the
    // channel and any buffered elements will be correctly returned. Moreover,
    // any future reads will also have |is_closed| == true and |read_fd| will be
    // ready forever.
    //
    // Once a ReadResult is returned with |item| == std::nullopt and
    // |is_closed| == true, no further values will ever be returned.
    bool is_closed;
  };
  struct WriteResult {
    WriteResult(bool _success, bool _is_closed)
        : success(std::move(_success)), is_closed(_is_closed) {}

    bool operator==(const WriteResult& res) const {
      return success == res.success && is_closed == res.is_closed;
    }

    // Returns whether the write to the channel was successful. If this is
    // false, callers can use |write_fd| to be notified when future writes
    // would succeed. Note that callers should also check |is_closed| as another
    // writer may have closed the channel.
    bool success;

    // Indicates that the channel is closed. If this value is true, |success|
    // will be |false| Moreover, any further writes will continue to return
    // |success| == false, |is_closed| == true and |write_fd| will be ready
    // forever.
    bool is_closed;
  };

  // Creates a channel with a capacity at least as large as |capacity_hint|. The
  // capacity *must* be greater than zero.
  //
  // Note that it's possible that a capacity > |capacity_hint| will be chosen:
  // it is implementation defined when this might happen.
  explicit Channel(uint32_t capacity_hint) : elements_(capacity_hint) {
    PERFETTO_DCHECK(capacity_hint > 0);

    // It's very important that we make sure |write_fd| is ready to avoid
    // deadlocks.
    write_fd_.Notify();
  }

  // Attempts to read from the channel and returns the result of the attempt.
  // See |ReadResult| for more information on the result.
  PERFETTO_WARN_UNUSED_RESULT ReadResult ReadNonBlocking() {
    std::lock_guard<std::mutex> lock(mutex_);
    if (elements_.empty()) {
      return ReadResult(std::nullopt, is_closed_);
    }
    if (elements_.capacity() == elements_.size()) {
      write_fd_.Notify();
    }
    T value = std::move(elements_.front());
    elements_.pop_front();
    if (!is_closed_ && elements_.empty()) {
      read_fd_.Clear();
    }
    return ReadResult(std::move(value), is_closed_);
  }

  // Attempts to write to the channel and returns the result of the attempt.
  // See |WriteResult| for more information on the result.
  //
  // IMPORTANT: if this function returns |success| == false, |element| *will
  // not* be modified. This allows the caller to try again with the same value.
  PERFETTO_WARN_UNUSED_RESULT WriteResult WriteNonBlocking(T&& element) {
    std::lock_guard<std::mutex> lock(mutex_);
    if (is_closed_) {
      return WriteResult{false, true};
    }
    if (elements_.size() == elements_.capacity()) {
      return WriteResult{false, false};
    }
    if (elements_.empty()) {
      read_fd_.Notify();
    }
    elements_.emplace_back(std::move(element));
    if (elements_.size() == elements_.capacity()) {
      write_fd_.Clear();
    }
    return WriteResult{true, false};
  }

  // Closes the channel for to any further writes.
  //
  // Note: this function will make both |read_fd| and |write_fd| ready to
  // avoid deadlocks. Callers should correctly handle |is_closed| being
  // false from |ReadNonBlocking| and |WriteNonBlocking| to stop watching the
  // fds to avoid poll returning immediately.
  //
  // We prefer this behaviour as it's a lot more obvious something is wrong when
  // it spins and takes 100% CPU rather than silently deadlocking.
  void Close() {
    std::lock_guard<std::mutex> lock(mutex_);
    is_closed_ = true;

    // Make both fds ready to avoid deadlocks.
    read_fd_.Notify();
    write_fd_.Notify();
  }

  // Notification FD for when |ReadNonBlocking| would succeed. Can be useful to
  // pass to AddFileDescriptorWatch to read data from the channel.
  base::PlatformHandle read_fd() const { return read_fd_.fd(); }

  // Notification FD for when |WriteNonBlocking| would succeed. Can be useful to
  // pass to AddFileDescriptorWatch to send data through the channel.
  base::PlatformHandle write_fd() const { return write_fd_.fd(); }

 private:
  std::mutex mutex_;
  base::CircularQueue<T> elements_ PERFETTO_GUARDED_BY(mutex_);
  bool is_closed_ PERFETTO_GUARDED_BY(mutex_) = false;

  base::EventFd read_fd_;
  base::EventFd write_fd_;
};

}  // namespace base
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_BASE_THREADING_CHANNEL_H_
