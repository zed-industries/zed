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
// This file contains the CodedInputStream and CodedOutputStream classes,
// which wrap a ZeroCopyInputStream or ZeroCopyOutputStream, respectively,
// and allow you to read or write individual pieces of data in various
// formats.  In particular, these implement the varint encoding for
// integers, a simple variable-length encoding in which smaller numbers
// take fewer bytes.
//
// Typically these classes will only be used internally by the protocol
// buffer library in order to encode and decode protocol buffers.  Clients
// of the library only need to know about this class if they wish to write
// custom message parsing or serialization procedures.
//
// CodedOutputStream example:
//   // Write some data to "myfile".  First we write a 4-byte "magic number"
//   // to identify the file type, then write a length-prefixed string.  The
//   // string is composed of a varint giving the length followed by the raw
//   // bytes.
//   int fd = open("myfile", O_CREAT | O_WRONLY);
//   ZeroCopyOutputStream* raw_output = new FileOutputStream(fd);
//   CodedOutputStream* coded_output = new CodedOutputStream(raw_output);
//
//   int magic_number = 1234;
//   char text[] = "Hello world!";
//   coded_output->WriteLittleEndian32(magic_number);
//   coded_output->WriteVarint32(strlen(text));
//   coded_output->WriteRaw(text, strlen(text));
//
//   delete coded_output;
//   delete raw_output;
//   close(fd);
//
// CodedInputStream example:
//   // Read a file created by the above code.
//   int fd = open("myfile", O_RDONLY);
//   ZeroCopyInputStream* raw_input = new FileInputStream(fd);
//   CodedInputStream* coded_input = new CodedInputStream(raw_input);
//
//   coded_input->ReadLittleEndian32(&magic_number);
//   if (magic_number != 1234) {
//     cerr << "File not in expected format." << endl;
//     return;
//   }
//
//   uint32_t size;
//   coded_input->ReadVarint32(&size);
//
//   char* text = new char[size + 1];
//   coded_input->ReadRaw(buffer, size);
//   text[size] = '\0';
//
//   delete coded_input;
//   delete raw_input;
//   close(fd);
//
//   cout << "Text is: " << text << endl;
//   delete [] text;
//
// For those who are interested, varint encoding is defined as follows:
//
// The encoding operates on unsigned integers of up to 64 bits in length.
// Each byte of the encoded value has the format:
// * bits 0-6: Seven bits of the number being encoded.
// * bit 7: Zero if this is the last byte in the encoding (in which
//   case all remaining bits of the number are zero) or 1 if
//   more bytes follow.
// The first byte contains the least-significant 7 bits of the number, the
// second byte (if present) contains the next-least-significant 7 bits,
// and so on.  So, the binary number 1011000101011 would be encoded in two
// bytes as "10101011 00101100".
//
// In theory, varint could be used to encode integers of any length.
// However, for practicality we set a limit at 64 bits.  The maximum encoded
// length of a number is thus 10 bytes.

#ifndef GOOGLE_PROTOBUF_IO_CODED_STREAM_H__
#define GOOGLE_PROTOBUF_IO_CODED_STREAM_H__

#include <assert.h>

#include <atomic>
#include <climits>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <limits>
#include <string>
#include <type_traits>
#include <utility>

#if defined(_MSC_VER) && _MSC_VER >= 1300 && !defined(__INTEL_COMPILER)
// If MSVC has "/RTCc" set, it will complain about truncating casts at
// runtime.  This file contains some intentional truncating casts.
#pragma runtime_checks("c", off)
#endif

#include "absl/log/absl_log.h"  // Replace with vlog_is_on.h after Abseil LTS 20240722

#include "absl/log/absl_check.h"
#include "absl/numeric/bits.h"
#include "absl/strings/cord.h"
#include "absl/strings/string_view.h"
#include "google/protobuf/endian.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {

class DescriptorPool;
class MessageFactory;
class ZeroCopyCodedInputStream;

namespace internal {
void MapTestForceDeterministic();
class EpsCopyByteStream;
}  // namespace internal

namespace io {

// Defined in this file.
class CodedInputStream;
class CodedOutputStream;

// Defined in other files.
class ZeroCopyInputStream;   // zero_copy_stream.h
class ZeroCopyOutputStream;  // zero_copy_stream.h

// Class which reads and decodes binary data which is composed of varint-
// encoded integers and fixed-width pieces.  Wraps a ZeroCopyInputStream.
// Most users will not need to deal with CodedInputStream.
//
// Most methods of CodedInputStream that return a bool return false if an
// underlying I/O error occurs or if the data is malformed.  Once such a
// failure occurs, the CodedInputStream is broken and is no longer useful.
// After a failure, callers also should assume writes to "out" args may have
// occurred, though nothing useful can be determined from those writes.
class PROTOBUF_EXPORT CodedInputStream {
 public:
  // Create a CodedInputStream that reads from the given ZeroCopyInputStream.
  explicit CodedInputStream(ZeroCopyInputStream* input);

  // Create a CodedInputStream that reads from the given flat array.  This is
  // faster than using an ArrayInputStream.  PushLimit(size) is implied by
  // this constructor.
  explicit CodedInputStream(const uint8_t* buffer, int size);
  CodedInputStream(const CodedInputStream&) = delete;
  CodedInputStream& operator=(const CodedInputStream&) = delete;

  // Destroy the CodedInputStream and position the underlying
  // ZeroCopyInputStream at the first unread byte.  If an error occurred while
  // reading (causing a method to return false), then the exact position of
  // the input stream may be anywhere between the last value that was read
  // successfully and the stream's byte limit.
  ~CodedInputStream();

  // Return true if this CodedInputStream reads from a flat array instead of
  // a ZeroCopyInputStream.
  inline bool IsFlat() const;

  // Skips a number of bytes.  Returns false if an underlying read error
  // occurs.
  inline bool Skip(int count);

  // Sets *data to point directly at the unread part of the CodedInputStream's
  // underlying buffer, and *size to the size of that buffer, but does not
  // advance the stream's current position.  This will always either produce
  // a non-empty buffer or return false.  If the caller consumes any of
  // this data, it should then call Skip() to skip over the consumed bytes.
  // This may be useful for implementing external fast parsing routines for
  // types of data not covered by the CodedInputStream interface.
  bool GetDirectBufferPointer(const void** data, int* size);

  // Like GetDirectBufferPointer, but this method is inlined, and does not
  // attempt to Refresh() if the buffer is currently empty.
  PROTOBUF_ALWAYS_INLINE
  void GetDirectBufferPointerInline(const void** data, int* size);

  // Read raw bytes, copying them into the given buffer.
  bool ReadRaw(void* buffer, int size);

  // Like ReadRaw, but reads into a string.
  bool ReadString(std::string* buffer, int size);

  // Like ReadString(), but reads to a Cord.
  bool ReadCord(absl::Cord* output, int size);


  // Read a 16-bit little-endian integer.
  bool ReadLittleEndian16(uint16_t* value);
  // Read a 32-bit little-endian integer.
  bool ReadLittleEndian32(uint32_t* value);
  // Read a 64-bit little-endian integer.
  bool ReadLittleEndian64(uint64_t* value);

  // These methods read from an externally provided buffer. The caller is
  // responsible for ensuring that the buffer has sufficient space.
  // Read a 16-bit little-endian integer.
  static const uint8_t* ReadLittleEndian16FromArray(const uint8_t* buffer,
                                                    uint16_t* value);
  // Read a 32-bit little-endian integer.
  static const uint8_t* ReadLittleEndian32FromArray(const uint8_t* buffer,
                                                    uint32_t* value);
  // Read a 64-bit little-endian integer.
  static const uint8_t* ReadLittleEndian64FromArray(const uint8_t* buffer,
                                                    uint64_t* value);

  // Read an unsigned integer with Varint encoding, truncating to 32 bits.
  // Reading a 32-bit value is equivalent to reading a 64-bit one and casting
  // it to uint32_t, but may be more efficient.
  bool ReadVarint32(uint32_t* value);
  // Read an unsigned integer with Varint encoding.
  bool ReadVarint64(uint64_t* value);

  // Reads a varint off the wire into an "int". This should be used for reading
  // sizes off the wire (sizes of strings, submessages, bytes fields, etc).
  //
  // The value from the wire is interpreted as unsigned.  If its value exceeds
  // the representable value of an integer on this platform, instead of
  // truncating we return false. Truncating (as performed by ReadVarint32()
  // above) is an acceptable approach for fields representing an integer, but
  // when we are parsing a size from the wire, truncating the value would result
  // in us misparsing the payload.
  bool ReadVarintSizeAsInt(int* value);

  // Read a tag.  This calls ReadVarint32() and returns the result, or returns
  // zero (which is not a valid tag) if ReadVarint32() fails.  Also, ReadTag
  // (but not ReadTagNoLastTag) updates the last tag value, which can be checked
  // with LastTagWas().
  //
  // Always inline because this is only called in one place per parse loop
  // but it is called for every iteration of said loop, so it should be fast.
  // GCC doesn't want to inline this by default.
  PROTOBUF_ALWAYS_INLINE uint32_t ReadTag() {
    return last_tag_ = ReadTagNoLastTag();
  }

  PROTOBUF_ALWAYS_INLINE uint32_t ReadTagNoLastTag();

