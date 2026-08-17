//
//
// Copyright 2015 gRPC authors.
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
//
//

#ifndef GRPCPP_SUPPORT_BYTE_BUFFER_H
#define GRPCPP_SUPPORT_BYTE_BUFFER_H

#include <grpc/byte_buffer.h>
#include <grpc/grpc.h>
#include <grpcpp/impl/serialization_traits.h>
#include <grpcpp/support/config.h>
#include <grpcpp/support/slice.h>
#include <grpcpp/support/status.h>

#include <vector>

namespace grpc {

class ServerInterface;
class ByteBuffer;
class ServerInterface;

namespace internal {
template <class RequestType, class ResponseType>
class CallbackUnaryHandler;
template <class RequestType, class ResponseType>
class CallbackServerStreamingHandler;
template <class RequestType>
void* UnaryDeserializeHelper(grpc_byte_buffer*, grpc::Status*, RequestType*);
template <class ServiceType, class RequestType, class ResponseType>
class ServerStreamingHandler;
template <grpc::StatusCode code>
class ErrorMethodHandler;
class CallOpSendMessage;
template <class R>
class CallOpRecvMessage;
class CallOpGenericRecvMessage;
class ExternalConnectionAcceptorImpl;
template <class R>
class DeserializeFuncType;
class GrpcByteBufferPeer;

}  // namespace internal
/// A sequence of bytes.
class ByteBuffer final {
 public:
  /// Construct an empty buffer.
  ByteBuffer() : buffer_(nullptr) {}

  /// Construct buffer from \a slices, of which there are \a nslices.
  ByteBuffer(const Slice* slices, size_t nslices) {
    // The following assertions check that the representation of a grpc::Slice
    // is identical to that of a grpc_slice:  it has a grpc_slice field, and
    // nothing else.
    static_assert(std::is_same<decltype(slices[0].slice_), grpc_slice>::value,
                  "Slice must have same representation as grpc_slice");
    static_assert(sizeof(Slice) == sizeof(grpc_slice),
                  "Slice must have same representation as grpc_slice");
    // The following assertions check that the representation of a ByteBuffer is
    // identical to grpc_byte_buffer*:  it has a grpc_byte_buffer* field,
    // and nothing else.
    static_assert(std::is_same<decltype(buffer_), grpc_byte_buffer*>::value,
                  "ByteBuffer must have same representation as "
                  "grpc_byte_buffer*");
    static_assert(sizeof(ByteBuffer) == sizeof(grpc_byte_buffer*),
                  "ByteBuffer must have same representation as "
                  "grpc_byte_buffer*");
    // The const_cast is legal if grpc_raw_byte_buffer_create() does no more
    // than its advertised side effect of increasing the reference count of the
    // slices it processes, and such an increase does not affect the semantics
    // seen by the caller of this constructor.
    buffer_ = grpc_raw_byte_buffer_create(
        reinterpret_cast<grpc_slice*>(const_cast<Slice*>(slices)), nslices);
  }

  /// Construct a byte buffer by referencing elements of existing buffer
  /// \a buf. Wrapper of core function grpc_byte_buffer_copy . This is not
  /// a deep copy; it is just a referencing. As a result, its performance is
  /// size-independent.
  ByteBuffer(const ByteBuffer& buf) : buffer_(nullptr) { operator=(buf); }

  ~ByteBuffer() {
    if (buffer_) {
      grpc_byte_buffer_destroy(buffer_);
    }
  }

  /// Wrapper of core function grpc_byte_buffer_copy . This is not
  /// a deep copy; it is just a referencing. As a result, its performance is
  /// size-independent.
  ByteBuffer& operator=(const ByteBuffer& buf) {
    if (this != &buf) {
      Clear();  // first remove existing data
    }
    if (buf.buffer_) {
      // then copy
      buffer_ = grpc_byte_buffer_copy(buf.buffer_);
    }
    return *this;
  }

  // If this ByteBuffer's representation is a single flat slice, returns a
  // slice referencing that array.
  Status TrySingleSlice(Slice* slice) const;

  /// Dump (read) the buffer contents into \a slics.
  Status DumpToSingleSlice(Slice* slice) const;

