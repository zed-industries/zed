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

#ifndef CRYPTO_CONTEXT_H_
#define CRYPTO_CONTEXT_H_

#include <stdint.h>

#include <memory>
#include <string>

#include "third_party/abseil-cpp/absl/strings/string_view.h"
#include "third_party/private-join-and-compute/base/private_join_and_compute_export.h"
#include "third_party/private-join-and-compute/src/chromium_patch.h"
#include "third_party/private-join-and-compute/src/crypto/big_num.h"
#include "third_party/private-join-and-compute/src/crypto/openssl.inc"

#define CRYPTO_CHECK(expr) CHECK(expr) << OpenSSLErrorString();

namespace private_join_and_compute {

std::string OpenSSLErrorString();

// Wrapper around various contexts needed for openssl operations. It holds a
// BN_CTX to be reused when doing BigNum arithmetic operations and an EVP_MD_CTX
// to be reused when doing hashing operations.
//
// This class provides factory methods for creating BigNum objects that take
// advantage of the BN_CTX structure for arithmetic operations.
//
// This class is not thread-safe, so each thread needs to have a unique Context
// initialized.
class PRIVATE_COMPUTE_EXPORT Context {
 public:
  // Deletes a BN_CTX.
  class BnCtxDeleter {
   public:
    void operator()(BN_CTX* ctx) { BN_CTX_free(ctx); }
  };
  typedef std::unique_ptr<BN_CTX, BnCtxDeleter> BnCtxPtr;

  // Deletes an EVP_MD_CTX.
  class EvpMdCtxDeleter {
   public:
    void operator()(EVP_MD_CTX* ctx) { EVP_MD_CTX_destroy(ctx); }
  };
  typedef std::unique_ptr<EVP_MD_CTX, EvpMdCtxDeleter> EvpMdCtxPtr;

  Context();

  // Context is neither copyable nor movable.
  Context(const Context&) = delete;
  Context& operator=(const Context&) = delete;

  virtual ~Context();

  // Returns a pointer to the openssl BN_CTX that can be reused for arithmetic
  // operations.
  BN_CTX* GetBnCtx();

  // Creates a BigNum initialized with the given BIGNUM value.
  BigNum CreateBigNum(BigNum::BignumPtr bn);

  // Creates a BigNum initialized with the given bytes string.
  BigNum CreateBigNum(absl::string_view bytes);

  // Creates a BigNum initialized with the given number.
  BigNum CreateBigNum(uint64_t number);

  // Hashes a string using SHA-256 to a byte string.
  virtual std::string Sha256String(absl::string_view bytes);

  // Hashes a string using SHA-384 to a byte string.
  virtual std::string Sha384String(absl::string_view bytes);

  // Hashes a string using SHA-512 to a byte string.
  virtual std::string Sha512String(absl::string_view bytes);

  // A random oracle function mapping x deterministically into a large domain.
  //
  // The random oracle is similar to the example given in the last paragraph of
  // Chapter 6 of [1] where the output is expanded by successively hashing the
  // concatenation of the input with a fixed sized counter starting from 1.
  //
  // [1] Bellare, Mihir, and Phillip Rogaway. "Random oracles are practical:
  // A paradigm for designing efficient protocols." Proceedings of the 1st ACM
  // conference on Computer and communications security. ACM, 1993.
  //
  // Returns a long value from the set [0, max_value).
  //
  // Check Error: if bit length of max_value is greater than 130048.
  // Since the counter used for expanding the output is expanded to 8 bit length
  // (hard-coded), any counter value that is greater than 256 would cause
  // variable length inputs passed to the underlying sha256/sha512 calls and
  // might make this random oracle's output not uniform across the output
  // domain.
  //
  // The output length is increased by a security value of 256/512 which reduces
  // the bias of selecting certain values more often than others when max_value
  // is not a multiple of 2.
  virtual BigNum RandomOracleSha256(absl::string_view x,
                                    const BigNum& max_value);
  virtual BigNum RandomOracleSha384(absl::string_view x,
                                    const BigNum& max_value);
  virtual BigNum RandomOracleSha512(absl::string_view x,
                                    const BigNum& max_value);

  // Evaluates a PRF keyed by 'key' on the given data. The returned value is
  // less than max_value.
  //
  // The maximum supported output length is 512. Causes a check failure if the
  // bit length of max_value is > 512.
  //
  // Security:
  //  The security of this function is given by the length of the key. The key
  //  should be at least 80 bits long which gives 80 bit security. Fails if the
  //  key is less than 80 bits.
  //
  //  This function is susceptible to timing attacks.
  BigNum PRF(absl::string_view key,
             absl::string_view data,
             const BigNum& max_value);

  // Creates a safe prime BigNum with the given bit-length.
  BigNum GenerateSafePrime(int prime_length);

  // Creates a prime BigNum with the given bit-length.
  //
  // Note: In many cases, we need to use a safe prime for cryptographic security
  // to hold. In this case, we should use GenerateSafePrime.
  BigNum GeneratePrime(int prime_length);

  // Generates a cryptographically strong pseudo-random in the range [0,
  // max_value).
  // Marked virtual for tests.
  virtual BigNum GenerateRandLessThan(const BigNum& max_value);

  // Generates a cryptographically strong pseudo-random in the range [start,
  // end).
  // Marked virtual for tests.
  virtual BigNum GenerateRandBetween(const BigNum& start, const BigNum& end);

  // Generates a cryptographically strong pseudo-random bytes of the specified
  // length.
  // Marked virtual for tests.
  virtual std::string GenerateRandomBytes(int num_bytes);

  // Returns a BigNum that is relatively prime to the num and less than the num.
  virtual BigNum RelativelyPrimeRandomLessThan(const BigNum& num);

  inline const BigNum& Zero() const { return zero_bn_; }
  inline const BigNum& One() const { return one_bn_; }
  inline const BigNum& Two() const { return two_bn_; }
  inline const BigNum& Three() const { return three_bn_; }

 private:
  BnCtxPtr bn_ctx_;
  EvpMdCtxPtr evp_md_ctx_;
  HMAC_CTX hmac_ctx_;
  const BigNum zero_bn_;
  const BigNum one_bn_;
  const BigNum two_bn_;
  const BigNum three_bn_;

  enum RandomOracleHashType {
    SHA256,
    SHA384,
    SHA512,
  };

  // If hash_type is invalid, this function will default to using SHA256.
  virtual BigNum RandomOracle(absl::string_view x,
                              const BigNum& max_value,
                              RandomOracleHashType hash_type);
};

}  // namespace private_join_and_compute

#endif  // CRYPTO_CONTEXT_H_
