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

#ifndef CRYPTO_EC_GROUP_H_
#define CRYPTO_EC_GROUP_H_

#include <cstddef>
#include <memory>
#include <string>

#include "third_party/abseil-cpp/absl/strings/string_view.h"
#include "third_party/private-join-and-compute/base/private_join_and_compute_export.h"
#include "third_party/private-join-and-compute/src/crypto/big_num.h"
#include "third_party/private-join-and-compute/src/crypto/context.h"
#include "third_party/private-join-and-compute/src/crypto/openssl.inc"
#include "third_party/private-join-and-compute/src/util/status.inc"

namespace private_join_and_compute {

class ECPoint;

// Wrapper class for openssl EC_GROUP.
class PRIVATE_COMPUTE_EXPORT ECGroup {
 public:
  // Deletes a EC_GROUP.
  class ECGroupDeleter {
   public:
    void operator()(EC_GROUP* group) { EC_GROUP_free(group); }
  };
  typedef std::unique_ptr<EC_GROUP, ECGroupDeleter> ECGroupPtr;

  // Constructs a new ECGroup object for the given named curve id.
  // See openssl header obj_mac.h for the available built-in curves.
  // Use a well-known prime curve such as NID_X9_62_prime256v1 recommended by
  // NIST. Returns INTERNAL error code if there is a failure in crypto
  // operations. Security: this function is secure only for prime order curves.
  // (All supported curves in BoringSSL have prime order.)
  static StatusOr<ECGroup> Create(int curve_id, Context* context);

  // Generates a new private key. The private key is a cryptographically strong
  // pseudo-random number in the range (0, order).
  BigNum GeneratePrivateKey() const;

  // Verifies that the random key is a valid number in the range (0, order).
  // Returns Status::OK if the key is valid, otherwise returns INVALID_ARGUMENT.
  Status CheckPrivateKey(const BigNum& priv_key) const;

  // Hashes m to a point on the elliptic curve y^2 = x^3 + ax + b over a
  // prime field using SHA256 with "try-and-increment" method.
  // See https://crypto.stanford.edu/~dabo/papers/bfibe.pdf, Section 5.2.
  // Returns an INVALID_ARGUMENT error code if an error occurs.
  //
  // Security: The number of operations required to hash a string depends on the
  // string, which could lead to a timing attack.
  // Security: This function is only secure for curves of prime order.
  StatusOr<ECPoint> GetPointByHashingToCurveSha256(absl::string_view m) const;
  StatusOr<ECPoint> GetPointByHashingToCurveSha384(absl::string_view m) const;
  StatusOr<ECPoint> GetPointByHashingToCurveSha512(absl::string_view m) const;
  StatusOr<ECPoint> GetPointByHashingToCurveSswuRo(absl::string_view m,
                                                   absl::string_view dst) const;

  // Pad the given message until it is a valid point on the curve. Padding is
  // achieved by left-shifting m by padding_bits_count bits, and appending
  // successive integers till a curve point is found.
  StatusOr<ECPoint> GetPointByPaddingX(const BigNum& m,
                                       size_t padding_bit_count) const;

  // Recover the original message from a point that was derived by message
  // padding using GetPointByPaddingX.
  StatusOr<BigNum> RecoverXFromPaddedPoint(const ECPoint& point,
                                           size_t padding_bit_count) const;

  // Returns y^2 for the given x. The returned value is computed as x^3 + ax + b
  // mod p, where a and b are the parameters of the curve.
  BigNum ComputeYSquare(const BigNum& x) const;

  // Returns a fixed generator for this group.
  // Returns an INTERNAL error code if it fails.
  StatusOr<ECPoint> GetFixedGenerator() const;

  // Returns a random generator for this group.
  // Returns an INTERNAL error code if it fails.
  StatusOr<ECPoint> GetRandomGenerator() const;

  // Creates an ECPoint from the given string.
  // Returns an INTERNAL error code if creating the point fails.
  // Returns an INVALID_ARGUMENT error code if the created point is not in this
  // group or if it is the point at infinity.
  StatusOr<ECPoint> CreateECPoint(absl::string_view bytes) const;

  // Creates an ECPoint object with the given x, y affine coordinates.
  // Returns an INVALID_ARGUMENT error code if the point (x, y) is not in this
  // group or if it is the point at infinity.
  StatusOr<ECPoint> CreateECPoint(const BigNum& x, const BigNum& y) const;

  // The parameters describing an elliptic curve given by the equation
  // y^2 = x^3 + a * x + b over a prime field Fp.
  struct CurveParams {
    BigNum p;
    BigNum a;
    BigNum b;
  };

  // Returns the order.
  const BigNum& GetOrder() const { return order_; }

  // Returns the cofactor.
  const BigNum& GetCofactor() const { return cofactor_; }

  // Returns the curve id.
  int GetCurveId() const { return EC_GROUP_get_curve_name(group_.get()); }

  // Creates an ECPoint which is the identity.
  StatusOr<ECPoint> GetPointAtInfinity() const;

 private:
  ECGroup(Context* context, ECGroupPtr group, BigNum order, BigNum cofactor,
          CurveParams curve_params, BigNum p_minus_one_over_two);

  // Returns true if q is a quadratic residue modulo curve_params_.p_.
  bool IsSquare(const BigNum& q) const;

  // Checks if the given point is valid. Returns false if the point is not in
  // the group or if it is the point is at infinity.
  bool IsValid(const ECPoint& point) const;

  // Returns true if the given point is in the group.
  bool IsOnCurve(const ECPoint& point) const;

  // Returns true if the given point is at infinity.
  bool IsAtInfinity(const ECPoint& point) const;

  Context* context_;
  ECGroupPtr group_;
  // The order of this group.
  BigNum order_;
  // The cofactor of this group.
  BigNum cofactor_;
  // The parameters of the curve. These values are used to hash a number to a
  // point on the curve.
  CurveParams curve_params_;
  // Constant used to evaluate if a number is a quadratic residue.
  BigNum p_minus_one_over_two_;

  StatusOr<ECPoint> GetPointByHashingToCurveInternal(const BigNum& x) const;
};

}  // namespace private_join_and_compute

#endif  // CRYPTO_EC_GROUP_H_