  // This usually a faster alternative to ReadTag() when cutoff is a manifest
  // constant.  It does particularly well for cutoff >= 127.  The first part
  // of the return value is the tag that was read, though it can also be 0 in
  // the cases where ReadTag() would return 0.  If the second part is true
  // then the tag is known to be in [0, cutoff].  If not, the tag either is
  // above cutoff or is 0.  (There's intentional wiggle room when tag is 0,
  // because that can arise in several ways, and for best performance we want
  // to avoid an extra "is tag == 0?" check here.)
  PROTOBUF_ALWAYS_INLINE
  std::pair<uint32_t, bool> ReadTagWithCutoff(uint32_t cutoff) {
    std::pair<uint32_t, bool> result = ReadTagWithCutoffNoLastTag(cutoff);
    last_tag_ = result.first;
    return result;
  }

  PROTOBUF_ALWAYS_INLINE
  std::pair<uint32_t, bool> ReadTagWithCutoffNoLastTag(uint32_t cutoff);

  // Usually returns true if calling ReadVarint32() now would produce the given
  // value.  Will always return false if ReadVarint32() would not return the
  // given value.  If ExpectTag() returns true, it also advances past
  // the varint.  For best performance, use a compile-time constant as the
  // parameter.
  // Always inline because this collapses to a small number of instructions
  // when given a constant parameter, but GCC doesn't want to inline by default.
  PROTOBUF_ALWAYS_INLINE bool ExpectTag(uint32_t expected);

  // Like above, except this reads from the specified buffer. The caller is
  // responsible for ensuring that the buffer is large enough to read a varint
  // of the expected size. For best performance, use a compile-time constant as
  // the expected tag parameter.
  //
  // Returns a pointer beyond the expected tag if it was found, or NULL if it
  // was not.
  PROTOBUF_ALWAYS_INLINE
  static const uint8_t* ExpectTagFromArray(const uint8_t* buffer,
                                           uint32_t expected);

  // Usually returns true if no more bytes can be read.  Always returns false
  // if more bytes can be read.  If ExpectAtEnd() returns true, a subsequent
  // call to LastTagWas() will act as if ReadTag() had been called and returned
  // zero, and ConsumedEntireMessage() will return true.
  bool ExpectAtEnd();

  // If the last call to ReadTag() or ReadTagWithCutoff() returned the given
  // value, returns true.  Otherwise, returns false.
  // ReadTagNoLastTag/ReadTagWithCutoffNoLastTag do not preserve the last
  // returned value.
  //
  // This is needed because parsers for some types of embedded messages
  // (with field type TYPE_GROUP) don't actually know that they've reached the
  // end of a message until they see an ENDGROUP tag, which was actually part
  // of the enclosing message.  The enclosing message would like to check that
  // tag to make sure it had the right number, so it calls LastTagWas() on
  // return from the embedded parser to check.
  bool LastTagWas(uint32_t expected);
  void SetLastTag(uint32_t tag) { last_tag_ = tag; }

  // When parsing message (but NOT a group), this method must be called
  // immediately after MergeFromCodedStream() returns (if it returns true)
  // to further verify that the message ended in a legitimate way.  For
  // example, this verifies that parsing did not end on an end-group tag.
  // It also checks for some cases where, due to optimizations,
  // MergeFromCodedStream() can incorrectly return true.
  bool ConsumedEntireMessage();
  void SetConsumed() { legitimate_message_end_ = true; }

  // Limits ----------------------------------------------------------
  // Limits are used when parsing length-prefixed embedded messages.
  // After the message's length is read, PushLimit() is used to prevent
  // the CodedInputStream from reading beyond that length.  Once the
  // embedded message has been parsed, PopLimit() is called to undo the
  // limit.

  // Opaque type used with PushLimit() and PopLimit().  Do not modify
  // values of this type yourself.  The only reason that this isn't a
  // struct with private internals is for efficiency.
  typedef int Limit;

  // Places a limit on the number of bytes that the stream may read,
  // starting from the current position.  Once the stream hits this limit,
  // it will act like the end of the input has been reached until PopLimit()
  // is called.
  //
  // As the names imply, the stream conceptually has a stack of limits.  The
  // shortest limit on the stack is always enforced, even if it is not the
  // top limit.
  //
  // The value returned by PushLimit() is opaque to the caller, and must
  // be passed unchanged to the corresponding call to PopLimit().
  Limit PushLimit(int byte_limit);

  // Pops the last limit pushed by PushLimit().  The input must be the value
  // returned by that call to PushLimit().
  void PopLimit(Limit limit);

  // Returns the number of bytes left until the nearest limit on the
  // stack is hit, or -1 if no limits are in place.
  int BytesUntilLimit() const;

  // Returns current position relative to the beginning of the input stream.
  int CurrentPosition() const;

  // Total Bytes Limit -----------------------------------------------
  // To prevent malicious users from sending excessively large messages
  // and causing memory exhaustion, CodedInputStream imposes a hard limit on
  // the total number of bytes it will read.

  // Sets the maximum number of bytes that this CodedInputStream will read
  // before refusing to continue.  To prevent servers from allocating enormous
  // amounts of memory to hold parsed messages, the maximum message length
  // should be limited to the shortest length that will not harm usability.
  // The default limit is INT_MAX (~2GB) and apps should set shorter limits
  // if possible. An error will always be printed to stderr if the limit is
  // reached.
  //
  // Note: setting a limit less than the current read position is interpreted
  // as a limit on the current position.
  //
  // This is unrelated to PushLimit()/PopLimit().
  void SetTotalBytesLimit(int total_bytes_limit);

  // The Total Bytes Limit minus the Current Position, or -1 if the total bytes
  // limit is INT_MAX.
  int BytesUntilTotalBytesLimit() const;

  // Recursion Limit -------------------------------------------------
  // To prevent corrupt or malicious messages from causing stack overflows,
  // we must keep track of the depth of recursion when parsing embedded
  // messages and groups.  CodedInputStream keeps track of this because it
  // is the only object that is passed down the stack during parsing.

  // Sets the maximum recursion depth.  The default is 100.
  void SetRecursionLimit(int limit);
  int RecursionBudget() { return recursion_budget_; }

  static int GetDefaultRecursionLimit() { return default_recursion_limit_; }

  // Increments the current recursion depth.  Returns true if the depth is
  // under the limit, false if it has gone over.
  bool IncrementRecursionDepth();

  // Decrements the recursion depth if possible.
  void DecrementRecursionDepth();

  // Decrements the recursion depth blindly.  This is faster than
  // DecrementRecursionDepth().  It should be used only if all previous
  // increments to recursion depth were successful.
  void UnsafeDecrementRecursionDepth();

  // Shorthand for make_pair(PushLimit(byte_limit), --recursion_budget_).
  // Using this can reduce code size and complexity in some cases.  The caller
  // is expected to check that the second part of the result is non-negative (to
  // bail out if the depth of recursion is too high) and, if all is well, to
  // later pass the first part of the result to PopLimit() or similar.
  std::pair<CodedInputStream::Limit, int> IncrementRecursionDepthAndPushLimit(
      int byte_limit);

  // Shorthand for PushLimit(ReadVarint32(&length) ? length : 0).
  Limit ReadLengthAndPushLimit();

  // Helper that is equivalent to: {
  //  bool result = ConsumedEntireMessage();
  //  PopLimit(limit);
  //  UnsafeDecrementRecursionDepth();
  //  return result; }
  // Using this can reduce code size and complexity in some cases.
  // Do not use unless the current recursion depth is greater than zero.
  bool DecrementRecursionDepthAndPopLimit(Limit limit);

  // Helper that is equivalent to: {
  //  bool result = ConsumedEntireMessage();
  //  PopLimit(limit);
  //  return result; }
  // Using this can reduce code size and complexity in some cases.
  bool CheckEntireMessageConsumedAndPopLimit(Limit limit);

  // Extension Registry ----------------------------------------------
  // ADVANCED USAGE:  99.9% of people can ignore this section.
  //
  // By default, when parsing extensions, the parser looks for extension
  // definitions in the pool which owns the outer message's Descriptor.
  // However, you may call SetExtensionRegistry() to provide an alternative
  // pool instead.  This makes it possible, for example, to parse a message
  // using a generated class, but represent some extensions using
  // DynamicMessage.

