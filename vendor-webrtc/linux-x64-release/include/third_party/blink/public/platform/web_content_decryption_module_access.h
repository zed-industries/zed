// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_CONTENT_DECRYPTION_MODULE_ACCESS_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_CONTENT_DECRYPTION_MODULE_ACCESS_H_

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_string.h"

namespace blink {

class WebContentDecryptionModuleResult;
struct WebMediaKeySystemConfiguration;

class BLINK_PLATFORM_EXPORT WebContentDecryptionModuleAccess {
 public:
  virtual ~WebContentDecryptionModuleAccess();
  virtual void CreateContentDecryptionModule(
      WebContentDecryptionModuleResult,
      scoped_refptr<base::SingleThreadTaskRunner>) = 0;
  virtual WebMediaKeySystemConfiguration GetConfiguration() = 0;
  virtual WebString GetKeySystem() = 0;
  virtual bool UseHardwareSecureCodecs() const = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_CONTENT_DECRYPTION_MODULE_ACCESS_H_
