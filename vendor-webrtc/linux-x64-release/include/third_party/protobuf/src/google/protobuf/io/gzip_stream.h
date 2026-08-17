// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: brianolson@google.com (Brian Olson)
//
// This file contains the definition for classes GzipInputStream and
// GzipOutputStream.
//
// GzipInputStream decompresses data from an underlying
// ZeroCopyInputStream and provides the decompressed data as a
// ZeroCopyInputStream.
//
// GzipOutputStream is an ZeroCopyOutputStream that compresses data to
// an underlying ZeroCopyOutputStream.

#ifndef GOOGLE_PROTOBUF_IO_GZIP_STREAM_H__
#define GOOGLE_PROTOBUF_IO_GZIP_STREAM_H__

#include "google/protobuf/stubs/common.h"
#include "google/protobuf/io/zero_copy_stream.h"
#include "google/protobuf/port.h"
#include <zlib.h>

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace io {

// A ZeroCopyInputStream that reads compressed data through zlib
class PROTOBUF_EXPORT GzipInputStream final : public ZeroCopyInputStream {
 public:
  // Format key for constructor
  enum Format {
    // zlib will autodetect gzip header or deflate stream
    AUTO = 0,

    // GZIP streams have some extra header data for file attributes.
    GZIP = 1,

    // Simpler zlib stream format.
    ZLIB = 2,
  };

  // buffer_size and format may be -1 for default of 64kB and GZIP format
  explicit GzipInputStream(ZeroCopyInputStream* sub_stream,
                           Format format = AUTO, int buffer_size = -1);
  GzipInputStream(const GzipInputStream&) = delete;
  GzipInputStream& operator=(const GzipInputStream&) = delete;
  ~GzipInputStream() override;

  // Return last error message or NULL if no error.
  inline const char* ZlibErrorMessage() const { return zcontext_.msg; }
  inline int ZlibErrorCode() const { return zerror_; }

  // implements ZeroCopyInputStream ----------------------------------
  bool Next(const void** data, int* size) override;
  void BackUp(int count) override;
  bool Skip(int count) override;
  int64_t ByteCount() const override;

 private:
  Format format_;

  ZeroCopyInputStream* sub_stream_;

  z_stream zcontext_;
  int zerror_;

  void* output_buffer_;
  void* output_position_;
  size_t output_buffer_length_;
  int64_t byte_count_;

  int Inflate(int flush);
  void DoNextOutput(const void** data, int* size);
};

class PROTOBUF_EXPORT GzipOutputStream final : public ZeroCopyOutputStream {
 public:
  // Format key for constructor
  enum Format {
    // GZIP streams have some extra header data for file attributes.
    GZIP = 1,

    // Simpler zlib stream format.
    ZLIB = 2,
  };

  struct PROTOBUF_EXPORT Options {
    // Defaults to GZIP.
    Format format;

    // What size buffer to use internally.  Defaults to 64kB.
    int buffer_size;

    // A number between 0 and 9, where 0 is no compression and 9 is best
    // compression.  Defaults to Z_DEFAULT_COMPRESSION (see zlib.h).
    int compression_level;

    // Defaults to Z_DEFAULT_STRATEGY.  Can also be set to Z_FILTERED,
    // Z_HUFFMAN_ONLY, or Z_RLE.  See the documentation for deflateInit2 in
    // zlib.h for definitions of these constants.
    int compression_strategy;

    Options();  // Initializes with default values.
  };

  // Create a GzipOutputStream with default options.
  explicit GzipOutputStream(ZeroCopyOutputStream* sub_stream);

  // Create a GzipOutputStream with the given options.
  GzipOutputStream(ZeroCopyOutputStream* sub_stream, const Options& options);
  GzipOutputStream(const GzipOutputStream&) = delete;
  GzipOutputStream& operator=(const GzipOutputStream&) = delete;

  ~GzipOutputStream() override;

  // Return last error message or NULL if no error.
  inline const char* ZlibErrorMessage() const { return zcontext_.msg; }
  inline int ZlibErrorCode() const { return zerror_; }

  // Flushes data written so far to zipped data in the underlying stream.
  // It is the caller's responsibility to flush the underlying stream if
  // necessary.
  // Compression may be less efficient stopping and starting around flushes.
  // Returns true if no error.
  //
  // Please ensure that block size is > 6. Here is an excerpt from the zlib
  // doc that explains why:
  //
  // In the case of a Z_FULL_FLUSH or Z_SYNC_FLUSH, make sure that avail_out
  // is greater than six to avoid repeated flush markers due to
  // avail_out == 0 on return.
  bool Flush();

  // Writes out all data and closes the gzip stream.
  // It is the caller's responsibility to close the underlying stream if
  // necessary.
  // Returns true if no error.
  bool Close();

  // implements ZeroCopyOutputStream ---------------------------------
  bool Next(void** data, int* size) override;
  void BackUp(int count) override;
  int64_t ByteCount() const override;

 private:
  ZeroCopyOutputStream* sub_stream_;
  // Result from calling Next() on sub_stream_
  void* sub_data_;
  int sub_data_size_;

  z_stream zcontext_;
  int zerror_;
  void* input_buffer_;
  size_t input_buffer_length_;

  // Shared constructor code.
  void Init(ZeroCopyOutputStream* sub_stream, const Options& options);

  // Do some compression.
  // Takes zlib flush mode.
  // Returns zlib error code.
  int Deflate(int flush);
};

}  // namespace io
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_IO_GZIP_STREAM_H__
