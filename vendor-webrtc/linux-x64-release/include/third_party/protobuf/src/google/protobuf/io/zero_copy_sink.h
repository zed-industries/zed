// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_UTIL_ZERO_COPY_SINK_H__
#define GOOGLE_PROTOBUF_UTIL_ZERO_COPY_SINK_H__

#include <cstddef>

#include "google/protobuf/io/zero_copy_stream.h"
#include "google/protobuf/port.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace io {
namespace zc_sink_internal {

// Internal helper class, for turning a ZeroCopyOutputStream into a sink.
class PROTOBUF_EXPORT ZeroCopyStreamByteSink {
 public:
  explicit ZeroCopyStreamByteSink(io::ZeroCopyOutputStream* stream)
      : stream_(stream) {}
  ZeroCopyStreamByteSink(const ZeroCopyStreamByteSink&) = delete;
  ZeroCopyStreamByteSink& operator=(const ZeroCopyStreamByteSink&) = delete;

  ~ZeroCopyStreamByteSink() {
    if (buffer_size_ > 0) {
      stream_->BackUp(buffer_size_);
    }
  }

  void Append(const char* bytes, size_t len);
  void Write(absl::string_view str) { Append(str.data(), str.size()); }

  size_t bytes_written() { return bytes_written_; }
  bool failed() { return failed_; }

 private:
  io::ZeroCopyOutputStream* stream_;
  void* buffer_ = nullptr;
  size_t buffer_size_ = 0;
  size_t bytes_written_ = 0;
  bool failed_ = false;
};
}  // namespace zc_sink_internal
}  // namespace io
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"
#endif  // GOOGLE_PROTOBUF_UTIL_ZERO_COPY_SINK_H__
