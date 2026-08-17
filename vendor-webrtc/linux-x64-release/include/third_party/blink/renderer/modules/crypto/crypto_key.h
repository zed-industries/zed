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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CRYPTO_CRYPTO_KEY_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CRYPTO_CRYPTO_KEY_H_

#include "third_party/blink/public/platform/web_crypto_key.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/modules/crypto/normalize_algorithm.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class CryptoResult;

class MODULES_EXPORT CryptoKey final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit CryptoKey(const WebCryptoKey&);
  ~CryptoKey() override;

  String type() const;
  bool extractable() const;
  ScriptObject algorithm(ScriptState*);
  ScriptObject usages(ScriptState*);

  const WebCryptoKey& Key() const { return key_; }

  // If the key cannot be used with the indicated algorithm, returns false
  // and completes the CryptoResult with an error.
  bool CanBeUsedForAlgorithm(const WebCryptoAlgorithm&,
                             WebCryptoKeyUsage,
                             CryptoResult*) const;

  // On failure, these return false and complete the CryptoResult with an error.
  static bool ParseFormat(const String&, WebCryptoKeyFormat&, ExceptionState&);
  static bool ParseUsageMask(const Vector<String>&,
                             WebCryptoKeyUsageMask&,
                             ExceptionState&);

 protected:
  const WebCryptoKey key_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CRYPTO_CRYPTO_KEY_H_
