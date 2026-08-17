//
//
// Copyright 2018 gRPC authors.
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

#ifndef GRPCPP_SUPPORT_PROTO_BUFFER_READER_H
#define GRPCPP_SUPPORT_PROTO_BUFFER_READER_H

#include <grpc/byte_buffer.h>
#include <grpc/byte_buffer_reader.h>
#include <grpc/impl/grpc_types.h>
#include <grpc/slice.h>
#include <grpcpp/impl/codegen/config_protobuf.h>
#include <grpcpp/impl/serialization_traits.h>
#include <grpcpp/support/byte_buffer.h>
#include <grpcpp/support/status.h>

#include <type_traits>

#include "absl/log/absl_check.h"
#include "absl/strings/cord.h"

/// This header provides an object that reads bytes directly from a
/// grpc::ByteBuffer, via the ZeroCopyInputStream interface

namespace grpc {

/// This is a specialization of the protobuf class ZeroCopyInputStream
/// The principle is to get one chunk of data at a time from the proto layer,
/// with options to backup (re-see some bytes) or skip (forward past some bytes)
///
/// Read more about ZeroCopyInputStream interface here:
/// https://developers.google.com/protocol-buffers/docs/reference/cpp/google.protobuf.io.zero_copy_stream#ZeroCopyInputStream
class ProtoBufferReader : public grpc::protobuf::io::ZeroCopyInputStream {
 public:
  /// Constructs buffer reader from \a buffer. Will set \a status() to non ok
  /// if \a buffer is invalid (the internal buffer has not been initialized).
  explicit ProtoBufferReader(ByteBuffer* buffer)
      : byte_count_(0), backup_count_(0), status_() {
    /// Implemented through a grpc_byte_buffer_reader which iterates
    /// over the slices that make up a byte buffer
    if (!buffer->Valid() ||
        !grpc_byte_buffer_reader_init(&reader_, buffer->c_buffer())) {
      status_ = Status(StatusCode::INTERNAL,
                       "Couldn't initialize byte buffer reader");
    }
  }

  ~ProtoBufferReader() override {
    if (status_.ok()) {
      grpc_byte_buffer_reader_destroy(&reader_);
    }
  }

  /// Give the proto library a chunk of data from the stream. The caller
  /// may safely read from data[0, size - 1].
  bool Next(const void** data, int* size) override {
    if (!status_.ok()) {
      return false;
    }
    /// If we have backed up previously, we need to return the backed-up slice
    if (backup_count_ > 0) {
      *data = GRPC_SLICE_START_PTR(*slice_) + GRPC_SLICE_LENGTH(*slice_) -
              backup_count_;
      ABSL_CHECK_LE(backup_count_, INT_MAX);
      *size = static_cast<int>(backup_count_);
      backup_count_ = 0;
      return true;
    }
    /// Otherwise get the next slice from the byte buffer reader
    if (!grpc_byte_buffer_reader_peek(&reader_, &slice_)) {
      return false;
    }
    *data = GRPC_SLICE_START_PTR(*slice_);
    // On win x64, int is only 32bit
    ABSL_CHECK_LE(GRPC_SLICE_LENGTH(*slice_), static_cast<size_t>(INT_MAX));
    byte_count_ += * size = static_cast<int>(GRPC_SLICE_LENGTH(*slice_));
    return true;
  }

  /// Returns the status of the buffer reader.
  Status status() const { return status_; }

  /// The proto library calls this to indicate that we should back up \a count
  /// bytes that have already been returned by the last call of Next.
  /// So do the backup and have that ready for a later Next.
  void BackUp(int count) override {
    ABSL_CHECK_LE(count, static_cast<int>(GRPC_SLICE_LENGTH(*slice_)));
    backup_count_ = count;
  }

  /// The proto library calls this to skip over \a count bytes. Implement this
  /// using Next and BackUp combined.
  bool Skip(int count) override {
    const void* data;
    int size;
    while (Next(&data, &size)) {
      if (size >= count) {
        BackUp(size - count);
        return true;
      }
      // size < count;
      count -= size;
    }
    // error or we have too large count;
    return false;
  }

