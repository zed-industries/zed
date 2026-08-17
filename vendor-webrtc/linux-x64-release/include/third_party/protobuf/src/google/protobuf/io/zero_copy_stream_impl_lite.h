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
// zero_copy_stream.h which are included in the "lite" protobuf library.
// These implementations cover I/O on raw arrays and strings, as well as
// adaptors which make it easy to implement streams based on traditional
// streams.  Of course, many users will probably want to write their own
// implementations of these interfaces specific to the particular I/O
// abstractions they prefer to use, but these should cover the most common
// cases.

#ifndef GOOGLE_PROTOBUF_IO_ZERO_COPY_STREAM_IMPL_LITE_H__
#define GOOGLE_PROTOBUF_IO_ZERO_COPY_STREAM_IMPL_LITE_H__

#include <iosfwd>
#include <memory>
#include <string>
#include <utility>

#include "google/protobuf/stubs/callback.h"
#include "google/protobuf/stubs/common.h"
#include "absl/base/attributes.h"
#include "absl/base/macros.h"
#include "absl/strings/cord.h"
#include "absl/strings/cord_buffer.h"
#include "google/protobuf/io/zero_copy_stream.h"
#include "google/protobuf/port.h"


// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace io {

// ===================================================================

// A ZeroCopyInputStream backed by an in-memory array of bytes.
class PROTOBUF_EXPORT ArrayInputStream final : public ZeroCopyInputStream {
 public:
  // Create an InputStream that returns the bytes pointed to by "data".
  // "data" remains the property of the caller but must remain valid until
  // the stream is destroyed.  If a block_size is given, calls to Next()
  // will return data blocks no larger than the given size.  Otherwise, the
  // first call to Next() returns the entire array.  block_size is mainly
  // useful for testing; in production you would probably never want to set
  // it.
  ArrayInputStream(const void* data, int size, int block_size = -1);
  ~ArrayInputStream() override = default;

  // `ArrayInputStream` is neither copiable nor assignable
  ArrayInputStream(const ArrayInputStream&) = delete;
  ArrayInputStream& operator=(const ArrayInputStream&) = delete;

  // implements ZeroCopyInputStream ----------------------------------
  bool Next(const void** data, int* size) override;
  void BackUp(int count) override;
  bool Skip(int count) override;
  int64_t ByteCount() const override;


 private:
  const uint8_t* const data_;  // The byte array.
  const int size_;           // Total size of the array.
  const int block_size_;     // How many bytes to return at a time.

  int position_;
  int last_returned_size_;  // How many bytes we returned last time Next()
                            // was called (used for error checking only).
};

// ===================================================================

// A ZeroCopyOutputStream backed by an in-memory array of bytes.
class PROTOBUF_EXPORT ArrayOutputStream final : public ZeroCopyOutputStream {
 public:
  // Create an OutputStream that writes to the bytes pointed to by "data".
  // "data" remains the property of the caller but must remain valid until
  // the stream is destroyed.  If a block_size is given, calls to Next()
  // will return data blocks no larger than the given size.  Otherwise, the
  // first call to Next() returns the entire array.  block_size is mainly
  // useful for testing; in production you would probably never want to set
  // it.
  ArrayOutputStream(void* data, int size, int block_size = -1);
  ~ArrayOutputStream() override = default;

  // `ArrayOutputStream` is neither copiable nor assignable
  ArrayOutputStream(const ArrayOutputStream&) = delete;
  ArrayOutputStream& operator=(const ArrayOutputStream&) = delete;

  // implements ZeroCopyOutputStream ---------------------------------
  bool Next(void** data, int* size) override;
  void BackUp(int count) override;
  int64_t ByteCount() const override;

 private:
  uint8_t* const data_;     // The byte array.
  const int size_;        // Total size of the array.
  const int block_size_;  // How many bytes to return at a time.

  int position_;
  int last_returned_size_;  // How many bytes we returned last time Next()
                            // was called (used for error checking only).
};

// ===================================================================