  // Set the pool used to look up extensions.  Most users do not need to call
  // this as the correct pool will be chosen automatically.
  //
  // WARNING:  It is very easy to misuse this.  Carefully read the requirements
  //   below.  Do not use this unless you are sure you need it.  Almost no one
  //   does.
  //
  // Let's say you are parsing a message into message object m, and you want
  // to take advantage of SetExtensionRegistry().  You must follow these
  // requirements:
  //
  // The given DescriptorPool must contain m->GetDescriptor().  It is not
  // sufficient for it to simply contain a descriptor that has the same name
  // and content -- it must be the *exact object*.  In other words:
  //   assert(pool->FindMessageTypeByName(m->GetDescriptor()->full_name()) ==
  //          m->GetDescriptor());
  // There are two ways to satisfy this requirement:
  // 1) Use m->GetDescriptor()->pool() as the pool.  This is generally useless
  //    because this is the pool that would be used anyway if you didn't call
  //    SetExtensionRegistry() at all.
  // 2) Use a DescriptorPool which has m->GetDescriptor()->pool() as an
  //    "underlay".  Read the documentation for DescriptorPool for more
  //    information about underlays.
  //
  // You must also provide a MessageFactory.  This factory will be used to
  // construct Message objects representing extensions.  The factory's
  // GetPrototype() MUST return non-NULL for any Descriptor which can be found
  // through the provided pool.
  //
  // If the provided factory might return instances of protocol-compiler-
  // generated (i.e. compiled-in) types, or if the outer message object m is
  // a generated type, then the given factory MUST have this property:  If
  // GetPrototype() is given a Descriptor which resides in
  // DescriptorPool::generated_pool(), the factory MUST return the same
  // prototype which MessageFactory::generated_factory() would return.  That
  // is, given a descriptor for a generated type, the factory must return an
  // instance of the generated class (NOT DynamicMessage).  However, when
  // given a descriptor for a type that is NOT in generated_pool, the factory
  // is free to return any implementation.
  //
  // The reason for this requirement is that generated sub-objects may be
  // accessed via the standard (non-reflection) extension accessor methods,
  // and these methods will down-cast the object to the generated class type.
  // If the object is not actually of that type, the results would be undefined.
  // On the other hand, if an extension is not compiled in, then there is no
  // way the code could end up accessing it via the standard accessors -- the
  // only way to access the extension is via reflection.  When using reflection,
  // DynamicMessage and generated messages are indistinguishable, so it's fine
  // if these objects are represented using DynamicMessage.
  //
  // Using DynamicMessageFactory on which you have called
  // SetDelegateToGeneratedFactory(true) should be sufficient to satisfy the
  // above requirement.
  //
  // If either pool or factory is NULL, both must be NULL.
  //
  // Note that this feature is ignored when parsing "lite" messages as they do
  // not have descriptors.
  void SetExtensionRegistry(const DescriptorPool* pool,
                            MessageFactory* factory);

  // Get the DescriptorPool set via SetExtensionRegistry(), or NULL if no pool
  // has been provided.
  const DescriptorPool* GetExtensionPool();

  // Get the MessageFactory set via SetExtensionRegistry(), or NULL if no
  // factory has been provided.
  MessageFactory* GetExtensionFactory();

 private:
  const uint8_t* buffer_;
  const uint8_t* buffer_end_;  // pointer to the end of the buffer.
  ZeroCopyInputStream* input_;
  int total_bytes_read_;  // total bytes read from input_, including
                          // the current buffer

  // If total_bytes_read_ surpasses INT_MAX, we record the extra bytes here
  // so that we can BackUp() on destruction.
  int overflow_bytes_;

  // LastTagWas() stuff.
  uint32_t last_tag_;  // result of last ReadTag() or ReadTagWithCutoff().

  // This is set true by ReadTag{Fallback/Slow}() if it is called when exactly
  // at EOF, or by ExpectAtEnd() when it returns true.  This happens when we
  // reach the end of a message and attempt to read another tag.
  bool legitimate_message_end_;

  // See EnableAliasing().
  bool aliasing_enabled_;

  // If true, set eager parsing mode to override lazy fields.
  bool force_eager_parsing_;

  // Limits
  Limit current_limit_;  // if position = -1, no limit is applied

  // For simplicity, if the current buffer crosses a limit (either a normal
  // limit created by PushLimit() or the total bytes limit), buffer_size_
  // only tracks the number of bytes before that limit.  This field
  // contains the number of bytes after it.  Note that this implies that if
  // buffer_size_ == 0 and buffer_size_after_limit_ > 0, we know we've
  // hit a limit.  However, if both are zero, it doesn't necessarily mean
  // we aren't at a limit -- the buffer may have ended exactly at the limit.
  int buffer_size_after_limit_;

  // Maximum number of bytes to read, period.  This is unrelated to
  // current_limit_.  Set using SetTotalBytesLimit().
  int total_bytes_limit_;

  // Current recursion budget, controlled by IncrementRecursionDepth() and
  // similar.  Starts at recursion_limit_ and goes down: if this reaches
  // -1 we are over budget.
  int recursion_budget_;
  // Recursion depth limit, set by SetRecursionLimit().
  int recursion_limit_;

  // See SetExtensionRegistry().
  const DescriptorPool* extension_pool_;
  MessageFactory* extension_factory_;

  // Private member functions.

  // Fallback when Skip() goes past the end of the current buffer.
  bool SkipFallback(int count, int original_buffer_size);

  // Advance the buffer by a given number of bytes.
  void Advance(int amount);

  // Back up input_ to the current buffer position.
  void BackUpInputToCurrentPosition();

  // Recomputes the value of buffer_size_after_limit_.  Must be called after
  // current_limit_ or total_bytes_limit_ changes.
  void RecomputeBufferLimits();

  // Writes an error message saying that we hit total_bytes_limit_.
  void PrintTotalBytesLimitError();

  // Called when the buffer runs out to request more data.  Implies an
  // Advance(BufferSize()).
  bool Refresh();

  // When parsing varints, we optimize for the common case of small values, and
  // then optimize for the case when the varint fits within the current buffer
  // piece. The Fallback method is used when we can't use the one-byte
  // optimization. The Slow method is yet another fallback when the buffer is
  // not large enough. Making the slow path out-of-line speeds up the common
  // case by 10-15%. The slow path is fairly uncommon: it only triggers when a
  // message crosses multiple buffers.  Note: ReadVarint32Fallback() and
  // ReadVarint64Fallback() are called frequently and generally not inlined, so
  // they have been optimized to avoid "out" parameters.  The former returns -1
  // if it fails and the uint32_t it read otherwise.  The latter has a bool
  // indicating success or failure as part of its return type.
  int64_t ReadVarint32Fallback(uint32_t first_byte_or_zero);
  int ReadVarintSizeAsIntFallback();
  std::pair<uint64_t, bool> ReadVarint64Fallback();
  bool ReadVarint32Slow(uint32_t* value);
  bool ReadVarint64Slow(uint64_t* value);
  int ReadVarintSizeAsIntSlow();
  bool ReadLittleEndian16Fallback(uint16_t* value);
  bool ReadLittleEndian32Fallback(uint32_t* value);
  bool ReadLittleEndian64Fallback(uint64_t* value);

  // Fallback/slow methods for reading tags. These do not update last_tag_,
  // but will set legitimate_message_end_ if we are at the end of the input
  // stream.
  uint32_t ReadTagFallback(uint32_t first_byte_or_zero);
  uint32_t ReadTagSlow();
  bool ReadStringFallback(std::string* buffer, int size);

  // Return the size of the buffer.
  int BufferSize() const;

  static const int kDefaultTotalBytesLimit = INT_MAX;

  static int default_recursion_limit_;  // 100 by default.

  friend class google::protobuf::ZeroCopyCodedInputStream;
  friend class google::protobuf::internal::EpsCopyByteStream;
};

// EpsCopyOutputStream wraps a ZeroCopyOutputStream and exposes a new stream,
// which has the property you can write kSlopBytes (16 bytes) from the current
// position without bounds checks. The cursor into the stream is managed by
// the user of the class and is an explicit parameter in the methods. Careful
// use of this class, ie. keep ptr a local variable, eliminates the need to
// for the compiler to sync the ptr value between register and memory.
class PROTOBUF_EXPORT EpsCopyOutputStream {
 public:
  enum { kSlopBytes = 16 };

  // Initialize from a stream.
  EpsCopyOutputStream(ZeroCopyOutputStream* stream, bool deterministic,
                      uint8_t** pp)
      : end_(buffer_),
        stream_(stream),
        is_serialization_deterministic_(deterministic) {
    *pp = buffer_;
  }

  // Only for array serialization. No overflow protection, end_ will be the
  // pointed to the end of the array. When using this the total size is already
  // known, so no need to maintain the slop region.
  EpsCopyOutputStream(void* data, int size, bool deterministic)
      : end_(static_cast<uint8_t*>(data) + size),
        buffer_end_(nullptr),
        stream_(nullptr),
        is_serialization_deterministic_(deterministic) {}

  // Initialize from stream but with the first buffer already given (eager).
  EpsCopyOutputStream(void* data, int size, ZeroCopyOutputStream* stream,
                      bool deterministic, uint8_t** pp)
      : stream_(stream), is_serialization_deterministic_(deterministic) {
    *pp = SetInitialBuffer(data, size);
  }

  // Flush everything that's written into the underlying ZeroCopyOutputStream
  // and trims the underlying stream to the location of ptr.
  uint8_t* Trim(uint8_t* ptr);

  // After this it's guaranteed you can safely write kSlopBytes to ptr. This
  // will never fail! The underlying stream can produce an error. Use HadError
  // to check for errors.
  [[nodiscard]] uint8_t* EnsureSpace(uint8_t* ptr) {
    if (ABSL_PREDICT_FALSE(ptr >= end_)) {
      return EnsureSpaceFallback(ptr);
    }
    return ptr;
  }

  uint8_t* WriteRaw(const void* data, int size, uint8_t* ptr) {
    if (ABSL_PREDICT_FALSE(end_ - ptr < size)) {
      return WriteRawFallback(data, size, ptr);
    }
    std::memcpy(ptr, data, static_cast<unsigned int>(size));
    return ptr + size;
  }
  // Writes the buffer specified by data, size to the stream. Possibly by
  // aliasing the buffer (ie. not copying the data). The caller is responsible
  // to make sure the buffer is alive for the duration of the
  // ZeroCopyOutputStream.
#ifndef NDEBUG
  PROTOBUF_NOINLINE
#endif
  uint8_t* WriteRawMaybeAliased(const void* data, int size, uint8_t* ptr) {
    if (aliasing_enabled_) {
      return WriteAliasedRaw(data, size, ptr);
    } else {
      return WriteRaw(data, size, ptr);
    }
  }

