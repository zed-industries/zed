/*
 * Copyright 2019 Google LLC.
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

#ifndef CRYPTO_BIG_NUM_H_
#define CRYPTO_BIG_NUM_H_

#include <stdint.h>

#include <memory>
#include <ostream>
#include <string>

#include "third_party/abseil-cpp/absl/base/attributes.h"
#include "third_party/abseil-cpp/absl/strings/string_view.h"
#include "third_party/private-join-and-compute/base/private_join_and_compute_export.h"
#include "third_party/private-join-and-compute/src/crypto/openssl.inc"
#include "third_party/private-join-and-compute/src/util/status.inc"

namespace private_join_and_compute {

// Immutable wrapper class for openssl BIGNUM numbers.
// Used for arithmetic operations on big numbers.
// Makes use of a BN_CTX structure that holds temporary BIGNUMs needed for
// arithmetic operations as dynamic memory allocation to create BIGNUMs is
// expensive.
class PRIVATE_COMPUTE_EXPORT BigNum {
 public:
  // Deletes a BIGNUM.
  class BnDeleter {
   public:
    void operator()(BIGNUM* bn) { BN_clear_free(bn); }
  };

  // Deletes a BN_MONT_CTX.
  class BnMontCtxDeleter {
   public:
    void operator()(BN_MONT_CTX* ctx) { BN_MONT_CTX_free(ctx); }
  };
  typedef std::unique_ptr<BN_MONT_CTX, BnMontCtxDeleter> BnMontCtxPtr;

  // Copies the given BigNum.
  BigNum(const BigNum& other);
  BigNum& operator=(const BigNum& other);

  // Moves the given BigNum.
  BigNum(BigNum&& other);
  BigNum& operator=(BigNum&& other);

  typedef std::unique_ptr<BIGNUM, BnDeleter> BignumPtr;

  // Returns the absolute value of this in big-endian form.
  std::string ToBytes() const;

  // Converts this BigNum to a uint64_t value. Returns an INVALID_ARGUMENT
  // error code if the value of *this is larger than 64 bits.
  StatusOr<uint64_t> ToIntValue() const;

  // Returns a string representation of the BigNum as a decimal number.
  std::string ToDecimalString() const;

  // Returns the bit length of this BigNum.
  int BitLength() const;

  // Returns False if the number is composite, True if it is prime with an
  // error probability of 1e-40, which gives at least 128 bit security.
  bool IsPrime(double prime_error_probability = 1e-40) const;

  // Returns False if the number is composite, True if it is safe prime with an
  // error probability of at most 1e-40.
  bool IsSafePrime(double prime_error_probability = 1e-40) const;

  // Return True if this BigNum is zero.
  bool IsZero() const;

  // Return True if this BigNum is one.
  bool IsOne() const;

  // Returns True if this BigNum is not negative.
  bool IsNonNegative() const;

  // Returns a BigNum that is equal to the last n bits of this BigNum.
  BigNum GetLastNBits(int n) const;

  // Returns true if n-th bit of this big_num is set, false otherwise.
  bool IsBitSet(int n) const;

  // Returns a BigNum whose value is (- *this).
  // Causes a check failure if the operation fails.
  BigNum Neg() const;

  // Returns a BigNum whose value is (*this + val).
  // Causes a check failure if the operation fails.
  BigNum Add(const BigNum& val) const;

  // Returns a BigNum whose value is (*this * val).
  // Causes a check failure if the operation fails.
  BigNum Mul(const BigNum& val) const;

  // Returns a BigNum whose value is (*this - val).
  // Causes a check failure if the operation fails.
  BigNum Sub(const BigNum& val) const;

  // Returns a BigNum whose value is (*this / val).
  // Causes a check failure if the remainder != 0 or if the operation fails.
  BigNum Div(const BigNum& val) const;

  // Returns a BigNum whose value is *this / val, rounding towards zero.
  // Causes a check failure if the remainder != 0 or if the operation fails.
  BigNum DivAndTruncate(const BigNum& val) const;

  // Compares this BigNum with the specified BigNum.
  // Returns -1 if *this < val, 0 if *this == val and 1 if *this > val.
  int CompareTo(const BigNum& val) const;

  // Returns a BigNum whose value is (*this ^ exponent).
  // Causes a check failure if the operation fails.
  BigNum Exp(const BigNum& exponent) const;

  // Returns a BigNum whose value is (*this mod m).
  BigNum Mod(const BigNum& m) const;

  // Returns a BigNum whose value is (*this + val mod m).
  // Causes a check failure if the operation fails.
  BigNum ModAdd(const BigNum& val, const BigNum& m) const;

  // Returns a BigNum whose value is (*this - val mod m).
  // Causes a check failure if the operation fails.
  BigNum ModSub(const BigNum& val, const BigNum& m) const;

  // Returns a BigNum whose value is (*this * val mod m).
  // For efficiency, please use Montgomery multiplication module if this is done
  // multiple times with the same modulus.
  // Causes a check failure if the operation fails.
  BigNum ModMul(const BigNum& val, const BigNum& m) const;

  // Returns a BigNum whose value is (*this ^ exponent mod m).
  // Causes a check failure if the operation fails.
  BigNum ModExp(const BigNum& exponent, const BigNum& m) const;

  // Return a BigNum whose value is (*this ^ 2 mod m).
  // Causes a check failure if the operation fails.
  BigNum ModSqr(const BigNum& m) const;

  // Returns a BigNum whose value is (*this ^ -1 mod m).
  // Returns a status error if the operation fails, for example if the inverse
  // doesn't exist.
  StatusOr<BigNum> ModInverse(const BigNum& m) const;

  // Returns r such that r ^ 2 == *this mod p.
  // Causes a check failure if the operation fails.
  BigNum ModSqrt(const BigNum& m) const;

  // Computes -a mod m.
  // Causes a check failure if the operation fails.
  BigNum ModNegate(const BigNum& m) const;

  // Returns a BigNum whose value is (*this >> n).
  BigNum Rshift(int n) const;

  // Returns a BigNum whose value is (*this << n).
  // Causes a check failure if the operation fails.
  BigNum Lshift(int n) const;

  // Computes the greatest common divisor of *this and val.
  // Causes a check failure if the operation fails.
  BigNum Gcd(const BigNum& val) const;

  // Returns a pointer to const BIGNUM to be used with openssl functions.
  const BIGNUM* GetConstBignumPtr() const;

 private:
  // Creates a new BigNum object from a bytes string.
  explicit BigNum(BN_CTX* bn_ctx, absl::string_view bytes);
  // Creates a new BigNum object from a char array.
  explicit BigNum(BN_CTX* bn_ctx, const unsigned char* bytes, int length);
  // Creates a new BigNum object from the number.
  explicit BigNum(BN_CTX* bn_ctx, uint64_t number);
  // Creates a new BigNum object with no defined value.
  explicit BigNum(BN_CTX* bn_ctx);
  // Creates a new BigNum object from the given BIGNUM value.
  explicit BigNum(BN_CTX* bn_ctx, BignumPtr bn);

  BignumPtr bn_;
  BN_CTX* bn_ctx_;

  // Context is a factory for BigNum objects.
  friend class Context;
};

inline BigNum operator-(const BigNum& a) { return a.Neg(); }

inline BigNum operator+(const BigNum& a, const BigNum& b) { return a.Add(b); }

inline BigNum operator*(const BigNum& a, const BigNum& b) { return a.Mul(b); }

inline BigNum operator-(const BigNum& a, const BigNum& b) { return a.Sub(b); }

// Returns a BigNum whose value is (a / b).
// Causes a check failure if the remainder != 0.
inline BigNum operator/(const BigNum& a, const BigNum& b) { return a.Div(b); }

inline BigNum& operator+=(BigNum& a, const BigNum& b) { return a = a + b; }

inline BigNum& operator*=(BigNum& a, const BigNum& b) { return a = a * b; }

inline BigNum& operator-=(BigNum& a, const BigNum& b) { return a = a - b; }

inline BigNum& operator/=(BigNum& a, const BigNum& b) { return a = a / b; }

inline bool operator==(const BigNum& a, const BigNum& b) {
  return 0 == a.CompareTo(b);
}

inline bool operator!=(const BigNum& a, const BigNum& b) { return !(a == b); }

inline bool operator<(const BigNum& a, const BigNum& b) {
  return -1 == a.CompareTo(b);
}

inline bool operator>(const BigNum& a, const BigNum& b) {
  return 1 == a.CompareTo(b);
}

inline bool operator<=(const BigNum& a, const BigNum& b) {
  return a.CompareTo(b) <= 0;
}

inline bool operator>=(const BigNum& a, const BigNum& b) {
  return a.CompareTo(b) >= 0;
}

inline BigNum operator%(const BigNum& a, const BigNum& m) { return a.Mod(m); }

inline BigNum operator>>(const BigNum& a, int n) { return a.Rshift(n); }

inline BigNum operator<<(const BigNum& a, int n) { return a.Lshift(n); }

inline BigNum& operator%=(BigNum& a, const BigNum& b) { return a = a % b; }

inline BigNum& operator>>=(BigNum& a, int n) { return a = a >> n; }

inline BigNum& operator<<=(BigNum& a, int n) { return a = a << n; }

inline std::ostream& operator<<(std::ostream& strm, const BigNum& a) {
  return strm << "BigNum(" << a.ToDecimalString() << ")";
}

}  // namespace private_join_and_compute

#endif  // CRYPTO_BIG_NUM_H_
