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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CRYPTO_CRYPTO_RESULT_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CRYPTO_CRYPTO_RESULT_IMPL_H_

#include "third_party/blink/public/platform/web_crypto.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/exception_code.h"
#include "third_party/blink/renderer/platform/crypto_result.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

MODULES_EXPORT ExceptionCode WebCryptoErrorToExceptionCode(WebCryptoErrorType);

// Wrapper around a Promise to notify completion of the crypto operation.
//
// The thread on which CryptoResultImpl was created on is referred to as the
// "origin thread".
//
//  * At creation time there must be an active ExecutionContext.
//  * All methods of the CryptoResult implementation must be called from
//    the origin thread. The exception is that destruction may happen on
//    another thread.
//  * One of the CompleteWith***() functions must be called, or the
//    |resolver_| will be leaked until the ExecutionContext is destroyed.
class MODULES_EXPORT CryptoResultImpl final
    : public CryptoResult,
      public ExecutionContextLifecycleObserver {
 public:
  enum class ResolverType { kAny, kTyped };

  template <typename IDLType>
  CryptoResultImpl(ScriptState* script_state,
                   ScriptPromiseResolver<IDLType>* resolver)
      : ExecutionContextLifecycleObserver(ExecutionContext::From(script_state)),
        resolver_(resolver),
        type_(std::is_same_v<IDLAny, IDLType> ? ResolverType::kAny
                                              : ResolverType::kTyped),
        cancel_(base::MakeRefCounted<CryptoResultCancel>()) {
    // Sync cancellation state.
    if (ExecutionContext::From(script_state)->IsContextDestroyed()) {
      Cancel();
    }
  }
  ~CryptoResultImpl() override;

  void CompleteWithError(WebCryptoErrorType, const WebString&) override;
  void CompleteWithBuffer(base::span<const uint8_t> bytes) override;
  void CompleteWithJson(std::string_view utf8_data) override;
  void CompleteWithBoolean(bool) override;
  void CompleteWithKey(const WebCryptoKey&) override;
  void CompleteWithKeyPair(const WebCryptoKey& public_key,
                           const WebCryptoKey& private_key) override;
  WebCryptoWarningType GetWarning() override { return warning_code_; }
  void SetWarning(WebCryptoWarningType code) override { warning_code_ = code; }
  ExecutionContext* GetExecutionContext() const override {
    return resolver_ ? resolver_->GetExecutionContext() : nullptr;
  }

  WebCryptoResult Result() { return WebCryptoResult(this, cancel_.get()); }

  // ExecutionContextLifecycleObserver override:
  void ContextDestroyed() override { Cancel(); }

  void Trace(Visitor*) const override;

 private:
  void Cancel();
  void ClearResolver();

  Member<ScriptPromiseResolverBase> resolver_;
  const ResolverType type_;

  // Separately communicate cancellation to WebCryptoResults so as to
  // allow this result object, which will be on the Oilpan heap, to be
  // GCed and destructed as needed. That is, it may end being GCed while
  // the thread owning the heap is detached and shut down, which will
  // in some cases happen before corresponding webcrypto operations have
  // all been processed. Hence these webcrypto operations cannot reliably
  // check cancellation status via this result object. So, keep a separate
  // cancellation status object for the purpose, which will outlive the
  // result object and can be safely accessed by multiple threads.
  scoped_refptr<CryptoResultCancel> cancel_;

  WebCryptoWarningType warning_code_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CRYPTO_CRYPTO_RESULT_IMPL_H_
