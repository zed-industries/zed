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

// The class defined in this file should only be used for testing purposes.

#ifndef RLWE_TESTING_COEFFICIENT_POLYNOMIAL_H_
#define RLWE_TESTING_COEFFICIENT_POLYNOMIAL_H_

#include <cmath>
#include <cstdint>
#include <string>
#include <vector>

#include <glog/logging.h>
#include "absl/strings/str_cat.h"
#include "status_macros.h"
#include "statusor.h"
#include "testing/coefficient_polynomial.pb.h"

namespace rlwe {
namespace testing {

// A polynomial with ModularInt coefficients that is automatically reduced
// modulo <x^n + 1>, where n is the number of coefficients provided in the
// constructor.
// SHould only be used for testing.
template <typename ModularInt>
class CoefficientPolynomial {
  using ModularIntParams = typename ModularInt::Params;

 public:
  // Copy constructor.
  CoefficientPolynomial(const CoefficientPolynomial& that) = default;

  // Constructor. The polynomial is initialized to the values of a vector.
  CoefficientPolynomial(std::vector<ModularInt> coeffs,
                        const ModularIntParams* modulus_params)
      : coeffs_(std::move(coeffs)), modulus_params_(modulus_params) {}

  // Constructs an empty CoefficientPolynomial.
  explicit CoefficientPolynomial(int len,
                                 const ModularIntParams* modulus_params)
      : CoefficientPolynomial(std::vector<ModularInt>(
                                  len, ModularInt::ImportZero(modulus_params)),
                              modulus_params) {}

  // Accessor for length.
  int Len() const { return coeffs_.size(); }

  // Accessor for coefficients.
  std::vector<ModularInt> Coeffs() const { return coeffs_; }

  // Accessor for Modulus Params.
  const ModularIntParams* ModulusParams() const { return modulus_params_; }

  // Compute the degree.
  int Degree() const {
    for (int i = Len() - 1; i >= 0; i--) {
      if (coeffs_[i].ExportInt(modulus_params_) != 0) {
        return i;
      }
    }

    return 0;
  }

  // Equality.
  bool operator==(const CoefficientPolynomial& that) const {
    if (Degree() != that.Degree()) {
      return false;
    }

    for (int i = 0; i <= Degree(); i++) {
      if (coeffs_[i] != that.coeffs_[i]) {
        return false;
      }
    }

    return true;
  }

  bool operator!=(const CoefficientPolynomial& that) const {
    return !(*this == that);
  }

  // Addition.
  rlwe::StatusOr<CoefficientPolynomial> operator+(
      const CoefficientPolynomial& that) const {
    // Ensure the polynomials' dimensions are equal.
    if (Len() != that.Len()) {
      return absl::InvalidArgumentError(
          "CoefficientPolynomial dimensions mismatched.");
    }

    // Add polynomials point-wise.
    CoefficientPolynomial out(*this);
    for (int i = 0; i < Len(); i++) {
      out.coeffs_[i].AddInPlace(that.coeffs_[i], modulus_params_);
    }

    return out;
  }

  // Substraction.
  rlwe::StatusOr<CoefficientPolynomial> operator-(
      const CoefficientPolynomial& that) const {
    // Ensure the polynomials' dimensions are equal.
    if (Len() != that.Len()) {
      return absl::InvalidArgumentError(
          "CoefficientPolynomial dimensions mismatched.");
    }

    // Add polynomials point-wise.
    CoefficientPolynomial out(*this);
    for (int i = 0; i < Len(); i++) {
      out.coeffs_[i].SubInPlace(that.coeffs_[i], modulus_params_);
    }

    return out;
  }

  // Scalar multiplication.
  CoefficientPolynomial operator*(ModularInt c) const {
    CoefficientPolynomial out(*this);
    for (auto& coeff : out.coeffs_) {
      coeff.MulInPlace(c, modulus_params_);
    }
    return out;
  }

