// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_WEB_CONTENT_DECRYPTION_MODULE_ACCESS_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_WEB_CONTENT_DECRYPTION_MODULE_ACCESS_IMPL_H_

#include <memory>

#include "base/memory/weak_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "media/base/cdm_config.h"
#include "third_party/blink/public/platform/web_content_decryption_module_access.h"
#include "third_party/blink/public/platform/web_content_decryption_module_result.h"
#include "third_party/blink/public/platform/web_media_key_system_configuration.h"
#include "third_party/blink/public/platform/web_security_origin.h"
#include "third_party/blink/public/platform/web_string.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {
class WebEncryptedMediaClientImpl;

class PLATFORM_EXPORT WebContentDecryptionModuleAccessImpl
    : public WebContentDecryptionModuleAccess {
 public:
  // Allow typecasting from blink type as this is the only implementation.
  static WebContentDecryptionModuleAccessImpl* From(
      WebContentDecryptionModuleAccess* cdm_access);

  static std::unique_ptr<WebContentDecryptionModuleAccessImpl> Create(
      const WebSecurityOrigin& security_origin,
      const WebMediaKeySystemConfiguration& configuration,
      const media::CdmConfig& cdm_config,
      const base::WeakPtr<WebEncryptedMediaClientImpl>& client);
  WebContentDecryptionModuleAccessImpl(
      const WebSecurityOrigin& security_origin,
      const WebMediaKeySystemConfiguration& configuration,
      const media::CdmConfig& cdm_config,
      const base::WeakPtr<WebEncryptedMediaClientImpl>& client);
  WebContentDecryptionModuleAccessImpl(
      const WebContentDecryptionModuleAccessImpl&) = delete;
  WebContentDecryptionModuleAccessImpl& operator=(
      const WebContentDecryptionModuleAccessImpl&) = delete;
  ~WebContentDecryptionModuleAccessImpl() override;

  // WebContentDecryptionModuleAccess interface.
  WebString GetKeySystem() override;
  WebMediaKeySystemConfiguration GetConfiguration() override;
  void CreateContentDecryptionModule(
      WebContentDecryptionModuleResult result,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner) override;
  bool UseHardwareSecureCodecs() const override;

 private:
  const WebSecurityOrigin security_origin_;
  const WebMediaKeySystemConfiguration configuration_;
  const media::CdmConfig cdm_config_;

  // Keep a WeakPtr as client is owned by render_frame_impl.
  base::WeakPtr<WebEncryptedMediaClientImpl> client_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_WEB_CONTENT_DECRYPTION_MODULE_ACCESS_IMPL_H_
