/*
 * Copyright 2017 Google LLC.
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

// Defines types that are necessary for Ring-Learning with Errors (rlwe)
// encryption.

#ifndef RLWE_MONTGOMERY_H_
#define RLWE_MONTGOMERY_H_

#include <cmath>
#include <cstdint>
#include <tuple>
#include <vector>

#include <glog/logging.h>
#include "absl/numeric/int128.h"
#include "absl/strings/str_cat.h"
#include "bits_util.h"
#include "constants.h"
#include "int256.h"
#include "prng/prng.h"
#include "serialization.pb.h"
#include "status_macros.h"
#include "statusor.h"
#include "third_party/shell-encryption/base/shell_encryption_export.h"
#include "third_party/shell-encryption/base/shell_encryption_export_template.h"

namespace rlwe {

namespace internal {

// Struct to capture the "bigger int" type.
template <typename T>
struct BigInt;
// Specialization for uint8, uint16, uint32, uint64, and uint128.
template <> 
struct EXPORT_TEMPLATE_DECLARE(SHELL_ENCRYPTION_EXPORT) BigInt<Uint8> {
  typedef Uint16 value_type;
};
template <> 
struct EXPORT_TEMPLATE_DECLARE(SHELL_ENCRYPTION_EXPORT) BigInt<Uint16> {
  typedef Uint32 value_type;
};
template <> 
struct EXPORT_TEMPLATE_DECLARE(SHELL_ENCRYPTION_EXPORT) BigInt<Uint32> {
  typedef Uint64 value_type;
};
template <> 
struct EXPORT_TEMPLATE_DECLARE(SHELL_ENCRYPTION_EXPORT) BigInt<Uint64> {
  typedef absl::uint128 value_type;
};
template <> 
struct EXPORT_TEMPLATE_DECLARE(SHELL_ENCRYPTION_EXPORT) BigInt<absl::uint128> {
  typedef uint256 value_type;
};

}  // namespace internal

// The parameters necessary for a Montgomery integer. Note that the template
// parameters ensure that T is an unsigned integral of at least 8 bits.
template <typename T>
struct EXPORT_TEMPLATE_DECLARE(SHELL_ENCRYPTION_EXPORT) MontgomeryIntParams{
  // Expose Int and its greater type. BigInt is required in order to multiply
  // two Int and ensure that no overflow occurs.
  //
  // Thread safe.
  using Int = T;
  using BigInt = typename internal::BigInt<Int>::value_type;
  static const size_t bitsize_int = sizeof(Int) * 8;

  // Factory function to create MontgomeryIntParams.
  static rlwe::StatusOr<std::unique_ptr<const MontgomeryIntParams>> Create(
      Int modulus);

  // The value R to be used in Montgomery multiplication. R will be selected as
  // 2^bitsize(Int) and hence automatically verifies R > modulus.
  const BigInt r = static_cast<BigInt>(1) << bitsize_int;

  // The modulus over which these modular operations are being performed.
  const Int modulus;

  // The modulus over which these modular operations are being performed, cast
  // as a BigInt.
  const BigInt modulus_bigint;

  // The number of bits in the modulus.
  const unsigned int log_modulus;

  // The value of R taken modulo the modulus.
  const Int r_mod_modulus;
  const Int
      r_mod_modulus_barrett;  // = (r_mod_modulus << bitsize_int) / modulus, to
                              // speed up multiplication by r_mod_modulus.

  // The values below, inv_modulus and inv_r, satisfy the formula:
  //     R * inv_r - modulus * inv_modulus = 1
  // Note that inv_modulus is not literally the multiplicative inverse of
  // modulus modulo R.
  const Int inv_modulus;
  const Int
      inv_r;  // needed to translate from Montgomery to normal representation
  const Int inv_r_barrett;  // = (inv_r << bitsize_int) / modulus, to speed up
                            // multiplication by inv_r.

  // The numerator used in the Barrett reduction.
  const BigInt barrett_numerator;  // = 2^(sizeof(Int)*8) / modulus

  inline const Int Zero() const { return 0; }
  inline const Int One() const { return 1; }
  inline const Int Two() const { return 2; }

  // Functions to perform Barrett reduction. For more details, see
  // https://en.wikipedia.org/wiki/Barrett_reduction.
  // This function should remains in the header file to avoid performance
  // regressions.
  inline Int BarrettReduce(Int input) const {
    Int out =
        static_cast<Int>((this->barrett_numerator * input) >> bitsize_int);
    out = input - (out * this->modulus);
    // The steps above produce an integer that is in the range [0, 2N).
    // We now reduce to the range [0, N).
    return (out >= this->modulus) ? out - this->modulus : out;
  }

  // Computes the serialized byte length of an integer.
  inline unsigned int SerializedSize() const { return (log_modulus + 7) / 8; }

  // Check whether (1 << log_n) fits into the underlying Int type.
  static bool DoesLogNFit(Uint64 log_n) { return (log_n < bitsize_int - 1); }

 private:
  MontgomeryIntParams(Int mod)
      : modulus(mod),
        modulus_bigint(static_cast<BigInt>(this->modulus)),
        log_modulus(internal::BitLength(this->modulus)),
        r_mod_modulus(static_cast<Int>(this->r % this->modulus_bigint)),
        r_mod_modulus_barrett(static_cast<Int>(
            (static_cast<BigInt>(r_mod_modulus) << bitsize_int) / modulus)),
        inv_modulus(static_cast<Int>(
            std::get<1>(MontgomeryIntParams::Inverses(modulus_bigint, r)))),
        inv_r(std::get<0>(MontgomeryIntParams::Inverses(modulus_bigint, r))),
        inv_r_barrett(static_cast<Int>(
            (static_cast<BigInt>(inv_r) << bitsize_int) / modulus)),
        barrett_numerator(this->r / this->modulus_bigint) {}

  // Computes the Montgomery inverse coefficients for r and modulus using
  // the Extended Euclidean Algorithm.
  //
  // modulus must be odd.
  // Returns a tuple of (inv_r, inv_modulus) such that:
  //     r * inv_r - modulus * inv_modulus = 1
  static SHELL_ENCRYPTION_EXPORT std::tuple<Int, Int> Inverses(BigInt modulus_bigint, BigInt r);
};

// Stores an integer in Montgomery representation. The goal of this
// class is to provide a static type that differentiates between integers
// in Montgomery representation and standard integers. Once a Montgomery
// integer is created, it has the * and + operators necessary to be treated
// as another integer.
// The underlying integer type T must be unsigned and must not be bool.
// This class is thread safe.
template <typename T>
class EXPORT_TEMPLATE_DECLARE(SHELL_ENCRYPTION_EXPORT) ABSL_MUST_USE_RESULT MontgomeryInt {
 public:
  // Expose Int and its greater type. BigInt is required in order to multiply
  // two Int and ensure that no overflow occurs. This should also be used by
  // external classes.
  using Int = T;
  using BigInt = typename internal::BigInt<Int>::value_type;

  // Expose the parameter type.
  using Params = MontgomeryIntParams<T>;

  // Static factory that converts a non-Montgomery representation integer, the
  // underlying integer type, into a Montgomery representation integer. Does not
  // take ownership of params. i.e., import "a".
  static SHELL_ENCRYPTION_EXPORT rlwe::StatusOr<MontgomeryInt> ImportInt(Int n, const Params* params);

  // Static functions to create a MontgomeryInt of 0 and 1.
  static SHELL_ENCRYPTION_EXPORT MontgomeryInt ImportZero(const Params* params);
  static SHELL_ENCRYPTION_EXPORT MontgomeryInt ImportOne(const Params* params);

  // Import a random integer using entropy from specified prng. Does not take
  // ownership of params or prng.
  template <typename Prng = rlwe::SecurePrng>
  static SHELL_ENCRYPTION_EXPORT rlwe::StatusOr<MontgomeryInt> ImportRandom(Prng* prng,
                                                    const Params* params) {
    // In order to generate unbiased randomness, we uniformly and randomly
    // sample integers in [0, 2^params->log_modulus) until the generated integer
    // is less than the modulus (i.e., we perform rejection sampling).
    RLWE_ASSIGN_OR_RETURN(Int random_int,
                          GenerateRandomInt(params->log_modulus, prng));
    while (random_int >= params->modulus) {
      RLWE_ASSIGN_OR_RETURN(random_int,
                            GenerateRandomInt(params->log_modulus, prng));
    }
    return MontgomeryInt(random_int);
  }

  static BigInt DivAndTruncate(BigInt dividend, BigInt divisor);

  // No default constructor.
  MontgomeryInt() = delete;

  // Default copy constructor.
  MontgomeryInt(const MontgomeryInt& that) = default;
  MontgomeryInt& operator=(const MontgomeryInt& that) = default;

  // Convert a Montgomery representation integer back to the underlying integer.
  // i.e., export "a".
  inline Int ExportInt(const Params* params) const {
    Int out =
        static_cast<Int>((static_cast<BigInt>(params->inv_r_barrett) * n_) >>
                         params->bitsize_int);
    out = n_ * params->inv_r - out * params->modulus;
    // The steps above produce an integer that is in the range [0, 2N).
    // We now reduce to the range [0, N).
    out -= (out >= params->modulus) ? params->modulus : 0;
    return out;
  }

  // Returns the least significant 64 bits of n.
  static Uint64 ExportUInt64(Int n) { return static_cast<Uint64>(n); }

  // Serialization.
  rlwe::StatusOr<std::string> Serialize(const Params* params) const;
  static SHELL_ENCRYPTION_EXPORT rlwe::StatusOr<std::string> SerializeVector(
      const std::vector<MontgomeryInt>& coeffs, const Params* params);

  // Deserialization.
  static SHELL_ENCRYPTION_EXPORT rlwe::StatusOr<MontgomeryInt> Deserialize(absl::string_view payload,
                                                   const Params* params);
  static SHELL_ENCRYPTION_EXPORT rlwe::StatusOr<std::vector<MontgomeryInt>> DeserializeVector(
      int num_coeffs, absl::string_view serialized, const Params* params);

  // Modular multiplication.
  // Perform a multiply followed by a modular reduction. Produces an output
  // in the range [0, N).
  // Taken from Hacker's Delight chapter on Montgomery multiplication.
  MontgomeryInt Mul(const MontgomeryInt& that, const Params* params) const {
    MontgomeryInt out(*this);
    return out.MulInPlace(that, params);
  }

  // This function should remains in the header file to avoid performance
  // regressions.
  MontgomeryInt& MulInPlace(const MontgomeryInt& that, const Params* params) {
    // This function computes the product of the two numbers (a and b), which
    // will equal a * R * b * R in Montgomery representation. It then performs
    // the reduction, creating a * b * R (mod N).
    Int u = static_cast<Int>(n_ * that.n_) * params->inv_modulus;
    BigInt t = static_cast<BigInt>(n_) * that.n_ + params->modulus_bigint * u;
    Int t_msb = static_cast<Int>(t >> Params::bitsize_int);

    // The steps above produce an integer that is in the range [0, 2N).
    // We now reduce to the range [0, N).
    n_ = (t_msb >= params->modulus) ? t_msb - params->modulus : t_msb;
    return *this;
  }

  // Modular multiplication by a constant with precomputation.
  // MontgomeryInt are stored under the form n * R, where R = 1 << bitsize_int.
  // When multiplying by a constant c, one can precompute
  //   constant = (c * R^(-1) mod modulus)
  //   constant_barrett = (constant << bitsize_int) / modulus
  // and perform modular reduction using Barrett reduction instead of
  // Montgomery multiplication.

  // The funciton GetConstant() returns a tuple (constant, constant_barrett):
  //       constant = ExportInt(params),
  // and
  //       constant_barrett = (constant << bitsize_int) / modulus
  std::tuple<Int, Int> GetConstant(const Params* params) const;

  MontgomeryInt MulConstant(const Int& constant, const Int& constant_barrett,
                            const Params* params) const {
    MontgomeryInt out(*this);
    return out.MulConstantInPlace(constant, constant_barrett, params);
  }

  // This function should remains in the header file to avoid performance
  // regressions.
  MontgomeryInt& MulConstantInPlace(const Int& constant,
                                    const Int& constant_barrett,
                                    const Params* params) {
    Int out = static_cast<Int>((static_cast<BigInt>(constant_barrett) * n_) >>
                               Params::bitsize_int);
    n_ = n_ * constant - out * params->modulus;
    // The steps above produce an integer that is in the range [0, 2N).
    // We now reduce to the range [0, N).
    n_ -= (n_ >= params->modulus) ? params->modulus : 0;
    return *this;
  }

  // Montgomery addition.
  MontgomeryInt Add(const MontgomeryInt& that, const Params* params) const {
    MontgomeryInt out(*this);
    return out.AddInPlace(that, params);
  }

  // This function should remains in the header file to avoid performance
  // regressions.
  MontgomeryInt& AddInPlace(const MontgomeryInt& that, const Params* params) {
    // We can use Barrett reduction because n_ <= modulus < Max(Int)/2.
    n_ = params->BarrettReduce(n_ + that.n_);
    return *this;
  }

  // Modular negation.
  MontgomeryInt Negate(const Params* params) const {
    return MontgomeryInt(params->modulus - n_);
  }

  // This function should remains in the header file to avoid performance
  // regressions.
  MontgomeryInt& NegateInPlace(const Params* params) {
    n_ = params->modulus - n_;
    return *this;
  }

  // Modular subtraction.
  MontgomeryInt Sub(const MontgomeryInt& that, const Params* params) const {
    MontgomeryInt out(*this);
    return out.SubInPlace(that, params);
  }

  // This function should remains in the header file to avoid performance
  // regressions.
  MontgomeryInt& SubInPlace(const MontgomeryInt& that, const Params* params) {
    // We can use Barrett reduction because n_ <= modulus < Max(Int)/2.
    n_ = params->BarrettReduce(n_ + (params->modulus - that.n_));
    return *this;
  }

  // Batch operations (and in-place variants) over vectors of MontgomeryInt.
  // We define two versions of the batch operations:
  // -  the first input is a vector and the second is a MontgomeryInt scalar:
  //    in that case, the scalar will be added/subtracted/multiplied with each
  //    element of the first input.
  // -  both inputs are vectors of same length: in that case, the operations
  //    will be performed component wise.
  // These batch operations may fail if the input vectors are not of the same
  // size.

  // Batch addition of two vectors.
  static SHELL_ENCRYPTION_EXPORT rlwe::StatusOr<std::vector<MontgomeryInt>> BatchAdd(
      const std::vector<MontgomeryInt>& in1,
      const std::vector<MontgomeryInt>& in2, const Params* params);
  static absl::Status BatchAddInPlace(std::vector<MontgomeryInt>* in1,
                                      const std::vector<MontgomeryInt>& in2,
                                      const Params* params);

  // Batch addition of one vector with a scalar.
  static SHELL_ENCRYPTION_EXPORT rlwe::StatusOr<std::vector<MontgomeryInt>> BatchAdd(
      const std::vector<MontgomeryInt>& in1, const MontgomeryInt& in2,
      const Params* params);
  static absl::Status BatchAddInPlace(std::vector<MontgomeryInt>* in1,
                                      const MontgomeryInt& in2,
                                      const Params* params);

  // Batch subtraction of two vectors.
  static SHELL_ENCRYPTION_EXPORT rlwe::StatusOr<std::vector<MontgomeryInt>> BatchSub(
      const std::vector<MontgomeryInt>& in1,
      const std::vector<MontgomeryInt>& in2, const Params* params);
  static SHELL_ENCRYPTION_EXPORT absl::Status BatchSubInPlace(std::vector<MontgomeryInt>* in1,
                                      const std::vector<MontgomeryInt>& in2,
                                      const Params* params);

  // Batch subtraction of one vector with a scalar.
  static SHELL_ENCRYPTION_EXPORT rlwe::StatusOr<std::vector<MontgomeryInt>> BatchSub(
      const std::vector<MontgomeryInt>& in1, const MontgomeryInt& in2,
      const Params* params);
  static SHELL_ENCRYPTION_EXPORT absl::Status BatchSubInPlace(std::vector<MontgomeryInt>* in1,
                                      const MontgomeryInt& in2,
                                      const Params* params);

  // Batch multiplication of two vectors.
  static SHELL_ENCRYPTION_EXPORT rlwe::StatusOr<std::vector<MontgomeryInt>> BatchMul(
      const std::vector<MontgomeryInt>& in1,
      const std::vector<MontgomeryInt>& in2, const Params* params);
  static SHELL_ENCRYPTION_EXPORT absl::Status BatchMulInPlace(std::vector<MontgomeryInt>* in1,
                                      const std::vector<MontgomeryInt>& in2,
                                      const Params* params);

  // Batch multiplication of two vectors, where the second vector is a constant.
  static SHELL_ENCRYPTION_EXPORT rlwe::StatusOr<std::vector<MontgomeryInt>> BatchMulConstant(
      const std::vector<MontgomeryInt>& in1,
      const std::vector<Int>& in2_constant,
      const std::vector<Int>& in2_constant_barrett, const Params* params);
  static SHELL_ENCRYPTION_EXPORT absl::Status BatchMulConstantInPlace(
      std::vector<MontgomeryInt>* in1, const std::vector<Int>& in2_constant,
      const std::vector<Int>& in2_constant_barrett, const Params* params);

  // Batch multiplication of a vector with a scalar.
  static SHELL_ENCRYPTION_EXPORT rlwe::StatusOr<std::vector<MontgomeryInt>> BatchMul(
      const std::vector<MontgomeryInt>& in1, const MontgomeryInt& in2,
      const Params* params);
  static SHELL_ENCRYPTION_EXPORT absl::Status BatchMulInPlace(std::vector<MontgomeryInt>* in1,
                                      const MontgomeryInt& in2,
                                      const Params* params);

  // Batch multiplication of a vector with a constant scalar.
  static SHELL_ENCRYPTION_EXPORT rlwe::StatusOr<std::vector<MontgomeryInt>> BatchMulConstant(
      const std::vector<MontgomeryInt>& in1, const Int& constant,
      const Int& constant_barrett, const Params* params);
  static SHELL_ENCRYPTION_EXPORT absl::Status BatchMulConstantInPlace(std::vector<MontgomeryInt>* in1,
                                              const Int& constant,
                                              const Int& constant_barrett,
                                              const Params* params);

  // Equality.
  bool operator==(const MontgomeryInt& that) const { return (n_ == that.n_); }
  bool operator!=(const MontgomeryInt& that) const { return !(*this == that); }

  // Modular exponentiation.
  SHELL_ENCRYPTION_EXPORT MontgomeryInt ModExp(Int exponent, const Params* params) const;

  // Inverse.
  SHELL_ENCRYPTION_EXPORT  MontgomeryInt MultiplicativeInverse(const Params* params) const;

 private:
  template <typename Prng = rlwe::SecurePrng>
  static SHELL_ENCRYPTION_EXPORT rlwe::StatusOr<Int> GenerateRandomInt(int log_modulus, Prng* prng) {
    // Generate a random Int. As the modulus is always smaller than max(Int),
    // there will be no issues with overflow.
    int max_bits_per_step = std::min((int)Params::bitsize_int, (int)64);
    auto bits_required = log_modulus;
    Int rand = 0;
    while (bits_required > 0) {
      Int rand_bits = 0;
      if (bits_required <= 8) {
        // Generate 8 bits of randomness.
        RLWE_ASSIGN_OR_RETURN(rand_bits, prng->Rand8());

        // Extract bits_required bits and add them to rand.
        Int needed_bits =
            rand_bits & ((static_cast<Int>(1) << bits_required) - 1);
        rand = (rand << bits_required) + needed_bits;
        break;
      } else {
        // Generate 64 bits of randomness.
        RLWE_ASSIGN_OR_RETURN(rand_bits, prng->Rand64());

        // Extract min(64, bits in Int, bits_required) bits and add them to rand
        int bits_to_extract = std::min(bits_required, max_bits_per_step);
        Int needed_bits =
            rand_bits & ((static_cast<Int>(1) << bits_to_extract) - 1);
        rand = (rand << bits_to_extract) + needed_bits;
        bits_required -= bits_to_extract;
      }
    }
    return rand;
  }

  explicit MontgomeryInt(Int n) : n_(n) {}

  Int n_;
};

// Instantiations of MontgomeryInt and MontgomeryIntParams with specific
// integral types.
extern template struct EXPORT_TEMPLATE_DECLARE(SHELL_ENCRYPTION_EXPORT) MontgomeryIntParams<Uint16>;
extern template struct EXPORT_TEMPLATE_DECLARE(SHELL_ENCRYPTION_EXPORT) MontgomeryIntParams<Uint32>;
extern template struct EXPORT_TEMPLATE_DECLARE(SHELL_ENCRYPTION_EXPORT) MontgomeryIntParams<Uint64>;
extern template struct EXPORT_TEMPLATE_DECLARE(SHELL_ENCRYPTION_EXPORT) MontgomeryIntParams<absl::uint128>;
extern template class EXPORT_TEMPLATE_DECLARE(SHELL_ENCRYPTION_EXPORT) MontgomeryInt<Uint16>;
extern template class EXPORT_TEMPLATE_DECLARE(SHELL_ENCRYPTION_EXPORT) MontgomeryInt<Uint32>;
extern template class EXPORT_TEMPLATE_DECLARE(SHELL_ENCRYPTION_EXPORT) MontgomeryInt<Uint64>;
extern template class EXPORT_TEMPLATE_DECLARE(SHELL_ENCRYPTION_EXPORT) MontgomeryInt<absl::uint128>;

}  // namespace rlwe

#endif  // RLWE_MONTGOMERY_H_