  /// Returns the total number of bytes read since this object was created.
  int64_t ByteCount() const override { return byte_count_ - backup_count_; }

#ifdef GRPC_PROTOBUF_CORD_SUPPORT_ENABLED
  /// Read the next `count` bytes and append it to the given Cord.
  // (override is conditionally omitted here to support old Protobuf which
  //  doesn't have ReadCord method)
  // NOLINTBEGIN(modernize-use-override,
  // clang-diagnostic-inconsistent-missing-override)
  virtual bool ReadCord(absl::Cord* cord, int count)
#if GOOGLE_PROTOBUF_VERSION >= 4022000
      override
#endif
  // NOLINTEND(modernize-use-override,
  // clang-diagnostic-inconsistent-missing-override)
  {
    if (!status().ok()) {
      return false;
    }
    // check for backed up data
    if (backup_count() > 0) {
      if (backup_count() <= count) {
        cord->Append(MakeCordFromSlice(grpc_slice_split_tail(
            slice(), GRPC_SLICE_LENGTH(*slice()) - backup_count())));
      } else {
        cord->Append(MakeCordFromSlice(grpc_slice_sub(
            *slice(), GRPC_SLICE_LENGTH(*slice()) - backup_count(),
            GRPC_SLICE_LENGTH(*slice()) - backup_count() + count)));
      }
      int64_t take = (std::min)(backup_count(), static_cast<int64_t>(count));
      set_backup_count(backup_count() - take);
      // This cast is safe as the size of a serialized protobuf message
      // should be smaller than 2GiB.
      // (https://protobuf.dev/programming-guides/encoding/#size-limit)
      count -= static_cast<int>(take);
      if (count == 0) {
        return true;
      }
    }
    while (count > 0) {
      if (!grpc_byte_buffer_reader_peek(reader(), mutable_slice_ptr())) {
        return false;
      }
      uint64_t slice_length = GRPC_SLICE_LENGTH(*slice());
      set_byte_count(ByteCount() + slice_length);
      if (slice_length <= static_cast<uint64_t>(count)) {
        cord->Append(MakeCordFromSlice(grpc_slice_ref(*slice())));
        // This cast is safe as above.
        count -= static_cast<int>(slice_length);
      } else {
        cord->Append(MakeCordFromSlice(grpc_slice_split_head(slice(), count)));
        set_backup_count(slice_length - count);
        return true;
      }
    }
    ABSL_CHECK_EQ(count, 0);
    return true;
  }
#endif  // GRPC_PROTOBUF_CORD_SUPPORT_ENABLED

  // These protected members are needed to support internal optimizations.
  // they expose internal bits of grpc core that are NOT stable. If you have
  // a use case needs to use one of these functions, please send an email to
  // https://groups.google.com/forum/#!forum/grpc-io.
 protected:
  void set_byte_count(int64_t byte_count) { byte_count_ = byte_count; }
  int64_t backup_count() { return backup_count_; }
  void set_backup_count(int64_t backup_count) { backup_count_ = backup_count; }
  grpc_byte_buffer_reader* reader() { return &reader_; }
  grpc_slice* slice() { return slice_; }
  grpc_slice** mutable_slice_ptr() { return &slice_; }

 private:
#ifdef GRPC_PROTOBUF_CORD_SUPPORT_ENABLED
  // This function takes ownership of slice and return a newly created Cord off
  // of it.
  static absl::Cord MakeCordFromSlice(grpc_slice slice) {
    // slice_for_cord is created to keep inlined data of the given slice
    grpc_slice* slice_for_cord = new grpc_slice;
    *slice_for_cord = slice;
    return absl::MakeCordFromExternal(
        absl::string_view(
            reinterpret_cast<char*>(GRPC_SLICE_START_PTR(*slice_for_cord)),
            GRPC_SLICE_LENGTH(*slice_for_cord)),
        [slice_for_cord](absl::string_view /* view */) {
          grpc_slice_unref(*slice_for_cord);
          delete slice_for_cord;
        });
  }
#endif  // GRPC_PROTOBUF_CORD_SUPPORT_ENABLED

  int64_t byte_count_;              ///< total bytes read since object creation
  int64_t backup_count_;            ///< how far backed up in the stream we are
  grpc_byte_buffer_reader reader_;  ///< internal object to read \a grpc_slice
                                    ///< from the \a grpc_byte_buffer
  grpc_slice* slice_;               ///< current slice passed back to the caller
  Status status_;                   ///< status of the entire object
};

}  // namespace grpc

#endif  // GRPCPP_SUPPORT_PROTO_BUFFER_READER_H
