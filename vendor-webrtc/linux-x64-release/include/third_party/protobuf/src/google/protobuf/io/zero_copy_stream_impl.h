// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: kenton@google.com (Kenton Varda)
//  Based on original Protocol Buffers design by
//  Sanjay Ghemawat, Jeff Dean, and others.
//
// This file contains common implementations of the interfaces defined in
// zero_copy_stream.h which are only included in the full (non-lite)
// protobuf library.  These implementations include Unix file descriptors
// and C++ iostreams.  See also:  zero_copy_stream_impl_lite.h

#ifndef GOOGLE_PROTOBUF_IO_ZERO_COPY_STREAM_IMPL_H__
#define GOOGLE_PROTOBUF_IO_ZERO_COPY_STREAM_IMPL_H__

#include <iosfwd>
#include <string>

#include "google/protobuf/stubs/common.h"
#include "google/protobuf/io/zero_copy_stream.h"
#include "google/protobuf/io/zero_copy_stream_impl_lite.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace io {

// ===================================================================

// A ZeroCopyInputStream which reads from a file descriptor.
//
// FileInputStream is preferred over using an ifstream with IstreamInputStream.
// The latter will introduce an extra layer of buffering, harming performance.
// Also, it's conceivable that FileInputStream could someday be enhanced
// to use zero-copy file descriptors on OSs which support them.
class PROTOBUF_EXPORT FileInputStream final : public ZeroCopyInputStream {
 public:
  // Creates a stream that reads from the given Unix file descriptor.
  // If a block_size is given, it specifies the number of bytes that
  // should be read and returned with each call to Next().  Otherwise,
  // a reasonable default is used.
  explicit FileInputStream(int file_descriptor, int block_size = -1);
  FileInputStream(const FileInputStream&) = delete;
  FileInputStream& operator=(const FileInputStream&) = delete;

  // Flushes any buffers and closes the underlying file.  Returns false if
  // an error occurs during the process; use GetErrno() to examine the error.
  // Even if an error occurs, the file descriptor is closed when this returns.
  bool Close();

  // By default, the file descriptor is not closed when the stream is
  // destroyed.  Call SetCloseOnDelete(true) to change that.  WARNING:
  // This leaves no way for the caller to detect if close() fails.  If
  // detecting close() errors is important to you, you should arrange
  // to close the descriptor yourself.
  void SetCloseOnDelete(bool value) { copying_input_.SetCloseOnDelete(value); }

  // If an I/O error has occurred on this file descriptor, this is the
  // errno from that error.  Otherwise, this is zero.  Once an error
  // occurs, the stream is broken and all subsequent operations will
  // fail.
  int GetErrno() const { return copying_input_.GetErrno(); }

  // implements ZeroCopyInputStream ----------------------------------
  bool Next(const void** data, int* size) override;
  void BackUp(int count) override;
  bool Skip(int count) override;
  int64_t ByteCount() const override;

 private:
  class PROTOBUF_EXPORT CopyingFileInputStream final
      : public CopyingInputStream {
   public:
    CopyingFileInputStream(int file_descriptor);
    CopyingFileInputStream(const CopyingFileInputStream&) = delete;
    CopyingFileInputStream& operator=(const CopyingFileInputStream&) = delete;
    ~CopyingFileInputStream() override;

    bool Close();
    void SetCloseOnDelete(bool value) { close_on_delete_ = value; }
    int GetErrno() const { return errno_; }

    // implements CopyingInputStream ---------------------------------
    int Read(void* buffer, int size) override;
    int Skip(int count) override;

   private:
    // The file descriptor.
    const int file_;
    bool close_on_delete_;
    bool is_closed_;

    // The errno of the I/O error, if one has occurred.  Otherwise, zero.
    int errno_;

    // Did we try to seek once and fail?  If so, we assume this file descriptor
    // doesn't support seeking and won't try again.
    bool previous_seek_failed_;
  };

  CopyingFileInputStream copying_input_;
  CopyingInputStreamAdaptor impl_;
};

// ===================================================================

// A ZeroCopyOutputStream which writes to a file descriptor.
//
// FileOutputStream is preferred over using an ofstream with
// OstreamOutputStream.  The latter will introduce an extra layer of buffering,
// harming performance.  Also, it's conceivable that FileOutputStream could
// someday be enhanced to use zero-copy file descriptors on OSs which
// support them.
class PROTOBUF_EXPORT FileOutputStream final
    : public CopyingOutputStreamAdaptor {
 public:
  // Creates a stream that writes to the given Unix file descriptor.
  // If a block_size is given, it specifies the size of the buffers
  // that should be returned by Next().  Otherwise, a reasonable default
  // is used.
  explicit FileOutputStream(int file_descriptor, int block_size = -1);
  FileOutputStream(const FileOutputStream&) = delete;
  FileOutputStream& operator=(const FileOutputStream&) = delete;

  ~FileOutputStream() override;

  // Flushes any buffers and closes the underlying file.  Returns false if
  // an error occurs during the process; use GetErrno() to examine the error.
  // Even if an error occurs, the file descriptor is closed when this returns.
  bool Close();

  // By default, the file descriptor is not closed when the stream is
  // destroyed.  Call SetCloseOnDelete(true) to change that.  WARNING:
  // This leaves no way for the caller to detect if close() fails.  If
  // detecting close() errors is important to you, you should arrange
  // to close the descriptor yourself.
  void SetCloseOnDelete(bool value) { copying_output_.SetCloseOnDelete(value); }

  // If an I/O error has occurred on this file descriptor, this is the
  // errno from that error.  Otherwise, this is zero.  Once an error
  // occurs, the stream is broken and all subsequent operations will
  // fail.
  int GetErrno() const { return copying_output_.GetErrno(); }

 private:
  class PROTOBUF_EXPORT CopyingFileOutputStream final
      : public CopyingOutputStream {
   public:
    CopyingFileOutputStream(int file_descriptor);
    CopyingFileOutputStream(const CopyingFileOutputStream&) = delete;
    CopyingFileOutputStream& operator=(const CopyingFileOutputStream&) = delete;
    ~CopyingFileOutputStream() override;

    bool Close();
    void SetCloseOnDelete(bool value) { close_on_delete_ = value; }
    int GetErrno() const { return errno_; }

    // implements CopyingOutputStream --------------------------------
    bool Write(const void* buffer, int size) override;

   private:
    // The file descriptor.
    const int file_;
    bool close_on_delete_;
    bool is_closed_;

    // The errno of the I/O error, if one has occurred.  Otherwise, zero.
    int errno_;
  };

  CopyingFileOutputStream copying_output_;
};

