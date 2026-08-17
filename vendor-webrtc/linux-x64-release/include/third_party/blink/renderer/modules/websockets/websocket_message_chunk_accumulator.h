// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBSOCKETS_WEBSOCKET_MESSAGE_CHUNK_ACCUMULATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBSOCKETS_WEBSOCKET_MESSAGE_CHUNK_ACCUMULATOR_H_

#include <memory>
#include "base/containers/span.h"
#include "base/memory/scoped_refptr.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace base {
class SingleThreadTaskRunner;
class TickClock;
}

namespace blink {

// WebSocketMessageChunkAccumulator stores chunks for one WebSocket message. A
// user can call Append() to append bytes, and call GetView() to get a list of
// base::spans of data previously stored.
// We don't use SharedBuffer due to an observed performance problem of FastFree.
// TODO(yhirano): Remove this once the performance problem is fixed in a general
// manner.
class MODULES_EXPORT WebSocketMessageChunkAccumulator final
    : public GarbageCollected<WebSocketMessageChunkAccumulator> {
 public:
  explicit WebSocketMessageChunkAccumulator(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);
  ~WebSocketMessageChunkAccumulator();

  // Appends |data| to this instance.
  void Append(base::span<const char> data);

  // Returns the number of bytes stored in this instance.
  size_t GetSize() const { return size_; }

  // Clears the stored data. Memory regions for chunks may be kept for future
  // uses for certain amount of time.
  void Clear();

  // Clear all stored data and cancel timers.
  void Reset();

  void Trace(Visitor*) const;

  // The regions will be available until Clear() is called.
  Vector<base::span<const char>> GetView() const;

  wtf_size_t GetPoolSizeForTesting() const { return pool_.size(); }
  bool IsTimerActiveForTesting() const { return timer_.IsActive(); }

  void SetTaskRunnerForTesting(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner,
      const base::TickClock* tick_clock);

  static constexpr size_t kSegmentSize = 16 * 1024;
  static constexpr base::TimeDelta kFreeDelay = base::Milliseconds(100);

 private:
  struct SegmentDeleter {
    void operator()(char* p) const { WTF::Partitions::FastFree(p); }
  };
  using SegmentPtr = std::unique_ptr<char[], SegmentDeleter>;
  static SegmentPtr CreateSegment() {
    return SegmentPtr(static_cast<char*>(WTF::Partitions::FastMalloc(
        kSegmentSize, "blink::WebSocketMessageChunkAccumulator::Segment")));
  }

  void OnTimerFired(TimerBase*);

  size_t GetLastSegmentSize() const {
    DCHECK(!segments_.empty());
    return size_ % kSegmentSize > 0 ? size_ % kSegmentSize : kSegmentSize;
  }

  Vector<SegmentPtr> segments_;
  Vector<SegmentPtr> pool_;
  size_t size_ = 0;
  wtf_size_t num_pooled_segments_to_be_removed_ = 0;
  HeapTaskRunnerTimer<WebSocketMessageChunkAccumulator> timer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBSOCKETS_WEBSOCKET_MESSAGE_CHUNK_ACCUMULATOR_H_