  uint8_t* WriteCord(const absl::Cord& cord, uint8_t* ptr);

#ifndef NDEBUG
  PROTOBUF_NOINLINE
#endif
  uint8_t* WriteStringMaybeAliased(uint32_t num, absl::string_view s,
                                   uint8_t* ptr) {
    std::ptrdiff_t size = s.size();
    if (ABSL_PREDICT_FALSE(size >= 128 ||
                           end_ - ptr + 16 - TagSize(num << 3) - 1 < size)) {
      return WriteStringMaybeAliasedOutline(num, s, ptr);
    }
    ptr = UnsafeVarint((num << 3) | 2, ptr);
    *ptr++ = static_cast<uint8_t>(size);
    std::memcpy(ptr, s.data(), size);
    return ptr + size;
  }
  uint8_t* WriteBytesMaybeAliased(uint32_t num, absl::string_view s,
                                  uint8_t* ptr) {
    return WriteStringMaybeAliased(num, s, ptr);
  }

  template <typename T>
  PROTOBUF_ALWAYS_INLINE uint8_t* WriteString(uint32_t num, const T& s,
                                              uint8_t* ptr) {
    std::ptrdiff_t size = s.size();
    if (ABSL_PREDICT_FALSE(size >= 128 ||
                           end_ - ptr + 16 - TagSize(num << 3) - 1 < size)) {
      return WriteStringOutline(num, s, ptr);
    }
    ptr = UnsafeVarint((num << 3) | 2, ptr);
    *ptr++ = static_cast<uint8_t>(size);
    std::memcpy(ptr, s.data(), size);
    return ptr + size;
  }

  uint8_t* WriteString(uint32_t num, const absl::Cord& s, uint8_t* ptr) {
    ptr = EnsureSpace(ptr);
    ptr = WriteTag(num, 2, ptr);
    return WriteCordOutline(s, ptr);
  }

  template <typename T>
#ifndef NDEBUG
  PROTOBUF_NOINLINE
#endif
  uint8_t* WriteBytes(uint32_t num, const T& s, uint8_t* ptr) {
    return WriteString(num, s, ptr);
  }

  template <typename T>
  PROTOBUF_ALWAYS_INLINE uint8_t* WriteInt32Packed(int num, const T& r,
                                                   int size, uint8_t* ptr) {
    return WriteVarintPacked(num, r, size, ptr, Encode64);
  }
  template <typename T>
  PROTOBUF_ALWAYS_INLINE uint8_t* WriteUInt32Packed(int num, const T& r,
                                                    int size, uint8_t* ptr) {
    return WriteVarintPacked(num, r, size, ptr, Encode32);
  }
  template <typename T>
  PROTOBUF_ALWAYS_INLINE uint8_t* WriteSInt32Packed(int num, const T& r,
                                                    int size, uint8_t* ptr) {
    return WriteVarintPacked(num, r, size, ptr, ZigZagEncode32);
  }
  template <typename T>
  PROTOBUF_ALWAYS_INLINE uint8_t* WriteInt64Packed(int num, const T& r,
                                                   int size, uint8_t* ptr) {
    return WriteVarintPacked(num, r, size, ptr, Encode64);
  }
  template <typename T>
  PROTOBUF_ALWAYS_INLINE uint8_t* WriteUInt64Packed(int num, const T& r,
                                                    int size, uint8_t* ptr) {
    return WriteVarintPacked(num, r, size, ptr, Encode64);
  }
  template <typename T>
  PROTOBUF_ALWAYS_INLINE uint8_t* WriteSInt64Packed(int num, const T& r,
                                                    int size, uint8_t* ptr) {
    return WriteVarintPacked(num, r, size, ptr, ZigZagEncode64);
  }
  template <typename T>
  PROTOBUF_ALWAYS_INLINE uint8_t* WriteEnumPacked(int num, const T& r, int size,
                                                  uint8_t* ptr) {
    return WriteVarintPacked(num, r, size, ptr, Encode64);
  }

  template <typename T>
  PROTOBUF_ALWAYS_INLINE uint8_t* WriteFixedPacked(int num, const T& r,
                                                   uint8_t* ptr) {
    ptr = EnsureSpace(ptr);
    constexpr auto element_size = sizeof(typename T::value_type);
    auto size = r.size() * element_size;
    ptr = WriteLengthDelim(num, size, ptr);
    return WriteRawLittleEndian<element_size>(r.data(), static_cast<int>(size),
                                              ptr);
  }

  // Returns true if there was an underlying I/O error since this object was
  // created.
  bool HadError() const { return had_error_; }

  // Instructs the EpsCopyOutputStream to allow the underlying
  // ZeroCopyOutputStream to hold pointers to the original structure instead of
  // copying, if it supports it (i.e. output->AllowsAliasing() is true).  If the
  // underlying stream does not support aliasing, then enabling it has no
  // affect.  For now, this only affects the behavior of
  // WriteRawMaybeAliased().
  //
  // NOTE: It is caller's responsibility to ensure that the chunk of memory
  // remains live until all of the data has been consumed from the stream.
  void EnableAliasing(bool enabled);

  // See documentation on CodedOutputStream::SetSerializationDeterministic.
  void SetSerializationDeterministic(bool value) {
    is_serialization_deterministic_ = value;
  }

  // See documentation on CodedOutputStream::IsSerializationDeterministic.
  bool IsSerializationDeterministic() const {
    return is_serialization_deterministic_;
  }

  // The number of bytes written to the stream at position ptr, relative to the
  // stream's overall position.
  int64_t ByteCount(uint8_t* ptr) const;


 private:
  uint8_t* end_;
  uint8_t* buffer_end_ = buffer_;
  uint8_t buffer_[2 * kSlopBytes];
  ZeroCopyOutputStream* stream_;
  bool had_error_ = false;
  bool aliasing_enabled_ = false;  // See EnableAliasing().
  bool is_serialization_deterministic_;
  bool skip_check_consistency = false;

  uint8_t* EnsureSpaceFallback(uint8_t* ptr);
  inline uint8_t* Next();
  int Flush(uint8_t* ptr);
  std::ptrdiff_t GetSize(uint8_t* ptr) const {
    ABSL_DCHECK(ptr <= end_ + kSlopBytes);  // NOLINT
    return end_ + kSlopBytes - ptr;
  }

  uint8_t* Error() {
    had_error_ = true;
    // We use the patch buffer to always guarantee space to write to.
    end_ = buffer_ + kSlopBytes;
    return buffer_;
  }

  static constexpr int TagSize(uint32_t tag) {
    return (tag < (1 << 7))    ? 1
           : (tag < (1 << 14)) ? 2
           : (tag < (1 << 21)) ? 3
           : (tag < (1 << 28)) ? 4
                               : 5;
  }

  PROTOBUF_ALWAYS_INLINE uint8_t* WriteTag(uint32_t num, uint32_t wt,
                                           uint8_t* ptr) {
    ABSL_DCHECK(ptr < end_);  // NOLINT
    return UnsafeVarint((num << 3) | wt, ptr);
  }

  PROTOBUF_ALWAYS_INLINE uint8_t* WriteLengthDelim(int num, uint32_t size,
                                                   uint8_t* ptr) {
    ptr = WriteTag(num, 2, ptr);
    return UnsafeWriteSize(size, ptr);
  }

  uint8_t* WriteRawFallback(const void* data, int size, uint8_t* ptr);

  uint8_t* WriteAliasedRaw(const void* data, int size, uint8_t* ptr);

  uint8_t* WriteStringMaybeAliasedOutline(uint32_t num, absl::string_view s,
                                          uint8_t* ptr);
  uint8_t* WriteStringOutline(uint32_t num, absl::string_view s, uint8_t* ptr);
  uint8_t* WriteCordOutline(const absl::Cord& c, uint8_t* ptr);

  template <typename T, typename E>
  PROTOBUF_ALWAYS_INLINE uint8_t* WriteVarintPacked(int num, const T& r,
                                                    int size, uint8_t* ptr,
                                                    const E& encode) {
    ptr = EnsureSpace(ptr);
    ptr = WriteLengthDelim(num, size, ptr);
    auto it = r.data();
    auto end = it + r.size();
    do {
      ptr = EnsureSpace(ptr);
      ptr = UnsafeVarint(encode(*it++), ptr);
    } while (it < end);
    return ptr;
  }

  static uint32_t Encode32(uint32_t v) { return v; }
  static uint64_t Encode64(uint64_t v) { return v; }
  static uint32_t ZigZagEncode32(int32_t v) {
    return (static_cast<uint32_t>(v) << 1) ^ static_cast<uint32_t>(v >> 31);
  }
  static uint64_t ZigZagEncode64(int64_t v) {
    return (static_cast<uint64_t>(v) << 1) ^ static_cast<uint64_t>(v >> 63);
  }

  template <typename T>
  PROTOBUF_ALWAYS_INLINE static uint8_t* UnsafeVarint(T value, uint8_t* ptr) {
    static_assert(std::is_unsigned<T>::value,
                  "Varint serialization must be unsigned");
    while (ABSL_PREDICT_FALSE(value >= 0x80)) {
      *ptr = static_cast<uint8_t>(value | 0x80);
      value >>= 7;
      ++ptr;
    }
    *ptr++ = static_cast<uint8_t>(value);
    return ptr;
  }

  PROTOBUF_ALWAYS_INLINE static uint8_t* UnsafeWriteSize(uint32_t value,
                                                         uint8_t* ptr) {
    while (ABSL_PREDICT_FALSE(value >= 0x80)) {
      *ptr = static_cast<uint8_t>(value | 0x80);
      value >>= 7;
      ++ptr;
    }
    *ptr++ = static_cast<uint8_t>(value);
    return ptr;
  }