// ===================================================================

// A ZeroCopyInputStream which reads from a C++ istream.
//
// Note that for reading files (or anything represented by a file descriptor),
// FileInputStream is more efficient.
class PROTOBUF_EXPORT IstreamInputStream final : public ZeroCopyInputStream {
 public:
  // Creates a stream that reads from the given C++ istream.
  // If a block_size is given, it specifies the number of bytes that
  // should be read and returned with each call to Next().  Otherwise,
  // a reasonable default is used.
  explicit IstreamInputStream(std::istream* stream, int block_size = -1);
  IstreamInputStream(const IstreamInputStream&) = delete;
  IstreamInputStream& operator=(const IstreamInputStream&) = delete;

  // implements ZeroCopyInputStream ----------------------------------
  bool Next(const void** data, int* size) override;
  void BackUp(int count) override;
  bool Skip(int count) override;
  int64_t ByteCount() const override;

 private:
  class PROTOBUF_EXPORT CopyingIstreamInputStream final
      : public CopyingInputStream {
   public:
    CopyingIstreamInputStream(std::istream* input);
    CopyingIstreamInputStream(const CopyingIstreamInputStream&) = delete;
    CopyingIstreamInputStream& operator=(const CopyingIstreamInputStream&) =
        delete;
    ~CopyingIstreamInputStream() override;

    // implements CopyingInputStream ---------------------------------
    int Read(void* buffer, int size) override;
    // (We use the default implementation of Skip().)

   private:
    // The stream.
    std::istream* input_;
  };

  CopyingIstreamInputStream copying_input_;
  CopyingInputStreamAdaptor impl_;
};

// ===================================================================

// A ZeroCopyOutputStream which writes to a C++ ostream.
//
// Note that for writing files (or anything represented by a file descriptor),
// FileOutputStream is more efficient.
class PROTOBUF_EXPORT OstreamOutputStream final : public ZeroCopyOutputStream {
 public:
  // Creates a stream that writes to the given C++ ostream.
  // If a block_size is given, it specifies the size of the buffers
  // that should be returned by Next().  Otherwise, a reasonable default
  // is used.
  explicit OstreamOutputStream(std::ostream* stream, int block_size = -1);
  OstreamOutputStream(const OstreamOutputStream&) = delete;
  OstreamOutputStream& operator=(const OstreamOutputStream&) = delete;
  ~OstreamOutputStream() override;

  // implements ZeroCopyOutputStream ---------------------------------
  bool Next(void** data, int* size) override;
  void BackUp(int count) override;
  int64_t ByteCount() const override;

 private:
  class PROTOBUF_EXPORT CopyingOstreamOutputStream final
      : public CopyingOutputStream {
   public:
    CopyingOstreamOutputStream(std::ostream* output);
    CopyingOstreamOutputStream(const CopyingOstreamOutputStream&) = delete;
    CopyingOstreamOutputStream& operator=(const CopyingOstreamOutputStream&) =
        delete;
    ~CopyingOstreamOutputStream() override;

    // implements CopyingOutputStream --------------------------------
    bool Write(const void* buffer, int size) override;

   private:
    // The stream.
    std::ostream* output_;
  };

  CopyingOstreamOutputStream copying_output_;
  CopyingOutputStreamAdaptor impl_;
};

// ===================================================================

// A ZeroCopyInputStream which reads from several other streams in sequence.
// ConcatenatingInputStream is unable to distinguish between end-of-stream
// and read errors in the underlying streams, so it assumes any errors mean
// end-of-stream.  So, if the underlying streams fail for any other reason,
// ConcatenatingInputStream may do odd things.  It is suggested that you do
// not use ConcatenatingInputStream on streams that might produce read errors
// other than end-of-stream.
class PROTOBUF_EXPORT ConcatenatingInputStream final
    : public ZeroCopyInputStream {
 public:
  // All streams passed in as well as the array itself must remain valid
  // until the ConcatenatingInputStream is destroyed.
  ConcatenatingInputStream(ZeroCopyInputStream* const streams[], int count);
  ConcatenatingInputStream(const ConcatenatingInputStream&) = delete;
  ConcatenatingInputStream& operator=(const ConcatenatingInputStream&) = delete;
  ~ConcatenatingInputStream() override = default;

  // implements ZeroCopyInputStream ----------------------------------
  bool Next(const void** data, int* size) override;
  void BackUp(int count) override;
  bool Skip(int count) override;
  int64_t ByteCount() const override;


 private:
  // As streams are retired, streams_ is incremented and count_ is
  // decremented.
  ZeroCopyInputStream* const* streams_;
  int stream_count_;
  int64_t bytes_retired_;  // Bytes read from previous streams.
};

// ===================================================================

}  // namespace io
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_IO_ZERO_COPY_STREAM_IMPL_H__
