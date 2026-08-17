// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_BYTE_BUFFER_QUEUE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_BYTE_BUFFER_QUEUE_H_

#include "base/containers/span.h"
#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

// A ByteBufferQueue is a byte buffer with O(1) append and O(n) read operations.
// Clients can append entire byte buffers then copy data out across buffer
// boundaries using |ReadInto|.
class MODULES_EXPORT ByteBufferQueue final {
  DISALLOW_NEW();

 public:
  // Number of bytes that can be read.
  wtf_size_t size() const { return size_; }

  // True if size() == 0.
  bool empty() const { return size_ == 0; }

  // Copies data into the given byte span. This will cause bytes to be consumed
  // so that the next call to ReadInto will return different bytes.
  // Returns the number of bytes written to |buffer_out|.
  wtf_size_t ReadInto(base::span<uint8_t> buffer_out);

  // Appends the contents of a byte buffer. This takes ownership of the buffer.
  void Append(Vector<uint8_t> buffer);

  // Clear stored buffers.
  void Clear();

 private:
#if DCHECK_IS_ON()
  void CheckInvariants() const;
#endif

  // Number of bytes that can be read.
  // |Append()| adds to this number.
  // |ReadInto()| subtracts from this number.
  // Invariant: |size_| = sum of |deque_of_buffers_| element sizes -
  //     |front_buffer_offset_|.
  wtf_size_t size_ = 0;

  // Double-ended queue of byte buffers.
  // |Append()| pushes to the right.
  // |ReadInto()| pops from the left (if an entire buffer has been read).
  // Invariant: No element in |deque_of_buffers_| is empty.
  Deque<Vector<uint8_t>> deque_of_buffers_;

  // The offset from which to start reading the buffer at the front of
  // |deque_of_buffers_|.
  // Invariants:
  // - If |deque_of_buffers_| is empty, |front_buffer_offset_| = 0.
  // - Otherwise, |front_buffer_offset_| < |deque_of_buffers_|.front().size().
  wtf_size_t front_buffer_offset_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_BYTE_BUFFER_QUEUE_H_