// A ZeroCopyOutputStream which appends bytes to a string.
class PROTOBUF_EXPORT StringOutputStream final : public ZeroCopyOutputStream {
 public:
  // Create a StringOutputStream which appends bytes to the given string.
  // The string remains property of the caller, but it is mutated in arbitrary
  // ways and MUST NOT be accessed in any way until you're done with the
  // stream. Either be sure there's no further usage, or (safest) destroy the
  // stream before using the contents.
  //
  // Hint:  If you call target->reserve(n) before creating the stream,
  //   the first call to Next() will return at least n bytes of buffer
  //   space.
  explicit StringOutputStream(std::string* target);
  ~StringOutputStream() override = default;

  // `StringOutputStream` is neither copiable nor assignable
  StringOutputStream(const StringOutputStream&) = delete;
  StringOutputStream& operator=(const StringOutputStream&) = delete;

  // implements ZeroCopyOutputStream ---------------------------------
  bool Next(void** data, int* size) override;
  void BackUp(int count) override;
  int64_t ByteCount() const override;

 private:
  static constexpr size_t kMinimumSize = 16;

  std::string* target_;
};

// Note:  There is no StringInputStream.  Instead, just create an
// ArrayInputStream as follows:
//   ArrayInputStream input(str.data(), str.size());

// ===================================================================

// A generic traditional input stream interface.
//
// Lots of traditional input streams (e.g. file descriptors, C stdio
// streams, and C++ iostreams) expose an interface where every read
// involves copying bytes into a buffer.  If you want to take such an
// interface and make a ZeroCopyInputStream based on it, simply implement
// CopyingInputStream and then use CopyingInputStreamAdaptor.
//
// CopyingInputStream implementations should avoid buffering if possible.
// CopyingInputStreamAdaptor does its own buffering and will read data
// in large blocks.
class PROTOBUF_EXPORT CopyingInputStream {
 public:
  virtual ~CopyingInputStream() {}

  // Reads up to "size" bytes into the given buffer.  Returns the number of
  // bytes read.  Read() waits until at least one byte is available, or
  // returns zero if no bytes will ever become available (EOF), or -1 if a
  // permanent read error occurred.
  virtual int Read(void* buffer, int size) = 0;

  // Skips the next "count" bytes of input.  Returns the number of bytes
  // actually skipped.  This will always be exactly equal to "count" unless
  // EOF was reached or a permanent read error occurred.
  //
  // The default implementation just repeatedly calls Read() into a scratch
  // buffer.
  virtual int Skip(int count);
};

// A ZeroCopyInputStream which reads from a CopyingInputStream.  This is
// useful for implementing ZeroCopyInputStreams that read from traditional
// streams.  Note that this class is not really zero-copy.
//
// If you want to read from file descriptors or C++ istreams, this is
// already implemented for you:  use FileInputStream or IstreamInputStream
// respectively.
class PROTOBUF_EXPORT CopyingInputStreamAdaptor : public ZeroCopyInputStream {
 public:
  // Creates a stream that reads from the given CopyingInputStream.
  // If a block_size is given, it specifies the number of bytes that
  // should be read and returned with each call to Next().  Otherwise,
  // a reasonable default is used.  The caller retains ownership of
  // copying_stream unless SetOwnsCopyingStream(true) is called.
  explicit CopyingInputStreamAdaptor(CopyingInputStream* copying_stream,
                                     int block_size = -1);
  ~CopyingInputStreamAdaptor() override;

  // `CopyingInputStreamAdaptor` is neither copiable nor assignable
  CopyingInputStreamAdaptor(const CopyingInputStreamAdaptor&) = delete;
  CopyingInputStreamAdaptor& operator=(const CopyingInputStreamAdaptor&) = delete;

  // Call SetOwnsCopyingStream(true) to tell the CopyingInputStreamAdaptor to
  // delete the underlying CopyingInputStream when it is destroyed.
  void SetOwnsCopyingStream(bool value) { owns_copying_stream_ = value; }

