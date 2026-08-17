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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CRYPTO_SUBTLE_CRYPTO_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CRYPTO_SUBTLE_CRYPTO_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_typedefs.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_typedefs.h"
#include "third_party/blink/renderer/modules/crypto/normalize_algorithm.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class CryptoKey;
class DOMArrayBuffer;

class SubtleCrypto final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  SubtleCrypto();

  ScriptPromise<IDLAny> encrypt(ScriptState*,
                                const V8AlgorithmIdentifier*,
                                CryptoKey*,
                                const V8BufferSource*,
                                ExceptionState&);
  ScriptPromise<IDLAny> decrypt(ScriptState*,
                                const V8AlgorithmIdentifier*,
                                CryptoKey*,
                                const V8BufferSource*,
                                ExceptionState&);
  ScriptPromise<IDLAny> sign(ScriptState*,
                             const V8AlgorithmIdentifier*,
                             CryptoKey*,
                             const V8BufferSource*,
                             ExceptionState&);
  // Note that this is not named "verify" because when compiling on Mac that
  // expands to a macro and breaks.
  ScriptPromise<IDLAny> verifySignature(ScriptState*,
                                        const V8AlgorithmIdentifier*,
                                        CryptoKey*,
                                        const V8BufferSource* signature,
                                        const V8BufferSource* data,
                                        ExceptionState&);
  ScriptPromise<IDLAny> digest(ScriptState*,
                               const V8AlgorithmIdentifier*,
                               const V8BufferSource* data,
                               ExceptionState&);

  ScriptPromise<IDLAny> generateKey(ScriptState*,
                                    const V8AlgorithmIdentifier*,
                                    bool extractable,
                                    const Vector<String>& key_usages,
                                    ExceptionState&);
  ScriptPromise<CryptoKey> importKey(ScriptState*,
                                     const String&,
                                     const V8UnionBufferSourceOrJsonWebKey*,
                                     const V8AlgorithmIdentifier*,
                                     bool extractable,
                                     const Vector<String>& key_usages,
                                     ExceptionState&);
  ScriptPromise<IDLAny> exportKey(ScriptState*,
                                  const String&,
                                  CryptoKey*,
                                  ExceptionState&);

  ScriptPromise<IDLAny> wrapKey(ScriptState*,
                                const String&,
                                CryptoKey*,
                                CryptoKey*,
                                const V8AlgorithmIdentifier*,
                                ExceptionState&);
  ScriptPromise<CryptoKey> unwrapKey(ScriptState*,
                                     const String&,
                                     const V8BufferSource*,
                                     CryptoKey*,
                                     const V8AlgorithmIdentifier*,
                                     const V8AlgorithmIdentifier*,
                                     bool,
                                     const Vector<String>&,
                                     ExceptionState&);

  ScriptPromise<DOMArrayBuffer> deriveBits(ScriptState*,
                                           const V8AlgorithmIdentifier*,
                                           CryptoKey*,
                                           std::optional<unsigned>,
                                           ExceptionState&);
  ScriptPromise<IDLAny> deriveKey(ScriptState*,
                                  const V8AlgorithmIdentifier*,
                                  CryptoKey*,
                                  const V8AlgorithmIdentifier*,
                                  bool extractable,
                                  const Vector<String>&,
                                  ExceptionState&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CRYPTO_SUBTLE_CRYPTO_H_
