/*
 * Copyright (C) 2021 The Android Open Source Project
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

#ifndef SRC_BASE_LOG_RING_BUFFER_H_
#define SRC_BASE_LOG_RING_BUFFER_H_

#include <stddef.h>
#include <stdio.h>

#include <array>
#include <atomic>

#include "perfetto/ext/base/string_view.h"
#include "perfetto/ext/base/thread_annotations.h"

namespace perfetto {
namespace base {

// Defined out of line because a static constexpr requires static storage if
// ODR-used, not worth adding a .cc file just for tests.
constexpr size_t kLogRingBufEntries = 8;
constexpr size_t kLogRingBufMsgLen = 256;

// A static non-allocating ring-buffer to hold the most recent log events.
// This class is really an implementation detail of logging.cc. The only reason
// why is fully defined in a dedicated header is for allowing unittesting,
// without leaking extra headers into logging.h (which is a high-fanout header).
// This is used to report the last logs in a crash report when a CHECK/FATAL
// is encountered.
// This class has just an Append() method to insert events into the buffer and
// a Read() to read the events in FIFO order. Read() is non-destructive.
//
// Thread safety considerations:
// - The Append() method can be called concurrently by several threads, unless
//   there are > kLogRingBufEntries concurrent threads. Even if that happens,
//   case some events will contain a mix of strings but the behavior of
//   further Append() and Read() is still defined.
// - The Read() method is not thread safe but it's fine in practice. Even if
//   it's called concurrently with other Append(), it only causes some partial
//   events to be emitted in output.
// In both cases, we never rely purely on \0, all operations are size-bound.
//
// See logging_unittest.cc for tests.
class LogRingBuffer {
 public:
  LogRingBuffer() = default;
  LogRingBuffer(const LogRingBuffer&) = delete;
  LogRingBuffer& operator=(const LogRingBuffer&) = delete;
  LogRingBuffer(LogRingBuffer&&) = delete;
  LogRingBuffer& operator=(LogRingBuffer&&) = delete;

  // This takes three arguments because it fits its only caller (logging.cc).
  // The args are just concatenated together (plus one space before the msg).
  void Append(StringView tstamp, StringView source, StringView log_msg) {
    // Reserve atomically a slot in the ring buffer, so any concurrent Append()
    // won't overlap (unless too many concurrent Append() happen together).
    // There is no strict synchronization here, |event_slot_| is atomic only for
    // the sake of avoiding colliding on the same slot but does NOT guarantee
    // full consistency and integrity of the log messages written in each slot.
    // A release-store (or acq+rel) won't be enough for full consistency. Two
    // threads that race on Append() and take the N+1 and N+2 slots could finish
    // the write in reverse order. So Read() would need to synchronize with
    // something else (either a per-slot atomic flag or with a second atomic
    // counter which is incremented after the snprintf). Both options increase
    // the cost of Append() with no huge benefits (90% of the perfetto services
    // where we use it is single thread, and the log ring buffer is disabled
    // on non-standalone builds like the SDK).
    uint32_t slot = event_slot_.fetch_add(1, std::memory_order_relaxed);
    slot = slot % kLogRingBufEntries;

    char* const msg = events_[slot];
    PERFETTO_ANNOTATE_BENIGN_RACE_SIZED(msg, kLogRingBufMsgLen,
                                        "see comments in log_ring_buffer.h")
    snprintf(msg, kLogRingBufMsgLen, "%.*s%.*s %.*s",
             static_cast<int>(tstamp.size()), tstamp.data(),
             static_cast<int>(source.size()), source.data(),
             static_cast<int>(log_msg.size()), log_msg.data());
  }

  // Reads back the buffer in FIFO order, up to |len - 1| characters at most
  // (the -1 is because a NUL terminator is always appended, unless |len| == 0).
  // The string written in |dst| is guaranteed to be NUL-terminated, even if
  // |len| < buffer contents length.
  // Returns the number of bytes written in output, excluding the \0 terminator.
  size_t Read(char* dst, size_t len) {
    if (len == 0)
      return 0;
    // This is a relaxed-load because we don't need to fully synchronize on the
    // writing path for the reasons described in the fetch_add() above.
    const uint32_t event_slot = event_slot_.load(std::memory_order_relaxed);
    size_t dst_written = 0;
    for (uint32_t pos = 0; pos < kLogRingBufEntries; ++pos) {
      const uint32_t slot = (event_slot + pos) % kLogRingBufEntries;
      const char* src = events_[slot];
      if (*src == '\0')
        continue;  // Empty slot. Skip.
      char* const wptr = dst + dst_written;
      // |src| might not be null terminated. This can happen if some
      // thread-race happened. Limit the copy length.
      const size_t limit = std::min(len - dst_written, kLogRingBufMsgLen);
      for (size_t i = 0; i < limit; ++i) {
        const char c = src[i];
        ++dst_written;
        if (c == '\0' || i == limit - 1) {
          wptr[i] = '\n';
          break;
        }
        // Skip non-printable ASCII characters to avoid confusing crash reports.
        // Note that this deliberately mangles \n. Log messages should not have
        // a \n in the middle and are NOT \n terminated. The trailing \n between
        // each line is appended by the if () branch above.
        const bool is_printable = c >= ' ' && c <= '~';
        wptr[i] = is_printable ? c : '?';
      }
    }
    // Ensure that the output string is null-terminated.
    PERFETTO_DCHECK(dst_written <= len);
    if (dst_written == len) {
      // In case of truncation we replace the last char with \0. But the return
      // value is the number of chars without \0, hence the --.
      dst[--dst_written] = '\0';
    } else {
      dst[dst_written] = '\0';
    }
    return dst_written;
  }

 private:
  using EventBuf = char[kLogRingBufMsgLen];
  EventBuf events_[kLogRingBufEntries]{};

  static_assert((kLogRingBufEntries & (kLogRingBufEntries - 1)) == 0,
                "kLogRingBufEntries must be a power of two");

  // A monotonically increasing counter incremented on each event written.
  // It determines which of the kLogRingBufEntries indexes in |events_| should
  // be used next.
  // It grows >> kLogRingBufEntries, it's supposed to be always used
  // mod(kLogRingBufEntries). A static_assert in the .cc file ensures that
  // kLogRingBufEntries is a power of two so wraps are aligned.
  std::atomic<uint32_t> event_slot_{};
};

}  // namespace base
}  // namespace perfetto

#endif  // SRC_BASE_LOG_RING_BUFFER_H_
