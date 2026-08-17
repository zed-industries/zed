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
// This file contains the ZeroCopyInputStream and ZeroCopyOutputStream
// interfaces, which represent abstract I/O streams to and from which
// protocol buffers can be read and written.  For a few simple
// implementations of these interfaces, see zero_copy_stream_impl.h.
//
// These interfaces are different from classic I/O streams in that they
// try to minimize the amount of data copying that needs to be done.
// To accomplish this, responsibility for allocating buffers is moved to
// the stream object, rather than being the responsibility of the caller.
// So, the stream can return a buffer which actually points directly into
// the final data structure where the bytes are to be stored, and the caller
// can interact directly with that buffer, eliminating an intermediate copy
// operation.
//
// As an example, consider the common case in which you are reading bytes
// from an array that is already in memory (or perhaps an mmap()ed file).
// With classic I/O streams, you would do something like:
//   char buffer[BUFFER_SIZE];
//   input->Read(buffer, BUFFER_SIZE);
//   DoSomething(buffer, BUFFER_SIZE);
// Then, the stream basically just calls memcpy() to copy the data from
// the array into your buffer.  With a ZeroCopyInputStream, you would do
// this instead:
//   const void* buffer;
//   int size;
//   input->Next(&buffer, &size);
//   DoSomething(buffer, size);
// Here, no copy is performed.  The input stream returns a pointer directly
// into the backing array, and the caller ends up reading directly from it.
//
// If you want to be able to read the old-fashion way, you can create
// a CodedInputStream or CodedOutputStream wrapping these objects and use
// their ReadRaw()/WriteRaw() methods.  These will, of course, add a copy
// step, but Coded*Stream will handle buffering so at least it will be
// reasonably efficient.
//
// ZeroCopyInputStream example:
//   // Read in a file and print its contents to stdout.
//   int fd = open("myfile", O_RDONLY);
//   ZeroCopyInputStream* input = new FileInputStream(fd);
//
//   const void* buffer;
//   int size;
//   while (input->Next(&buffer, &size)) {
//     cout.write(buffer, size);
//   }
//
//   delete input;
//   close(fd);
//
// ZeroCopyOutputStream example:
//   // Copy the contents of "infile" to "outfile", using plain read() for
//   // "infile" but a ZeroCopyOutputStream for "outfile".
//   int infd = open("infile", O_RDONLY);
//   int outfd = open("outfile", O_WRONLY);
//   ZeroCopyOutputStream* output = new FileOutputStream(outfd);
//
//   void* buffer;
//   int size;
//   while (output->Next(&buffer, &size)) {
//     int bytes = read(infd, buffer, size);
//     if (bytes < size) {
//       // Reached EOF.
//       output->BackUp(size - bytes);
//       break;
//     }
//   }
//
//   delete output;
//   close(infd);
//   close(outfd);

#ifndef GOOGLE_PROTOBUF_IO_ZERO_COPY_STREAM_H__
#define GOOGLE_PROTOBUF_IO_ZERO_COPY_STREAM_H__

#include "google/protobuf/stubs/common.h"
#include "absl/strings/cord.h"
#include "google/protobuf/port.h"


// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace io {

// Abstract interface similar to an input stream but designed to minimize
// copying.
class PROTOBUF_EXPORT ZeroCopyInputStream {
 public:
  ZeroCopyInputStream() = default;
  virtual ~ZeroCopyInputStream() = default;

  ZeroCopyInputStream(const ZeroCopyInputStream&) = delete;
  ZeroCopyInputStream& operator=(const ZeroCopyInputStream&) = delete;
  ZeroCopyInputStream(ZeroCopyInputStream&&) = delete;
  ZeroCopyInputStream& operator=(ZeroCopyInputStream&&) = delete;

  // Obtains a chunk of data from the stream.
  //
  // Preconditions:
  // * "size" and "data" are not NULL.
  //
  // Postconditions:
  // * If the returned value is false, there is no more data to return or
  //   an error occurred.  All errors are permanent.
  // * Otherwise, "size" points to the actual number of bytes read and "data"
  //   points to a pointer to a buffer containing these bytes.
  // * Ownership of this buffer remains with the stream, and the buffer
  //   remains valid only until some other method of the stream is called
  //   or the stream is destroyed.
  // * It is legal for the returned buffer to have zero size, as long
  //   as repeatedly calling Next() eventually yields a buffer with non-zero
  //   size.
  virtual bool Next(const void** data, int* size) = 0;

