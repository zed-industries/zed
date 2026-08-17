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

// Implementation of the ElGamal encryption scheme, over an Elliptic Curve.
//
// ElGamal is a multiplicatively homomorphic encryption scheme. See [1] for
// more information.
//
// The function elgamal::GenerateKeyPair generates a fresh public-private key
// pair for the scheme. Using these keys, one can instantiate ElGamalEncrypter
// and ElGamalDecrypter objects, which allow encrypting and decrypting
// messages that lie on the elliptic curve.
//
// The function elgamal::Mul allows homomorphic multiplication of two
// ciphertexts. The function elgamal::Exp allows homomorphic exponentiation of
// a ciphertext by a scalar.
// (Note: these operations actually correspond to addition and multiplication
// in the underlying EC group, but we refer to them as multiplication and
// exponentiation to match the standard description of ElGamal as
// multiplicatively homomorphic.)
//
// [1] https://en.wikipedia.org/wiki/ElGamal_encryption

#ifndef CRYPTO_ELGAMAL_H_
#define CRYPTO_ELGAMAL_H_

#include <memory>
#include <utility>
#include <vector>

#include "third_party/private-join-and-compute/src/crypto/ec_group.h"
#include "third_party/private-join-and-compute/src/crypto/ec_point.h"
#include "third_party/private-join-and-compute/src/util/status.inc"

namespace private_join_and_compute {

class BigNum;
class ECPoint;
class ECGroup;

// Containers and utility functions
namespace elgamal {

struct Ciphertext {
  // Encryption of an ECPoint m using randomness r, under public key (g,y).
  ECPoint u;  // = g^r
  ECPoint e;  // = m * y^r
};

struct PublicKey {
  ECPoint g;
  ECPoint y;  // = g^x, where x is the secret key.
};

struct PrivateKey {
  BigNum x;
};

// Generates a new ElGamal public-private key pair.
StatusOr<std::pair<std::unique_ptr<PublicKey>, std::unique_ptr<PrivateKey>>>
GenerateKeyPair(const ECGroup& ec_group);

// Joins the public key shares in a public key. The shares should be nonempty.
StatusOr<std::unique_ptr<PublicKey>> GeneratePublicKeyFromShares(
    const std::vector<std::unique_ptr<elgamal::PublicKey>>& shares);

// Homomorphically multiply two ciphertexts.
// (Note: this corresponds to addition in the EC group.)
StatusOr<elgamal::Ciphertext> Mul(const elgamal::Ciphertext& ciphertext1,
                                  const elgamal::Ciphertext& ciphertext2);

// Homomorphically exponentiate a ciphertext by a scalar.
// (Note: this corresponds to multiplication in the EC group.)
StatusOr<elgamal::Ciphertext> Exp(const elgamal::Ciphertext& ciphertext,
                                  const BigNum& scalar);

// Returns a ciphertext encrypting the point at infinity, using fixed randomness
// "0". This is a multiplicative identity for ElGamal ciphertexts.
StatusOr<Ciphertext> GetZero(const ECGroup* group);

// A convenience function that creates a copy of this ciphertext with the same
// randomness and underlying message.
StatusOr<Ciphertext> CloneCiphertext(const Ciphertext& ciphertext);

// Checks if the given ciphertext is an encryption of the point of infinity
// using randomness "0".
bool IsCiphertextZero(const Ciphertext& ciphertext);

}  // namespace elgamal

// Implements ElGamal encryption with a public key.
class PRIVATE_COMPUTE_EXPORT ElGamalEncrypter {
 public:
  // Creates a ElGamalEncrypter object from a given public key.
  // Takes ownership of the public key.
  ElGamalEncrypter(const ECGroup* ec_group,
                   std::unique_ptr<elgamal::PublicKey> elgamal_public_key);

  // ElGamalEncrypter cannot be copied or assigned
  ElGamalEncrypter(const ElGamalEncrypter&) = delete;
  ElGamalEncrypter operator=(const ElGamalEncrypter&) = delete;

  ~ElGamalEncrypter() = default;

  // Encrypts a message m, that has already been mapped onto the curve.
  StatusOr<elgamal::Ciphertext> Encrypt(const ECPoint& message) const;

  // Re-randomizes a ciphertext. After the re-randomization, the new ciphertext
  // is an encryption of the same message as before.
  StatusOr<elgamal::Ciphertext> ReRandomize(
      const elgamal::Ciphertext& elgamal_ciphertext) const;

  // Returns a pointer to the owned ElGamal public key
  const elgamal::PublicKey* getPublicKey() const { return public_key_.get(); }

 private:
  const ECGroup* ec_group_;  // not owned
  std::unique_ptr<elgamal::PublicKey> public_key_;
};

// Implements ElGamal decryption using the private key.
class PRIVATE_COMPUTE_EXPORT ElGamalDecrypter {
 public:
  // Creates a ElGamalDecrypter object from a given private key.
  // Takes ownership of the private key.
  explicit ElGamalDecrypter(
      std::unique_ptr<elgamal::PrivateKey> elgamal_private_key);

  // ElGamalDecrypter cannot be copied or assigned
  ElGamalDecrypter(const ElGamalDecrypter&) = delete;
  ElGamalDecrypter operator=(const ElGamalDecrypter&) = delete;

  ~ElGamalDecrypter() = default;

  // Decrypts a given ElGamal ciphertext.
  StatusOr<ECPoint> Decrypt(const elgamal::Ciphertext& ciphertext) const;

  // Partially decrypts a given ElGamal ciphertext with a share of the secret
  // key. The caller should rerandomize the ciphertext using the remaining
  // partial public keys.
  StatusOr<elgamal::Ciphertext> PartialDecrypt(
      const elgamal::Ciphertext& ciphertext) const;

  // Returns a pointer to the owned ElGamal private key
  const elgamal::PrivateKey* getPrivateKey() const {
    return private_key_.get();
  }

 private:
  std::unique_ptr<elgamal::PrivateKey> private_key_;
};

}  // namespace private_join_and_compute

#endif  // CRYPTO_ELGAMAL_H_
