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

#ifndef RLWE_SYMMETRIC_ENCRYPTION_H_
#define RLWE_SYMMETRIC_ENCRYPTION_H_

#include <algorithm>
#include <cstdint>
#include <vector>

#include "error_params.h"
#include "polynomial.h"
#include "prng/integral_prng_types.h"
#include "prng/prng.h"
#include "sample_error.h"
#include "serialization.pb.h"
#include "status_macros.h"

namespace rlwe {

// This file implements the somewhat homomorphic symmetric-key encryption scheme
// from "Fully Homomorphic Encryption from Ring-LWE and Security for Key
// Dependent Messages" by Zvika Brakerski and Vinod Vaikuntanathan. This
// encryption scheme uses Ring Learning with Errors (RLWE).
// http://www.wisdom.weizmann.ac.il/~zvikab/localpapers/IdealHom.pdf
//
// The scheme has CPA security under the hardness of the
// Ring-Learning with Errors problem (see reference above for details). We do
// not implement protections against timing attacks.
//
// The encryption scheme in this file is not fully homomorphic. It does not
// implement any sort of bootstrapping.

// Represents a ciphertext encrypted using a symmetric-key version of the ring
// learning-with-errors (RLWE) encryption scheme. See the comments that follow
// throughout this file for full details on the particular encryption scheme.
//
// This implementation supports the following homomorphic operations:
//  - Homomorphic addition.
//  - Scalar multiplication by a polynomial (absorption)
//  - Homomorphic multiplication.
//
// This implementation is only "somewhat homomorphic," not fully homomorphic.
// There is no bootstrapping, so a limited number of homomorphic operations can
// be performed before so much error accumulates that decryption is impossible.
//
// Each ciphertext comprises a vector of polynomials <c0, ..., cN>. Initially,
// a ciphertext comprises a pair <c0, c1>. Homomorphic multiplications cause
// the vector to grow longer.
template <typename ModularInt>
class SymmetricRlweCiphertext {
  using Int = typename ModularInt::Int;
  // BigInt is required in order to multiply two Int and ensure that no overflow
  // occurs during the multiplication of two ciphertexts.
  using BigInt = typename ModularInt::BigInt;

 public:
  // Default and copy constructors.
  explicit SymmetricRlweCiphertext(const typename ModularInt::Params* params,
                                   const ErrorParams<ModularInt>* error_params)
      : modulus_params_(params),
        error_params_(error_params),
        power_of_s_(1),
        error_(0) {}
  SymmetricRlweCiphertext(const SymmetricRlweCiphertext& that) = default;

  // Create a ciphertext by supplying the vector of components.
  explicit SymmetricRlweCiphertext(std::vector<Polynomial<ModularInt>> c,
                                   int power_of_s, double error,
                                   const typename ModularInt::Params* params,
                                   const ErrorParams<ModularInt>* error_params)
      : c_(std::move(c)),
        modulus_params_(params),
        error_params_(error_params),
        power_of_s_(power_of_s),
        error_(error) {}

  // Homomorphic addition: add the polynomials representing the ciphertexts
  // component-wise. The example below demonstrates why this procedure works
  // properly in the two-component case. The quantities a, s, m, t, and e are
  // introduced during encryption and are explained in the SymmetricRlweKey
  // class.
  //
  //   (a1 * s + m1 + t * e1, -a1)
  // + (a2 * s + m2 + t * e2, -a2)
  // ------------------------------
  //   ((a1 + a2) * s + (m1 + m2) + t * (e1 + e2), -(a1 + a2))
  //
  // Substitute (a1 + a2) = a3, (e1 + e2) = e3:
  //
  //   (a3 * s + (m1 + m2) + t * e3, -a3)
  //
  // This result is a valid ciphertext where the value of a has changed, the
  // error has increased, and the encoded plaintext contains the sum of the
  // plaintexts that were encoded in the original two ciphertexts.
  rlwe::StatusOr<SymmetricRlweCiphertext> operator+(
      const SymmetricRlweCiphertext& that) const {
    SymmetricRlweCiphertext out = *this;
    RLWE_RETURN_IF_ERROR(out.AddInPlace(that));
    return out;
  }

  absl::Status AddInPlace(const SymmetricRlweCiphertext& that) {
    if (power_of_s_ != that.power_of_s_) {
      return absl::InvalidArgumentError(
          "Ciphertexts must be encrypted with the same key power.");
    }

    if (c_.size() < that.c_.size()) {
      Polynomial<ModularInt> zero(that.c_[0].Len(), modulus_params_);
      c_.resize(that.c_.size(), zero);
    }

    for (int i = 0; i < that.c_.size(); i++) {
      RLWE_RETURN_IF_ERROR(c_[i].AddInPlace(that.c_[i], modulus_params_));
    }

    error_ += that.error_;
    return absl::OkStatus();
  }

