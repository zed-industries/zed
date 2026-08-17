/*
 * Copyright (C) 2019 The Android Open Source Project
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

#ifndef SRC_PROFILING_MEMORY_SHARED_RING_BUFFER_H_
#define SRC_PROFILING_MEMORY_SHARED_RING_BUFFER_H_
#include <optional>

#include "perfetto/ext/base/unix_socket.h"
#include "perfetto/ext/base/utils.h"
#include "src/profiling/memory/scoped_spinlock.h"
#include "src/profiling/memory/util.h"

#include <atomic>
#include <limits>
#include <map>
#include <memory>
#include <type_traits>

#include <stdint.h>

namespace perfetto {
namespace profiling {

// A concurrent, multi-writer single-reader ring buffer FIFO, based on a
// circular buffer over shared memory. It has similar semantics to a SEQ_PACKET
// + O_NONBLOCK socket, specifically:
//
// - Writes are atomic, data is either written fully in the buffer or not.
// - New writes are discarded if the buffer is full.
// - If a write succeeds, the reader is guaranteed to see the whole buffer.
// - Reads are atomic, no fragmentation.
// - The reader sees writes in write order (% discarding).
//
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// *IMPORTANT*: The ring buffer must be written under the assumption that the
// other end modifies arbitrary shared memory without holding the spin-lock.
// This means we must make local copies of read and write pointers for doing
// bounds checks followed by reads / writes, as they might change in the
// meantime.
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
//
// TODO:
// - Write a benchmark.
class SharedRingBuffer {
 public:
  class Buffer {
   public:
    Buffer() {}
    Buffer(uint8_t* d, size_t s, uint64_t f)
        : data(d), size(s), bytes_free(f) {}

    Buffer(const Buffer&) = delete;
    Buffer& operator=(const Buffer&) = delete;

    Buffer(Buffer&&) = default;
    Buffer& operator=(Buffer&&) = default;

    explicit operator bool() const { return data != nullptr; }

    uint8_t* data = nullptr;
    size_t size = 0;
    uint64_t bytes_free = 0;
  };

  enum ErrorState : uint64_t {
    kNoError = 0,
    kHitTimeout = 1,
    kInvalidStackBounds = 2,
  };

  struct Stats {
    PERFETTO_CROSS_ABI_ALIGNED(uint64_t) bytes_written;
    PERFETTO_CROSS_ABI_ALIGNED(uint64_t) num_writes_succeeded;
    PERFETTO_CROSS_ABI_ALIGNED(uint64_t) num_writes_corrupt;
    PERFETTO_CROSS_ABI_ALIGNED(uint64_t) num_writes_overflow;

    PERFETTO_CROSS_ABI_ALIGNED(uint64_t) num_reads_succeeded;
    PERFETTO_CROSS_ABI_ALIGNED(uint64_t) num_reads_corrupt;
    PERFETTO_CROSS_ABI_ALIGNED(uint64_t) num_reads_nodata;

    // Fields below get set by GetStats as copies of atomics in MetadataPage.
    PERFETTO_CROSS_ABI_ALIGNED(uint64_t) failed_spinlocks;
    PERFETTO_CROSS_ABI_ALIGNED(uint64_t) client_spinlock_blocked_us;
    PERFETTO_CROSS_ABI_ALIGNED(ErrorState) error_state;
  };

  static std::optional<SharedRingBuffer> Create(size_t);
  static std::optional<SharedRingBuffer> Attach(base::ScopedFile);

  ~SharedRingBuffer();
  SharedRingBuffer() = default;

  SharedRingBuffer(SharedRingBuffer&&) noexcept;
  SharedRingBuffer& operator=(SharedRingBuffer&&) noexcept;

  bool is_valid() const { return !!mem_; }
  size_t size() const { return size_; }
  int fd() const { return *mem_fd_; }
  size_t write_avail() {
    auto pos = GetPointerPositions();
    if (!pos)
      return 0;
    return write_avail(*pos);
  }
  size_t read_avail() {
    auto pos = GetPointerPositions();
    if (!pos)
      return 0;
    return read_avail(*pos);
  }

  Buffer BeginWrite(const ScopedSpinlock& spinlock, size_t size);
  void EndWrite(Buffer buf);

  Buffer BeginRead();
  // Returns the number bytes read from the shared memory buffer. This is
  // different than the number of bytes returned in the Buffer, because it
  // includes the header size.
  size_t EndRead(Buffer);

  Stats GetStats(ScopedSpinlock& spinlock) {
    PERFETTO_DCHECK(spinlock.locked());
    Stats stats = meta_->stats;
    stats.failed_spinlocks =
        meta_->failed_spinlocks.load(std::memory_order_relaxed);
    stats.error_state = meta_->error_state.load(std::memory_order_relaxed);
    stats.client_spinlock_blocked_us =
        meta_->client_spinlock_blocked_us.load(std::memory_order_relaxed);
    return stats;
  }

  void SetErrorState(ErrorState error) { meta_->error_state.store(error); }

  // This is used by the caller to be able to hold the SpinLock after
  // BeginWrite has returned. This is so that additional bookkeeping can be
  // done under the lock. This will be used to increment the sequence_number.
  ScopedSpinlock AcquireLock(ScopedSpinlock::Mode mode) {
    auto lock = ScopedSpinlock(&meta_->spinlock, mode);
    if (PERFETTO_UNLIKELY(!lock.locked()))
      meta_->failed_spinlocks.fetch_add(1, std::memory_order_relaxed);
    return lock;
  }

  void AddClientSpinlockBlockedUs(size_t n) {
    meta_->client_spinlock_blocked_us.fetch_add(n, std::memory_order_relaxed);
  }

  uint64_t client_spinlock_blocked_us() {
    return meta_->client_spinlock_blocked_us;
  }

  void SetShuttingDown() {
    meta_->shutting_down.store(true, std::memory_order_relaxed);
  }

  bool shutting_down() {
    return meta_->shutting_down.load(std::memory_order_relaxed);
  }

  void SetReaderPaused() {
    meta_->reader_paused.store(true, std::memory_order_relaxed);
  }

  bool GetAndResetReaderPaused() {
    return meta_->reader_paused.exchange(false, std::memory_order_relaxed);
  }

  void InfiniteBufferForTesting() {
    // Pretend this buffer is really large, while keeping size_mask_ as
    // original so it keeps wrapping in circles.
    size_ = std::numeric_limits<size_t>::max() / 2;
  }

  // Exposed for fuzzers.
  struct MetadataPage {
    alignas(8) Spinlock spinlock;
    PERFETTO_CROSS_ABI_ALIGNED(std::atomic<uint64_t>) read_pos;
    PERFETTO_CROSS_ABI_ALIGNED(std::atomic<uint64_t>) write_pos;

    PERFETTO_CROSS_ABI_ALIGNED(std::atomic<uint64_t>)
    client_spinlock_blocked_us;
    PERFETTO_CROSS_ABI_ALIGNED(std::atomic<uint64_t>) failed_spinlocks;
    PERFETTO_CROSS_ABI_ALIGNED(std::atomic<ErrorState>) error_state;
    alignas(sizeof(uint64_t)) std::atomic<bool> shutting_down;
    alignas(sizeof(uint64_t)) std::atomic<bool> reader_paused;
    // For stats that are only accessed by a single thread or under the
    // spinlock, members of this struct are directly modified. Other stats use
    // the atomics above this struct.
    //
    // When the user requests stats, the atomics above get copied into this
    // struct, which is then returned.
    alignas(sizeof(uint64_t)) Stats stats;
  };

  static_assert(sizeof(MetadataPage) == 144,
                "metadata page size needs to be ABI independent");

 private:
  struct PointerPositions {
    uint64_t read_pos;
    uint64_t write_pos;
  };

  struct CreateFlag {};
  struct AttachFlag {};
  SharedRingBuffer(const SharedRingBuffer&) = delete;
  SharedRingBuffer& operator=(const SharedRingBuffer&) = delete;
  SharedRingBuffer(CreateFlag, size_t size);
  SharedRingBuffer(AttachFlag, base::ScopedFile mem_fd) {
    Initialize(std::move(mem_fd));
  }

  void Initialize(base::ScopedFile mem_fd);
  bool IsCorrupt(const PointerPositions& pos);

  inline std::optional<PointerPositions> GetPointerPositions() {
    PointerPositions pos;
    // We need to acquire load the write_pos to make sure we observe a
    // consistent ring buffer in BeginRead, otherwise it is possible that we
    // observe the write_pos increment, but not the size field write of the
    // payload.
    //
    // This is matched by a release at the end of BeginWrite.
    pos.write_pos = meta_->write_pos.load(std::memory_order_acquire);
    pos.read_pos = meta_->read_pos.load(std::memory_order_relaxed);

    std::optional<PointerPositions> result;
    if (IsCorrupt(pos))
      return result;
    result = pos;
    return result;
  }

  inline void set_size(size_t size) {
    size_ = size;
    size_mask_ = size - 1;
  }

  inline size_t read_avail(const PointerPositions& pos) {
    PERFETTO_DCHECK(pos.write_pos >= pos.read_pos);
    auto res = static_cast<size_t>(pos.write_pos - pos.read_pos);
    PERFETTO_DCHECK(res <= size_);
    return res;
  }

  inline size_t write_avail(const PointerPositions& pos) {
    return size_ - read_avail(pos);
  }

  inline uint8_t* at(uint64_t pos) { return mem_ + (pos & size_mask_); }

  base::ScopedFile mem_fd_;
  MetadataPage* meta_ = nullptr;  // Start of the mmaped region.
  uint8_t* mem_ = nullptr;  // Start of the contents (i.e. meta_ + pagesize).

  // Size of the ring buffer contents, without including metadata or the 2nd
  // mmap.
  size_t size_ = 0;
  size_t size_mask_ = 0;

  // Remember to update the move ctor when adding new fields.
};

}  // namespace profiling
}  // namespace perfetto

#endif  // SRC_PROFILING_MEMORY_SHARED_RING_BUFFER_H_
