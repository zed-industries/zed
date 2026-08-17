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

#ifndef CRYPTO_EC_POINT_H_
#define CRYPTO_EC_POINT_H_

#include <memory>
#include <string>
#include <utility>

#include "third_party/private-join-and-compute/base/private_join_and_compute_export.h"
#include "third_party/private-join-and-compute/src/crypto/big_num.h"
#include "third_party/private-join-and-compute/src/crypto/openssl.inc"
#include "third_party/private-join-and-compute/src/util/status.inc"

namespace private_join_and_compute {

class ECGroup;

// Wrapper class for openssl EC_POINT.
class PRIVATE_COMPUTE_EXPORT ECPoint {
 public:
  // Deletes an EC_POINT.
  class ECPointDeleter {
   public:
    void operator()(EC_POINT* point) { EC_POINT_clear_free(point); }
  };
  typedef std::unique_ptr<EC_POINT, ECPointDeleter> ECPointPtr;

  // ECPoint is movable.
  ECPoint(ECPoint&& that) = default;
  ECPoint& operator=(ECPoint&& that) = default;

  // ECPoint is not copyable. Use Clone to copy, instead.
  explicit ECPoint(const ECPoint& that) = delete;
  ECPoint& operator=(const ECPoint& that) = delete;

  // Converts this point to octet string in compressed form as defined in ANSI
  // X9.62 ECDSA.
  StatusOr<std::string> ToBytesCompressed() const;

  // Allows faster conversions than ToBytesCompressed but doubles the size of
  // the serialized point.
  StatusOr<std::string> ToBytesUnCompressed() const;

  // Returns an ECPoint whose value is (this * scalar).
  // Returns an INTERNAL error code if it fails.
  StatusOr<ECPoint> Mul(const BigNum& scalar) const;

  // Returns an ECPoint whose value is (this + point).
  // Returns an INTERNAL error code if it fails.
  StatusOr<ECPoint> Add(const ECPoint& point) const;

  // Returns the affine coordinates of this point.
  StatusOr<std::pair<BigNum::BignumPtr, BigNum::BignumPtr>>
  GetAffineCoordinates() const;

  // Returns an ECPoint whose value is (- this), the additive inverse of this.
  // Returns an INTERNAL error code if it fails.
  StatusOr<ECPoint> Inverse() const;

  // Returns "true" if the value of this ECPoint is the point-at-infinity.
  // (The point-at-infinity is the additive unit in the EC group).
  bool IsPointAtInfinity() const;

  // Returns true if this equals point, false otherwise.
  bool CompareTo(const ECPoint& point) const;

  // Returns an ECPoint that is a copy of this.
  StatusOr<ECPoint> Clone() const;

 private:
  // Creates an ECPoint on the given group;
  ECPoint(const EC_GROUP* group, BN_CTX* bn_ctx);

  // Creates an ECPoint on the given group from the given EC_POINT;
  ECPoint(const EC_GROUP* group, BN_CTX* bn_ctx, ECPointPtr point);

  // Creates an ECPoint object with the given x, y affine coordinates.
  ECPoint(const EC_GROUP* group, BN_CTX* bn_ctx, const BigNum& x,
          const BigNum& y);

  BN_CTX* bn_ctx_;
  const EC_GROUP* group_;
  ECPointPtr point_;

  // ECGroup is a factory for ECPoint.
  friend class ECGroup;
};

inline bool operator==(const ECPoint& a, const ECPoint& b) {
  return a.CompareTo(b);
}

inline bool operator!=(const ECPoint& a, const ECPoint& b) { return !(a == b); }

}  // namespace private_join_and_compute

#endif  // CRYPTO_EC_POINT_H_