  // Homomorphic subtraction: subtract the polynomials representing the
  // ciphertexts component-wise. The example below demonstrates why this
  // procedure works properly in the two-component case. The quantities a, s, m,
  // t, and e are introduced during encryption and are explained in the
  // SymmetricRlweKey class.
  //
  //   (a1 * s + m1 + t * e1, -a1)
  // - (a2 * s + m2 + t * e2, -a2)
  // ------------------------------
  //   ((a1 - a2) * s + (m1 - m2) + t * (e1 - e2), -(a1 - a2))
  //
  // Substitute (a1 - a2) = a3, (e1 - e2) = e3:
  //
  //   (a3 * s + (m1 - m2) + t * e3, -a3)
  //
  // This result is a valid ciphertext where the value of a has changed, the
  // error has increased, and the encoded plaintext contains the sum of the
  // plaintexts that were encoded in the original two ciphertexts.
  rlwe::StatusOr<SymmetricRlweCiphertext> operator-(
      const SymmetricRlweCiphertext& that) const {
    SymmetricRlweCiphertext out = *this;
    RLWE_RETURN_IF_ERROR(out.SubInPlace(that));
    return out;
  }

  absl::Status SubInPlace(const SymmetricRlweCiphertext& that) {
    if (power_of_s_ != that.power_of_s_) {
      return absl::InvalidArgumentError(
          "Ciphertexts must be encrypted with the same key power.");
    }

    if (c_.size() < that.c_.size()) {
      Polynomial<ModularInt> zero(that.c_[0].Len(), modulus_params_);
      c_.resize(that.c_.size(), zero);
    }

    for (int i = 0; i < that.c_.size(); i++) {
      RLWE_RETURN_IF_ERROR(c_[i].SubInPlace(that.c_[i], modulus_params_));
    }

    error_ += that.error_;
    return absl::OkStatus();
  }

  // Homomorphic absorbtion. Multiplies the current ciphertext {m1}_s (plaintext
  // m1 encrypted  with symmetric key s) by a plaintext m2, resulting in a
  // ciphertext {m1 * m2}_s that stores m1 * m2 encrypted with symmetric key s.
  //
  // DO NOT CONFUSE THIS OPERATION WITH HOMOMORPHIC MULTIPLICATION.
  //
  // To perform this operation, multiply the each component of the
  // ciphertext by the plaintext polynomial. The example below demonstrates why
  // this procedure works properly in the two-component case. The quantities a,
  // s, m, t, and e are introduced during encryption and are explained in the
  // Encrypt() function later in this file.
  //
  //    (a1 * s + m1 + t * e1, -a1) * p
  //  = (a1 * s * p + m1 * p + t * e1 * p)
  //
  // Substitute (a1 * p) = a2 and (e1 * p) = e2:
  //
  //    (a2 * s + m1 * p + t * e2)
  //
  // This result is a valid ciphertext where the value of a has changed, the
  // error has increased, and the encoded plaintext contains the product of
  // m1 and p.
  //
  // A few more details about the multiplication that takes place:
  //
  // The value stored in the resulting ciphertext is (m1 * m2) (mod 2^N + 1)
  // (mod t), where N is the number of coefficients in s (or m1 or m2, since
  // the all have the same number of coefficients). In other words, the
  // result is the remainder of (m1 * m2) mod the polynomial (2^N + 1) with
  // each of the coefficients the ntaken mod t. Any coefficient between 0 and
  // modulus / 2 is treated as a positive number for the purposes of the final
  // (mod t); any coefficient between modulus/2 and modulus is treated as
  // a negative number for the purposes of the final (mod t).
  rlwe::StatusOr<SymmetricRlweCiphertext> operator*(
      const Polynomial<ModularInt>& that) const {
    SymmetricRlweCiphertext out = *this;
    RLWE_RETURN_IF_ERROR(out.AbsorbInPlace(that));
    return out;
  }

  absl::Status AbsorbInPlace(const Polynomial<ModularInt>& that) {
    for (auto& component : this->c_) {
      RLWE_RETURN_IF_ERROR(component.MulInPlace(that, modulus_params_));
    }
    error_ *= error_params_->B_plaintext();
    return absl::OkStatus();
  }

  // Homomorphically absorb a plaintext scalar. This function is exactly like
  // homomorphic absorb above, except the plaintext is a constant.
  rlwe::StatusOr<SymmetricRlweCiphertext> operator*(
      const ModularInt& that) const {
    SymmetricRlweCiphertext out = *this;
    RLWE_RETURN_IF_ERROR(out.AbsorbInPlace(that));
    return out;
  }

  absl::Status AbsorbInPlace(const ModularInt& that) {
    for (auto& component : this->c_) {
      RLWE_RETURN_IF_ERROR(component.MulInPlace(that, modulus_params_));
    }
    error_ *= static_cast<double>(that.ExportInt(modulus_params_));
    return absl::OkStatus();
  }

