// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_ENCRYPTED_MEDIA_CLIENT_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_ENCRYPTED_MEDIA_CLIENT_H_

#include "third_party/blink/public/platform/web_common.h"

namespace blink {

class WebEncryptedMediaRequest;

class BLINK_PLATFORM_EXPORT WebEncryptedMediaClient {
 public:
  virtual ~WebEncryptedMediaClient();
  virtual void RequestMediaKeySystemAccess(WebEncryptedMediaRequest) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_ENCRYPTED_MEDIA_CLIENT_H_