  template <int S>
  uint8_t* WriteRawLittleEndian(const void* data, int size, uint8_t* ptr);
#if !defined(ABSL_IS_LITTLE_ENDIAN) || \
    defined(PROTOBUF_DISABLE_LITTLE_ENDIAN_OPT_FOR_TEST)
  uint8_t* WriteRawLittleEndian32(const void* data, int size, uint8_t* ptr);
  uint8_t* WriteRawLittleEndian64(const void* data, int size, uint8_t* ptr);
#endif

  // These methods are for CodedOutputStream. Ideally they should be private
  // but to match current behavior of CodedOutputStream as close as possible
  // we allow it some functionality.
 public:
  uint8_t* SetInitialBuffer(void* data, int size) {
    auto ptr = static_cast<uint8_t*>(data);
    if (size > kSlopBytes) {
      end_ = ptr + size - kSlopBytes;
      buffer_end_ = nullptr;
      return ptr;
    } else {
      end_ = buffer_ + size;
      buffer_end_ = ptr;
      return buffer_;
    }
  }

 private:
  // Needed by CodedOutputStream HadError. HadError needs to flush the patch
  // buffers to ensure there is no error as of yet.
  uint8_t* FlushAndResetBuffer(uint8_t*);

  // The following functions mimic the old CodedOutputStream behavior as close
  // as possible. They flush the current state to the stream, behave as
  // the old CodedOutputStream and then return to normal operation.
  bool Skip(int count, uint8_t** pp);
  bool GetDirectBufferPointer(void** data, int* size, uint8_t** pp);
  uint8_t* GetDirectBufferForNBytesAndAdvance(int size, uint8_t** pp);

  friend class CodedOutputStream;
};

template <>
inline uint8_t* EpsCopyOutputStream::WriteRawLittleEndian<1>(const void* data,
                                                             int size,
                                                             uint8_t* ptr) {
  return WriteRaw(data, size, ptr);
}
template <>
inline uint8_t* EpsCopyOutputStream::WriteRawLittleEndian<4>(const void* data,
                                                             int size,
                                                             uint8_t* ptr) {
#if defined(ABSL_IS_LITTLE_ENDIAN) && \
    !defined(PROTOBUF_DISABLE_LITTLE_ENDIAN_OPT_FOR_TEST)
  return WriteRaw(data, size, ptr);
#else
  return WriteRawLittleEndian32(data, size, ptr);
#endif
}
template <>
inline uint8_t* EpsCopyOutputStream::WriteRawLittleEndian<8>(const void* data,
                                                             int size,
                                                             uint8_t* ptr) {
#if defined(ABSL_IS_LITTLE_ENDIAN) && \
    !defined(PROTOBUF_DISABLE_LITTLE_ENDIAN_OPT_FOR_TEST)
  return WriteRaw(data, size, ptr);
#else
  return WriteRawLittleEndian64(data, size, ptr);
#endif
}

// Class which encodes and writes binary data which is composed of varint-
// encoded integers and fixed-width pieces.  Wraps a ZeroCopyOutputStream.
// Most users will not need to deal with CodedOutputStream.
//
// Most methods of CodedOutputStream which return a bool return false if an
// underlying I/O error occurs.  Once such a failure occurs, the
// CodedOutputStream is broken and is no longer useful. The Write* methods do
// not return the stream status, but will invalidate the stream if an error
// occurs. The client can probe HadError() to determine the status.
//
// Note that every method of CodedOutputStream which writes some data has
// a corresponding static "ToArray" version. These versions write directly
// to the provided buffer, returning a pointer past the last written byte.
// They require that the buffer has sufficient capacity for the encoded data.
// This allows an optimization where we check if an output stream has enough
// space for an entire message before we start writing and, if there is, we
// call only the ToArray methods to avoid doing bound checks for each
// individual value.
// i.e., in the example above:
//
//   CodedOutputStream* coded_output = new CodedOutputStream(raw_output);
//   int magic_number = 1234;
//   char text[] = "Hello world!";
//
//   int coded_size = sizeof(magic_number) +
//                    CodedOutputStream::VarintSize32(strlen(text)) +
//                    strlen(text);
//
//   uint8_t* buffer =
//       coded_output->GetDirectBufferForNBytesAndAdvance(coded_size);
//   if (buffer != nullptr) {
//     // The output stream has enough space in the buffer: write directly to
//     // the array.
//     buffer = CodedOutputStream::WriteLittleEndian32ToArray(magic_number,
//                                                            buffer);
//     buffer = CodedOutputStream::WriteVarint32ToArray(strlen(text), buffer);
//     buffer = CodedOutputStream::WriteRawToArray(text, strlen(text), buffer);
//   } else {
//     // Make bound-checked writes, which will ask the underlying stream for
//     // more space as needed.
//     coded_output->WriteLittleEndian32(magic_number);
//     coded_output->WriteVarint32(strlen(text));
//     coded_output->WriteRaw(text, strlen(text));
//   }
//
//   delete coded_output;
class PROTOBUF_EXPORT CodedOutputStream {
 public:
  // Creates a CodedOutputStream that writes to the given `stream`.
  // The provided stream must publicly derive from `ZeroCopyOutputStream`.
  template <class Stream, class = typename std::enable_if<std::is_base_of<
                              ZeroCopyOutputStream, Stream>::value>::type>
  explicit CodedOutputStream(Stream* stream);

  // Creates a CodedOutputStream that writes to the given `stream`, and does
  // an 'eager initialization' of the internal state if `eager_init` is true.
  // The provided stream must publicly derive from `ZeroCopyOutputStream`.
  template <class Stream, class = typename std::enable_if<std::is_base_of<
                              ZeroCopyOutputStream, Stream>::value>::type>
  CodedOutputStream(Stream* stream, bool eager_init);
  CodedOutputStream(const CodedOutputStream&) = delete;
  CodedOutputStream& operator=(const CodedOutputStream&) = delete;

  // Destroy the CodedOutputStream and position the underlying
  // ZeroCopyOutputStream immediately after the last byte written.
  ~CodedOutputStream();

  // Returns true if there was an underlying I/O error since this object was
  // created. On should call Trim before this function in order to catch all
  // errors.
  bool HadError() {
    cur_ = impl_.FlushAndResetBuffer(cur_);
    ABSL_DCHECK(cur_);
    return impl_.HadError();
  }

  // Trims any unused space in the underlying buffer so that its size matches
  // the number of bytes written by this stream. The underlying buffer will
  // automatically be trimmed when this stream is destroyed; this call is only
  // necessary if the underlying buffer is accessed *before* the stream is
  // destroyed.
  void Trim() { cur_ = impl_.Trim(cur_); }

  // Skips a number of bytes, leaving the bytes unmodified in the underlying
  // buffer.  Returns false if an underlying write error occurs.  This is
  // mainly useful with GetDirectBufferPointer().
  // Note of caution, the skipped bytes may contain uninitialized data. The
  // caller must make sure that the skipped bytes are properly initialized,
  // otherwise you might leak bytes from your heap.
  bool Skip(int count) { return impl_.Skip(count, &cur_); }

  // Sets *data to point directly at the unwritten part of the
  // CodedOutputStream's underlying buffer, and *size to the size of that
  // buffer, but does not advance the stream's current position.  This will
  // always either produce a non-empty buffer or return false.  If the caller
  // writes any data to this buffer, it should then call Skip() to skip over
  // the consumed bytes.  This may be useful for implementing external fast
  // serialization routines for types of data not covered by the
  // CodedOutputStream interface.
  bool GetDirectBufferPointer(void** data, int* size) {
    return impl_.GetDirectBufferPointer(data, size, &cur_);
  }

  // If there are at least "size" bytes available in the current buffer,
  // returns a pointer directly into the buffer and advances over these bytes.
  // The caller may then write directly into this buffer (e.g. using the
  // *ToArray static methods) rather than go through CodedOutputStream.  If
  // there are not enough bytes available, returns NULL.  The return pointer is
  // invalidated as soon as any other non-const method of CodedOutputStream
  // is called.
  inline uint8_t* GetDirectBufferForNBytesAndAdvance(int size) {
    return impl_.GetDirectBufferForNBytesAndAdvance(size, &cur_);
  }

  // Write raw bytes, copying them from the given buffer.
  void WriteRaw(const void* buffer, int size) {
    cur_ = impl_.WriteRaw(buffer, size, cur_);
  }
  // Like WriteRaw()  but will try to write aliased data if aliasing is
  // turned on.
  void WriteRawMaybeAliased(const void* data, int size);
  // Like WriteRaw()  but writing directly to the target array.
  // This is _not_ inlined, as the compiler often optimizes memcpy into inline
  // copy loops. Since this gets called by every field with string or bytes
  // type, inlining may lead to a significant amount of code bloat, with only a
  // minor performance gain.
  static uint8_t* WriteRawToArray(const void* buffer, int size,
                                  uint8_t* target);

  // Equivalent to WriteRaw(str.data(), str.size()).
  void WriteString(absl::string_view str);
  // Like WriteString()  but writing directly to the target array.
  static uint8_t* WriteStringToArray(absl::string_view str, uint8_t* target);
  // Write the varint-encoded size of str followed by str.
  static uint8_t* WriteStringWithSizeToArray(absl::string_view str,
                                             uint8_t* target);