  // implements ZeroCopyInputStream ----------------------------------
  bool Next(const void** data, int* size) override;
  void BackUp(int count) override;
  bool Skip(int count) override;
  int64_t ByteCount() const override;

 private:
  // Insures that buffer_ is not NULL.
  void AllocateBufferIfNeeded();
  // Frees the buffer and resets buffer_used_.
  void FreeBuffer();

  // The underlying copying stream.
  CopyingInputStream* copying_stream_;
  bool owns_copying_stream_;

  // True if we have seen a permanent error from the underlying stream.
  bool failed_;

  // The current position of copying_stream_, relative to the point where
  // we started reading.
  int64_t position_;

  // Data is read into this buffer.  It may be NULL if no buffer is currently
  // in use.  Otherwise, it points to an array of size buffer_size_.
  std::unique_ptr<uint8_t[]> buffer_;
  const int buffer_size_;

  // Number of valid bytes currently in the buffer (i.e. the size last
  // returned by Next()).  0 <= buffer_used_ <= buffer_size_.
  int buffer_used_;

  // Number of bytes in the buffer which were backed up over by a call to
  // BackUp().  These need to be returned again.
  // 0 <= backup_bytes_ <= buffer_used_
  int backup_bytes_;
};

// ===================================================================

// A generic traditional output stream interface.
//
// Lots of traditional output streams (e.g. file descriptors, C stdio
// streams, and C++ iostreams) expose an interface where every write
// involves copying bytes from a buffer.  If you want to take such an
// interface and make a ZeroCopyOutputStream based on it, simply implement
// CopyingOutputStream and then use CopyingOutputStreamAdaptor.
//
// CopyingOutputStream implementations should avoid buffering if possible.
// CopyingOutputStreamAdaptor does its own buffering and will write data
// in large blocks.
class PROTOBUF_EXPORT CopyingOutputStream {
 public:
  virtual ~CopyingOutputStream() {}

  // Writes "size" bytes from the given buffer to the output.  Returns true
  // if successful, false on a write error.
  virtual bool Write(const void* buffer, int size) = 0;
};

// A ZeroCopyOutputStream which writes to a CopyingOutputStream.  This is
// useful for implementing ZeroCopyOutputStreams that write to traditional
// streams.  Note that this class is not really zero-copy.
//
// If you want to write to file descriptors or C++ ostreams, this is
// already implemented for you:  use FileOutputStream or OstreamOutputStream
// respectively.
class PROTOBUF_EXPORT CopyingOutputStreamAdaptor : public ZeroCopyOutputStream {
 public:
  // Creates a stream that writes to the given Unix file descriptor.
  // If a block_size is given, it specifies the size of the buffers
  // that should be returned by Next().  Otherwise, a reasonable default
  // is used.
  explicit CopyingOutputStreamAdaptor(CopyingOutputStream* copying_stream,
                                      int block_size = -1);
  ~CopyingOutputStreamAdaptor() override;

  // `CopyingOutputStreamAdaptor` is neither copiable nor assignable
  CopyingOutputStreamAdaptor(const CopyingOutputStreamAdaptor&) = delete;
  CopyingOutputStreamAdaptor& operator=(const CopyingOutputStreamAdaptor&) = delete;

  // Writes all pending data to the underlying stream.  Returns false if a
  // write error occurred on the underlying stream.  (The underlying
  // stream itself is not necessarily flushed.)
  bool Flush();

  // Call SetOwnsCopyingStream(true) to tell the CopyingOutputStreamAdaptor to
  // delete the underlying CopyingOutputStream when it is destroyed.
  void SetOwnsCopyingStream(bool value) { owns_copying_stream_ = value; }

  // implements ZeroCopyOutputStream ---------------------------------
  bool Next(void** data, int* size) override;
  void BackUp(int count) override;
  int64_t ByteCount() const override;
  bool WriteAliasedRaw(const void* data, int size) override;
  bool AllowsAliasing() const override { return true; }
  bool WriteCord(const absl::Cord& cord) override;

 private:
  // Write the current buffer, if it is present.
  bool WriteBuffer();
  // Insures that buffer_ is not NULL.
  void AllocateBufferIfNeeded();
  // Frees the buffer.
  void FreeBuffer();