  // Homomorphic multiply. Given two ciphertexts {m1}_s, {m2}_s containing
  // messages m1 and m2 encrypted with the same secret key s, return the
  // ciphertext {m1 * m2}_s containing the product of the messages.
  //
  // To perform this operation, treat the two ciphertext vectors as polynomials
  // and perform a polynomial multiplication:
  //
  //   <c0, c1> * <c0', c1'> = <c0 * c0, c0 * c1 + c1 * c0, c1 * c1>
  //
  // If the two ciphertext vectors are of length m and n, the resulting
  // ciphertext is of length m + n - 1.
  //
  // The details of the multiplication that takes place between m1 and m2 are
  // the same as in the homomorphic absorb operation above (the other overload
  // of the * operator).
  rlwe::StatusOr<SymmetricRlweCiphertext> operator*(
      const SymmetricRlweCiphertext& that) {
    if (power_of_s_ != that.power_of_s_) {
      return absl::InvalidArgumentError(
          "Ciphertexts must be encrypted with the same key power.");
    }
    if (c_.size() <= 0 || that.c_.size() <= 0) {
      return absl::InvalidArgumentError(
          "Cannot multiply using an empty ciphertext.");
    }
    if (c_[0].Len() <= 0 || that.c_[0].Len() <= 0) {
      return absl::InvalidArgumentError(
          "Cannot multiply using an empty polynomial in the ciphertext.");
    }
    Polynomial<ModularInt> temp(c_[0].Len(), modulus_params_);
    std::vector<Polynomial<ModularInt>> result(c_.size() + that.c_.size() - 1,
                                               temp);
    for (int i = 0; i < c_.size(); i++) {
      for (int j = 0; j < that.c_.size(); j++) {
        RLWE_ASSIGN_OR_RETURN(temp, c_[i].Mul(that.c_[j], modulus_params_));
        RLWE_RETURN_IF_ERROR(result[i + j].AddInPlace(temp, modulus_params_));
      }
    }

    return SymmetricRlweCiphertext(std::move(result), power_of_s_,
                                   error_ * that.error_, modulus_params_,
                                   error_params_);
  }

  // Convert this ciphertext from (mod p) to (mod q).
  // Assumes that ModularInt::Int and ModularIntQ::Int are the same type.
  //
  // The current modulus (mod t) must be equal to modulus q (mod t).
  // This will always be true. For NTT to work properly, any modulus must be
  // of the form 2N + 1, where N is a power of 2. Likewise, the implementation
  // requires that t is a power of 2. This means that, for any modulus q and
  // modulus t allowed by the RLWE implementation, q % t == 1.
  template <typename ModularIntQ>
  rlwe::StatusOr<SymmetricRlweCiphertext<ModularIntQ>> SwitchModulus(
      const NttParameters<ModularInt>* ntt_params_p,
      const typename ModularIntQ::Params* modulus_params_q,
      const NttParameters<ModularIntQ>* ntt_params_q,
      const ErrorParams<ModularIntQ>* error_params_q, const Int& t) {
    Int p = modulus_params_->modulus;
    Int q = modulus_params_q->modulus;

    // Configuration error.
    if (p % t != q % t) {
      return absl::InvalidArgumentError("p % t != q % t");
    }

    SymmetricRlweCiphertext<ModularIntQ> output(modulus_params_q,
                                                error_params_q);
    output.power_of_s_ = power_of_s_;
    // Overestimate the ratio of the two moduli.
    double modulus_ratio = static_cast<double>(modulus_params_q->log_modulus) /
                           modulus_params_->log_modulus;
    output.error_ = modulus_ratio * error_ + error_params_q->B_scale();

    output.c_.reserve(c_.size());
    for (const Polynomial<ModularInt>& c : c_) {
      // Extract each component of the ciphertext from NTT form.
      std::vector<ModularInt> coeffs_p =
          c.InverseNtt(ntt_params_p, modulus_params_);
      std::vector<ModularIntQ> coeffs_q;
      coeffs_q.reserve(coeffs_p.size());

      // Convert each coefficient of the polynomial from (mod p) to (mod q)
      for (const ModularInt& coeff_p : coeffs_p) {
        Int int_p = coeff_p.ExportInt(modulus_params_);

        // Scale the integer.
        Int int_q = static_cast<Int>(ModularInt::DivAndTruncate(
            static_cast<BigInt>(int_p) * static_cast<BigInt>(q),
            static_cast<BigInt>(p)));

        // Ensure that int_p = int_q mod t by changing int_q as little as
        // possible.
        Int int_p_mod_t = int_p % t;
        Int int_q_mod_t = int_q % t;
        Int adjustment_up = modulus_params_->Zero();
        Int adjustment_down = modulus_params_->Zero();

        // Determine whether to adjust int_q up or down to make sure int_q =
        // int_p (mod t).
        adjustment_up = int_p_mod_t - int_q_mod_t;
        adjustment_down = t + int_q_mod_t - int_p_mod_t;
        if (int_p_mod_t < int_q_mod_t) {
          adjustment_up = adjustment_up + t;
          adjustment_down = adjustment_down - t;
        }

        RLWE_ASSIGN_OR_RETURN(auto m_int_q,
                              ModularIntQ::ImportInt(int_q, modulus_params_q));
        if (adjustment_up > adjustment_down) {
          RLWE_ASSIGN_OR_RETURN(
              auto m_adjustment_up,
              ModularIntQ::ImportInt(adjustment_up, modulus_params_q));
          // Adjust up.
          coeffs_q.push_back(
              std::move(m_adjustment_up.AddInPlace(m_int_q, modulus_params_q)));
        } else {
          RLWE_ASSIGN_OR_RETURN(
              auto m_adjustment_down,
              ModularIntQ::ImportInt(q - adjustment_down, modulus_params_q));
          // Adjust down.
          coeffs_q.push_back(std::move(
              m_adjustment_down.AddInPlace(m_int_q, modulus_params_q)));
        }
      }

      // Convert back to NTT.
      output.c_.push_back(Polynomial<ModularIntQ>::ConvertToNtt(
          std::move(coeffs_q), ntt_params_q, modulus_params_q));
    }

    return output;
  }