  // Like WriteString() but writes a Cord.
  void WriteCord(const absl::Cord& cord) { cur_ = impl_.WriteCord(cord, cur_); }

  // Like WriteCord() but writing directly to the target array.
  static uint8_t* WriteCordToArray(const absl::Cord& cord, uint8_t* target);


  // Write a 16-bit little-endian integer.
  void WriteLittleEndian16(uint16_t value) {
    cur_ = impl_.EnsureSpace(cur_);
    SetCur(WriteLittleEndian16ToArray(value, Cur()));
  }
  // Like WriteLittleEndian16() but writing directly to the target array.
  static uint8_t* WriteLittleEndian16ToArray(uint16_t value, uint8_t* target);
  // Write a 32-bit little-endian integer.
  void WriteLittleEndian32(uint32_t value) {
    cur_ = impl_.EnsureSpace(cur_);
    SetCur(WriteLittleEndian32ToArray(value, Cur()));
  }
  // Like WriteLittleEndian32() but writing directly to the target array.
  static uint8_t* WriteLittleEndian32ToArray(uint32_t value, uint8_t* target);
  // Write a 64-bit little-endian integer.
  void WriteLittleEndian64(uint64_t value) {
    cur_ = impl_.EnsureSpace(cur_);
    SetCur(WriteLittleEndian64ToArray(value, Cur()));
  }
  // Like WriteLittleEndian64() but writing directly to the target array.
  static uint8_t* WriteLittleEndian64ToArray(uint64_t value, uint8_t* target);

  // Write an unsigned integer with Varint encoding.  Writing a 32-bit value
  // is equivalent to casting it to uint64_t and writing it as a 64-bit value,
  // but may be more efficient.
  void WriteVarint32(uint32_t value);
  // Like WriteVarint32()  but writing directly to the target array.
  static uint8_t* WriteVarint32ToArray(uint32_t value, uint8_t* target);
  // Like WriteVarint32ToArray()
  [[deprecated("Please use WriteVarint32ToArray() instead")]] static uint8_t*
  WriteVarint32ToArrayOutOfLine(uint32_t value, uint8_t* target) {
    return WriteVarint32ToArray(value, target);
  }
  // Write an unsigned integer with Varint encoding.
  void WriteVarint64(uint64_t value);
  // Like WriteVarint64()  but writing directly to the target array.
  static uint8_t* WriteVarint64ToArray(uint64_t value, uint8_t* target);

  // Equivalent to WriteVarint32() except when the value is negative,
  // in which case it must be sign-extended to a full 10 bytes.
  void WriteVarint32SignExtended(int32_t value);
  // Like WriteVarint32SignExtended()  but writing directly to the target array.
  static uint8_t* WriteVarint32SignExtendedToArray(int32_t value,
                                                   uint8_t* target);

  // This is identical to WriteVarint32(), but optimized for writing tags.
  // In particular, if the input is a compile-time constant, this method
  // compiles down to a couple instructions.
  // Always inline because otherwise the aforementioned optimization can't work,
  // but GCC by default doesn't want to inline this.
  void WriteTag(uint32_t value);
  // Like WriteTag()  but writing directly to the target array.
  PROTOBUF_ALWAYS_INLINE
  static uint8_t* WriteTagToArray(uint32_t value, uint8_t* target);

  // Returns the number of bytes needed to encode the given value as a varint.
  static size_t VarintSize32(uint32_t value);
  // Returns the number of bytes needed to encode the given value as a varint.
  static size_t VarintSize64(uint64_t value);

  // If negative, 10 bytes.  Otherwise, same as VarintSize32().
  static size_t VarintSize32SignExtended(int32_t value);

  // Same as above, plus one.  The additional one comes at no compute cost.
  static size_t VarintSize32PlusOne(uint32_t value);
  static size_t VarintSize64PlusOne(uint64_t value);
  static size_t VarintSize32SignExtendedPlusOne(int32_t value);

  // Compile-time equivalent of VarintSize32().
  template <uint32_t Value>
  struct StaticVarintSize32 {
    static const size_t value = (Value < (1 << 7))    ? 1
                                : (Value < (1 << 14)) ? 2
                                : (Value < (1 << 21)) ? 3
                                : (Value < (1 << 28)) ? 4
                                                      : 5;
  };

  // Returns the total number of bytes written since this object was created.
  int ByteCount() const {
    return static_cast<int>(impl_.ByteCount(cur_) - start_count_);
  }

  // Instructs the CodedOutputStream to allow the underlying
  // ZeroCopyOutputStream to hold pointers to the original structure instead of
  // copying, if it supports it (i.e. output->AllowsAliasing() is true).  If the
  // underlying stream does not support aliasing, then enabling it has no
  // affect.  For now, this only affects the behavior of
  // WriteRawMaybeAliased().
  //
  // NOTE: It is caller's responsibility to ensure that the chunk of memory
  // remains live until all of the data has been consumed from the stream.
  void EnableAliasing(bool enabled) { impl_.EnableAliasing(enabled); }

  // Indicate to the serializer whether the user wants deterministic
  // serialization. The default when this is not called comes from the global
  // default, controlled by SetDefaultSerializationDeterministic.
  //
  // What deterministic serialization means is entirely up to the driver of the
  // serialization process (i.e. the caller of methods like WriteVarint32). In
  // the case of serializing a proto buffer message using one of the methods of
  // MessageLite, this means that for a given binary equal messages will always
  // be serialized to the same bytes. This implies:
  //
  //   * Repeated serialization of a message will return the same bytes.
  //
  //   * Different processes running the same binary (including on different
  //     machines) will serialize equal messages to the same bytes.
  //
  // Note that this is *not* canonical across languages. It is also unstable
  // across different builds with intervening message definition changes, due to
  // unknown fields. Users who need canonical serialization (e.g. persistent
  // storage in a canonical form, fingerprinting) should define their own
  // canonicalization specification and implement the serializer using
  // reflection APIs rather than relying on this API.
  void SetSerializationDeterministic(bool value) {
    impl_.SetSerializationDeterministic(value);
  }

  // Return whether the user wants deterministic serialization. See above.
  bool IsSerializationDeterministic() const {
    return impl_.IsSerializationDeterministic();
  }

  static bool IsDefaultSerializationDeterministic() {
    return default_serialization_deterministic_.load(
               std::memory_order_relaxed) != 0;
  }

  template <typename Func>
  void Serialize(const Func& func);

  uint8_t* Cur() const { return cur_; }
  void SetCur(uint8_t* ptr) { cur_ = ptr; }
  EpsCopyOutputStream* EpsCopy() { return &impl_; }

 private:
  template <class Stream>
  void InitEagerly(Stream* stream);

  EpsCopyOutputStream impl_;
  uint8_t* cur_;
  int64_t start_count_;
  static std::atomic<bool> default_serialization_deterministic_;