  // The underlying copying stream.
  CopyingOutputStream* copying_stream_;
  bool owns_copying_stream_;

  // True if we have seen a permanent error from the underlying stream.
  bool failed_;

  // The current position of copying_stream_, relative to the point where
  // we started writing.
  int64_t position_;

  // Data is written from this buffer.  It may be NULL if no buffer is
  // currently in use.  Otherwise, it points to an array of size buffer_size_.
  std::unique_ptr<uint8_t[]> buffer_;
  const int buffer_size_;

  // Number of valid bytes currently in the buffer (i.e. the size last
  // returned by Next()).  When BackUp() is called, we just reduce this.
  // 0 <= buffer_used_ <= buffer_size_.
  int buffer_used_;
};

// ===================================================================

// A ZeroCopyInputStream which wraps some other stream and limits it to
// a particular byte count.
class PROTOBUF_EXPORT LimitingInputStream final : public ZeroCopyInputStream {
 public:
  LimitingInputStream(ZeroCopyInputStream* input, int64_t limit);
  ~LimitingInputStream() override;

  // `LimitingInputStream` is neither copiable nor assignable
  LimitingInputStream(const LimitingInputStream&) = delete;
  LimitingInputStream& operator=(const LimitingInputStream&) = delete;

  // implements ZeroCopyInputStream ----------------------------------
  bool Next(const void** data, int* size) override;
  void BackUp(int count) override;
  bool Skip(int count) override;
  int64_t ByteCount() const override;
  bool ReadCord(absl::Cord* cord, int count) override;


 private:
  ZeroCopyInputStream* input_;
  int64_t limit_;  // Decreases as we go, becomes negative if we overshoot.
  int64_t prior_bytes_read_;  // Bytes read on underlying stream at construction
};

// ===================================================================

// A ZeroCopyInputStream backed by a Cord.  This stream implements ReadCord()
// in a way that can share memory between the source and destination cords
// rather than copying.
class PROTOBUF_EXPORT CordInputStream final : public ZeroCopyInputStream {
 public:
  // Creates an InputStream that reads from the given Cord. `cord` must
  // not be null and must outlive this CordInputStream instance. `cord` must
  // not be modified while this instance is actively being used: any change
  // to `cord` will lead to undefined behavior on any subsequent call into
  // this instance.
  explicit CordInputStream(
      const absl::Cord* cord ABSL_ATTRIBUTE_LIFETIME_BOUND);


  // `CordInputStream` is neither copiable nor assignable
  CordInputStream(const CordInputStream&) = delete;
  CordInputStream& operator=(const CordInputStream&) = delete;

  // implements ZeroCopyInputStream ----------------------------------
  bool Next(const void** data, int* size) override;
  void BackUp(int count) override;
  bool Skip(int count) override;
  int64_t ByteCount() const override;
  bool ReadCord(absl::Cord* cord, int count) override;


 private:
  // Moves `it_` to the next available chunk skipping `skip` extra bytes
  // and updates the chunk data pointers.
  bool NextChunk(size_t skip);

  // Updates the current chunk data context `data_`, `size_` and `available_`.
  // If `bytes_remaining_` is zero, sets `size_` and `available_` to zero.
  // Returns true if more data is available, false otherwise.
  bool LoadChunkData();

  absl::Cord::CharIterator it_;
  size_t length_;
  size_t bytes_remaining_;
  const char* data_;
  size_t size_;
  size_t available_;
};

// ===================================================================

// A ZeroCopyOutputStream that writes to a Cord.  This stream implements
// WriteCord() in a way that can share memory between the source and
// destination cords rather than copying.
class PROTOBUF_EXPORT CordOutputStream final : public ZeroCopyOutputStream {
 public:
  // Creates an OutputStream streaming serialized data into a Cord. `size_hint`,
  // if given, is the expected total size of the resulting Cord. This is a hint
  // only, used for optimization. Callers can obtain the generated Cord value by
  // invoking `Consume()`.
  explicit CordOutputStream(size_t size_hint = 0);

