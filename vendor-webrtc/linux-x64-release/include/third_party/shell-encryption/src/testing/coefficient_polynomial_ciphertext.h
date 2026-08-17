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

// The class defined in this file should only be used for testing purposes.

#ifndef RLWE_TESTING_COEFFICIENT_POLYNOMIAL_CIPHERTEXT_H_
#define RLWE_TESTING_COEFFICIENT_POLYNOMIAL_CIPHERTEXT_H_

#include <cstdint>
#include <vector>

#include "error_params.h"
#include "polynomial.h"
#include "statusor.h"
#include "symmetric_encryption.h"
#include "testing/coefficient_polynomial.h"

namespace rlwe {
namespace testing {

// Container for a vector of polynomials in coefficient representation. This
// class is the analogue of SymmetricRlweCiphertext with component polynomials
// in coefficient representation instead of in NTT form. It only provides a
// subset of the homomorphic functionality of SymmetricRlweCiphertext, namely
// homomorphic addition, multiplication by monomials, and substitution.
//
// Homomorphic operations between ciphertexts encrypted under different
// powers_of_s fail.
//
// Should only be used for testing.
template <typename ModularInt>
class CoefficientPolynomialCiphertext {
 public:
  CoefficientPolynomialCiphertext(const CoefficientPolynomialCiphertext& that) =
      default;

  // Creates a coefficient polynomial ciphertext out of a vector of polynomials.
  // Fails if an empty vector is provided.
  explicit CoefficientPolynomialCiphertext(
      std::vector<CoefficientPolynomial<ModularInt>> p, const int power_of_s,
      const ErrorParams<ModularInt>* error_params)
      : p_(std::move(p)),
        modulus_params_(p_[0].ModulusParams()),
        error_params_(error_params), 
        power_of_s_(power_of_s){}

  // Static method that creates a CoefficientPolynomialCiphertext from a
  // SymmetricRlweCiphertext by performing InverseNtt on each ciphertext
  // component.
  static rlwe::StatusOr<CoefficientPolynomialCiphertext> ConvertToCoefficients(
      const SymmetricRlweCiphertext<ModularInt>& ntt_ciphertext,
      const NttParameters<ModularInt>* ntt_params) {
    std::vector<CoefficientPolynomial<ModularInt>> result;

    for (int i = 0; i < ntt_ciphertext.Len(); i++) {
      RLWE_ASSIGN_OR_RETURN(auto comp, ntt_ciphertext.Component(i));
      result.push_back(CoefficientPolynomial<ModularInt>(
          comp.InverseNtt(ntt_params, ntt_ciphertext.ModulusParams()),
          ntt_ciphertext.ModulusParams()));
    }

    return CoefficientPolynomialCiphertext(std::move(result),
                                           ntt_ciphertext.PowerOfS(),
                                           ntt_ciphertext.ErrorParams());
  }

  // Converts the polynomial to a SymmetricRlweCiphertext in NTT representation.
  SymmetricRlweCiphertext<ModularInt> ConvertToNtt(
      const NttParameters<ModularInt>* ntt_params) {
    std::vector<Polynomial<ModularInt>> c;

    for (const CoefficientPolynomial<ModularInt>& p : p_) {
      c.push_back(Polynomial<ModularInt>::ConvertToNtt(p.Coeffs(), ntt_params,
                                                       modulus_params_));
    }

    return SymmetricRlweCiphertext<ModularInt>(
        std::move(c), power_of_s_, error_, modulus_params_, error_params_);
  }

  // Add two CoefficientPolynomialCiphertexts.
  rlwe::StatusOr<CoefficientPolynomialCiphertext> operator+(
      const CoefficientPolynomialCiphertext& that) const {
    // Ensure that the polynomials' power_of_s matches.
    if (power_of_s_ != that.power_of_s_) {
      return absl::InvalidArgumentError(
          "Ciphertexts must be encrypted with the same key power.");
    }

    const CoefficientPolynomialCiphertext* longer = this;
    const CoefficientPolynomialCiphertext* shorter = &that;
    if (p_.size() < that.p_.size()) {
      std::swap(longer, shorter);
    }

    std::vector<CoefficientPolynomial<ModularInt>> result(longer->p_);

    for (int i = 0; i < shorter->p_.size(); i++) {
      RLWE_ASSIGN_OR_RETURN(result[i], result[i] + shorter->p_[i]);
    }

    return CoefficientPolynomialCiphertext(std::move(result), power_of_s_,
                                           error_params_);
  }

  // Absorb a monomial x^power, where power is less than the dimension of the
  // polynomials.
  rlwe::StatusOr<CoefficientPolynomialCiphertext> MonomialAbsorb(
      const int power) const {
    std::vector<CoefficientPolynomial<ModularInt>> result;

    for (const CoefficientPolynomial<ModularInt>& p : p_) {
      RLWE_ASSIGN_OR_RETURN(auto elt, p.MonomialMultiplication(power));
      result.push_back(elt);
    }

    return CoefficientPolynomialCiphertext(std::move(result), power_of_s_,
                                           error_params_);
  }

  // Given a ciphertext <c0(x), c1(x), ... cn(x)>, returns a
  // CoefficientPolynomialCiphertext representing <c0(x^power), ... ,
  // cn(x^power)> which can be decrypted under secret key s(x^power). Updates
  // the power_of_s member.
  rlwe::StatusOr<CoefficientPolynomialCiphertext> Substitute(
      const int substitution_power) const {
    std::vector<CoefficientPolynomial<ModularInt>> result;

    for (const CoefficientPolynomial<ModularInt>& p : p_) {
      RLWE_ASSIGN_OR_RETURN(auto elt, p.Substitute(substitution_power));
      result.push_back(elt);
    }

    return CoefficientPolynomialCiphertext(
        std::move(result),
        (power_of_s_ * substitution_power) % (2 * p_[0].Len()), error_params_);
  }

  // Accessor for power of s.
  int PowerOfS() const { return power_of_s_; }

  // Accessor for the amount of error in the ciphertext.
  double Error() const { return error_; }

 private:
  // The vector of polynomials.
  std::vector<CoefficientPolynomial<ModularInt>> p_;

  // ModularInt parameters.
  const typename ModularInt::Params* modulus_params_;

  // ErrorParams.
  const rlwe::ErrorParams<ModularInt>* error_params_;

  // The power a in s(x^a) that the ciphertext can be decrypted with.
  int power_of_s_;

  // A heuristic on the amount of error in the ciphertext.
  double error_;
};

}  //  namespace testing
}  //  namespace rlwe

#endif  // RLWE_TESTING_COEFFICIENT_POLYNOMIAL_CIPHERTEXT_H_
