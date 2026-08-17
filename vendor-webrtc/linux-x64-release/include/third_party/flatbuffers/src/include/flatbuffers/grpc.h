/*
 * Copyright 2014 Google Inc. All rights reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef FLATBUFFERS_GRPC_H_
#define FLATBUFFERS_GRPC_H_

// Helper functionality to glue FlatBuffers and GRPC.

#include "flatbuffers/flatbuffers.h"
#include "grpcpp/support/byte_buffer.h"
#include "grpcpp/support/slice.h"

namespace flatbuffers {
namespace grpc {

// Message is a typed wrapper around a buffer that manages the underlying
// `grpc_slice` and also provides flatbuffers-specific helpers such as `Verify`
// and `GetRoot`. Since it is backed by a `grpc_slice`, the underlying buffer
// is refcounted and ownership is be managed automatically.
template<class T> class Message {
 public:
  Message() {}

  Message(::grpc::Slice slice) : slice_(slice) {}

  Message &operator=(const Message &other) = delete;

  Message(Message &&other) = default;

  Message(const Message &other) = delete;

  Message &operator=(Message &&other) = default;

  const uint8_t *mutable_data() const { return slice_.begin(); }

  const uint8_t *data() const { return slice_.begin(); }

  size_t size() const { return slice_.size(); }

  bool Verify() const {
    Verifier verifier(data(), size());
    return verifier.VerifyBuffer<T>(nullptr);
  }

  T *GetMutableRoot() { return flatbuffers::GetMutableRoot<T>(mutable_data()); }

  const T *GetRoot() const { return flatbuffers::GetRoot<T>(data()); }

  // This is only intended for serializer use, or if you know what you're doing
  const ::grpc::Slice &BorrowSlice() const { return slice_; }

 private:
  ::grpc::Slice slice_;
};

class MessageBuilder;

// SliceAllocator is a gRPC-specific allocator that uses the `grpc_slice`
// refcounted slices to manage memory ownership. This makes it easy and
// efficient to transfer buffers to gRPC.
class SliceAllocator : public Allocator {
 public:
  SliceAllocator() {}

  SliceAllocator(const SliceAllocator &other) = delete;
  SliceAllocator &operator=(const SliceAllocator &other) = delete;

  SliceAllocator(SliceAllocator &&other) {
    // default-construct and swap idiom
    swap(other);
  }

  SliceAllocator &operator=(SliceAllocator &&other) {
    // move-construct and swap idiom
    SliceAllocator temp(std::move(other));
    swap(temp);
    return *this;
  }

  void swap(SliceAllocator &other) {
    using std::swap;
    swap(slice_, other.slice_);
  }

  virtual ~SliceAllocator() {}

  virtual uint8_t *allocate(size_t size) override {
    FLATBUFFERS_ASSERT(slice_.size() == 0);
    slice_ = ::grpc::Slice(size);
    return const_cast<uint8_t *>(slice_.begin());
  }

  virtual void deallocate(uint8_t *p, size_t size) override {
    FLATBUFFERS_ASSERT(p == slice_.begin());
    FLATBUFFERS_ASSERT(size == slice_.size());
    slice_ = ::grpc::Slice();
  }

  virtual uint8_t *reallocate_downward(uint8_t *old_p, size_t old_size,
                                       size_t new_size, size_t in_use_back,
                                       size_t in_use_front) override {
    FLATBUFFERS_ASSERT(old_p == slice_.begin());
    FLATBUFFERS_ASSERT(old_size == slice_.size());
    FLATBUFFERS_ASSERT(new_size > old_size);
    ::grpc::Slice old_slice = slice_;
    ::grpc::Slice new_slice = ::grpc::Slice(new_size);
    uint8_t *new_p = const_cast<uint8_t *>(new_slice.begin());
    memcpy_downward(old_p, old_size, new_p, new_size, in_use_back,
                    in_use_front);
    slice_ = new_slice;
    return new_p;
  }

 private:
  ::grpc::Slice &get_slice(uint8_t *p, size_t size) {
    FLATBUFFERS_ASSERT(p == slice_.begin());
    FLATBUFFERS_ASSERT(size == slice_.size());
    return slice_;
  }

  ::grpc::Slice slice_;

  friend class MessageBuilder;
};

// SliceAllocatorMember is a hack to ensure that the MessageBuilder's
// slice_allocator_ member is constructed before the FlatBufferBuilder, since
// the allocator is used in the FlatBufferBuilder ctor.
namespace detail {
struct SliceAllocatorMember {
  SliceAllocator slice_allocator_;
};
}  // namespace detail

// MessageBuilder is a gRPC-specific FlatBufferBuilder that uses SliceAllocator
// to allocate gRPC buffers.
class MessageBuilder : private detail::SliceAllocatorMember,
                       public FlatBufferBuilder {
 public:
  explicit MessageBuilder(uoffset_t initial_size = 1024)
      : FlatBufferBuilder(initial_size, &slice_allocator_, false) {}

  MessageBuilder(const MessageBuilder &other) = delete;
  MessageBuilder &operator=(const MessageBuilder &other) = delete;

  MessageBuilder(MessageBuilder &&other)
      : FlatBufferBuilder(1024, &slice_allocator_, false) {
    // Default construct and swap idiom.
    Swap(other);
  }

  /// Create a MessageBuilder from a FlatBufferBuilder.
  explicit MessageBuilder(FlatBufferBuilder &&src,
                          void (*dealloc)(void *,
                                          size_t) = &DefaultAllocator::dealloc)
      : FlatBufferBuilder(1024, &slice_allocator_, false) {
    src.Swap(*this);
    src.SwapBufAllocator(*this);
    if (buf_.capacity()) {
      uint8_t *buf = buf_.scratch_data();  // pointer to memory
      size_t capacity = buf_.capacity();   // size of memory
      slice_allocator_.slice_ = ::grpc::Slice(buf, capacity, dealloc);
    } else {
      slice_allocator_.slice_ = ::grpc::Slice();
    }
  }

  /// Move-assign a FlatBufferBuilder to a MessageBuilder.
  /// Only FlatBufferBuilder with default allocator (basically, nullptr) is
  /// supported.
  MessageBuilder &operator=(FlatBufferBuilder &&src) {
    // Move construct a temporary and swap
    MessageBuilder temp(std::move(src));
    Swap(temp);
    return *this;
  }

  MessageBuilder &operator=(MessageBuilder &&other) {
    // Move construct a temporary and swap
    MessageBuilder temp(std::move(other));
    Swap(temp);
    return *this;
  }

  void Swap(MessageBuilder &other) {
    slice_allocator_.swap(other.slice_allocator_);
    FlatBufferBuilder::Swap(other);
    // After swapping the FlatBufferBuilder, we swap back the allocator, which
    // restores the original allocator back in place. This is necessary because
    // MessageBuilder's allocator is its own member (SliceAllocatorMember). The
    // allocator passed to FlatBufferBuilder::vector_downward must point to this
    // member.
    buf_.swap_allocator(other.buf_);
  }

  // Releases the ownership of the buffer pointer.
  // Returns the size, offset, and the original grpc_slice that
  // allocated the buffer. Also see grpc_slice_unref().
  uint8_t *ReleaseRaw(size_t &size, size_t &offset, ::grpc::Slice &slice) {
    uint8_t *buf = FlatBufferBuilder::ReleaseRaw(size, offset);
    slice = slice_allocator_.slice_;
    slice_allocator_.slice_ = ::grpc::Slice();
    return buf;
  }

  ~MessageBuilder() {}

  // GetMessage extracts the subslice of the buffer corresponding to the
  // flatbuffers-encoded region and wraps it in a `Message<T>` to handle buffer
  // ownership.
  template<class T> Message<T> GetMessage() {
    auto buf_data = buf_.scratch_data();  // pointer to memory
    auto buf_size = buf_.capacity();      // size of memory
    auto msg_data = buf_.data();          // pointer to msg
    auto msg_size = buf_.size();          // size of msg
    // Do some sanity checks on data/size
    FLATBUFFERS_ASSERT(msg_data);
    FLATBUFFERS_ASSERT(msg_size);
    FLATBUFFERS_ASSERT(msg_data >= buf_data);
    FLATBUFFERS_ASSERT(msg_data + msg_size <= buf_data + buf_size);
    // Calculate offsets from the buffer start
    auto begin = msg_data - buf_data;
    auto end = begin + msg_size;
    // Get the slice we are working with (no refcount change)
    ::grpc::Slice slice = slice_allocator_.get_slice(buf_data, buf_size);
    // Extract a subslice of the existing slice (increment refcount)
    ::grpc::Slice subslice = slice.sub(begin, end);
    // Wrap the subslice in a `Message<T>`, but don't increment refcount
    Message<T> msg(subslice);
    return msg;
  }

  template<class T> Message<T> ReleaseMessage() {
    Message<T> msg = GetMessage<T>();
    Reset();
    return msg;
  }

 private:
  // SliceAllocator slice_allocator_;  // part of SliceAllocatorMember
};

}  // namespace grpc
}  // namespace flatbuffers

namespace grpc {

template<class T> class SerializationTraits<flatbuffers::grpc::Message<T>> {
 public:
  static grpc::Status Serialize(const flatbuffers::grpc::Message<T> &msg,
                                ByteBuffer *buffer, bool *own_buffer) {
    // Package the single slice into a `ByteBuffer`,
    // incrementing the refcount in the process.
    *buffer = ByteBuffer(&msg.BorrowSlice(), 1);
    *own_buffer = true;
    return grpc::Status::OK;
  }

  // Deserialize by pulling the
  static grpc::Status Deserialize(ByteBuffer *buf,
                                  flatbuffers::grpc::Message<T> *msg) {
    Slice slice;
    if (!buf->TrySingleSlice(&slice).ok()) {
      if (!buf->DumpToSingleSlice(&slice).ok()) {
        buf->Clear();
        return ::grpc::Status(::grpc::StatusCode::INTERNAL, "No payload");
      }
    }
    *msg = flatbuffers::grpc::Message<T>(slice);
    buf->Clear();
#if FLATBUFFERS_GRPC_DISABLE_AUTO_VERIFICATION
    return ::grpc::Status::OK;
#else
    if (msg->Verify()) {
      return ::grpc::Status::OK;
    } else {
      return ::grpc::Status(::grpc::StatusCode::INTERNAL,
                            "Message verification failed");
    }
#endif
  }
};

}  // namespace grpc

#endif  // FLATBUFFERS_GRPC_H_
