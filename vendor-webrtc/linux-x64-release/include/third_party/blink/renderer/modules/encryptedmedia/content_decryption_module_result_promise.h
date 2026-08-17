// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ENCRYPTEDMEDIA_CONTENT_DECRYPTION_MODULE_RESULT_PROMISE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ENCRYPTEDMEDIA_CONTENT_DECRYPTION_MODULE_RESULT_PROMISE_H_

#include "third_party/blink/public/platform/web_encrypted_media_key_information.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/modules/encryptedmedia/encrypted_media_utils.h"
#include "third_party/blink/renderer/platform/bindings/exception_code.h"
#include "third_party/blink/renderer/platform/content_decryption_module_result.h"

namespace blink {

void WebCdmExceptionToPromiseRejection(ScriptPromiseResolverBase*,
                                       WebContentDecryptionModuleException,
                                       const String& message);

// This class wraps the promise resolver to simplify creation of
// ContentDecryptionModuleResult objects. The default implementations of the
// complete(), completeWithSession(), etc. methods will reject the promise
// with an error. It needs to be subclassed and the appropriate complete()
// method overridden to resolve the promise as needed.
//
// Subclasses need to keep a Member<> to the object that created them so
// that the creator remains around as long as this promise is pending. This
// promise is not referenced by the object that created it (e.g MediaKeys,
// MediaKeySession, Navigator.requestMediaKeySystemAccess), so this promise
// may be cleaned up before or after it's creator once both become unreachable.
// If it is after, the destruction of the creator may trigger this promise,
// so use isValidToFulfillPromise() to verify that it is safe to fulfill
// the promise.
class ContentDecryptionModuleResultPromise
    : public ContentDecryptionModuleResult {
 public:
  ~ContentDecryptionModuleResultPromise() override;

  // ContentDecryptionModuleResult implementation.
  void Complete() override;
  void CompleteWithContentDecryptionModule(
      std::unique_ptr<WebContentDecryptionModule>) override;
  void CompleteWithSession(
      WebContentDecryptionModuleResult::SessionStatus) override;
  void CompleteWithKeyStatus(
      WebEncryptedMediaKeyInformation::KeyStatus) override;
  void CompleteWithError(WebContentDecryptionModuleException,
                         uint32_t system_code,
                         const WebString&) override;

  void Trace(Visitor*) const override;

 protected:
  // |interface_name| and |property_name| must have static life time.
  ContentDecryptionModuleResultPromise(ScriptPromiseResolverBase*,
                                       const MediaKeysConfig&,
                                       EmeApiType api_type);

  // Resolves the promise with |value|. Used by subclasses to resolve the
  // promise.
  template <typename IDLType, typename... BlinkType>
  void Resolve(BlinkType&&... value) {
    DCHECK(IsValidToFulfillPromise());

    resolver_->DowncastTo<IDLType>()->Resolve(
        std::forward<BlinkType>(value)...);
    resolver_.Clear();
  }

  ExecutionContext* GetExecutionContext() const;

  // Determine if it's OK to resolve/reject this promise.
  bool IsValidToFulfillPromise();

  // Returns |config_|.
  MediaKeysConfig GetMediaKeysConfig();

 private:
  Member<ScriptPromiseResolverBase> resolver_;
  const MediaKeysConfig config_;
  const EmeApiType api_type_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ENCRYPTEDMEDIA_CONTENT_DECRYPTION_MODULE_RESULT_PROMISE_H_