  /// Dump (read) the buffer contents into \a slices.
  Status Dump(std::vector<Slice>* slices) const;

  /// Remove all data.
  void Clear() {
    if (buffer_) {
      grpc_byte_buffer_destroy(buffer_);
      buffer_ = nullptr;
    }
  }

  /// Make a duplicate copy of the internals of this byte
  /// buffer so that we have our own owned version of it.
  /// bbuf.Duplicate(); is equivalent to bbuf=bbuf; but is actually readable.
  /// This is not a deep copy; it is a referencing and its performance
  /// is size-independent.
  void Duplicate() { buffer_ = grpc_byte_buffer_copy(buffer_); }

  /// Forget underlying byte buffer without destroying
  /// Use this only for un-owned byte buffers
  void Release() { buffer_ = nullptr; }

  /// Buffer size in bytes.
  size_t Length() const {
    return buffer_ == nullptr ? 0 : grpc_byte_buffer_length(buffer_);
  }

  /// Swap the state of *this and *other.
  void Swap(ByteBuffer* other) {
    grpc_byte_buffer* tmp = other->buffer_;
    other->buffer_ = buffer_;
    buffer_ = tmp;
  }

  /// Is this ByteBuffer valid?
  bool Valid() const { return (buffer_ != nullptr); }

 private:
  friend class SerializationTraits<ByteBuffer, void>;
  friend class ServerInterface;
  friend class internal::CallOpSendMessage;
  template <class R>
  friend class internal::CallOpRecvMessage;
  friend class internal::CallOpGenericRecvMessage;
  template <class RequestType>
  friend void* internal::UnaryDeserializeHelper(grpc_byte_buffer*,
                                                grpc::Status*, RequestType*);
  template <class ServiceType, class RequestType, class ResponseType>
  friend class internal::ServerStreamingHandler;
  template <class RequestType, class ResponseType>
  friend class internal::CallbackUnaryHandler;
  template <class RequestType, class ResponseType>
  friend class internal::CallbackServerStreamingHandler;
  template <StatusCode code>
  friend class internal::ErrorMethodHandler;
  template <class R>
  friend class internal::DeserializeFuncType;
  friend class ProtoBufferReader;
  friend class ProtoBufferWriter;
  friend class internal::GrpcByteBufferPeer;
  friend class internal::ExternalConnectionAcceptorImpl;

  grpc_byte_buffer* buffer_;

  // takes ownership
  void set_buffer(grpc_byte_buffer* buf) {
    if (buffer_) {
      Clear();
    }
    buffer_ = buf;
  }

  grpc_byte_buffer* c_buffer() { return buffer_; }
  grpc_byte_buffer** c_buffer_ptr() { return &buffer_; }

  class ByteBufferPointer {
   public:
    // NOLINTNEXTLINE(google-explicit-constructor)
    ByteBufferPointer(const ByteBuffer* b)
        : bbuf_(const_cast<ByteBuffer*>(b)) {}
    // NOLINTNEXTLINE(google-explicit-constructor)
    operator ByteBuffer*() { return bbuf_; }
    // NOLINTNEXTLINE(google-explicit-constructor)
    operator grpc_byte_buffer*() { return bbuf_->buffer_; }
    // NOLINTNEXTLINE(google-explicit-constructor)
    operator grpc_byte_buffer**() { return &bbuf_->buffer_; }

   private:
    ByteBuffer* bbuf_;
  };
  ByteBufferPointer bbuf_ptr() const { return ByteBufferPointer(this); }
};

template <>
class SerializationTraits<ByteBuffer, void> {
 public:
  static Status Deserialize(ByteBuffer* byte_buffer, ByteBuffer* dest) {
    dest->set_buffer(byte_buffer->buffer_);
    return Status::OK;
  }
  static Status Serialize(const ByteBuffer& source, ByteBuffer* buffer,
                          bool* own_buffer) {
    *buffer = source;
    *own_buffer = true;
    return grpc::Status::OK;
  }
};

}  // namespace grpc

#endif  // GRPCPP_SUPPORT_BYTE_BUFFER_H
