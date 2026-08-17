/*
 * Copyright 2021 Google LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef DISTRIBUTED_POINT_FUNCTIONS_DPF_INTERNAL_AES_128_FIXED_KEY_HASH_H_
#define DISTRIBUTED_POINT_FUNCTIONS_DPF_INTERNAL_AES_128_FIXED_KEY_HASH_H_

#include "absl/numeric/int128.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/types/span.h"
#include "openssl/cipher.h"

namespace distributed_point_functions {

// Aes128FixedKeyHash is a circular correlation-robust hash function based on
// AES. For key `key`, input `in` and output `out`, the hash function is defined
// as
//
//     out[i] = AES.Encrypt(key, sigma(in[i])) ^ sigma(in[i]),
//
// where sigma(x) = (x.high64 ^ x.low64, x.high64). This is the
// circular correlation-robust MMO construction from
// https://eprint.iacr.org/2019/074.pdf (pp. 18-19). Note that unlike
// cryptographic hash functions such as SHA-256, this hash function is *not*
// compressing and is not designed to provide any security guarantees beyond
// circular correlation-robustness. Use with appropriate caution.
class Aes128FixedKeyHash {
 public:
  // Creates a new Aes128FixedKeyHash with the given `key`.
  //
  // Returns INTERNAL in case of allocation failures or OpenSSL errors.
  static absl::StatusOr<Aes128FixedKeyHash> Create(absl::uint128 key);

  // Computes hash values of each block in `in`, writing the output to `out`.
  // It is safe to call this method if `in` and `out` overlap.
  //
  // Returns INVALID_ARGUMENT if sizes of `in` and `out` don't match or their
  // sizes in bytes exceed an `int`, or INTERNAL in case of OpenSSL errors.
  absl::Status Evaluate(absl::Span<const absl::uint128> in,
                        absl::Span<absl::uint128> out) const;

  // Aes128FixedKeyHash is not copyable.
  Aes128FixedKeyHash(const Aes128FixedKeyHash&) = delete;
  Aes128FixedKeyHash& operator=(const Aes128FixedKeyHash&) = delete;

  // Aes128FixedKeyHash is movable (it just wraps a bssl::UniquePtr).
  Aes128FixedKeyHash(Aes128FixedKeyHash&&) = default;
  Aes128FixedKeyHash& operator=(Aes128FixedKeyHash&&) = default;

  // Returns the key used to construct this hash function.
  // DO NOT SEND THIS TO ANY OTHER PARTY!
  const absl::uint128& key() const { return key_; }

  // The maximum number of AES blocks encrypted at once. Chosen to pipeline AES
  // as much as possible, while still allowing both source and destination to
  // comfortably fit in the L1 CPU cache.
  static constexpr int kBatchSize = 64;

 private:
  // Called by `Create`.
  Aes128FixedKeyHash(bssl::UniquePtr<EVP_CIPHER_CTX> cipher_ctx,
                     absl::uint128 key);

  // The OpenSSL encryption context used by `Evaluate`.
  bssl::UniquePtr<EVP_CIPHER_CTX> cipher_ctx_;

  // The key used to construct this hash function.
  absl::uint128 key_;
};

}  // namespace distributed_point_functions

#endif  // DISTRIBUTED_POINT_FUNCTIONS_DPF_INTERNAL_AES_128_FIXED_KEY_HASH_H_
