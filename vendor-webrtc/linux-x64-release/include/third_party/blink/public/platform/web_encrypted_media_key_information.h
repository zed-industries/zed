// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_ENCRYPTED_MEDIA_KEY_INFORMATION_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_ENCRYPTED_MEDIA_KEY_INFORMATION_H_

#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_data.h"

namespace blink {

class BLINK_PLATFORM_EXPORT WebEncryptedMediaKeyInformation {
 public:
  enum class KeyStatus {
    kUsable,
    kExpired,
    kReleased,
    kOutputRestricted,
    kOutputDownscaled,
    kStatusPending,
    kInternalError,
    kUsableInFuture,
  };

  WebEncryptedMediaKeyInformation();
  ~WebEncryptedMediaKeyInformation();

  WebData Id() const;
  void SetId(const WebData&);

  KeyStatus Status() const;
  void SetStatus(KeyStatus);

  uint32_t SystemCode() const;
  void SetSystemCode(uint32_t);

 private:
  WebData id_;
  KeyStatus status_;
  uint32_t system_code_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_ENCRYPTED_MEDIA_KEY_INFORMATION_H_