  // Given a ciphertext c encrypting a plaintext p(x) under secret key s(x),
  // returns a ciphertext c' encrypting p(x^power) under the secret key
  // s(x^power).
  // Power must be an odd non-negative integer less than 2 * num_coeffs.
  // This method uses NTT conversions to apply the substitution in the
  // coefficient domain, and should be avoided if performance is an issue.
  // Substitutions of the form 2^j + 1 are used to obliviously expand a query
  // ciphertext into a query vector.
  rlwe::StatusOr<SymmetricRlweCiphertext> Substitute(
      int substitution_power,
      const NttParameters<ModularInt>* ntt_params) const {
    SymmetricRlweCiphertext output(modulus_params_, error_params_);
    output.c_.reserve(c_.size());

    for (const Polynomial<ModularInt>& c : c_) {
      RLWE_ASSIGN_OR_RETURN(
          auto elt,
          c.Substitute(substitution_power, ntt_params, modulus_params_));
      output.c_.push_back(std::move(elt));
    }
    output.power_of_s_ = (power_of_s_ * substitution_power) % (2 * c_[0].Len());
    output.error_ = error_;
    return output;
  }

  rlwe::StatusOr<SerializedSymmetricRlweCiphertext> Serialize() const {
    SerializedSymmetricRlweCiphertext output;
    output.set_power_of_s(power_of_s_);
    output.set_error(error_);

    for (const Polynomial<ModularInt>& c : c_) {
      RLWE_ASSIGN_OR_RETURN(*output.add_c(), c.Serialize(modulus_params_));
    }

    return output;
  }

  static rlwe::StatusOr<SymmetricRlweCiphertext> Deserialize(
      const SerializedSymmetricRlweCiphertext& serialized,
      const typename ModularInt::Params* modulus_params,
      const ErrorParams<ModularInt>* error_params) {
    SymmetricRlweCiphertext output(modulus_params, error_params);
    output.power_of_s_ = serialized.power_of_s();
    output.error_ = serialized.error();

    if (serialized.c_size() <= 0) {
      return absl::InvalidArgumentError("Ciphertext cannot be empty.");
    } else if (serialized.c_size() > kMaxNumCoeffs) {
      return absl::InvalidArgumentError(
          absl::StrCat("Number of coefficients, ", serialized.c_size(),
                       ", cannot be more than ", kMaxNumCoeffs, "."));
    }

    for (int i = 0; i < serialized.c_size(); i++) {
      RLWE_ASSIGN_OR_RETURN(auto elt, Polynomial<ModularInt>::Deserialize(
                                          serialized.c(i), modulus_params));
      output.c_.push_back(std::move(elt));
    }

    return output;
  }

  // Accessors.
  unsigned int Len() const { return c_.size(); }

  rlwe::StatusOr<Polynomial<ModularInt>> Component(int index) const {
    if (0 > index || index >= c_.size()) {
      return absl::InvalidArgumentError("Index out of range.");
    }
    return c_[index];
  }

  const typename ModularInt::Params* ModulusParams() const {
    return modulus_params_;
  }
  const rlwe::ErrorParams<ModularInt>* ErrorParams() const {
    return error_params_;
  }
  int PowerOfS() const { return power_of_s_; }
  double Error() const { return error_; }
  void SetError(double error) { error_ = error; }

 private:
  // The ciphertext.
  std::vector<Polynomial<ModularInt>> c_;

  // ModularInt parameters.
  const typename ModularInt::Params* modulus_params_;

  // Error parameters.
  const rlwe::ErrorParams<ModularInt>* error_params_;

  // The power a in s(x^a) that the ciphertext can be decrypted with.
  int power_of_s_;

  // A heuristic on the error of the ciphertext.
  double error_;

  // Make this class a friend of any version of this class, no matter the
  // template.
  template <typename Q>
  friend class SymmetricRlweCiphertext;
};

// Holds a key that can be used to encrypt messages using the RLWE-based
// encryption scheme.
template <typename ModularInt>
class SymmetricRlweKey {
  using Int = typename ModularInt::Int;

 public:
  // Allow copy, copy-assign, move and move-assign.
  SymmetricRlweKey(const SymmetricRlweKey&) = default;
  SymmetricRlweKey& operator=(const SymmetricRlweKey&) = default;
  SymmetricRlweKey(SymmetricRlweKey&&) = default;
  SymmetricRlweKey& operator=(SymmetricRlweKey&&) = default;
  ~SymmetricRlweKey() = default;

  // Static factory that samples a key from the error distribution. The
  // polynomial representing the key must have a number of coefficients that is
  // a power of two, which is enforced by the first argument.
  //
  // Does not take ownership of rand, modulus_params or ntt_params.
  static rlwe::StatusOr<SymmetricRlweKey> Sample(
      unsigned int log_num_coeffs, uint64_t variance, uint64_t log_t,
      const typename ModularInt::Params* modulus_params,
      const NttParameters<ModularInt>* ntt_params, SecurePrng* prng) {
    RLWE_ASSIGN_OR_RETURN(
        auto error, SampleFromErrorDistribution<ModularInt>(
                        1 << log_num_coeffs, variance, prng, modulus_params));
    Polynomial<ModularInt> key = Polynomial<ModularInt>::ConvertToNtt(
        std::move(error), ntt_params, modulus_params);
    RLWE_ASSIGN_OR_RETURN(
        auto t_mod, ModularInt::ImportInt((modulus_params->One() << log_t) +
                                              modulus_params->One(),
                                          modulus_params));
    return SymmetricRlweKey(std::move(key), variance, log_t, std::move(t_mod),
                            modulus_params, modulus_params, ntt_params);
  }

