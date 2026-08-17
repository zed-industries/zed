/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_CRYPTO_ALGORITHM_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_CRYPTO_ALGORITHM_H_

#include <array>

#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_private_ptr.h"

#if INSIDE_BLINK
#include <memory>
#endif

namespace blink {

enum WebCryptoOperation {
  kWebCryptoOperationEncrypt,
  kWebCryptoOperationDecrypt,
  kWebCryptoOperationSign,
  kWebCryptoOperationVerify,
  kWebCryptoOperationDigest,
  kWebCryptoOperationGenerateKey,
  kWebCryptoOperationImportKey,
  kWebCryptoOperationGetKeyLength,
  kWebCryptoOperationDeriveBits,
  kWebCryptoOperationWrapKey,
  kWebCryptoOperationUnwrapKey,
  kWebCryptoOperationLast = kWebCryptoOperationUnwrapKey,
};

enum WebCryptoAlgorithmId {
  kWebCryptoAlgorithmIdAesCbc,
  kWebCryptoAlgorithmIdHmac,
  kWebCryptoAlgorithmIdRsaSsaPkcs1v1_5,
  kWebCryptoAlgorithmIdSha1,
  kWebCryptoAlgorithmIdSha256,
  kWebCryptoAlgorithmIdSha384,
  kWebCryptoAlgorithmIdSha512,
  kWebCryptoAlgorithmIdAesGcm,
  kWebCryptoAlgorithmIdRsaOaep,
  kWebCryptoAlgorithmIdAesCtr,
  kWebCryptoAlgorithmIdAesKw,
  kWebCryptoAlgorithmIdRsaPss,
  kWebCryptoAlgorithmIdEcdsa,
  kWebCryptoAlgorithmIdEcdh,
  kWebCryptoAlgorithmIdHkdf,
  kWebCryptoAlgorithmIdPbkdf2,
  kWebCryptoAlgorithmIdEd25519,
  kWebCryptoAlgorithmIdX25519,
  kWebCryptoAlgorithmIdLast = kWebCryptoAlgorithmIdX25519,
};

enum WebCryptoNamedCurve {
  kWebCryptoNamedCurveP256,
  kWebCryptoNamedCurveP384,
  kWebCryptoNamedCurveP521,
  kWebCryptoNamedCurveLast = kWebCryptoNamedCurveP521,
};

enum WebCryptoAlgorithmParamsType {
  kWebCryptoAlgorithmParamsTypeNone,
  kWebCryptoAlgorithmParamsTypeAesCbcParams,
  kWebCryptoAlgorithmParamsTypeAesKeyGenParams,
  kWebCryptoAlgorithmParamsTypeHmacImportParams,
  kWebCryptoAlgorithmParamsTypeHmacKeyGenParams,
  kWebCryptoAlgorithmParamsTypeRsaHashedKeyGenParams,
  kWebCryptoAlgorithmParamsTypeRsaHashedImportParams,
  kWebCryptoAlgorithmParamsTypeAesGcmParams,
  kWebCryptoAlgorithmParamsTypeRsaOaepParams,
  kWebCryptoAlgorithmParamsTypeAesCtrParams,
  kWebCryptoAlgorithmParamsTypeRsaPssParams,
  kWebCryptoAlgorithmParamsTypeEcdsaParams,
  kWebCryptoAlgorithmParamsTypeEcKeyGenParams,
  kWebCryptoAlgorithmParamsTypeEcKeyImportParams,
  kWebCryptoAlgorithmParamsTypeEcdhKeyDeriveParams,
  kWebCryptoAlgorithmParamsTypeAesDerivedKeyParams,
  kWebCryptoAlgorithmParamsTypeHkdfParams,
  kWebCryptoAlgorithmParamsTypePbkdf2Params,
};

struct WebCryptoAlgorithmInfo {
  typedef char ParamsTypeOrUndefined;
  static const ParamsTypeOrUndefined kUndefined = -1;

  // The canonical (case-sensitive) name for the algorithm as a
  // null-terminated C-string literal.
  const char* name;

