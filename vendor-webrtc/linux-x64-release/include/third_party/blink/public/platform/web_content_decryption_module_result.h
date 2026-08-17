// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_CONTENT_DECRYPTION_MODULE_RESULT_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_CONTENT_DECRYPTION_MODULE_RESULT_H_

#include <memory>

#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_content_decryption_module_exception.h"
#include "third_party/blink/public/platform/web_encrypted_media_key_information.h"
#include "third_party/blink/public/platform/web_private_ptr.h"

namespace blink {

class ContentDecryptionModuleResult;
class WebContentDecryptionModule;
class WebString;

class BLINK_PLATFORM_EXPORT WebContentDecryptionModuleResult {
 public:
  enum SessionStatus {
    // New session has been initialized.
    kNewSession,

    // CDM could not find the requested session.
    kSessionNotFound,

    // CDM already has a non-closed session that matches the provided
    // parameters.
    kSessionAlreadyExists,
  };

  WebContentDecryptionModuleResult(const WebContentDecryptionModuleResult& o) {
    Assign(o);
  }

  ~WebContentDecryptionModuleResult() { Reset(); }

  WebContentDecryptionModuleResult& operator=(
      const WebContentDecryptionModuleResult& o) {
    Assign(o);
    return *this;
  }

  // Called when the CDM completes an operation and has no additional data to
  // pass back.
  void Complete();

  // Called when a CDM is created.
  void CompleteWithContentDecryptionModule(
      std::unique_ptr<WebContentDecryptionModule>);

  // Called when the CDM completes a session operation.
  void CompleteWithSession(SessionStatus);

  // Called when the CDM completes getting key status for policy.
  void CompleteWithKeyStatus(WebEncryptedMediaKeyInformation::KeyStatus);

  // Called when the operation fails.
  void CompleteWithError(WebContentDecryptionModuleException,
                         uint32_t system_code,
                         const WebString& message);

#if INSIDE_BLINK
  explicit WebContentDecryptionModuleResult(ContentDecryptionModuleResult*);
#endif

 private:
  void Reset();
  void Assign(const WebContentDecryptionModuleResult&);

  WebPrivatePtrForGC<ContentDecryptionModuleResult,
                     WebPrivatePtrDestruction::kCrossThread>
      impl_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_CONTENT_DECRYPTION_MODULE_RESULT_H_