  // Backs up a number of bytes, so that the next call to Next() returns
  // data again that was already returned by the last call to Next().  This
  // is useful when writing procedures that are only supposed to read up
  // to a certain point in the input, then return.  If Next() returns a
  // buffer that goes beyond what you wanted to read, you can use BackUp()
  // to return to the point where you intended to finish.
  //
  // Preconditions:
  // * The last method called must have been Next().
  // * count must be less than or equal to the size of the last buffer
  //   returned by Next().
  //
  // Postconditions:
  // * The last "count" bytes of the last buffer returned by Next() will be
  //   pushed back into the stream.  Subsequent calls to Next() will return
  //   the same data again before producing new data.
  virtual void BackUp(int count) = 0;

  // Skips `count` number of bytes.
  // Returns true on success, or false if some input error occurred, or `count`
  // exceeds the end of the stream. This function may skip up to `count - 1`
  // bytes in case of failure.
  //
  // Preconditions:
  // * `count` is non-negative.
  //
  virtual bool Skip(int count) = 0;

  // Returns the total number of bytes read since this object was created.
  virtual int64_t ByteCount() const = 0;

  // Read the next `count` bytes and append it to the given Cord.
  //
  // In the case of a read error, the method reads as much data as possible into
  // the cord before returning false. The default implementation iterates over
  // the buffers and appends up to `count` bytes of data into `cord` using the
  // `absl::CordBuffer` API.
  //
  // Some streams may implement this in a way that avoids copying by sharing or
  // reference counting existing data managed by the stream implementation.
  //
  virtual bool ReadCord(absl::Cord* cord, int count);

};

// Abstract interface similar to an output stream but designed to minimize
// copying.
class PROTOBUF_EXPORT ZeroCopyOutputStream {
 public:
  ZeroCopyOutputStream() {}
  ZeroCopyOutputStream(const ZeroCopyOutputStream&) = delete;
  ZeroCopyOutputStream& operator=(const ZeroCopyOutputStream&) = delete;
  virtual ~ZeroCopyOutputStream() {}

  // Obtains a buffer into which data can be written.  Any data written
  // into this buffer will eventually (maybe instantly, maybe later on)
  // be written to the output.
  //
  // Preconditions:
  // * "size" and "data" are not NULL.
  //
  // Postconditions:
  // * If the returned value is false, an error occurred.  All errors are
  //   permanent.
  // * Otherwise, "size" points to the actual number of bytes in the buffer
  //   and "data" points to the buffer.
  // * Ownership of this buffer remains with the stream, and the buffer
  //   remains valid only until some other method of the stream is called
  //   or the stream is destroyed.
  // * Any data which the caller stores in this buffer will eventually be
  //   written to the output (unless BackUp() is called).
  // * It is legal for the returned buffer to have zero size, as long
  //   as repeatedly calling Next() eventually yields a buffer with non-zero
  //   size.
  virtual bool Next(void** data, int* size) = 0;

  // Backs up a number of bytes, so that the end of the last buffer returned
  // by Next() is not actually written.  This is needed when you finish
  // writing all the data you want to write, but the last buffer was bigger
  // than you needed.  You don't want to write a bunch of garbage after the
  // end of your data, so you use BackUp() to back up.
  //
  // This method can be called with `count = 0` to finalize (flush) any
  // previously returned buffer. For example, a file output stream can
  // flush buffers returned from a previous call to Next() upon such
  // BackUp(0) invocations. ZeroCopyOutputStream callers should always
  // invoke BackUp() after a final Next() call, even if there is no
  // excess buffer data to be backed up to indicate a flush point.
  //
  // Preconditions:
  // * The last method called must have been Next().
  // * count must be less than or equal to the size of the last buffer
  //   returned by Next().
  // * The caller must not have written anything to the last "count" bytes
  //   of that buffer.
  //
  // Postconditions:
  // * The last "count" bytes of the last buffer returned by Next() will be
  //   ignored.
  virtual void BackUp(int count) = 0;

  // Returns the total number of bytes written since this object was created.
  virtual int64_t ByteCount() const = 0;

  // Write a given chunk of data to the output.  Some output streams may
  // implement this in a way that avoids copying. Check AllowsAliasing() before
  // calling WriteAliasedRaw(). It will ABSL_CHECK fail if WriteAliasedRaw() is
  // called on a stream that does not allow aliasing.
  //
  // NOTE: It is caller's responsibility to ensure that the chunk of memory
  // remains live until all of the data has been consumed from the stream.
  virtual bool WriteAliasedRaw(const void* data, int size);
  virtual bool AllowsAliasing() const { return false; }

  // Writes the given Cord to the output.
  //
  // The default implementation iterates over all Cord chunks copying all cord
  // data into the buffer(s) returned by the stream's `Next()` method.
  //
  // Some streams may implement this in a way that avoids copying the cord
  // data by copying and managing a copy of the provided cord instead.
  virtual bool WriteCord(const absl::Cord& cord);

};

}  // namespace io
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_IO_ZERO_COPY_STREAM_H__