  // A map from the operation to the expected parameter type of the algorithm.
  // If an operation is not applicable for the algorithm, set to Undefined.
  const std::array<ParamsTypeOrUndefined, kWebCryptoOperationLast + 1>
      operation_to_params_type;
};

class WebCryptoAesCbcParams;
class WebCryptoAesKeyGenParams;
class WebCryptoHmacImportParams;
class WebCryptoHmacKeyGenParams;
class WebCryptoAesGcmParams;
class WebCryptoRsaOaepParams;
class WebCryptoAesCtrParams;
class WebCryptoRsaHashedKeyGenParams;
class WebCryptoRsaHashedImportParams;
class WebCryptoRsaPssParams;
class WebCryptoEcdsaParams;
class WebCryptoEcKeyGenParams;
class WebCryptoEcKeyImportParams;
class WebCryptoEcdhKeyDeriveParams;
class WebCryptoAesDerivedKeyParams;
class WebCryptoHkdfParams;
class WebCryptoPbkdf2Params;

class WebCryptoAlgorithmParams;
class WebCryptoAlgorithmPrivate;

// The WebCryptoAlgorithm represents a normalized algorithm and its parameters.
//   * Immutable
//   * Threadsafe
//   * Copiable (cheaply)
//
// If WebCryptoAlgorithm "IsNull()" then it is invalid to call any of the other
// methods on it (other than destruction, assignment, or IsNull()).
class BLINK_PLATFORM_EXPORT WebCryptoAlgorithm {
 public:
  WebCryptoAlgorithm() = default;
  WebCryptoAlgorithm(WebCryptoAlgorithmId,
                     std::unique_ptr<WebCryptoAlgorithmParams>);

  static WebCryptoAlgorithm CreateNull();
  static WebCryptoAlgorithm AdoptParamsAndCreate(WebCryptoAlgorithmId,
                                                 WebCryptoAlgorithmParams*);

  // Returns a WebCryptoAlgorithmInfo for the algorithm with the given ID. If
  // the ID is invalid, return 0. The caller can assume the pointer will be
  // valid for the program's entire runtime.
  static const WebCryptoAlgorithmInfo* LookupAlgorithmInfo(
      WebCryptoAlgorithmId);

  ~WebCryptoAlgorithm() { Reset(); }

  WebCryptoAlgorithm(const WebCryptoAlgorithm& other) { Assign(other); }
  WebCryptoAlgorithm& operator=(const WebCryptoAlgorithm& other) {
    Assign(other);
    return *this;
  }

  bool IsNull() const;

  WebCryptoAlgorithmId Id() const;

  WebCryptoAlgorithmParamsType ParamsType() const;

  // Retrieves the type-specific parameters. The algorithm contains at most 1
  // type of parameters. Retrieving an invalid parameter will return 0.
  const WebCryptoAesCbcParams* AesCbcParams() const;
  const WebCryptoAesKeyGenParams* AesKeyGenParams() const;
  const WebCryptoHmacImportParams* HmacImportParams() const;
  const WebCryptoHmacKeyGenParams* HmacKeyGenParams() const;
  const WebCryptoAesGcmParams* AesGcmParams() const;
  const WebCryptoRsaOaepParams* RsaOaepParams() const;
  const WebCryptoAesCtrParams* AesCtrParams() const;
  const WebCryptoRsaHashedImportParams* RsaHashedImportParams() const;
  const WebCryptoRsaHashedKeyGenParams* RsaHashedKeyGenParams() const;
  const WebCryptoRsaPssParams* RsaPssParams() const;
  const WebCryptoEcdsaParams* EcdsaParams() const;
  const WebCryptoEcKeyGenParams* EcKeyGenParams() const;
  const WebCryptoEcKeyImportParams* EcKeyImportParams() const;
  const WebCryptoEcdhKeyDeriveParams* EcdhKeyDeriveParams() const;
  const WebCryptoAesDerivedKeyParams* AesDerivedKeyParams() const;
  const WebCryptoHkdfParams* HkdfParams() const;
  const WebCryptoPbkdf2Params* Pbkdf2Params() const;

  // Returns true if the provided algorithm ID is for a hash (in other words,
  // SHA-*)
  static bool IsHash(WebCryptoAlgorithmId);
  // Returns true if the provided algorithm ID is for a key derivation function
  static bool IsKdf(WebCryptoAlgorithmId);

 private:
  void Assign(const WebCryptoAlgorithm& other);
  void Reset();

  WebPrivatePtrForRefCounted<WebCryptoAlgorithmPrivate> private_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_CRYPTO_ALGORITHM_H_
