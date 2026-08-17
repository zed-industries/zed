// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_CONTENT_DECRYPTION_MODULE_RESULT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_CONTENT_DECRYPTION_MODULE_RESULT_H_

#include <memory>

#include "third_party/blink/public/platform/web_content_decryption_module_exception.h"
#include "third_party/blink/public/platform/web_content_decryption_module_result.h"
#include "third_party/blink/public/platform/web_encrypted_media_key_information.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class WebContentDecryptionModule;
class WebString;

// Used to notify completion of a CDM operation.
class ContentDecryptionModuleResult
    : public GarbageCollected<ContentDecryptionModuleResult> {
 public:
  virtual ~ContentDecryptionModuleResult() = default;

  virtual void Complete() = 0;
  virtual void CompleteWithContentDecryptionModule(
      std::unique_ptr<WebContentDecryptionModule>) = 0;
  virtual void CompleteWithSession(
      WebContentDecryptionModuleResult::SessionStatus) = 0;
  virtual void CompleteWithKeyStatus(
      WebEncryptedMediaKeyInformation::KeyStatus) = 0;
  virtual void CompleteWithError(WebContentDecryptionModuleException,
                                 uint32_t system_code,
                                 const WebString&) = 0;

  WebContentDecryptionModuleResult Result() {
    return WebContentDecryptionModuleResult(this);
  }

  virtual void Trace(Visitor* visitor) const {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_CONTENT_DECRYPTION_MODULE_RESULT_H_
