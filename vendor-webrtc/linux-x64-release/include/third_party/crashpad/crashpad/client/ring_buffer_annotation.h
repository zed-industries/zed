// Copyright 2023 The Crashpad Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef CRASHPAD_CLIENT_RING_BUFFER_ANNOTATION_H_
#define CRASHPAD_CLIENT_RING_BUFFER_ANNOTATION_H_

#include <stdio.h>

#include "client/annotation.h"
#include "client/length_delimited_ring_buffer.h"

namespace crashpad {

//! \brief Capacity of `RingBufferAnnotation`, in bytes.
using RingBufferAnnotationCapacity = RingBufferCapacity;

namespace internal {

//! \brief Default capacity of `RingBufferAnnotation`, in bytes.
inline constexpr RingBufferAnnotationCapacity
    kDefaultRingBufferAnnotationCapacity = 8192;

}  // namespace internal

//! \brief An `Annotation` which wraps a `LengthDelimitedRingBuffer`
//!     of up to `Capacity` bytes in length.
//!
//! Supports writing variable-length data via `Push()`. When the ring buffer is
//! full, it will drop old data items in FIFO order until enough space is
//! available for the write.
//!
//! Supports guarding concurrent reads from writes via `ScopedSpinGuard`, so
//! writing to this object is thread-safe.
//!
//! Clients which read this `Annotation`'s memory can optionally invoke
//! `TryCreateScopedSpinGuard()` on this object to ensure any pending write
//! finishes before the memory is read.
//!
//! Each item in this ring buffer is delimited by its length encoded in
//! little-endian Base 128 varint encoding.
//!
//! `RingBufferAnnotation` uses varint-encoded delimiters to enable
//! zero-copy deserialization of the ringbuffer's contents when storing
//! protobufs inside the ringbuffer, e.g. via
//! `google::protobuf::util::ParseDelimitedFromZeroCopyStream()` or similar.
//!
//! \sa
//! https://github.com/protocolbuffers/protobuf/blob/3202b9da88ceb75b65bbabaf4033c95e872f828d/src/google/protobuf/util/delimited_message_util.h#L85
//! \sa
//! https://github.com/protocolbuffers/protobuf/blob/8bd49dea5e167a389d94b71d24c981d8f9fa0c99/src/google/protobuf/io/zero_copy_stream_impl_lite.h#L68
//! \sa
//! https://github.com/protocolbuffers/protobuf/blob/8bd49dea5e167a389d94b71d24c981d8f9fa0c99/src/google/protobuf/io/coded_stream.h#L171
//!
//! To deserialize the items stored in this annotation, use
//! `LengthDelimitedRingBufferReader`.
template <RingBufferAnnotationCapacity Capacity =
              internal::kDefaultRingBufferAnnotationCapacity>
class RingBufferAnnotation final : public Annotation {
 public:
  //! \brief Constructs a `RingBufferAnnotation`.
  //! \param[in] type A unique identifier for the type of data in the ring
  //!     buffer.
  //! \param[in] name The name of the annotation.
  constexpr RingBufferAnnotation(Annotation::Type type, const char name[])
      : Annotation(type,
                   name,
                   reinterpret_cast<void* const>(&ring_buffer_data_),
                   ConcurrentAccessGuardMode::kScopedSpinGuard),
        ring_buffer_data_(),
        ring_buffer_writer_(ring_buffer_data_) {}
  RingBufferAnnotation(const RingBufferAnnotation&) = delete;
  RingBufferAnnotation& operator=(const RingBufferAnnotation&) = delete;
  RingBufferAnnotation(RingBufferAnnotation&&) = default;
  RingBufferAnnotation& operator=(RingBufferAnnotation&&) = default;

  //! \brief Pushes data onto this annotation's ring buffer.
  //!
  //! If the ring buffer does not have enough space to store `buffer_length`
  //! bytes of data, old data items are dropped in FIFO order until
  //! enough space is available to store the new data.
  bool Push(const void* const buffer,
            RingBufferAnnotationCapacity buffer_length) {
    // Use a zero timeout so the operation immediately fails if another thread
    // or process is currently reading this Annotation.
    constexpr uint64_t kSpinGuardTimeoutNanoseconds = 0;

    auto spin_guard = TryCreateScopedSpinGuard(kSpinGuardTimeoutNanoseconds);
    if (!spin_guard) {
      return false;
    }
    bool success = ring_buffer_writer_.Push(buffer, buffer_length);
    if (success) {
      SetSize(ring_buffer_data_.GetRingBufferLength());
    }
    return success;
  }

  //! \brief Reset the annotation (e.g., for testing).
  //! This method is not thread-safe.
  void ResetForTesting() {
    ring_buffer_data_.ResetForTesting();
    ring_buffer_writer_.ResetForTesting();
  }

 private:
  using RingBufferWriter =
      LengthDelimitedRingBufferWriter<RingBufferData<Capacity>>;

  //! \brief The ring buffer data stored in this Anotation.
  RingBufferData<Capacity> ring_buffer_data_;

  //! \brief The writer which wraps `ring_buffer_data_`.
  RingBufferWriter ring_buffer_writer_;
};

// Allow just `RingBufferAnnotation foo;` to be declared without template
// arguments using C++17 class template argument deduction.
template <RingBufferAnnotationCapacity Capacity =
              internal::kDefaultRingBufferAnnotationCapacity>
RingBufferAnnotation(Annotation::Type type, const char name[])
    -> RingBufferAnnotation<Capacity>;

}  // namespace crashpad

#endif  // CRASHPAD_CLIENT_RING_BUFFER_ANNOTATION_H_