  // See above.  Other projects may use "friend" to allow them to call this.
  // After SetDefaultSerializationDeterministic() completes, all protocol
  // buffer serializations will be deterministic by default.  Thread safe.
  // However, the meaning of "after" is subtle here: to be safe, each thread
  // that wants deterministic serialization by default needs to call
  // SetDefaultSerializationDeterministic() or ensure on its own that another
  // thread has done so.
  friend void google::protobuf::internal::MapTestForceDeterministic();
  static void SetDefaultSerializationDeterministic() {
    default_serialization_deterministic_.store(true, std::memory_order_relaxed);
  }
};

// inline methods ====================================================
// The vast majority of varints are only one byte.  These inline
// methods optimize for that case.

inline bool CodedInputStream::ReadVarint32(uint32_t* value) {
  uint32_t v = 0;
  if (ABSL_PREDICT_TRUE(buffer_ < buffer_end_)) {
    v = *buffer_;
    if (v < 0x80) {
      *value = v;
      Advance(1);
      return true;
    }
  }
  int64_t result = ReadVarint32Fallback(v);
  *value = static_cast<uint32_t>(result);
  return result >= 0;
}

inline bool CodedInputStream::ReadVarint64(uint64_t* value) {
  if (ABSL_PREDICT_TRUE(buffer_ < buffer_end_) && *buffer_ < 0x80) {
    *value = *buffer_;
    Advance(1);
    return true;
  }
  std::pair<uint64_t, bool> p = ReadVarint64Fallback();
  *value = p.first;
  return p.second;
}

inline bool CodedInputStream::ReadVarintSizeAsInt(int* value) {
  if (ABSL_PREDICT_TRUE(buffer_ < buffer_end_)) {
    int v = *buffer_;
    if (v < 0x80) {
      *value = v;
      Advance(1);
      return true;
    }
  }
  *value = ReadVarintSizeAsIntFallback();
  return *value >= 0;
}

// static
inline const uint8_t* CodedInputStream::ReadLittleEndian16FromArray(
    const uint8_t* buffer, uint16_t* value) {
  memcpy(value, buffer, sizeof(*value));
  *value = google::protobuf::internal::little_endian::ToHost(*value);
  return buffer + sizeof(*value);
}
// static
inline const uint8_t* CodedInputStream::ReadLittleEndian32FromArray(
    const uint8_t* buffer, uint32_t* value) {
  memcpy(value, buffer, sizeof(*value));
  *value = google::protobuf::internal::little_endian::ToHost(*value);
  return buffer + sizeof(*value);
}
// static
inline const uint8_t* CodedInputStream::ReadLittleEndian64FromArray(
    const uint8_t* buffer, uint64_t* value) {
  memcpy(value, buffer, sizeof(*value));
  *value = google::protobuf::internal::little_endian::ToHost(*value);
  return buffer + sizeof(*value);
}

inline bool CodedInputStream::ReadLittleEndian16(uint16_t* value) {
  if (ABSL_PREDICT_TRUE(BufferSize() >= static_cast<int>(sizeof(*value)))) {
    buffer_ = ReadLittleEndian16FromArray(buffer_, value);
    return true;
  } else {
    return ReadLittleEndian16Fallback(value);
  }
}

inline bool CodedInputStream::ReadLittleEndian32(uint32_t* value) {
#if defined(ABSL_IS_LITTLE_ENDIAN) && \
    !defined(PROTOBUF_DISABLE_LITTLE_ENDIAN_OPT_FOR_TEST)
  if (ABSL_PREDICT_TRUE(BufferSize() >= static_cast<int>(sizeof(*value)))) {
    buffer_ = ReadLittleEndian32FromArray(buffer_, value);
    return true;
  } else {
    return ReadLittleEndian32Fallback(value);
  }
#else
  return ReadLittleEndian32Fallback(value);
#endif
}

inline bool CodedInputStream::ReadLittleEndian64(uint64_t* value) {
#if defined(ABSL_IS_LITTLE_ENDIAN) && \
    !defined(PROTOBUF_DISABLE_LITTLE_ENDIAN_OPT_FOR_TEST)
  if (ABSL_PREDICT_TRUE(BufferSize() >= static_cast<int>(sizeof(*value)))) {
    buffer_ = ReadLittleEndian64FromArray(buffer_, value);
    return true;
  } else {
    return ReadLittleEndian64Fallback(value);
  }
#else
  return ReadLittleEndian64Fallback(value);
#endif
}

inline uint32_t CodedInputStream::ReadTagNoLastTag() {
  uint32_t v = 0;
  if (ABSL_PREDICT_TRUE(buffer_ < buffer_end_)) {
    v = *buffer_;
    if (v < 0x80) {
      Advance(1);
      return v;
    }
  }
  v = ReadTagFallback(v);
  return v;
}

inline std::pair<uint32_t, bool> CodedInputStream::ReadTagWithCutoffNoLastTag(
    uint32_t cutoff) {
  // In performance-sensitive code we can expect cutoff to be a compile-time
  // constant, and things like "cutoff >= kMax1ByteVarint" to be evaluated at
  // compile time.
  uint32_t first_byte_or_zero = 0;
  if (ABSL_PREDICT_TRUE(buffer_ < buffer_end_)) {
    // Hot case: buffer_ non_empty, buffer_[0] in [1, 128).
    // TODO: Is it worth rearranging this? E.g., if the number of fields
    // is large enough then is it better to check for the two-byte case first?
    first_byte_or_zero = buffer_[0];
    if (static_cast<int8_t>(buffer_[0]) > 0) {
      const uint32_t kMax1ByteVarint = 0x7f;
      uint32_t tag = buffer_[0];
      Advance(1);
      return std::make_pair(tag, cutoff >= kMax1ByteVarint || tag <= cutoff);
    }
    // Other hot case: cutoff >= 0x80, buffer_ has at least two bytes available,
    // and tag is two bytes.  The latter is tested by bitwise-and-not of the
    // first byte and the second byte.
    if (cutoff >= 0x80 && ABSL_PREDICT_TRUE(buffer_ + 1 < buffer_end_) &&
        ABSL_PREDICT_TRUE((buffer_[0] & ~buffer_[1]) >= 0x80)) {
      const uint32_t kMax2ByteVarint = (0x7f << 7) + 0x7f;
      uint32_t tag = (1u << 7) * buffer_[1] + (buffer_[0] - 0x80);
      Advance(2);
      // It might make sense to test for tag == 0 now, but it is so rare that
      // that we don't bother.  A varint-encoded 0 should be one byte unless
      // the encoder lost its mind.  The second part of the return value of
      // this function is allowed to be either true or false if the tag is 0,
      // so we don't have to check for tag == 0.  We may need to check whether
      // it exceeds cutoff.
      bool at_or_below_cutoff = cutoff >= kMax2ByteVarint || tag <= cutoff;
      return std::make_pair(tag, at_or_below_cutoff);
    }
  }
  // Slow path
  const uint32_t tag = ReadTagFallback(first_byte_or_zero);
  return std::make_pair(tag, static_cast<uint32_t>(tag - 1) < cutoff);
}

inline bool CodedInputStream::LastTagWas(uint32_t expected) {
  return last_tag_ == expected;
}

inline bool CodedInputStream::ConsumedEntireMessage() {
  return legitimate_message_end_;
}

inline bool CodedInputStream::ExpectTag(uint32_t expected) {
  if (expected < (1 << 7)) {
    if (ABSL_PREDICT_TRUE(buffer_ < buffer_end_) && buffer_[0] == expected) {
      Advance(1);
      return true;
    } else {
      return false;
    }
  } else if (expected < (1 << 14)) {
    if (ABSL_PREDICT_TRUE(BufferSize() >= 2) &&
        buffer_[0] == static_cast<uint8_t>(expected | 0x80) &&
        buffer_[1] == static_cast<uint8_t>(expected >> 7)) {
      Advance(2);
      return true;
    } else {
      return false;
    }
  } else {
    // Don't bother optimizing for larger values.
    return false;
  }
}

inline const uint8_t* CodedInputStream::ExpectTagFromArray(
    const uint8_t* buffer, uint32_t expected) {
  if (expected < (1 << 7)) {
    if (buffer[0] == expected) {
      return buffer + 1;
    }
  } else if (expected < (1 << 14)) {
    if (buffer[0] == static_cast<uint8_t>(expected | 0x80) &&
        buffer[1] == static_cast<uint8_t>(expected >> 7)) {
      return buffer + 2;
    }
  }
  return nullptr;
}

inline void CodedInputStream::GetDirectBufferPointerInline(const void** data,
                                                           int* size) {
  *data = buffer_;
  *size = static_cast<int>(buffer_end_ - buffer_);
}

inline bool CodedInputStream::ExpectAtEnd() {
  // If we are at a limit we know no more bytes can be read.  Otherwise, it's
  // hard to say without calling Refresh(), and we'd rather not do that.

  if (buffer_ == buffer_end_ && ((buffer_size_after_limit_ != 0) ||
                                 (total_bytes_read_ == current_limit_))) {
    last_tag_ = 0;                   // Pretend we called ReadTag()...
    legitimate_message_end_ = true;  // ... and it hit EOF.
    return true;
  } else {
    return false;
  }
}

inline int CodedInputStream::CurrentPosition() const {
  return total_bytes_read_ - (BufferSize() + buffer_size_after_limit_);
}

inline void CodedInputStream::Advance(int amount) { buffer_ += amount; }

inline void CodedInputStream::SetRecursionLimit(int limit) {
  recursion_budget_ += limit - recursion_limit_;
  recursion_limit_ = limit;
}

inline bool CodedInputStream::IncrementRecursionDepth() {
  --recursion_budget_;
  return recursion_budget_ >= 0;
}

inline void CodedInputStream::DecrementRecursionDepth() {
  if (recursion_budget_ < recursion_limit_) ++recursion_budget_;
}

inline void CodedInputStream::UnsafeDecrementRecursionDepth() {
  assert(recursion_budget_ < recursion_limit_);
  ++recursion_budget_;
}

inline void CodedInputStream::SetExtensionRegistry(const DescriptorPool* pool,
                                                   MessageFactory* factory) {
  extension_pool_ = pool;
  extension_factory_ = factory;
}

inline const DescriptorPool* CodedInputStream::GetExtensionPool() {
  return extension_pool_;
}

inline MessageFactory* CodedInputStream::GetExtensionFactory() {
  return extension_factory_;
}

inline int CodedInputStream::BufferSize() const {
  return static_cast<int>(buffer_end_ - buffer_);
}

inline CodedInputStream::CodedInputStream(ZeroCopyInputStream* input)
    : buffer_(nullptr),
      buffer_end_(nullptr),
      input_(input),
      total_bytes_read_(0),
      overflow_bytes_(0),
      last_tag_(0),
      legitimate_message_end_(false),
      aliasing_enabled_(false),
      force_eager_parsing_(false),
      current_limit_(std::numeric_limits<int32_t>::max()),
      buffer_size_after_limit_(0),
      total_bytes_limit_(kDefaultTotalBytesLimit),
      recursion_budget_(default_recursion_limit_),
      recursion_limit_(default_recursion_limit_),
      extension_pool_(nullptr),
      extension_factory_(nullptr) {
  // Eagerly Refresh() so buffer space is immediately available.
  Refresh();
}

inline CodedInputStream::CodedInputStream(const uint8_t* buffer, int size)
    : buffer_(buffer),
      buffer_end_(buffer + size),
      input_(nullptr),
      total_bytes_read_(size),
      overflow_bytes_(0),
      last_tag_(0),
      legitimate_message_end_(false),
      aliasing_enabled_(false),
      force_eager_parsing_(false),
      current_limit_(size),
      buffer_size_after_limit_(0),
      total_bytes_limit_(kDefaultTotalBytesLimit),
      recursion_budget_(default_recursion_limit_),
      recursion_limit_(default_recursion_limit_),
      extension_pool_(nullptr),
      extension_factory_(nullptr) {
  // Note that setting current_limit_ == size is important to prevent some
  // code paths from trying to access input_ and segfaulting.
}

inline bool CodedInputStream::IsFlat() const { return input_ == nullptr; }

inline bool CodedInputStream::Skip(int count) {
  if (count < 0) return false;  // security: count is often user-supplied

  const int original_buffer_size = BufferSize();

  if (count <= original_buffer_size) {
    // Just skipping within the current buffer.  Easy.
    Advance(count);
    return true;
  }

  return SkipFallback(count, original_buffer_size);
}

template <class Stream, class>
inline CodedOutputStream::CodedOutputStream(Stream* stream)
    : impl_(stream, IsDefaultSerializationDeterministic(), &cur_),
      start_count_(stream->ByteCount()) {
  InitEagerly(stream);
}

template <class Stream, class>
inline CodedOutputStream::CodedOutputStream(Stream* stream, bool eager_init)
    : impl_(stream, IsDefaultSerializationDeterministic(), &cur_),
      start_count_(stream->ByteCount()) {
  if (eager_init) {
    InitEagerly(stream);
  }
}

template <class Stream>
inline void CodedOutputStream::InitEagerly(Stream* stream) {
  void* data;
  int size;
  if (ABSL_PREDICT_TRUE(stream->Next(&data, &size) && size > 0)) {
    cur_ = impl_.SetInitialBuffer(data, size);
  }
}

inline uint8_t* CodedOutputStream::WriteVarint32ToArray(uint32_t value,
                                                        uint8_t* target) {
  return EpsCopyOutputStream::UnsafeVarint(value, target);
}

inline uint8_t* CodedOutputStream::WriteVarint64ToArray(uint64_t value,
                                                        uint8_t* target) {
  return EpsCopyOutputStream::UnsafeVarint(value, target);
}

inline void CodedOutputStream::WriteVarint32SignExtended(int32_t value) {
  WriteVarint64(static_cast<uint64_t>(value));
}

inline uint8_t* CodedOutputStream::WriteVarint32SignExtendedToArray(
    int32_t value, uint8_t* target) {
  return WriteVarint64ToArray(static_cast<uint64_t>(value), target);
}

inline uint8_t* CodedOutputStream::WriteLittleEndian16ToArray(uint16_t value,
                                                              uint8_t* target) {
  uint16_t little_endian_value = google::protobuf::internal::little_endian::ToHost(value);
  memcpy(target, &little_endian_value, sizeof(value));
  return target + sizeof(value);
}

inline uint8_t* CodedOutputStream::WriteLittleEndian32ToArray(uint32_t value,
                                                              uint8_t* target) {
#if defined(ABSL_IS_LITTLE_ENDIAN) && \
    !defined(PROTOBUF_DISABLE_LITTLE_ENDIAN_OPT_FOR_TEST)
  memcpy(target, &value, sizeof(value));
#else
  target[0] = static_cast<uint8_t>(value);
  target[1] = static_cast<uint8_t>(value >> 8);
  target[2] = static_cast<uint8_t>(value >> 16);
  target[3] = static_cast<uint8_t>(value >> 24);
#endif
  return target + sizeof(value);
}

inline uint8_t* CodedOutputStream::WriteLittleEndian64ToArray(uint64_t value,
                                                              uint8_t* target) {
#if defined(ABSL_IS_LITTLE_ENDIAN) && \
    !defined(PROTOBUF_DISABLE_LITTLE_ENDIAN_OPT_FOR_TEST)
  memcpy(target, &value, sizeof(value));
#else
  uint32_t part0 = static_cast<uint32_t>(value);
  uint32_t part1 = static_cast<uint32_t>(value >> 32);

  target[0] = static_cast<uint8_t>(part0);
  target[1] = static_cast<uint8_t>(part0 >> 8);
  target[2] = static_cast<uint8_t>(part0 >> 16);
  target[3] = static_cast<uint8_t>(part0 >> 24);
  target[4] = static_cast<uint8_t>(part1);
  target[5] = static_cast<uint8_t>(part1 >> 8);
  target[6] = static_cast<uint8_t>(part1 >> 16);
  target[7] = static_cast<uint8_t>(part1 >> 24);
#endif
  return target + sizeof(value);
}

inline void CodedOutputStream::WriteVarint32(uint32_t value) {
  cur_ = impl_.EnsureSpace(cur_);
  SetCur(WriteVarint32ToArray(value, Cur()));
}

inline void CodedOutputStream::WriteVarint64(uint64_t value) {
  cur_ = impl_.EnsureSpace(cur_);
  SetCur(WriteVarint64ToArray(value, Cur()));
}

inline void CodedOutputStream::WriteTag(uint32_t value) {
  WriteVarint32(value);
}

inline uint8_t* CodedOutputStream::WriteTagToArray(uint32_t value,
                                                   uint8_t* target) {
  return WriteVarint32ToArray(value, target);
}

#if (defined(__x86__) || defined(__x86_64__) || defined(_M_IX86) || \
     defined(_M_X64)) &&                                            \
    !(defined(__LZCNT__) || defined(__AVX2__))