  // Creates an OutputStream with an initial Cord value. This constructor can be
  // used by applications wanting to directly append serialization data to a
  // given cord. In such cases, donating the existing value as in:
  //
  //   CordOutputStream stream(std::move(cord));
  //   message.SerializeToZeroCopyStream(&stream);
  //   cord = std::move(stream.Consume());
  //
  // is more efficient then appending the serialized cord in application code:
  //
  //   CordOutputStream stream;
  //   message.SerializeToZeroCopyStream(&stream);
  //   cord.Append(stream.Consume());
  //
  // The former allows `CordOutputStream` to utilize pre-existing privately
  // owned Cord buffers from the donated cord where the latter does not, which
  // may lead to more memory usage when serialuzing data into existing cords.
  explicit CordOutputStream(absl::Cord cord, size_t size_hint = 0);

  // Creates an OutputStream with an initial Cord value and initial buffer.
  // This donates both the preexisting cord in `cord`, as well as any
  // pre-existing data and additional capacity in `buffer`.
  // This function is mainly intended to be used in internal serialization logic
  // using eager buffer initialization in EpsCopyOutputStream.
  // The donated buffer can be empty, partially empty or full: the outputstream
  // will DTRT in all cases and preserve any pre-existing data.
  explicit CordOutputStream(absl::Cord cord, absl::CordBuffer buffer,
                            size_t size_hint = 0);

  // Creates an OutputStream with an initial buffer.
  // This method is logically identical to, but more efficient than:
  //   `CordOutputStream(absl::Cord(), std::move(buffer), size_hint)`
  explicit CordOutputStream(absl::CordBuffer buffer, size_t size_hint = 0);

  // `CordOutputStream` is neither copiable nor assignable
  CordOutputStream(const CordOutputStream&) = delete;
  CordOutputStream& operator=(const CordOutputStream&) = delete;

  // implements `ZeroCopyOutputStream` ---------------------------------
  bool Next(void** data, int* size) final;
  void BackUp(int count) final;
  int64_t ByteCount() const final;
  bool WriteCord(const absl::Cord& cord) final;

  // Consumes the serialized data as a cord value. `Consume()` internally
  // flushes any pending state 'as if' BackUp(0) was called. While a final call
  // to BackUp() is generally required by the `ZeroCopyOutputStream` contract,
  // applications using `CordOutputStream` directly can call `Consume()` without
  // a preceding call to `BackUp()`.
  //
  // While it will rarely be useful in practice (and especially in the presence
  // of size hints) an instance is safe to be used after a call to `Consume()`.
  // The only logical change in state is that all serialized data is extracted,
  // and any new serialization calls will serialize into new cord data.
  absl::Cord Consume();

 private:
  // State of `buffer_` and 'cord_. As a default CordBuffer instance always has
  // inlined capacity, we track state explicitly to avoid returning 'existing
  // capacity' from the default or 'moved from' CordBuffer. 'kSteal' indicates
  // we should (attempt to) steal the next buffer from the cord.
  enum class State { kEmpty, kFull, kPartial, kSteal };

  absl::Cord cord_;
  size_t size_hint_;
  State state_ = State::kEmpty;
  absl::CordBuffer buffer_;
};


// ===================================================================

// Return a pointer to mutable characters underlying the given string.  The
// return value is valid until the next time the string is resized.  We
// trust the caller to treat the return value as an array of length s->size().
inline char* mutable_string_data(std::string* s) {
  return &(*s)[0];
}

// as_string_data(s) is equivalent to
//  ({ char* p = mutable_string_data(s); make_pair(p, p != NULL); })
// Sometimes it's faster: in some scenarios p cannot be NULL, and then the
// code can avoid that check.
inline std::pair<char*, bool> as_string_data(std::string* s) {
  char* p = mutable_string_data(s);
  return std::make_pair(p, true);
}

}  // namespace io
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_IO_ZERO_COPY_STREAM_IMPL_LITE_H__
