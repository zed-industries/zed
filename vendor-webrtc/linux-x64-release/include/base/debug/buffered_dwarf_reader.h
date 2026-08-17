// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_DEBUG_BUFFERED_DWARF_READER_H_
#define BASE_DEBUG_BUFFERED_DWARF_READER_H_

#include <array>
#include <cstddef>
#include <cstdint>

#ifdef USE_SYMBOLIZE

namespace base {
namespace debug {

class BufferedDwarfReader {
 public:
  // Constructs a BufferedDwarfReader for a given `fd` starting
  // `position` bytes from the start of the file.
  //
  // BufferedDwarfReader does not affect the `fd` state so it is completely
  // okay to have multiple BufferedDwarfReader attached to one `fd` to act
  // as cursors into different parts of the file.
  BufferedDwarfReader(int fd, uint64_t position);

  // Gets and Sets the absolute position from the start of the file.
  uint64_t position() const { return last_chunk_start_ + cursor_in_buffer_; }

  void set_position(uint64_t position) {
    last_chunk_start_ = next_chunk_start_ = position;

    // Invalidate buffer.
    cursor_in_buffer_ = 0;
    unconsumed_amount_ = 0;
  }

  bool ReadChar(char& value) { return ReadInt(value); }
  bool ReadInt8(uint8_t& value) { return ReadInt(value); }
  bool ReadInt8(int8_t& value) { return ReadInt(value); }
  bool ReadInt16(uint16_t& value) { return ReadInt(value); }
  bool ReadInt32(uint32_t& value) { return ReadInt(value); }
  bool ReadInt64(uint64_t& value) { return ReadInt(value); }

  // Helper to read a null-terminated sequence of bytes.
  //
  // Reads at most `max_position - position()` bytes.
  //
  // Returns the number of bytes written into `out`.
  // On a read error, the internal position of the BufferedDwarfReader may
  // still be advanced. This should only happen if something funky
  // happens at the OS layer at which case it's all best-effort
  // recovery afterwards anyways.
  size_t ReadCString(uint64_t max_position, char* out, size_t out_size);

  // Leb128 is a variable-length integer encoding format. This reads
  // both the signed and unsigned variants of this field.
  bool ReadLeb128(uint64_t& value);
  bool ReadLeb128(int64_t& value);

  // Headers in DWARF often start with a length field of type "initial length."
  // This is a variable-length field that both indicates if the entry is in
  // 32-bit or 64-bit DWARF format and the length of the entry.
  //
  // Note: is_64bit refers to the DWARF format, not the target architecture.
  // Most 64-bit binaries use 32-bit DWARF so is_64bit is frequently false.
  bool ReadInitialLength(bool& is_64bit, uint64_t& length);

  // Offsets inside DWARF are encoded at different sizes based on if it is
  // 32-bit DWARF or 64-bit DWARF. The value of `is_64bit` is usually retrieved
  // by a prior ReadInitialLength() call.
  bool ReadOffset(bool is_64bit, uint64_t& offset);

  // Addresses in DWARF are stored based on address size of the
  // target architecture. This helper reads the correct sized field
  // but then up-converts to a uint64_t type. It does an unsigned
  // extension so 0xffffffff on a 32-bit system will still read out
  // as 0xffffffff.
  bool ReadAddress(uint8_t address_size, uint64_t& address);

  // Many DWARF headers seem to start with
  //   length (initial length)
  //   version (ushort)
  //   offset (32bit or 64-bit dependent on initial length parsing)
  //   address_size (ubyte)
  //
  // The initial length also encodes if this is 32-bit or 64-bit dwarf.
  // This function parses the above sequence of fields.
  //
  // It also returns `end_position` which is the first position after `length`.
  bool ReadCommonHeader(bool& is_64bit,
                        uint64_t& length,
                        uint16_t& version,
                        uint64_t& offset,
                        uint8_t& address_size,
                        uint64_t& end_position);

 private:
  // Generic helper to read an integral value. The size read is determined by
  // the width of `value`
  template <typename IntType>
  bool ReadInt(IntType& value) {
    return BufferedRead(&value, sizeof(value));
  }

  bool BufferedRead(void* buf, const size_t count);

  // The buffer and counters. In local testing, buffer sizes larger than 4096
  // bytes provides negligible benefit, while buffer sizes less than 4096 bytes
  // incur a significant performance penalty: compared to the original buffer
  // size of 256 bytes, 4096 bytes is 2x faster.
  std::array<char, 4096> buf_;
  size_t unconsumed_amount_ = 0;
  size_t cursor_in_buffer_ = 0;

  // The file descriptor for the file being read.
  const int fd_;

  // The position of the next chunk to read.
  uint64_t next_chunk_start_;

  // The position of the last chunk read.
  uint64_t last_chunk_start_;
};

}  // namespace debug
}  // namespace base

#endif

#endif  // BASE_DEBUG_BUFFERED_DWARF_READER_H_
