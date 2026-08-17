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

#ifndef INCLUDE_PERFETTO_PROTOZERO_SCATTERED_STREAM_WRITER_H_
#define INCLUDE_PERFETTO_PROTOZERO_SCATTERED_STREAM_WRITER_H_

#include <assert.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>

#include <algorithm>

#include "perfetto/base/compiler.h"
#include "perfetto/base/export.h"
#include "perfetto/base/logging.h"
#include "perfetto/protozero/contiguous_memory_range.h"

namespace protozero {

// This class deals with the following problem: append-only proto messages want
// to write a stream of bytes, without caring about the implementation of the
// underlying buffer (which concretely will be either the trace ring buffer
// or a heap-allocated buffer). The main deal is: proto messages don't know in
// advance what their size will be.
// Due to the tracing buffer being split into fixed-size chunks, on some
// occasions, these writes need to be spread over two (or more) non-contiguous
// chunks of memory. Similarly, when the buffer is backed by the heap, we want
// to avoid realloc() calls, as they might cause a full copy of the contents
// of the buffer.
// The purpose of this class is to abstract away the non-contiguous write logic.
// This class knows how to deal with writes as long as they fall in the same
// ContiguousMemoryRange and defers the chunk-chaining logic to the Delegate.
class PERFETTO_EXPORT_COMPONENT ScatteredStreamWriter {
 public:
  class PERFETTO_EXPORT_COMPONENT Delegate {
   public:
    static constexpr size_t kPatchSize = 4;
    virtual ~Delegate();

    // Returns a new chunk for writing.
    virtual ContiguousMemoryRange GetNewBuffer() = 0;

    // Signals the delegate that the location pointed by `to_patch` (which must
    // be in the last chunk returned by GetNewBuffer()), kPatchSize long, needs
    // to be updated later (after potentially multiple GetNewBuffer calls).
    //
    // The caller must write to the returned location later. If the returned
    // pointer is nullptr, the caller should not write anything.
    //
    // The implementation considers the patch ready to apply when the caller
    // writes the first byte a value that's different than 0 (the
    // implementation periodically checks for this).
    virtual uint8_t* AnnotatePatch(uint8_t* patch_addr);
  };

  explicit ScatteredStreamWriter(Delegate* delegate);
  ~ScatteredStreamWriter();

  inline void WriteByte(uint8_t value) {
    if (write_ptr_ >= cur_range_.end)
      Extend();
    *write_ptr_++ = value;
  }

  // Assumes that the caller checked that there is enough headroom.
  // TODO(primiano): perf optimization, this is a tracing hot path. The
  // compiler can make strong optimization on std::copy if the size arg is a
  // constexpr. Make a templated variant of this for fixed-size writes.
  // TODO(primiano): restrict / noalias might also help.
  inline void WriteBytesUnsafe(const uint8_t* src, size_t size) {
    uint8_t* const end = write_ptr_ + size;
    assert(end <= cur_range_.end);
    std::copy(src, src + size, write_ptr_);
    write_ptr_ = end;
  }

  inline void WriteBytes(const uint8_t* src,
                         size_t size) PERFETTO_NO_SANITIZE_UNDEFINED {
    // If the stream writer hasn't been initialized, constructing the end
    // pointer below invokes undefined behavior because `write_ptr_` is null.
    // Since this function is on the hot path, we suppress the warning instead
    // of adding a conditional branch.
    uint8_t* const end = write_ptr_ + size;
    if (PERFETTO_LIKELY(end <= cur_range_.end))
      return WriteBytesUnsafe(src, size);
    WriteBytesSlowPath(src, size);
  }

  void WriteBytesSlowPath(const uint8_t* src, size_t size);

  // Reserves a fixed amount of bytes to be backfilled later. The reserved range
  // is guaranteed to be contiguous and not span across chunks. |size| has to be
  // <= than the size of a new buffer returned by the Delegate::GetNewBuffer().
  uint8_t* ReserveBytes(size_t size);

  // Fast (but unsafe) version of the above. The caller must have previously
  // checked that there are at least |size| contiguous bytes available.
  // Returns only the start pointer of the reservation.
  uint8_t* ReserveBytesUnsafe(size_t size) {
    uint8_t* begin = write_ptr_;
    write_ptr_ += size;
    assert(write_ptr_ <= cur_range_.end);
    return begin;
  }

  // Shifts the previously written `size` bytes backwards in memory by `offset`
  // bytes, moving the write pointer back accordingly. The shifted result must
  // still be fully contained by the current range.
  void Rewind(size_t size, size_t offset) {
    uint8_t* src = write_ptr_ - size;
    uint8_t* dst = src - offset;
    PERFETTO_DCHECK(src >= cur_range_.begin);
    PERFETTO_DCHECK(src + size <= cur_range_.end);
    PERFETTO_DCHECK(dst >= cur_range_.begin);
    PERFETTO_DCHECK(dst + size <= cur_range_.end);
    memmove(dst, src, size);
    write_ptr_ -= offset;
  }

  // Resets the buffer boundaries and the write pointer to the given |range|.
  // Subsequent WriteByte(s) will write into |range|.
  void Reset(ContiguousMemoryRange range);

  // Commits the current chunk and gets a new chunk from the delegate.
  void Extend();

  // Number of contiguous free bytes in |cur_range_| that can be written without
  // requesting a new buffer.
  size_t bytes_available() const {
    return static_cast<size_t>(cur_range_.end - write_ptr_);
  }

  ContiguousMemoryRange cur_range() const { return cur_range_; }

  uint8_t* write_ptr() const { return write_ptr_; }

  void set_write_ptr(uint8_t* write_ptr) {
    assert(cur_range_.begin <= write_ptr && write_ptr <= cur_range_.end);
    write_ptr_ = write_ptr;
  }

  uint64_t written() const {
    return written_previously_ +
           static_cast<uint64_t>(write_ptr_ - cur_range_.begin);
  }

  uint64_t written_previously() const { return written_previously_; }

  uint8_t* AnnotatePatch(uint8_t* patch_addr) {
    return delegate_->AnnotatePatch(patch_addr);
  }

 private:
  ScatteredStreamWriter(const ScatteredStreamWriter&) = delete;
  ScatteredStreamWriter& operator=(const ScatteredStreamWriter&) = delete;

  Delegate* const delegate_;
  ContiguousMemoryRange cur_range_;
  uint8_t* write_ptr_;
  uint64_t written_previously_ = 0;
};

}  // namespace protozero

#endif  // INCLUDE_PERFETTO_PROTOZERO_SCATTERED_STREAM_WRITER_H_
