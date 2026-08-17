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

#ifndef GRPC_SRC_CORE_LIB_SLICE_SLICE_BUFFER_H
#define GRPC_SRC_CORE_LIB_SLICE_SLICE_BUFFER_H

#include <grpc/slice.h>
#include <grpc/slice_buffer.h>
#include <grpc/support/port_platform.h>
#include <stdint.h>
#include <string.h>

#include <memory>
#include <string>

#include "src/core/lib/slice/slice.h"

// Copy the first n bytes of src into memory pointed to by dst.
void grpc_slice_buffer_copy_first_into_buffer(const grpc_slice_buffer* src,
                                              size_t n, void* dst);

void grpc_slice_buffer_move_first_no_inline(grpc_slice_buffer* src, size_t n,
                                            grpc_slice_buffer* dst);

void grpc_slice_buffer_trim_end_no_inline(grpc_slice_buffer* sb, size_t n,
                                          grpc_slice_buffer* garbage);

namespace grpc_core {

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
  explicit SliceBuffer() { grpc_slice_buffer_init(&slice_buffer_); }
  explicit SliceBuffer(Slice slice) : SliceBuffer() {
    Append(std::move(slice));
  }
  SliceBuffer(const SliceBuffer& other) = delete;
  SliceBuffer(SliceBuffer&& other) noexcept {
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

  /// Appends a new slice into the SliceBuffer and makes an attempt to merge
  /// this slice with the last slice in the SliceBuffer.
  void Append(Slice slice);
  /// Appends a SliceBuffer into the SliceBuffer and makes an attempt to merge
  /// this slice with the last slice in the SliceBuffer.
  void Append(const SliceBuffer& other);
  void TakeAndAppend(SliceBuffer& other) {
    grpc_slice_buffer_move_into(&other.slice_buffer_, &slice_buffer_);
  }

  /// Adds a new slice into the SliceBuffer at the next available index.
  /// Returns the index at which the new slice is added.
  size_t AppendIndexed(Slice slice);

  /// Returns the number of slices held by the SliceBuffer.
  size_t Count() const { return slice_buffer_.count; }

  /// Copy the entire contents to a memory buffer.
  void CopyToBuffer(uint8_t* dst) {
    grpc_slice_buffer_copy_first_into_buffer(&slice_buffer_,
                                             slice_buffer_.length, dst);
  }

  /// Removes/deletes the last n bytes in the SliceBuffer.
  void RemoveLastNBytes(size_t n) {
    grpc_slice_buffer_trim_end(&slice_buffer_, n, nullptr);
  }

  /// Removes/deletes the last n bytes in the SliceBuffer while avoiding the
  /// the creation of inline slices.
  void RemoveLastNBytesNoInline(size_t n) {
    grpc_slice_buffer_trim_end_no_inline(&slice_buffer_, n, nullptr);
  }

  /// Move the first n bytes of the SliceBuffer into a memory pointed to by
  /// dst.
  void MoveFirstNBytesIntoBuffer(size_t n, void* dst) {
    grpc_slice_buffer_move_first_into_buffer(&slice_buffer_, n, dst);
  }

  /// Copy the first n bytes of the SliceBuffer into a memory pointed to by
  /// dst. The original SliceBuffer remains unchanged.
  void CopyFirstNBytesIntoBuffer(size_t n, void* dst) {
    grpc_slice_buffer_copy_first_into_buffer(&slice_buffer_, n, dst);
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
  GRPC_REINITIALIZES void Clear() {
    grpc_slice_buffer_reset_and_unref(&slice_buffer_);
  }

  /// Removes the first slice in the SliceBuffer and returns it.
  Slice TakeFirst();

  /// Prepends the slice to the the front of the SliceBuffer.
  void Prepend(Slice slice);

  /// Increased the ref-count of slice at the specified index and returns the
  /// associated slice.
  Slice RefSlice(size_t index) const;

  const Slice& operator[](size_t index) const {
    return grpc_event_engine::experimental::internal::SliceCast<Slice>(
        slice_buffer_.slices[index]);
  }

  /// The total number of bytes held by the SliceBuffer
  size_t Length() const { return slice_buffer_.length; }

  /// Swap with another slice buffer
  void Swap(SliceBuffer* other) {
    grpc_slice_buffer_swap(c_slice_buffer(), other->c_slice_buffer());
  }

  /// Concatenate all slices and return the resulting string.
  std::string JoinIntoString() const;

  /// Concatenate all slices and return the resulting slice.
  Slice JoinIntoSlice() const;

  // Return a copy of the slice buffer
  SliceBuffer Copy() const {
    SliceBuffer copy;
    for (size_t i = 0; i < Count(); i++) {
      copy.Append(RefSlice(i));
    }
    return copy;
  }

  /// Add a small amount to the end of the slice buffer.
  uint8_t* AddTiny(size_t n) {
    return grpc_slice_buffer_tiny_add(&slice_buffer_, n);
  }

  /// Return a pointer to the back raw grpc_slice_buffer
  grpc_slice_buffer* c_slice_buffer() { return &slice_buffer_; }

  /// Return a pointer to the back raw grpc_slice_buffer
  const grpc_slice_buffer* c_slice_buffer() const { return &slice_buffer_; }

  const grpc_slice& c_slice_at(size_t index) {
    return slice_buffer_.slices[index];
  }

 private:
  /// The backing raw slice buffer.
  grpc_slice_buffer slice_buffer_;

// Make failure to destruct show up in ASAN builds.
#ifndef NDEBUG
  std::unique_ptr<int> asan_canary_ = std::make_unique<int>(0);
#endif
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_SLICE_SLICE_BUFFER_H