  rlwe::StatusOr<SerializedNttPolynomial> Serialize() const {
    return key_.Serialize(modulus_params_);
  }

  // Deserialize using modulus params as also the plaintext modulus params. Use
  // this when deserializing a non-modulus switched key.
  static rlwe::StatusOr<SymmetricRlweKey> Deserialize(
      Uint64 variance, Uint64 log_t,
      const SerializedNttPolynomial& serialized_key,
      const typename ModularInt::Params* modulus_params,
      const NttParameters<ModularInt>* ntt_params) {
    return Deserialize(variance, log_t, serialized_key, modulus_params,
                       modulus_params, ntt_params);
  }

  static rlwe::StatusOr<SymmetricRlweKey> Deserialize(
      Uint64 variance, Uint64 log_t,
      const SerializedNttPolynomial& serialized_key,
      const typename ModularInt::Params* modulus_params,
      const typename ModularInt::Params* plaintext_modulus_params,
      const NttParameters<ModularInt>* ntt_params) {
    // Check that log_t is no larger than the log_modulus - 1.
    if (log_t > modulus_params->log_modulus - 1) {
      return absl::InvalidArgumentError(absl::StrCat(
          "The value of log_t, ", log_t, ", must be smaller than ",
          "log_modulus - 1, ", modulus_params->log_modulus - 1, "."));
    }
    RLWE_ASSIGN_OR_RETURN(
        Polynomial<ModularInt> key,
        Polynomial<ModularInt>::Deserialize(serialized_key, modulus_params));
    RLWE_ASSIGN_OR_RETURN(
        auto t_mod,
        ModularInt::ImportInt((plaintext_modulus_params->One() << log_t) +
                                  plaintext_modulus_params->One(),
                              plaintext_modulus_params));
    return SymmetricRlweKey(std::move(key), variance, log_t, std::move(t_mod),
                            modulus_params, plaintext_modulus_params,
                            ntt_params);
  }

  // Generate a copy of this key in modulus q.
  //
  // The current modulus (mod t) must be equal to modulus q (mod t). This
  // property is implicitly enforced by the design of the code as described
  // by the corresponding comment on SymmetricRlweKey::SwitchModulus. This
  // property is also dynamically enforced.
  //
  // The algorithms for modulus-switching ciphertexts and keys are similar but
  // slightly different. In particular, RLWE keys are guaranteed to have small
  // coefficients, and thus modulus switching can be made very simple. Hence
  // we have 2 separate implementations of SwitchModulus for keys and
  // ciphertexts.
  template <typename ModularIntQ>
  rlwe::StatusOr<SymmetricRlweKey<ModularIntQ>> SwitchModulus(
      const typename ModularIntQ::Params* modulus_params_q,
      const NttParameters<ModularIntQ>* ntt_params_q) const {
    // Configuration failure.
    Int t = (modulus_params_q->One() << log_t_) + modulus_params_q->One();
    if (modulus_params_->modulus % t != modulus_params_q->modulus % t) {
      return absl::InvalidArgumentError("p % t != q % t");
    }

    typename ModularIntQ::Int p_mod_q =
        modulus_params_->modulus % modulus_params_q->modulus;
    std::vector<ModularInt> coeffs_p =
        key_.InverseNtt(ntt_params_, modulus_params_);
    std::vector<ModularIntQ> coeffs_q;

    // Convert each coefficient of the polynomial from (mod p) to (mod q)
    for (const ModularInt& coeff_p : coeffs_p) {
      // Ensure that negative numbers (mod p) are translated into negative
      // numbers (mod q).
      Int int_p = coeff_p.ExportInt(modulus_params_);
      if (int_p > modulus_params_->modulus >> 1) {
        int_p = int_p - p_mod_q;
      }

      RLWE_ASSIGN_OR_RETURN(auto m_int_p,
                            ModularIntQ::ImportInt(int_p, modulus_params_q));
      coeffs_q.push_back(std::move(m_int_p));
    }

    // Convert back to NTT.
    auto key_q = Polynomial<ModularIntQ>::ConvertToNtt(
        std::move(coeffs_q), ntt_params_q, modulus_params_q);

    RLWE_ASSIGN_OR_RETURN(
        auto t_mod, ModularInt::ImportInt((modulus_params_q->One() << log_t_) +
                                              modulus_params_q->One(),
                                          modulus_params_q));
    return SymmetricRlweKey<ModularIntQ>(std::move(key_q), variance_, log_t_,
                                         std::move(t_mod), modulus_params_q,
                                         modulus_params_q, ntt_params_q);
  }

  // Given s(x), returns a secret key s(x^a).
  // This performs an Inverse NTT on the key, substitutes the key in polynomial
  // representation, and then performs an NTT again.
  rlwe::StatusOr<SymmetricRlweKey> Substitute(const int power) const {
    RLWE_ASSIGN_OR_RETURN(
        auto t_mod, ModularInt::ImportInt((modulus_params_->One() << log_t_) +
                                              modulus_params_->One(),
                                          modulus_params_));
    RLWE_ASSIGN_OR_RETURN(auto sub,
                          key_.Substitute(power, ntt_params_, modulus_params_));
    return SymmetricRlweKey(std::move(sub), variance_, log_t_, std::move(t_mod),
                            modulus_params_, plaintext_modulus_params_,
                            ntt_params_);
  }

