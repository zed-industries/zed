// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_RW_BUFFER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_RW_BUFFER_H_

#include "base/functional/callback.h"
#include "base/memory/raw_ptr.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"

namespace blink {

class ROBuffer;

/**
 * Accumulates bytes of memory that are "appended" to it, growing internal
 * storage as needed. The growth is done such that at any time in the writer's
 * thread, an ROBuffer or StreamAsset can be snapped off (and safely passed to
 * another thread). The ROBuffer/StreamAsset snapshot can see the previously
 * stored bytes, but will be unaware of any future writes.
 */
class PLATFORM_EXPORT RWBuffer {
 public:
  struct BufferHead;
  struct BufferBlock;
  class PLATFORM_EXPORT ROIter {
   public:
    explicit ROIter(RWBuffer*, size_t);

    // Returns a span of the current continuous block of memory. The span will
    // be empty if the iterator is exhausted.
    base::span<const uint8_t> operator*() const;
    // Checks whether there is another block available and advances the iterator
    // if there is.
    bool Next();
    // Checks whether there is another block available. Does not advance the
    // iterator.
    bool HasNext() const;

   private:
    raw_ptr<const RWBuffer> rw_buffer_;
    raw_ptr<const RWBuffer::BufferBlock> block_;
    size_t remaining_;
  };

  explicit RWBuffer(size_t initialCapacity = 0);
  // |writer| is a function used to initialize the RWBuffer.
  // |writer| is responsible for not writing off the edge of the buffer.
  // |writer| should return the amount of memory written to the buffer.
  RWBuffer(base::OnceCallback<size_t(base::span<uint8_t>)> writer,
           size_t initial_capacity);

  ~RWBuffer();

  RWBuffer& operator=(const RWBuffer&) = delete;
  RWBuffer(const RWBuffer&) = delete;

  size_t size() const { return total_used_; }

  /**
   * Append bytes from |buffer|.
   *
   * If the caller knows in advance how much more data they are going to
   * append, they can pass a |reserve| hint (representing the number of upcoming
   * bytes *in addition* to the current append), to minimize the number of
   * internal allocations.
   */
  void Append(base::span<const uint8_t> buffer, size_t reserve = 0);

  scoped_refptr<ROBuffer> MakeROBufferSnapshot() const;

  // This should only be called from the same thread that we are creating the
  // RWBuffer and the snapshots on.
  // If true is returned, it is guaranteed that |this| has unique ownership of
  // its underlying buffer.
  bool HasNoSnapshots() const;

  void Validate() const;

 private:
  raw_ptr<BufferHead> head_ = nullptr;
  raw_ptr<BufferBlock> tail_ = nullptr;
  size_t total_used_ = 0;
};

/**
 * Contains a read-only, thread-sharable block of memory. To access the memory,
 * the caller must instantiate a local iterator, as the memory is stored in 1 or
 * more contiguous blocks.
 */
class PLATFORM_EXPORT ROBuffer : public WTF::ThreadSafeRefCounted<ROBuffer> {
 public:
  /**
   * Return the logical length of the data owned/shared by this buffer. It may
   * be stored in multiple contiguous blocks, accessible via the iterator.
   */
  size_t size() const { return available_; }

  class PLATFORM_EXPORT Iter {
   public:
    explicit Iter(const ROBuffer*);
    explicit Iter(const scoped_refptr<ROBuffer>&);

    void Reset(const ROBuffer*);

    /**
     * Returns a span of the current continuous block of memory. The span will
     * be empty if the iterator is exhausted.
     */
    base::span<const uint8_t> operator*() const;

    /**
     * Advance to the next contiguous block of memory, returning true if there
     * is another block, or false if the iterator is exhausted.
     */
    bool Next();

   private:
    raw_ptr<const RWBuffer::BufferBlock> block_;
    size_t remaining_;
    raw_ptr<const ROBuffer> buffer_;
  };

 private:
  friend class WTF::ThreadSafeRefCounted<ROBuffer>;
  ROBuffer(const RWBuffer::BufferHead* head,
           size_t available,
           const RWBuffer::BufferBlock* tail);
  ~ROBuffer();

  raw_ptr<const RWBuffer::BufferHead> head_;
  const size_t available_;
  raw_ptr<const RWBuffer::BufferBlock> tail_;

  friend class RWBuffer;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_RW_BUFFER_H_