  // Multiplication modulo x^N + 1.
  rlwe::StatusOr<CoefficientPolynomial> operator*(
      const CoefficientPolynomial& that) const {
    // Ensure the polynomials' dimensions are equal.
    if (Len() != that.Len()) {
      return absl::InvalidArgumentError(
          "CoefficientPolynomial dimensions mismatched.");
    }

    // Create a zero polynomial of the correct dimension.
    CoefficientPolynomial out(Len(), modulus_params_);

    for (int i = 0; i < Len(); i++) {
      for (int j = 0; j < Len(); j++) {
        if ((i + j) >= coeffs_.size()) {
          // Since multiplciation is mod (x^N + 1), if the coefficient computed
          // has degree k (= i + j) larger than N, it contributes to the (k -
          // N)'th coefficient with a negative factor.
          out.coeffs_[(i + j) - coeffs_.size()].SubInPlace(
              coeffs_[i].Mul(that.coeffs_[j], modulus_params_),
              modulus_params_);
        } else {
          // Otherwise, contributes to the k'th coefficient as  normal.
          out.coeffs_[i + j].AddInPlace(
              coeffs_[i].Mul(that.coeffs_[j], modulus_params_),
              modulus_params_);
        }
      }
    }
    return out;
  }

  // A more efficient multiplication by a monomial x^power, where power <
  // 2*dimension.
  rlwe::StatusOr<CoefficientPolynomial> MonomialMultiplication(
      int power) const {
    // Check that the monomial is in range.
    if (0 > power || power >= 2 * Len()) {
      return absl::InvalidArgumentError(
          "Monomial to absorb must have non-negative degree less than 2n.");
    }

    CoefficientPolynomial out(*this);

    // Monomial multiplication be x^{k} where n <= k < 2*n is monomial
    // multiplication by -x^{k - n}.
    ModularInt multiplier = ModularInt::ImportOne(modulus_params_);
    if (power >= Len()) {
      multiplier.NegateInPlace(modulus_params_);
      power = power - Len();
    }
    ModularInt negative_multiplier = multiplier.Negate(modulus_params_);

    for (int i = 0; i < power; i++) {
      out.coeffs_[i] =
          negative_multiplier.Mul(coeffs_[i - power + Len()], modulus_params_);
    }
    for (int i = power; i < Len(); i++) {
      out.coeffs_[i] = multiplier.Mul(coeffs_[i - power], modulus_params_);
    }

    return out;
  }

  // Given a polynomial p(x), returns a polynomial p(x^a). Expects a power <
  // 2n, where n is the dimension of the polynomial.
  rlwe::StatusOr<CoefficientPolynomial> Substitute(const int power) const {
    // Check that the substitution is in range. The power must be relatively
    // prime to 2*n. Since our dimensions are always a power of two, this is
    // equivalent to the power being odd.
    if (0 > power || (power % 2) == 0 || power >= 2 * Len()) {
      return absl::InvalidArgumentError(
          absl::StrCat("Substitution power must be a non-negative odd ",
                       "integer less than 2*n."));
    }
    CoefficientPolynomial out(*this);

    // The ith coefficient of the original polynomial p(x) is sent to the (i *
    // power % Len())-th coefficient under the substitution. However, in the
    // polynomial ring mod (x^N + 1), x^N = -1, so we multiply the i-th
    // coefficient by (-1)^{(power * i) / Len()}.
    // In the loop, current_index keeps track of (i * power % Len()), and
    // multiplier keeps track of the power of -1 for the current coefficient.
    int current_index = 0;
    ModularInt multiplier = ModularInt::ImportOne(modulus_params_);
    for (int i = 0; i < Len(); i++) {
      out.coeffs_[current_index] = coeffs_[i].Mul(multiplier, modulus_params_);
      current_index += power;

      while (current_index > Len()) {
        multiplier.NegateInPlace(modulus_params_);
        current_index -= Len();
      }
    }

    return out;
  }

  rlwe::StatusOr<SerializedCoefficientPolynomial> Serialize() const {
    SerializedCoefficientPolynomial output;
    RLWE_ASSIGN_OR_RETURN(
        *(output.mutable_coeffs()),
        ModularInt::SerializeVector(coeffs_, modulus_params_));
    output.set_num_coeffs(coeffs_.size());

    return output;
  }

  static rlwe::StatusOr<CoefficientPolynomial> Deserialize(
      const SerializedCoefficientPolynomial& serialized,
      const ModularIntParams* modulus_params) {
    CoefficientPolynomial output(serialized.num_coeffs(), modulus_params);
    RLWE_ASSIGN_OR_RETURN(
        output.coeffs_,
        ModularInt::DeserializeVector(serialized.num_coeffs(),
                                      serialized.coeffs(), modulus_params));

    return output;
  }

 private:
  std::vector<ModularInt> coeffs_;
  const ModularIntParams* modulus_params_;
};

}  // namespace testing
}  // namespace rlwe

#endif  // RLWE_TESTING_COEFFICIENT_POLYNOMIAL_H_
