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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_CRYPTO_RESULT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_CRYPTO_RESULT_H_

#include "base/containers/span.h"
#include "base/synchronization/atomic_flag.h"
#include "third_party/blink/public/platform/web_crypto.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"

namespace blink {

// Allows non-Blink webcrypto threads to query for cancellation status.
class CryptoResultCancel : public ThreadSafeRefCounted<CryptoResultCancel> {
 public:
  bool Cancelled() const { return cancelled_.IsSet(); }
  void Cancel() { cancelled_.Set(); }

 private:
  base::AtomicFlag cancelled_;
};

// Receives notification of completion of the crypto operation.
class PLATFORM_EXPORT CryptoResult : public GarbageCollected<CryptoResult> {
 public:
  virtual ~CryptoResult() = default;

  virtual void CompleteWithError(WebCryptoErrorType, const WebString&) = 0;
  virtual void CompleteWithBuffer(base::span<const uint8_t> bytes) = 0;
  virtual void CompleteWithJson(std::string_view utf8_data) = 0;
  virtual void CompleteWithBoolean(bool) = 0;
  virtual void CompleteWithKey(const WebCryptoKey&) = 0;
  virtual void CompleteWithKeyPair(const WebCryptoKey& public_key,
                                   const WebCryptoKey& private_key) = 0;

  virtual WebCryptoWarningType GetWarning() = 0;
  virtual void SetWarning(WebCryptoWarningType code) = 0;
  virtual ExecutionContext* GetExecutionContext() const = 0;

  virtual void Trace(Visitor* visitor) const {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_CRYPTO_RESULT_H_
