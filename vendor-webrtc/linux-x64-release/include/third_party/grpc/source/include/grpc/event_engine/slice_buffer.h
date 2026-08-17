// Copyright 2022 gRPC authors.
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

#ifndef GRPC_EVENT_ENGINE_SLICE_BUFFER_H
#define GRPC_EVENT_ENGINE_SLICE_BUFFER_H

#include <grpc/event_engine/internal/slice_cast.h>
#include <grpc/event_engine/slice.h>
#include <grpc/impl/codegen/slice.h>
#include <grpc/slice.h>
#include <grpc/slice_buffer.h>
#include <grpc/support/port_platform.h>
#include <string.h>

#include <cstdint>
#include <string>

#include "absl/strings/string_view.h"
#include "absl/utility/utility.h"

namespace grpc_event_engine {
namespace experimental {

/// A Wrapper around \a grpc_slice_buffer pointer.
///
/// A slice buffer holds the memory for a collection of slices.
/// The SliceBuffer object itself is meant to only hide the C-style API,
/// and won't hold the data itself. In terms of lifespan, the
/// grpc_slice_buffer ought to be kept somewhere inside the caller's objects,
/// like a transport or an endpoint.
///
/// This lifespan rule is likely to change in the future, as we may
/// collapse the grpc_slice_buffer structure straight into this class.
///
/// The SliceBuffer API is basically a replica of the grpc_slice_buffer's,
/// and its documentation will move here once we remove the C structure,
/// which should happen before the EventEngine's API is no longer
/// an experimental API.
class SliceBuffer {
 public:
  SliceBuffer() { grpc_slice_buffer_init(&slice_buffer_); }
  SliceBuffer(const SliceBuffer& other) = delete;
  SliceBuffer(SliceBuffer&& other) noexcept
      : slice_buffer_(other.slice_buffer_) {
    grpc_slice_buffer_init(&slice_buffer_);
    grpc_slice_buffer_swap(&slice_buffer_, &other.slice_buffer_);
  }
  /// Upon destruction, the underlying raw slice buffer is cleaned out and all
  /// slices are unreffed.
  ~SliceBuffer() { grpc_slice_buffer_destroy(&slice_buffer_); }

  SliceBuffer& operator=(const SliceBuffer&) = delete;
  SliceBuffer& operator=(SliceBuffer&& other) noexcept {
    grpc_slice_buffer_swap(&slice_buffer_, &other.slice_buffer_);
    return *this;
  }

  /// Swap the contents of this SliceBuffer with the contents of another.
  void Swap(SliceBuffer& other) {
    grpc_slice_buffer_swap(&slice_buffer_, &other.slice_buffer_);
  }

  /// Appends a new slice into the SliceBuffer and makes an attempt to merge
  /// this slice with the last slice in the SliceBuffer.
  void Append(Slice slice);

  /// Adds a new slice into the SliceBuffer at the next available index.
  /// Returns the index at which the new slice is added.
  size_t AppendIndexed(Slice slice);

  /// Returns the number of slices held by the SliceBuffer.
  size_t Count() { return slice_buffer_.count; }

  /// Removes/deletes the last n bytes in the SliceBuffer.
  void RemoveLastNBytes(size_t n) {
    grpc_slice_buffer_trim_end(&slice_buffer_, n, nullptr);
  }

  /// Move the first n bytes of the SliceBuffer into a memory pointed to by dst.
  void MoveFirstNBytesIntoBuffer(size_t n, void* dst) {
    grpc_slice_buffer_move_first_into_buffer(&slice_buffer_, n, dst);
  }

  /// Removes/deletes the last n bytes in the SliceBuffer and add it to the
  /// other SliceBuffer
  void MoveLastNBytesIntoSliceBuffer(size_t n, SliceBuffer& other) {
    grpc_slice_buffer_trim_end(&slice_buffer_, n, &other.slice_buffer_);
  }

  /// Move the first n bytes of the SliceBuffer into the other SliceBuffer
  void MoveFirstNBytesIntoSliceBuffer(size_t n, SliceBuffer& other) {
    grpc_slice_buffer_move_first(&slice_buffer_, n, &other.slice_buffer_);
  }

  /// Removes and unrefs all slices in the SliceBuffer.
  void Clear() { grpc_slice_buffer_reset_and_unref(&slice_buffer_); }

  /// Removes the first slice in the SliceBuffer and returns it.
  Slice TakeFirst();

  /// Prepends the slice to the the front of the SliceBuffer.
  void Prepend(Slice slice);

  /// Increased the ref-count of slice at the specified index and returns the
  /// associated slice.
  Slice RefSlice(size_t index);

  /// Array access into the SliceBuffer. It returns a non mutable reference to
  /// the slice at the specified index
  const Slice& operator[](size_t index) const {
    return internal::SliceCast<Slice>(slice_buffer_.slices[index]);
  }

  /// Return mutable reference to the slice at the specified index
  Slice& MutableSliceAt(size_t index) const {
    return internal::SliceCast<Slice>(slice_buffer_.slices[index]);
  }

  /// The total number of bytes held by the SliceBuffer
  size_t Length() const { return slice_buffer_.length; }

  /// Return a pointer to the back raw grpc_slice_buffer
  grpc_slice_buffer* c_slice_buffer() { return &slice_buffer_; }

  // Returns a SliceBuffer that transfers slices into this new SliceBuffer,
  // leaving the input parameter empty.
  static SliceBuffer TakeCSliceBuffer(grpc_slice_buffer& slice_buffer) {
    return SliceBuffer(&slice_buffer);
  }

 private:
  // Transfers slices into this new SliceBuffer, leaving the parameter empty.
  // Does not take ownership of the slice_buffer argument.
  explicit SliceBuffer(grpc_slice_buffer* slice_buffer) {
    grpc_slice_buffer_init(&slice_buffer_);
    grpc_slice_buffer_swap(&slice_buffer_, slice_buffer);
  }
  /// The backing raw slice buffer.
  grpc_slice_buffer slice_buffer_;
};

}  // namespace experimental
}  // namespace grpc_event_engine

#endif  // GRPC_EVENT_ENGINE_SLICE_BUFFER_H
