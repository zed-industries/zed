/*
 * Copyright 2018 Google LLC.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef RLWE_INT256_H_
#define RLWE_INT256_H_

#include "absl/numeric/int128.h"
#include "integral_types.h"
#include "third_party/shell-encryption/base/shell_encryption_export.h"
#include "third_party/shell-encryption/base/shell_encryption_export_template.h"

namespace rlwe {

struct uint256_pod;

// An unsigned 256-bit integer type. Thread-compatible.
class SHELL_ENCRYPTION_EXPORT uint256 {
 public:
  constexpr uint256();
  constexpr uint256(absl::uint128 top, absl::uint128 bottom);

  // Implicit type conversion is allowed so these behave like familiar int types
#ifndef SWIG
  constexpr uint256(int bottom);
  constexpr uint256(Uint32 bottom);
#endif
  constexpr uint256(Uint8 bottom);
  constexpr uint256(unsigned long bottom);
  constexpr uint256(unsigned long long bottom);
  constexpr uint256(absl::uint128 bottom);
  constexpr uint256(const uint256_pod &val);

  // Conversion operators to other arithmetic types
  constexpr explicit operator bool() const;
  constexpr explicit operator char() const;
  constexpr explicit operator signed char() const;
  constexpr explicit operator unsigned char() const;
  constexpr explicit operator char16_t() const;
  constexpr explicit operator char32_t() const;
  constexpr explicit operator short() const;

  constexpr explicit operator unsigned short() const;
  constexpr explicit operator int() const;
  constexpr explicit operator unsigned int() const;
  constexpr explicit operator long() const;

  constexpr explicit operator unsigned long() const;

  constexpr explicit operator long long() const;

  constexpr explicit operator unsigned long long() const;
  constexpr explicit operator absl::int128() const;
  constexpr explicit operator absl::uint128() const;
  explicit operator float() const;
  explicit operator double() const;
  explicit operator long double() const;

  // Trivial copy constructor, assignment operator and destructor.

  void Initialize(absl::uint128 top, absl::uint128 bottom);

  // Arithmetic operators.
  uint256& operator+=(const uint256& b);
  uint256& operator-=(const uint256& b);
  uint256& operator*=(const uint256& b);
  // Long division/modulo for uint256.
  SHELL_ENCRYPTION_EXPORT uint256& operator/=(const uint256& b);
  SHELL_ENCRYPTION_EXPORT uint256& operator%=(const uint256& b);
  uint256 operator++(int);
  uint256 operator--(int);
  SHELL_ENCRYPTION_EXPORT uint256& operator<<=(int);
  uint256& operator>>=(int);
  uint256& operator&=(const uint256& b);
  uint256& operator|=(const uint256& b);
  uint256& operator^=(const uint256& b);
  uint256& operator++();
  uint256& operator--();

  friend absl::uint128 Uint256Low128(const uint256& v);
  friend absl::uint128 Uint256High128(const uint256& v);

  // We add "std::" to avoid including all of port.h.
  friend SHELL_ENCRYPTION_EXPORT std::ostream& operator<<(std::ostream& o, const uint256& b);

 private:
  static void DivModImpl(uint256 dividend, uint256 divisor,
                         uint256* quotient_ret, uint256* remainder_ret);

  // Little-endian memory order optimizations can benefit from
  // having lo_ first, hi_ last.
  // See util/endian/endian.h and Load256/Store256 for storing a uint256.
  // Adding any new members will cause sizeof(uint256) tests to fail.
  absl::uint128 lo_;
  absl::uint128 hi_;

  // Uint256Max()
  //
  // Returns the highest value for a 256-bit unsigned integer.
  friend constexpr uint256 Uint256Max();

  // Not implemented, just declared for catching automatic type conversions.
  uint256(Uint16);
  uint256(float v);
  uint256(double v);
};

constexpr uint256 Uint256Max() {
  return uint256((std::numeric_limits<absl::uint128>::max)(),
                 (std::numeric_limits<absl::uint128>::max)());
}

// This is a POD form of uint256 which can be used for static variables which
// need to be operated on as uint256.
struct SHELL_ENCRYPTION_EXPORT uint256_pod {
  // Note: The ordering of fields is different than 'class uint256' but the
  // same as its 2-arg constructor.  This enables more obvious initialization
  // of static instances, which is the primary reason for this struct in the
  // first place.  This does not seem to defeat any optimizations wrt
  // operations involving this struct.
  absl::uint128 hi;
  absl::uint128 lo;
};

constexpr uint256_pod kuint256max = {absl::Uint128Max(), absl::Uint128Max()};

// allow uint256 to be logged
extern std::ostream& operator<<(std::ostream& o, const uint256& b);

// Methods to access low and high pieces of 256-bit value.
// Defined externally from uint256 to facilitate conversion
// to native 256-bit types when compilers support them.
inline absl::uint128 Uint256Low128(const uint256& v) { return v.lo_; }
inline absl::uint128 Uint256High128(const uint256& v) { return v.hi_; }

// --------------------------------------------------------------------------
//                      Implementation details follow
// --------------------------------------------------------------------------
inline bool operator==(const uint256& lhs, const uint256& rhs) {
  return (Uint256Low128(lhs) == Uint256Low128(rhs) &&
          Uint256High128(lhs) == Uint256High128(rhs));
}
inline bool operator!=(const uint256& lhs, const uint256& rhs) {
  return !(lhs == rhs);
}

inline constexpr uint256::uint256() : lo_(0), hi_(0) {}
inline constexpr uint256::uint256(absl::uint128 top, absl::uint128 bottom)
    : lo_(bottom), hi_(top) {}
inline constexpr uint256::uint256(const uint256_pod& v)
    : lo_(v.lo), hi_(v.hi) {}
inline constexpr uint256::uint256(absl::uint128 bottom) : lo_(bottom), hi_(0) {}
#ifndef SWIG
inline constexpr uint256::uint256(int bottom)
      : lo_(bottom), hi_((bottom < 0) ? -1 : 0) {}
inline constexpr uint256::uint256(Uint32 bottom) : lo_(bottom), hi_(0) {}
#endif
inline constexpr uint256::uint256(Uint8 bottom) : lo_(bottom), hi_(0) {}

inline constexpr uint256::uint256(unsigned long bottom)
    : lo_(bottom), hi_(0) {}

inline constexpr uint256::uint256(unsigned long long bottom)
    : lo_(bottom), hi_(0) {}

inline void uint256::Initialize(absl::uint128 top, absl::uint128 bottom) {
  hi_ = top;
  lo_ = bottom;
}

// Conversion operators to integer types.

constexpr uint256::operator bool() const { return lo_ || hi_; }

constexpr uint256::operator char() const { return static_cast<char>(lo_); }

constexpr uint256::operator signed char() const {
  return static_cast<signed char>(lo_);
}

constexpr uint256::operator unsigned char() const {
  return static_cast<unsigned char>(lo_);
}

constexpr uint256::operator char16_t() const {
  return static_cast<char16_t>(lo_);
}

constexpr uint256::operator char32_t() const {
  return static_cast<char32_t>(lo_);
}


constexpr uint256::operator short() const { return static_cast<short>(lo_); }

constexpr uint256::operator unsigned short() const {
  return static_cast<unsigned short>(lo_);
}

constexpr uint256::operator int() const { return static_cast<int>(lo_); }

constexpr uint256::operator unsigned int() const {
  return static_cast<unsigned int>(lo_);
}


constexpr uint256::operator long() const { return static_cast<long>(lo_); }

constexpr uint256::operator unsigned long() const {
  return static_cast<unsigned long>(lo_);
}

constexpr uint256::operator long long() const {
  return static_cast<long long>(lo_);
}

constexpr uint256::operator unsigned long long() const {
  return static_cast<unsigned long long>(lo_);
}


constexpr uint256::operator absl::uint128() const { return lo_; }
constexpr uint256::operator absl::int128() const {
  return static_cast<absl::int128>(lo_);
}

// Conversion operators to floating point types.

inline uint256::operator float() const {
  return static_cast<float>(lo_) + std::ldexp(static_cast<float>(hi_), 128);
}

inline uint256::operator double() const {
  return static_cast<double>(lo_) + std::ldexp(static_cast<double>(hi_), 128);
}

inline uint256::operator long double() const {
  return static_cast<long double>(lo_) +
         std::ldexp(static_cast<long double>(hi_), 128);
}

// Comparison operators.

#define CMP256(op)                                                  \
  inline bool operator op(const uint256& lhs, const uint256& rhs) { \
    return (Uint256High128(lhs) == Uint256High128(rhs))             \
               ? (Uint256Low128(lhs) op Uint256Low128(rhs))         \
               : (Uint256High128(lhs) op Uint256High128(rhs));      \
  }

CMP256(<)
CMP256(>)
CMP256(>=)
CMP256(<=)

#undef CMP256

// Unary operators

inline uint256 operator-(const uint256& val) {
  const absl::uint128 hi_flip = ~Uint256High128(val);
  const absl::uint128 lo_flip = ~Uint256Low128(val);
  const absl::uint128 lo_add = lo_flip + 1;
  if (lo_add < lo_flip) {
    return uint256(hi_flip + 1, lo_add);
  }
  return uint256(hi_flip, lo_add);
}

inline bool operator!(const uint256& val) {
  return !Uint256High128(val) && !Uint256Low128(val);
}

// Logical operators.

inline uint256 operator~(const uint256& val) {
  return uint256(~Uint256High128(val), ~Uint256Low128(val));
}

#define LOGIC256(op)                                                   \
  inline uint256 operator op(const uint256& lhs, const uint256& rhs) { \
    return uint256(Uint256High128(lhs) op Uint256High128(rhs),         \
                   Uint256Low128(lhs) op Uint256Low128(rhs));          \
  }

LOGIC256(|)
LOGIC256(&)
LOGIC256(^)

#undef LOGIC256

#define LOGICASSIGN256(op)                                 \
  inline uint256& uint256::operator op(const uint256& b) { \
    hi_ op b.hi_;                                          \
    lo_ op b.lo_;                                          \
    return *this;                                          \
  }

LOGICASSIGN256(|=)
LOGICASSIGN256(&=)
LOGICASSIGN256(^=)

#undef LOGICASSIGN256

// Shift operators.

inline uint256 operator<<(const uint256& val, int amount) {
  uint256 out(val);
  out <<= amount;
  return out;
}

inline uint256 operator>>(const uint256& val, int amount) {
  uint256 out(val);
  out >>= amount;
  return out;
}

inline uint256& uint256::operator<<=(int amount) {
  // uint128 shifts of >= 128 are undefined, so we will need some special-casing
  if (amount < 128) {
    if (amount != 0) {
      hi_ = (hi_ << amount) | (lo_ >> (128 - amount));
      lo_ = lo_ << amount;
    }
  } else if (amount < 256) {
    hi_ = lo_ << (amount - 128);
    lo_ = 0;
  } else {
    hi_ = 0;
    lo_ = 0;
  }
  return *this;
}

inline uint256& uint256::operator>>=(int amount) {
  // uint128 shifts of >= 128 are undefined, so we will need some special-casing
  if (amount < 128) {
    if (amount != 0) {
      lo_ = (lo_ >> amount) | (hi_ << (128 - amount));
      hi_ = hi_ >> amount;
    }
  } else if (amount < 256) {
    lo_ = hi_ >> (amount - 128);
    hi_ = 0;
  } else {
    lo_ = 0;
    hi_ = 0;
  }
  return *this;
}

inline uint256 operator+(const uint256& lhs, const uint256& rhs) {
  return uint256(lhs) += rhs;
}

inline uint256 operator-(const uint256& lhs, const uint256& rhs) {
  return uint256(lhs) -= rhs;
}

inline uint256 operator*(const uint256& lhs, const uint256& rhs) {
  return uint256(lhs) *= rhs;
}

inline uint256 operator/(const uint256& lhs, const uint256& rhs) {
  return uint256(lhs) /= rhs;
}

inline uint256 operator%(const uint256& lhs, const uint256& rhs) {
  return uint256(lhs) %= rhs;
}

inline uint256& uint256::operator+=(const uint256& b) {
  hi_ += b.hi_;
  absl::uint128 lolo = lo_ + b.lo_;
  if (lolo < lo_)
    ++hi_;
  lo_ = lolo;
  return *this;
}

inline uint256& uint256::operator-=(const uint256& b) {
  hi_ -= b.hi_;
  if (b.lo_ > lo_)
    --hi_;
  lo_ -= b.lo_;
  return *this;
}

inline uint256& uint256::operator*=(const uint256& b) {
  // Computes the product c = a * b modulo 2^256.
  //
  // We have that
  //   a = [a.hi_ || a.lo_] and b = [b.hi_ || b.lo_]
  // where hi_, lo_ are 128-bit numbers. Further, we have that
  //   a.lo_ = [a64 || a00] and b.lo_ = [b64 || b00]
  // where a64, a00, b64, b00 are 64-bit numbers.
  //
  // The product c = (a * b mod 2^256) is equal to
  //   (a.hi_ * b.lo_ + a64 * b64 + b.hi_ * a.lo_ mod 2^128) * 2^128 +
  //   (a64 * b00 + a00 * b64) * 2^64 +
  //   (a00 * b00)
  //
  // The first and last lines can be computed without worrying about the
  // carries, and then we add the two elements from the second line.
  absl::uint128 a64 = absl::Uint128High64(lo_);
  absl::uint128 a00 = absl::Uint128Low64(lo_);
  absl::uint128 b64 = absl::Uint128High64(b.lo_);
  absl::uint128 b00 = absl::Uint128Low64(b.lo_);

  // Compute the high order and low order part of c (safe to ignore carry bits).
  this->hi_ = hi_ * b.lo_ + a64 * b64 + lo_ * b.hi_;
  this->lo_ = a00 * b00;

  // add middle term and capture carry
  uint256 middle_term = uint256(a64 * b00) + uint256(a00 * b64);
  *this += middle_term << 64;
  return *this;
}

inline uint256 uint256::operator++(int) {
  uint256 tmp(*this);
  lo_++;
  if (lo_ == 0) hi_++;  // If there was a wrap around, increase the high word.
  return tmp;
}

inline uint256 uint256::operator--(int) {
  uint256 tmp(*this);
  if (lo_ == 0) hi_--;  // If it wraps around, decrease the high word.
  lo_--;
  return tmp;
}

inline uint256& uint256::operator++() {
  lo_++;
  if (lo_ == 0) hi_++;  // If there was a wrap around, increase the high word.
  return *this;
}

inline uint256& uint256::operator--() {
  if (lo_ == 0) hi_--;  // If it wraps around, decrease the high word.
  lo_--;
  return *this;
}

}  // namespace rlwe

// Specialized numeric_limits for uint256.
namespace std {
template <>
class numeric_limits<rlwe::uint256> {
 public:
  static constexpr bool is_specialized = true;
  static constexpr bool is_signed = false;
  static constexpr bool is_integer = true;
  static constexpr bool is_exact = true;
  static constexpr bool has_infinity = false;
  static constexpr bool has_quiet_NaN = false;
  static constexpr bool has_signaling_NaN = false;
  static constexpr float_denorm_style has_denorm = denorm_absent;
  static constexpr bool has_denorm_loss = false;
  static constexpr float_round_style round_style = round_toward_zero;
  static constexpr bool is_iec559 = false;
  static constexpr bool is_bounded = true;
  static constexpr bool is_modulo = true;
  static constexpr int digits = 256;
  static constexpr int digits10 = 77;
  static constexpr int max_digits10 = 0;
  static constexpr int radix = 2;
  static constexpr int min_exponent = 0;
  static constexpr int min_exponent10 = 0;
  static constexpr int max_exponent = 0;
  static constexpr int max_exponent10 = 0;
  static constexpr bool traps = numeric_limits<absl::uint128>::traps;
  static constexpr bool tinyness_before = false;

  static constexpr rlwe::uint256(min)() { return 0; }
  static constexpr rlwe::uint256 lowest() { return 0; }
  static constexpr rlwe::uint256(max)() { return rlwe::Uint256Max(); }
  static constexpr rlwe::uint256 epsilon() { return 0; }
  static constexpr rlwe::uint256 round_error() { return 0; }
  static constexpr rlwe::uint256 infinity() { return 0; }
  static constexpr rlwe::uint256 quiet_NaN() { return 0; }
  static constexpr rlwe::uint256 signaling_NaN() { return 0; }
  static constexpr rlwe::uint256 denorm_min() { return 0; }
};
}  // namespace std
#endif  // RLWE_INT256_H_