  // Accessors.
  unsigned int Len() const { return key_.Len(); }
  const NttParameters<ModularInt>* NttParams() const { return ntt_params_; }
  const typename ModularInt::Params* ModulusParams() const {
    return modulus_params_;
  }
  unsigned int BitsPerCoeff() const { return log_t_; }
  Uint64 Variance() const { return variance_; }
  unsigned int LogT() const { return log_t_; }
  const ModularInt& PlaintextModulus() const { return t_mod_; }
  const typename ModularInt::Params* PlaintextModulusParams() const {
    return plaintext_modulus_params_;
  }
  const Polynomial<ModularInt>& Key() const { return key_; }

  // Add two homomorphic encryption keys.
  rlwe::StatusOr<SymmetricRlweKey<ModularInt>> Add(
      const SymmetricRlweKey<ModularInt>& other_key) {
    if (variance_ != other_key.variance_) {
      return absl::InvalidArgumentError(absl::StrCat(
          "The variance of the other key, ", other_key.variance_,
          ", is different than the variance of this key, ", variance_, "."));
    }
    if (log_t_ != other_key.log_t_) {
      return absl::InvalidArgumentError(absl::StrCat(
          "The log_t of the other key, ", other_key.log_t_,
          ", is different than the log_t of this key, ", log_t_, "."));
    }
    if (t_mod_ != other_key.t_mod_) {
      return absl::InvalidArgumentError(
          absl::StrCat("The plaintext space of the other key is different than "
                       "the plaintext space of this key."));
    }
    RLWE_ASSIGN_OR_RETURN(auto key, key_.Add(other_key.key_, modulus_params_));
    return SymmetricRlweKey<ModularInt>(std::move(key), variance_, log_t_,
                                        t_mod_, modulus_params_,
                                        plaintext_modulus_params_, ntt_params_);
  }

  // Substract two homomorphic encryption keys.
  rlwe::StatusOr<SymmetricRlweKey<ModularInt>> Sub(
      const SymmetricRlweKey<ModularInt>& other_key) {
    if (variance_ != other_key.variance_) {
      return absl::InvalidArgumentError(absl::StrCat(
          "The variance of the other key, ", other_key.variance_,
          ", is different than the variance of this key, ", variance_, "."));
    }
    if (log_t_ != other_key.log_t_) {
      return absl::InvalidArgumentError(absl::StrCat(
          "The log_t of the other key, ", other_key.log_t_,
          ", is different than the log_t of this key, ", log_t_, "."));
    }
    if (t_mod_ != other_key.t_mod_) {
      return absl::InvalidArgumentError(
          absl::StrCat("The plaintext space of the other key is different than "
                       "the plaintext space of this key."));
    }
    RLWE_ASSIGN_OR_RETURN(auto key, key_.Sub(other_key.key_, modulus_params_));
    return SymmetricRlweKey<ModularInt>(std::move(key), variance_, log_t_,
                                        t_mod_, modulus_params_,
                                        plaintext_modulus_params_, ntt_params_);
  }

  // Static function to create a null key (with value 0).
  static rlwe::StatusOr<SymmetricRlweKey> NullKey(
      unsigned int log_num_coeffs, Uint64 variance, Uint64 log_t,
      const typename ModularInt::Params* modulus_params,
      const NttParameters<ModularInt>* ntt_params) {
    Polynomial<ModularInt> zero(1 << log_num_coeffs, modulus_params);
    RLWE_ASSIGN_OR_RETURN(
        auto t_mod, ModularInt::ImportInt((modulus_params->One() << log_t) +
                                              modulus_params->One(),
                                          modulus_params));
    return SymmetricRlweKey(std::move(zero), variance, log_t, std::move(t_mod),
                            modulus_params, modulus_params, ntt_params);
  }

 private:
  // The contents of the key itself.
  Polynomial<ModularInt> key_;

  // The variance of the binomial distribution from which the key and error are
  // drawn.
  Uint64 variance_;

  // The maximum size of any one coefficient of the polynomial representing a
  // plaintext message.
  unsigned int log_t_;
  ModularInt t_mod_;

  // NTT parameters.
  const NttParameters<ModularInt>* ntt_params_;

  // ModularInt parameters.
  const typename ModularInt::Params* modulus_params_;
  const typename ModularInt::Params* plaintext_modulus_params_;

  // A constructor. Does not take ownership of params.
  SymmetricRlweKey(Polynomial<ModularInt> key, Uint64 variance,
                   unsigned int log_t, ModularInt t_mod,
                   const typename ModularInt::Params* modulus_params,
                   const typename ModularInt::Params* plaintext_modulus_params,
                   const NttParameters<ModularInt>* ntt_params)
      : key_(std::move(key)),
        variance_(variance),
        log_t_(log_t),
        t_mod_(std::move(t_mod)),
        ntt_params_(ntt_params),
        modulus_params_(modulus_params),
        plaintext_modulus_params_(plaintext_modulus_params) {}