// X86 CPUs lacking the lzcnt instruction are faster with the bsr-based
// implementation. MSVC does not define __LZCNT__, the nearest option that
// it interprets as lzcnt availability is __AVX2__.
#define PROTOBUF_CODED_STREAM_H_PREFER_BSR 1
#else
#define PROTOBUF_CODED_STREAM_H_PREFER_BSR 0
#endif
inline size_t CodedOutputStream::VarintSize32(uint32_t value) {
#if PROTOBUF_CODED_STREAM_H_PREFER_BSR
  // Explicit OR 0x1 to avoid calling absl::countl_zero(0), which
  // requires a branch to check for on platforms without a clz instruction.
  uint32_t log2value = (std::numeric_limits<uint32_t>::digits - 1) -
                       absl::countl_zero(value | 0x1);
  return static_cast<size_t>((log2value * 9 + (64 + 9)) / 64);
#else
  uint32_t clz = absl::countl_zero(value);
  return static_cast<size_t>(
      ((std::numeric_limits<uint32_t>::digits * 9 + 64) - (clz * 9)) / 64);
#endif
}

inline size_t CodedOutputStream::VarintSize32PlusOne(uint32_t value) {
  // Same as above, but one more.
#if PROTOBUF_CODED_STREAM_H_PREFER_BSR
  uint32_t log2value = (std::numeric_limits<uint32_t>::digits - 1) -
                       absl::countl_zero(value | 0x1);
  return static_cast<size_t>((log2value * 9 + (64 + 9) + 64) / 64);
#else
  uint32_t clz = absl::countl_zero(value);
  return static_cast<size_t>(
      ((std::numeric_limits<uint32_t>::digits * 9 + 64 + 64) - (clz * 9)) / 64);
#endif
}

inline size_t CodedOutputStream::VarintSize64(uint64_t value) {
#if PROTOBUF_CODED_STREAM_H_PREFER_BSR
  // Explicit OR 0x1 to avoid calling absl::countl_zero(0), which
  // requires a branch to check for on platforms without a clz instruction.
  uint32_t log2value = (std::numeric_limits<uint64_t>::digits - 1) -
                       absl::countl_zero(value | 0x1);
  return static_cast<size_t>((log2value * 9 + (64 + 9)) / 64);
#else
  uint32_t clz = absl::countl_zero(value);
  return static_cast<size_t>(
      ((std::numeric_limits<uint64_t>::digits * 9 + 64) - (clz * 9)) / 64);
#endif
}

inline size_t CodedOutputStream::VarintSize64PlusOne(uint64_t value) {
  // Same as above, but one more.
#if PROTOBUF_CODED_STREAM_H_PREFER_BSR
  uint32_t log2value = (std::numeric_limits<uint64_t>::digits - 1) -
                       absl::countl_zero(value | 0x1);
  return static_cast<size_t>((log2value * 9 + (64 + 9) + 64) / 64);
#else
  uint32_t clz = absl::countl_zero(value);
  return static_cast<size_t>(
      ((std::numeric_limits<uint64_t>::digits * 9 + 64 + 64) - (clz * 9)) / 64);
#endif
}

inline size_t CodedOutputStream::VarintSize32SignExtended(int32_t value) {
  return VarintSize64(static_cast<uint64_t>(int64_t{value}));
}

inline size_t CodedOutputStream::VarintSize32SignExtendedPlusOne(
    int32_t value) {
  return VarintSize64PlusOne(static_cast<uint64_t>(int64_t{value}));
}
#undef PROTOBUF_CODED_STREAM_H_PREFER_BSR

inline void CodedOutputStream::WriteString(absl::string_view str) {
  WriteRaw(str.data(), static_cast<int>(str.size()));
}

inline void CodedOutputStream::WriteRawMaybeAliased(const void* data,
                                                    int size) {
  cur_ = impl_.WriteRawMaybeAliased(data, size, cur_);
}

inline uint8_t* CodedOutputStream::WriteRawToArray(const void* data, int size,
                                                   uint8_t* target) {
  memcpy(target, data, static_cast<unsigned int>(size));
  return target + size;
}

inline uint8_t* CodedOutputStream::WriteStringToArray(absl::string_view str,
                                                      uint8_t* target) {
  return WriteRawToArray(str.data(), static_cast<int>(str.size()), target);
}

}  // namespace io
}  // namespace protobuf
}  // namespace google

#if defined(_MSC_VER) && _MSC_VER >= 1300 && !defined(__INTEL_COMPILER)
#pragma runtime_checks("c", restore)
#endif  // _MSC_VER && !defined(__INTEL_COMPILER)

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_IO_CODED_STREAM_H__
