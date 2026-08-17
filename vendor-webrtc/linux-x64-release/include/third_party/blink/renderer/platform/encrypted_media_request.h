// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_ENCRYPTED_MEDIA_REQUEST_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_ENCRYPTED_MEDIA_REQUEST_H_

#include <memory>
#include <vector>

#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class SecurityOrigin;
class WebContentDecryptionModuleAccess;
struct WebMediaKeySystemConfiguration;
class WebString;

class EncryptedMediaRequest : public GarbageCollected<EncryptedMediaRequest> {
 public:
  virtual ~EncryptedMediaRequest() = default;

  virtual WebString KeySystem() const = 0;
  virtual const std::vector<WebMediaKeySystemConfiguration>&
  SupportedConfigurations() const = 0;

  virtual const SecurityOrigin* GetSecurityOrigin() const = 0;

  virtual void RequestSucceeded(
      std::unique_ptr<WebContentDecryptionModuleAccess>) = 0;
  virtual void RequestNotSupported(const WebString& error_message) = 0;

  virtual void Trace(Visitor* visitor) const {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_ENCRYPTED_MEDIA_REQUEST_H_
