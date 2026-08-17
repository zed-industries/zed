// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_MODULES_V8_SERIALIZATION_WEB_CRYPTO_SUB_TAGS_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_MODULES_V8_SERIALIZATION_WEB_CRYPTO_SUB_TAGS_H_

#include <cstdint>

namespace blink {

enum CryptoKeyAlgorithmTag : uint32_t {
  kAesCbcTag = 1,
  kHmacTag = 2,
  kRsaSsaPkcs1v1_5Tag = 3,
  // ID 4 was used by RsaEs, while still behind experimental flag.
  kSha1Tag = 5,
  kSha256Tag = 6,
  kSha384Tag = 7,
  kSha512Tag = 8,
  kAesGcmTag = 9,
  kRsaOaepTag = 10,
  kAesCtrTag = 11,
  kAesKwTag = 12,
  kRsaPssTag = 13,
  kEcdsaTag = 14,
  kEcdhTag = 15,
  kHkdfTag = 16,
  kPbkdf2Tag = 17,
  kEd25519Tag = 18,
  kX25519Tag = 19,
  // Maximum allowed value is 2^32-1
};

enum NamedCurveTag : uint32_t {
  kP256Tag = 1,
  kP384Tag = 2,
  kP521Tag = 3,
  // Maximum allowed value is 2^32-1
};

enum CryptoKeyUsage : uint32_t {
  // Extractability is not a "usage" in the WebCryptoKeyUsages sense, however
  // it fits conveniently into this bitfield.
  kExtractableUsage = 1 << 0,

  kEncryptUsage = 1 << 1,
  kDecryptUsage = 1 << 2,
  kSignUsage = 1 << 3,
  kVerifyUsage = 1 << 4,
  kDeriveKeyUsage = 1 << 5,
  kWrapKeyUsage = 1 << 6,
  kUnwrapKeyUsage = 1 << 7,
  kDeriveBitsUsage = 1 << 8,
  // Maximum allowed value is 1 << 31
};

enum CryptoKeySubTag : uint8_t {
  kAesKeyTag = 1,
  kHmacKeyTag = 2,
  // ID 3 was used by RsaKeyTag, while still behind experimental flag.
  kRsaHashedKeyTag = 4,
  kEcKeyTag = 5,
  kNoParamsKeyTag = 6,
  kEd25519KeyTag = 7,
  kX25519KeyTag = 8,
  // Maximum allowed value is 255
};

enum AsymmetricCryptoKeyType : uint32_t {
  kPublicKeyType = 1,
  kPrivateKeyType = 2,
  // Maximum allowed value is 2^32-1
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_MODULES_V8_SERIALIZATION_WEB_CRYPTO_SUB_TAGS_H_
