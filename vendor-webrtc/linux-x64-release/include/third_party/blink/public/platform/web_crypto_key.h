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

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_CRYPTO_KEY_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_CRYPTO_KEY_H_

#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_private_ptr.h"

namespace blink {

enum WebCryptoKeyType {
  kWebCryptoKeyTypeSecret,
  kWebCryptoKeyTypePublic,
  kWebCryptoKeyTypePrivate,
};

enum WebCryptoKeyUsage {
  kWebCryptoKeyUsageEncrypt = 1 << 0,
  kWebCryptoKeyUsageDecrypt = 1 << 1,
  kWebCryptoKeyUsageSign = 1 << 2,
  kWebCryptoKeyUsageVerify = 1 << 3,
  kWebCryptoKeyUsageDeriveKey = 1 << 4,
  kWebCryptoKeyUsageWrapKey = 1 << 5,
  kWebCryptoKeyUsageUnwrapKey = 1 << 6,
  kWebCryptoKeyUsageDeriveBits = 1 << 7,
  kEndOfWebCryptoKeyUsage,
};

// A bitfield of WebCryptoKeyUsage
typedef int WebCryptoKeyUsageMask;

enum WebCryptoKeyFormat {
  kWebCryptoKeyFormatRaw,
  kWebCryptoKeyFormatPkcs8,
  kWebCryptoKeyFormatSpki,
  kWebCryptoKeyFormatJwk,
};

class WebCryptoKeyAlgorithm;
class WebCryptoKeyPrivate;
class WebCryptoKeyHandle;

// The WebCryptoKey represents a key from the Web Crypto API:
//
// https://dvcs.w3.org/hg/webcrypto-api/raw-file/tip/spec/Overview.html#key-interface
//
// WebCryptoKey is just a reference-counted wrapper that manages the lifetime of
// a "WebCryptoKeyHandle*".
//
// WebCryptoKey is:
//   * Copiable (cheaply)
//   * Threadsafe if the embedder's WebCryptoKeyHandle is also threadsafe.
//
// The embedder is responsible for creating all WebCryptoKeys, and therefore can
// safely assume any details regarding the type of the wrapped
// WebCryptoKeyHandle*.
//
// If WebCryptoKey "IsNull()" then it is invalid to call any of the other
// methods on it (other than destruction, assignment, or IsNull()).
class BLINK_PLATFORM_EXPORT WebCryptoKey {
 public:
  // Constructs a "null" key (One for which isNull() returns true).
  WebCryptoKey() = default;
  ~WebCryptoKey() { Reset(); }

  WebCryptoKey(const WebCryptoKey& other) { Assign(other); }
  WebCryptoKey& operator=(const WebCryptoKey& other) {
    Assign(other);
    return *this;
  }

  // For an explanation of these parameters see:
  // https://dvcs.w3.org/hg/webcrypto-api/raw-file/tip/spec/Overview.html#key-interface-members
  //
  // Note that the caller is passing ownership of the WebCryptoKeyHandle*.
  static WebCryptoKey Create(WebCryptoKeyHandle*,
                             WebCryptoKeyType,
                             bool extractable,
                             const WebCryptoKeyAlgorithm&,
                             WebCryptoKeyUsageMask);

  static WebCryptoKey CreateNull();

  // Returns the opaque key handle that was set by the embedder.
  //   * Safe to downcast to known type (since embedder creates all the keys)
  //   * Returned pointer's lifetime is bound to |this|
  WebCryptoKeyHandle* Handle() const;

  WebCryptoKeyType GetType() const;
  bool Extractable() const;
  const WebCryptoKeyAlgorithm& Algorithm() const;
  WebCryptoKeyUsageMask Usages() const;

  bool IsNull() const;

  bool KeyUsageAllows(const blink::WebCryptoKeyUsage) const;

 private:
  void Assign(const WebCryptoKey& other);
  void Reset();

  WebPrivatePtrForRefCounted<WebCryptoKeyPrivate> private_;
};

// Base class for the embedder to define its own opaque key handle. The lifetime
// of this object is controlled by WebCryptoKey using reference counting.
class WebCryptoKeyHandle {
 public:
  virtual ~WebCryptoKeyHandle() = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_CRYPTO_KEY_H_
