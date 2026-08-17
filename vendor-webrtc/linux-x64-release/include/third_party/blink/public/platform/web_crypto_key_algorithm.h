/*
 * Copyright (C) 2014 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_CRYPTO_KEY_ALGORITHM_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_CRYPTO_KEY_ALGORITHM_H_

#include "base/containers/span.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_crypto_algorithm.h"
#include "third_party/blink/public/platform/web_crypto_key_algorithm_params.h"
#include "third_party/blink/public/platform/web_private_ptr.h"

#if INSIDE_BLINK
#include <memory>
#endif

namespace blink {

class WebCryptoKeyAlgorithmPrivate;

// WebCryptoKeyAlgorithm represents the algorithm used to generate a
// WebCryptoKey instance.
//   * Immutable
//   * Threadsafe
//   * Copiable (cheaply)
//
// The WebCryptoKey represents a key from the Web Crypto API:
// https://w3c.github.io/webcrypto/#cryptokey-interface
class BLINK_PLATFORM_EXPORT WebCryptoKeyAlgorithm {
 public:
  WebCryptoKeyAlgorithm() = default;

#if INSIDE_BLINK
  WebCryptoKeyAlgorithm(WebCryptoAlgorithmId,
                        std::unique_ptr<WebCryptoKeyAlgorithmParams>);
#endif

  // FIXME: Delete this in favor of the Create*() functions.
  static WebCryptoKeyAlgorithm AdoptParamsAndCreate(
      WebCryptoAlgorithmId,
      WebCryptoKeyAlgorithmParams*);

  static WebCryptoKeyAlgorithm CreateAes(WebCryptoAlgorithmId,
                                         uint16_t key_length_bits);
  static WebCryptoKeyAlgorithm CreateHmac(WebCryptoAlgorithmId hash,
                                          unsigned key_length_bits);
  static WebCryptoKeyAlgorithm CreateRsaHashed(
      WebCryptoAlgorithmId,
      unsigned modulus_length_bits,
      base::span<const unsigned char> public_exponent,
      WebCryptoAlgorithmId hash);
  static WebCryptoKeyAlgorithm CreateEc(WebCryptoAlgorithmId,
                                        WebCryptoNamedCurve);
  static WebCryptoKeyAlgorithm CreateEd25519(WebCryptoAlgorithmId);
  static WebCryptoKeyAlgorithm CreateX25519(WebCryptoAlgorithmId);
  static WebCryptoKeyAlgorithm CreateWithoutParams(WebCryptoAlgorithmId);

  ~WebCryptoKeyAlgorithm() { Reset(); }

  WebCryptoKeyAlgorithm(const WebCryptoKeyAlgorithm& other) { Assign(other); }
  WebCryptoKeyAlgorithm& operator=(const WebCryptoKeyAlgorithm& other) {
    Assign(other);
    return *this;
  }

  bool IsNull() const;

  WebCryptoAlgorithmId Id() const;

  WebCryptoKeyAlgorithmParamsType ParamsType() const;

  // Returns the type-specific parameters for this key. If the requested
  // parameters are not applicable (for instance an HMAC key does not have
  // any AES parameters) then returns 0.
  WebCryptoAesKeyAlgorithmParams* AesParams() const;
  WebCryptoHmacKeyAlgorithmParams* HmacParams() const;
  WebCryptoRsaHashedKeyAlgorithmParams* RsaHashedParams() const;
  WebCryptoEcKeyAlgorithmParams* EcParams() const;

  // Write the algorithm parameters to a dictionary.
  void WriteToDictionary(WebCryptoKeyAlgorithmDictionary*) const;

 private:
  void Assign(const WebCryptoKeyAlgorithm& other);
  void Reset();

  WebPrivatePtrForRefCounted<WebCryptoKeyAlgorithmPrivate> private_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_CRYPTO_KEY_ALGORITHM_H_
