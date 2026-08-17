// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_MULTI_BUFFER_READER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_MULTI_BUFFER_READER_H_

#include <stdint.h>

#include "base/functional/callback.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "third_party/blink/renderer/platform/media/multi_buffer.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace base {
class SingleThreadTaskRunner;
}

namespace blink {

// Wrapper for MultiBuffer that offers a simple byte-reading
// interface with prefetch.
class PLATFORM_EXPORT MultiBufferReader : public MultiBuffer::Reader {
 public:
  // Note that |progress_callback| is guaranteed to be called if
  // a redirect happens and the url_data is updated. Otherwise
  // origin checks will become insecure.
  // Users probably want to call SetMaxBuffer & SetPreload after
  // creating the a MultiBufferReader.
  // The progress callback will be called when the "available range"
  // changes. (The number of bytes available for reading before and
  // after the current position.) The arguments for the progress
  // callback is the first byte available (from beginning of file)
  // and the last byte available + 1. Note that there may be other
  // regions of available data in the cache as well.
  // If |end| is not known, use -1.
  MultiBufferReader(
      MultiBuffer* multibuffer,
      int64_t start,
      int64_t end,
      bool is_client_audio_element,
      base::RepeatingCallback<void(int64_t, int64_t)> progress_callback,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);

  ~MultiBufferReader() override;

  // Returns number of bytes available for reading. At position |pos|
  // When the rest of the file is available, the number returned will
  // be greater than the number or readable bytes. If an error occurs,
  // -1 is returned.
  int64_t AvailableAt(int64_t pos) const;

  // Returns number of bytes available for reading at the current position.
  int64_t Available() const { return AvailableAt(pos_); }

  // Seek to a different position.
  // If there is a pending Wait(), it will be cancelled.
  void Seek(int64_t pos);

  // Returns the current position.
  int64_t Tell() const { return pos_; }

  // Tries to read |len| bytes from position |pos|.
  // Returns number of bytes read.
  // Safe to call from any thread.
  int64_t TryReadAt(int64_t pos, uint8_t* data, int64_t len);

  // Tries to read |len| bytes and update current position.
  // Returns number of bytes read.
  int64_t TryRead(uint8_t* data, int64_t len);

  // Wait until |len| bytes are available for reading.
  // Returns net::OK if |len| bytes are already available, otherwise it will
  // return net::ERR_IO_PENDING and call |cb| at some point later.
  // |len| must be smaller or equal to max_buffer_forward.
  int Wait(int64_t len, base::OnceClosure cb);

  // Set how much data we try to preload into the cache ahead of our current
  // location. Normally, we preload until we have preload_high bytes, then
  // stop until we fall below preload_low bytes. Note that preload can be
  // set higher than max_buffer_forward, but the result is usually that
  // some blocks will be freed between the current position and the preload
  // position.
  void SetPreload(int64_t preload_high, int64_t preload_low);

  // Change how much data we pin to the cache.
  // The range [current_position - backward ... current_position + forward)
  // will be locked in the cache. Calling Wait() or TryRead() with values
  // larger than |forward| is not supported.
  void SetPinRange(int64_t backward, int64_t forward);

  // Set how much memory usage we target. This memory is added to the global
  // LRU and shared between all multibuffers. We may end up using more memory
  // if no memory can be freed due to pinning.
  void SetMaxBuffer(int64_t bytes);

  // Returns true if we are currently loading data.
  bool IsLoading() const;

  // Reader implementation.
  void NotifyAvailableRange(const Interval<MultiBufferBlockId>& range) override;

  // Getters
  int64_t preload_high() const { return preload_high_; }
  int64_t preload_low() const { return preload_low_; }

 private:
  friend class MultiBufferDataSourceTest;

  // Returns the block for a particular byte position.
  MultiBufferBlockId block(int64_t byte_pos) const {
    return static_cast<MultiBufferBlockId>(byte_pos >>
                                           multibuffer_->block_size_shift());
  }

  // Returns the block for a particular byte position, rounding up.
  MultiBufferBlockId block_ceil(int64_t byte_pos) const {
    return block(byte_pos + (1LL << multibuffer_->block_size_shift()) - 1);
  }

  // Unpin previous range, then pin the new range.
  void PinRange(MultiBuffer::BlockId begin, MultiBuffer::BlockId end);

  // Check if wait operation can complete now.
  void CheckWait();

  // Recalculate preload_pos_ and update our entry in the multibuffer
  // reader index. Also call CheckWait(). This function is basically
  // called anything changes, like when we get more data or seek to
  // a new position.
  void UpdateInternalState();

  // Update end_ if p-1 contains an end-of-stream block.
  void UpdateEnd(MultiBufferBlockId p);

  // Indirection function used to call callbacks. When we post a callback
  // we indirect it through a weak_ptr and this function to make sure we
  // don't call any callbacks after this object has been destroyed.
  void Call(base::OnceClosure cb) const;

  // The multibuffer we're wrapping, not owned.
  raw_ptr<MultiBuffer> multibuffer_;

  // We're not interested in reading past this position.
  int64_t end_;

  // Defer reading once we have this much data.
  int64_t preload_high_;
  // Stop deferring once we have this much data.
  int64_t preload_low_;

  // Pin this much data in the cache from the current position.
  int64_t max_buffer_forward_;
  int64_t max_buffer_backward_;

  // The amount of buffer we've added to the global LRU.
  int64_t current_buffer_size_;

  // Currently pinned range.
  Interval<MultiBuffer::BlockId> pinned_range_;

  // Current position in bytes.
  int64_t pos_;

  // Is the client an audio element?
  bool is_client_audio_element_ = false;

  // [block(pos_)..preload_pos_) are known to be in the cache.
  // preload_pos_ is only allowed to point to a filled
  // cache position if it is equal to end_ or pos_+preload_.
  // This is a pointer to a slot in the cache, so the unit is
  // blocks.
  MultiBufferBlockId preload_pos_;

  // True if we've requested data from the cache by calling WaitFor().
  bool loading_;

  // When Available() > current_wait_size_ we call cb_.
  int64_t current_wait_size_;
  base::OnceClosure cb_;

  // Progress callback.
  base::RepeatingCallback<void(int64_t, int64_t)> progress_callback_;

  const scoped_refptr<base::SingleThreadTaskRunner> task_runner_;

  base::WeakPtrFactory<MultiBufferReader> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_MULTI_BUFFER_READER_H_
