// Copyright 2015 The Chromium Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef BSSL_DER_INPUT_H_
#define BSSL_DER_INPUT_H_

#include <stddef.h>
#include <stdint.h>

#include <string>
#include <string_view>

#include <openssl/base.h>
#include <openssl/span.h>

#if __has_include(<version>)
#include <version>
#endif

#if defined(__cpp_lib_ranges) && __cpp_lib_ranges >= 201911L
#include <ranges>
BSSL_NAMESPACE_BEGIN
namespace der {
class OPENSSL_EXPORT Input;
}
BSSL_NAMESPACE_END

// Mark `Input` as satisfying the `view` and `borrowed_range` concepts. This
// should be done before the definition of `Input`, so that any inlined calls to
// range functionality use the correct specializations.
template <>
inline constexpr bool std::ranges::enable_view<bssl::der::Input> = true;
template <>
inline constexpr bool std::ranges::enable_borrowed_range<bssl::der::Input> =
    true;
#endif

BSSL_NAMESPACE_BEGIN
namespace der {

// An opaque class that represents a fixed buffer of data of a fixed length,
// to be used as an input to other operations. An Input object does not own
// the data it references, so callers are responsible for making sure that
// the data outlives the Input object and any other associated objects.
//
// All data access for an Input should be done through the ByteReader class.
// This class and associated classes are designed with safety in mind to make it
// difficult to read memory outside of an Input. ByteReader provides a simple
// API for reading through the Input sequentially. For more complicated uses,
// multiple instances of a ByteReader for a particular Input can be created.
//
// TODO(crbug.com/boringssl/661): This class will gradually be replaced with
// bssl::Span<const uint8_t>. Avoid relying on APIs that are not part of
// bssl::Span.
class OPENSSL_EXPORT Input {
 public:
  // Creates an empty Input, one from which no data can be read.
  constexpr Input() = default;

  // Creates an Input from a span. The constructed Input is only valid as long
  // as |data| points to live memory. If constructed from, say, a
  // |std::vector<uint8_t>|, mutating the vector will invalidate the Input.
  constexpr Input(bssl::Span<const uint8_t> data) : data_(data) {}

  // Creates an Input from the given |data| and |len|.
  constexpr explicit Input(const uint8_t *data, size_t len)
      : data_(Span(data, len)) {}

  // Deprecated: Use StringAsBytes.
  //
  // Creates an Input from a std::string_view. The constructed Input is only
  // valid as long as |data| points to live memory. If constructed from, say, a
  // |std::string|, mutating the vector will invalidate the Input.
  explicit Input(std::string_view str) : data_(StringAsBytes(str)) {}

  // The following APIs have the same semantics as in |bssl::Span|.
  constexpr Span<const uint8_t>::iterator begin() const {
    return data_.begin();
  }
  constexpr Span<const uint8_t>::iterator end() const { return data_.end(); }
  constexpr const uint8_t *data() const { return data_.data(); }
  constexpr size_t size() const { return data_.size(); }
  constexpr bool empty() const { return data_.empty(); }
  constexpr uint8_t operator[](size_t idx) const { return data_[idx]; }
  constexpr uint8_t front() const { return data_.front(); }
  constexpr uint8_t back() const { return data_.back(); }
  constexpr Input subspan(size_t pos = 0,
                          size_t len = Span<const uint8_t>::npos) const {
    return Input(data_.subspan(pos, len));
  }
  constexpr Input first(size_t len) const { return Input(data_.first(len)); }
  constexpr Input last(size_t len) const { return Input(data_.last(len)); }

  // Deprecated: use BytesAsStringView and convert to std::string.
  //
  // Returns a copy of the data represented by this object as a std::string.
  std::string AsString() const;

  // Deprecated: Use ByteAsString. 
  //
  // Returns a std::string_view pointing to the same data as the Input. The
  // resulting string_view must not outlive the data that was used to construct
  // this Input.
  std::string_view AsStringView() const { return BytesAsStringView(data_); }

  // Deprecated: This class implicitly converts to bssl::Span<const uint8_t>.
  //
  // Returns a span pointing to the same data as the Input. The resulting span
  // must not outlive the data that was used to construct this Input.
  Span<const uint8_t> AsSpan() const { return *this; }

  // Deprecated: Use size() instead.
  constexpr size_t Length() const { return size(); }

  // Deprecated: Use data() instead.
  constexpr const uint8_t *UnsafeData() const { return data(); }

 private:
  // TODO(crbug.com/770501): Replace this type with span altogether.
  Span<const uint8_t> data_;
};

// Return true if |lhs|'s data and |rhs|'s data are byte-wise equal.
OPENSSL_EXPORT bool operator==(Input lhs, Input rhs);

// Return true if |lhs|'s data and |rhs|'s data are not byte-wise equal.
OPENSSL_EXPORT bool operator!=(Input lhs, Input rhs);

// Returns true if |lhs|'s data is lexicographically less than |rhs|'s data.
OPENSSL_EXPORT constexpr bool operator<(Input lhs, Input rhs) {
  // This is `std::lexicographical_compare`, but that's not `constexpr` until
  // C++-20.
  auto *it1 = lhs.data();
  auto *it2 = rhs.data();
  const auto *end1 = lhs.data() + lhs.size();
  const auto *end2 = rhs.data() + rhs.size();
  for (; it1 != end1 && it2 != end2; ++it1, ++it2) {
    if (*it1 < *it2) {
      return true;
    } else if (*it2 < *it1) {
      return false;
    }
  }

  return it2 != end2;
}

// This class provides ways to read data from an Input in a bounds-checked way.
// The ByteReader is designed to read through the input sequentially. Once a
// byte has been read with a ByteReader, the caller can't go back and re-read
// that byte with the same reader. Of course, the caller can create multiple
// ByteReaders for the same input (or copy an existing ByteReader).
//
// For something simple like a single byte lookahead, the easiest way to do
// that is to copy the ByteReader and call ReadByte() on the copy - the original
// ByteReader will be unaffected and the peeked byte will be read through
// ReadByte(). For other read patterns, it can be useful to mark where one is
// in a ByteReader to be able to return to that spot.
//
// Some operations using Mark can also be done by creating a copy of the
// ByteReader. By using a Mark instead, you use less memory, but more
// importantly, you end up with an immutable object that matches the semantics
// of what is intended.
class OPENSSL_EXPORT ByteReader {
 public:
  // Creates a ByteReader to read the data represented by an Input.
  explicit ByteReader(Input in);

  // Reads a single byte from the input source, putting the byte read in
  // |*byte_p|. If a byte cannot be read from the input (because there is
  // no input left), then this method returns false.
  [[nodiscard]] bool ReadByte(uint8_t *out);

  // Reads |len| bytes from the input source, and initializes an Input to
  // point to that data. If there aren't enough bytes left in the input source,
  // then this method returns false.
  [[nodiscard]] bool ReadBytes(size_t len, Input *out);

  // Returns how many bytes are left to read.
  size_t BytesLeft() const { return data_.size(); }

  // Returns whether there is any more data to be read.
  bool HasMore();

 private:
  void Advance(size_t len);

  bssl::Span<const uint8_t> data_;
};

}  // namespace der
BSSL_NAMESPACE_END

#endif  // BSSL_DER_INPUT_H_
