// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_HAS_BITS_H__
#define GOOGLE_PROTOBUF_HAS_BITS_H__

#include <cassert>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <initializer_list>

// Must be included last.
#include "google/protobuf/port_def.inc"

#ifdef SWIG
#error "You cannot SWIG proto headers"
#endif

namespace google {
namespace protobuf {
namespace internal {

template <int doublewords>
class HasBits {
 public:
  PROTOBUF_NDEBUG_INLINE constexpr HasBits() : has_bits_{} {}

  constexpr HasBits(std::initializer_list<uint32_t> has_bits) : has_bits_{} {
    Copy(has_bits_, &*has_bits.begin(), has_bits.size());
  }

  PROTOBUF_NDEBUG_INLINE void Clear() {
    memset(has_bits_, 0, sizeof(has_bits_));
  }

  PROTOBUF_NDEBUG_INLINE uint32_t& operator[](int index) {
    return has_bits_[index];
  }

  PROTOBUF_NDEBUG_INLINE const uint32_t& operator[](int index) const {
    return has_bits_[index];
  }

  bool operator==(const HasBits<doublewords>& rhs) const {
    return memcmp(has_bits_, rhs.has_bits_, sizeof(has_bits_)) == 0;
  }

  bool operator!=(const HasBits<doublewords>& rhs) const {
    return !(*this == rhs);
  }

  void Or(const HasBits<doublewords>& rhs) {
    for (int i = 0; (i + 1) < doublewords; i += 2) {
      Write64B(Read64B(i) | rhs.Read64B(i), i);
    }
    if ((doublewords % 2) != 0) {
      has_bits_[doublewords - 1] |= rhs.has_bits_[doublewords - 1];
    }
  }

  bool empty() const;

 private:
  // Unfortunately, older GCC compilers (and perhaps others) fail on initializer
  // arguments for an std::array<> or any type of array constructor. Below is a
  // handrolled constexpr 'Copy' function that we use to make a constexpr
  // constructor that accepts a `std::initializer` list.
  static inline constexpr void Copy(uint32_t* dst, const uint32_t* src,
                                    size_t n) {
    assert(n <= doublewords);
    for (size_t ix = 0; ix < n; ++ix) {
      dst[ix] = src[ix];
    }
    for (size_t ix = n; ix < doublewords; ++ix) {
      dst[ix] = 0;
    }
  }

  uint64_t Read64B(int index) const {
    uint64_t v;
    memcpy(&v, has_bits_ + index, sizeof(v));
    return v;
  }

  void Write64B(uint64_t v, int index) {
    memcpy(has_bits_ + index, &v, sizeof(v));
  }

  uint32_t has_bits_[doublewords];
};

template <>
inline bool HasBits<1>::empty() const {
  return !has_bits_[0];
}

template <>
inline bool HasBits<2>::empty() const {
  return !(has_bits_[0] | has_bits_[1]);
}

template <>
inline bool HasBits<3>::empty() const {
  return !(has_bits_[0] | has_bits_[1] | has_bits_[2]);
}

template <>
inline bool HasBits<4>::empty() const {
  return !(has_bits_[0] | has_bits_[1] | has_bits_[2] | has_bits_[3]);
}

template <int doublewords>
inline bool HasBits<doublewords>::empty() const {
  for (uint32_t bits : has_bits_) {
    if (bits) return false;
  }
  return true;
}

}  // namespace internal
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_HAS_BITS_H__
