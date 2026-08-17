// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_ENCRYPTED_MEDIA_REQUEST_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_ENCRYPTED_MEDIA_REQUEST_H_

#include <memory>
#include <vector>

#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_content_decryption_module_access.h"
#include "third_party/blink/public/platform/web_private_ptr.h"
#include "third_party/blink/public/platform/web_string.h"

namespace blink {

class EncryptedMediaRequest;
struct WebMediaKeySystemConfiguration;
class WebSecurityOrigin;

class BLINK_PLATFORM_EXPORT WebEncryptedMediaRequest {
 public:
  WebEncryptedMediaRequest(const WebEncryptedMediaRequest&);
  ~WebEncryptedMediaRequest();

  WebString KeySystem() const;
  const std::vector<WebMediaKeySystemConfiguration>& SupportedConfigurations()
      const;

  WebSecurityOrigin GetSecurityOrigin() const;

  void RequestSucceeded(std::unique_ptr<WebContentDecryptionModuleAccess>);
  void RequestNotSupported(const WebString& error_message);

#if INSIDE_BLINK
  explicit WebEncryptedMediaRequest(EncryptedMediaRequest*);
#endif

 private:
  void Assign(const WebEncryptedMediaRequest&);
  void Reset();

  WebPrivatePtrForGC<EncryptedMediaRequest> private_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_ENCRYPTED_MEDIA_REQUEST_H_