  // Make this class a friend of any version of this class, no matter the
  // template.
  template <typename Q>
  friend class SymmetricRlweKey;
};

// Encrypts the plaintext using ring learning-with-errors (RLWE) encryption.
// (b/79577340): The parameter t is specified by log_t right, but is equal to
// (1 << log_t) + 1 so that t is odd. This is to allow multiplicative inverses
// of powers of 2, which are used to compress and obliviously expand a query
// ciphertext.
//
// The scheme works as follows:
//   KeyGen(n, modulus q, error distr):
//     Sample a degree (n-1) polynomial whose coefficients are drawn from the
//     error distribution (mod q). This is our secret key. Call it s.
//
//   Encrypt(secret key s, plaintext m, modulus q, modulus t, error distr):
//     1) Sample a degree (n-1) polynomial whose coefficients are drawn
//        uniformly from any integer (mod q). Call this polynomial a.
//     2) Sample a degree (n-1) polynomial whose coefficients are drawn from
//        the error distribution (mod q). Call this polynomial e.
//     3) Our secret key s and plaintext m are both degree (n-1) polynomials.
//        For decryption to work, each coefficient of m must be < t.
//        Compute (a * s + t * e + m) (mod x^n + 1). Call this polynomial b.
//     4) The ciphertext is the pair (b, -a). We refer to the pair of
//        polynomials representing a ciphertext as (c0, c1) =
//        (a * s + m + e * t, -a).
//
//    Decrypt(secret key s, ciphertext (b, -a), modulus t):
//      // Decryption when the ciphertext has two components.
//      Compute and return (b - as) (mod t). Doing out the algebra:
//          b - as (mod t)
//        = as + te + m - as (mod t)
//        = te + m (mod t)
//        = m
//      Quoting the paper, "the condition for correct decryption is that the
//      L_infinity norm of the polynomial [te + m] is smaller than q/2." In
//      other words, the largest of the values te + m (recall that e is
//      sampled from a distribution) cannot exceed q/2.
//
//   When the ciphertext has more than two components <c0, c1, ..., cN>,
//   it can be decrypted by taking the dot product with the vector
//   <s^0, s^1, ..., s^N> containing powers of the secret key:
//       te + m = <c0, 1, ..., cN> dot <s^0, s^1, ..., s^N>
//              = c0 * s^0 + c1 * s^1 + ... + cN * s^N
//
// Note that the Encrypt() function takes the original plaintext as
// an Polynomial<ModularInt>, while the corresponding Decrypt() method
// returns a std::vector<typename ModularInt::Int>. The two values will be the
// same once the original plaintext is converted out of NTT and Montgomery form.
//   - The Encrypt() function takes an NTT polynomial so that, if the same
//     plaintext is to be encrypted repeatedly, the NTT conversion only needs
//     to be performed once by the caller.
//   - The Decrypt() function returns a vector of integers because the final
//     (mod t) step requires taking the polynomial (te + m) out of NTT and
//     Montgomery form.
// It would be straightforward to write a wrapper of Encrypt() that takes
// a vector of integers as input, thereby making the plaintext types of the
// Encrypt() and Decrypt() functions symmetric.

namespace internal {

// This functions allows injecting a specific polynomial "a" as the randomness
// of the encryption (that is the negation of the c1 component of the
// ciphertext) and returns only the resulting c1 component of the ciphertext.
// This function is intended for internal use only.
template <typename ModularInt>
rlwe::StatusOr<Polynomial<ModularInt>> Encrypt(
    const SymmetricRlweKey<ModularInt>& key,
    const Polynomial<ModularInt>& plaintext, const Polynomial<ModularInt>& a,
    SecurePrng* prng) {
  // Sample the error term from the error distribution.
  unsigned int num_coeffs = key.Len();
  RLWE_ASSIGN_OR_RETURN(
      std::vector<ModularInt> e_coeffs,
      SampleFromErrorDistribution<ModularInt>(num_coeffs, key.Variance(), prng,
                                              key.ModulusParams()));

  // Create and return c0.
  auto e = Polynomial<ModularInt>::ConvertToNtt(
      std::move(e_coeffs), key.NttParams(), key.ModulusParams());
  RLWE_ASSIGN_OR_RETURN(Polynomial<ModularInt> temp,
                        a.Mul(key.Key(), key.ModulusParams()));
  RLWE_RETURN_IF_ERROR(
      e.MulInPlace(key.PlaintextModulus(), key.ModulusParams()));
  RLWE_RETURN_IF_ERROR(temp.AddInPlace(e, key.ModulusParams()));
  RLWE_RETURN_IF_ERROR(temp.AddInPlace(plaintext, key.ModulusParams()));
  return temp;
}

}  // namespace internal

// Encrypts the supplied plaintext using the given key. Randomness is drawn from
// the key's underlying ModulusParams.
template <typename ModularInt>
rlwe::StatusOr<SymmetricRlweCiphertext<ModularInt>> Encrypt(
    const SymmetricRlweKey<ModularInt>& key,
    const Polynomial<ModularInt>& plaintext,
    const ErrorParams<ModularInt>* error_params, SecurePrng* prng) {
  // Sample a from the uniform distribution.
  RLWE_ASSIGN_OR_RETURN(auto a, SamplePolynomialFromPrng<ModularInt>(
                                    key.Len(), prng, key.ModulusParams()));

  // Create c0.
  RLWE_ASSIGN_OR_RETURN(Polynomial<ModularInt> c0,
                        internal::Encrypt(key, plaintext, a, prng));

  // Compute c1 = -a and return the ciphertext.
  return SymmetricRlweCiphertext<ModularInt>(
      std::vector<Polynomial<ModularInt>>{
          std::move(c0), std::move(a.NegateInPlace(key.ModulusParams()))},
      1, error_params->B_encryption(), key.ModulusParams(), error_params);
}

// Takes as input the result of decrypting a RLWE plaintext that still contains
// the error. Concretely, it contains m + e * t (mod q). This function
// eliminates the error and returns the message. For reasons described below,
// this operation is more complicated than a simple (mod t).
//
// The error is drawn from a binomial distribution centered at zero and
// multiplied by t, meaning error values are either positive or negative
// multiples of t. Since each coefficient of the plaintext is smaller than
// t, some coefficients of the quantity m + e * t (which is all that's
// left in the vector error_and_message) could be negative. We are using
// modular arithmetic, so negative values become large positive values.
//
// Unfortunately, these negative values caues the naive error elimination
// strategy to fail. In theory we could take (m + e * t) mod t to
// eliminate the error portion and extract the message. However, consider
// a case where the error is negative. Suppose that t=2, m=1, and e=-1
// with a modulus q=7:
//
//    m +  e * t (mod q) =
//    1 + -1 * 2 (mod 7) =
//            -1 (mod 7) =
//             6 (mod 7)
//
// When we take 6 (mod t) = 6 (mod 2), we get 0, which is not the original
// bit of m. To avoid this problem, we treat negative values as negative
// values, not as their equivalents mod q.
//
// We consider (m + e * t) to be negative whenever it is between q/2
// and q. Recall that, if |m + e * t| is greater than q/2, decryption
// fails.
//
// When the quantity (m + e * t) (mod q) represents a negative number
// mod q, we can re-create its non-modular negative form by computing
// ((m + e * t) - q). We can then take this value mod t to extract the
// correct answer.
//
// 1. (m + e * t (mod q)) =                    // in the range [q/2, q)
// 2. (m + e * t - q)     =                    // in the range [-q/2, 0)
// 3. m (mod t) + e * t (mod t) - q (mod t) =  // taken (mod t)
// 4. m - (q (mod t))
//
// If we subtract q at step 2, we return negative numbers to their
// original form. Since we are going to perform a (mod t) operation
// anyway, we can subtract q (mod t) at step 2 to get the same result.
// Subtracting q (mod t) instead ensures that the quantity at step 2
// does not become negative, which is convenient because we are using
// an unsigned integer type.
//
// Concluding the example from before with the fix:
//
//    m +  e * t (mod q) - q (mod t) =
//    1 + -1 * 2 (mod 7) - 7 (mod 2) =
//            -1 (mod 7) - 7 (mod 2) = 6 - 1 = 5
//
// 5 (mod t) = 1, which is the original message.
template <typename ModularInt>
std::vector<typename ModularInt::Int> RemoveError(
    const std::vector<ModularInt>& error_and_message,
    const typename ModularInt::Int& q, const typename ModularInt::Int& t,
    const typename ModularInt::Params* modulus_params_q) {
  using Int = typename ModularInt::Int;
  Int q_mod_t = q % t;
  Int zero = modulus_params_q->Zero();
  std::vector<Int> plaintext(error_and_message.size(), zero);

  for (int i = 0; i < error_and_message.size(); i++) {
    plaintext[i] = error_and_message[i].ExportInt(modulus_params_q);

    if (plaintext[i] > (q >> 1)) {
      plaintext[i] = plaintext[i] - q_mod_t;
    }

    plaintext[i] = plaintext[i] % t;
  }

  return plaintext;
}

template <typename ModularInt>
rlwe::StatusOr<std::vector<typename ModularInt::Int>> Decrypt(
    const SymmetricRlweKey<ModularInt>& key,
    const SymmetricRlweCiphertext<ModularInt>& ciphertext) {
  // Extract the error and message. To do so, take the dot product of the
  // ciphertext vector <c0, c1, ..., cN> and the vector of the powers of
  // the key <s^0, s^1, ..., s^N>.

  // Accumulator variables.
  Polynomial<ModularInt> error_and_message_ntt(key.Len(), key.ModulusParams());
  Polynomial<ModularInt> key_powers = key.Key();
  unsigned int ciphertext_len = ciphertext.Len();

  for (unsigned int i = 0; i < ciphertext_len; i++) {
    // Extract component i.
    RLWE_ASSIGN_OR_RETURN(Polynomial<ModularInt> ci, ciphertext.Component(i));

    // Lazily increase the exponent of the key.
    if (i > 1) {
      RLWE_RETURN_IF_ERROR(
          key_powers.MulInPlace(key.Key(), key.ModulusParams()));
    }

    // Beyond c0, multiply the exponentiated key in.
    if (i > 0) {
      RLWE_RETURN_IF_ERROR(
          ci.MulInPlace(key_powers, ciphertext.ModulusParams()));
    }

    RLWE_RETURN_IF_ERROR(
        error_and_message_ntt.AddInPlace(ci, key.ModulusParams()));
  }

  // Invert the NTT process.
  std::vector<ModularInt> error_and_message =
      error_and_message_ntt.InverseNtt(key.NttParams(), key.ModulusParams());

  // Extract the message.
  return RemoveError<ModularInt>(
      error_and_message, key.ModulusParams()->modulus,
      key.PlaintextModulus().ExportInt(key.PlaintextModulusParams()),
      key.ModulusParams());
}

}  // namespace rlwe

#endif  // RLWE_SYMMETRIC_ENCRYPTION_H_
