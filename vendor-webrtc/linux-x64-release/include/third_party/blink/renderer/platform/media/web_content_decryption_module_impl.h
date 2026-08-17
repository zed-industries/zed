// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_WEB_CONTENT_DECRYPTION_MODULE_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_WEB_CONTENT_DECRYPTION_MODULE_IMPL_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <string>

#include "base/functional/callback_forward.h"
#include "base/memory/scoped_refptr.h"
#include "base/types/pass_key.h"
#include "media/base/cdm_config.h"
#include "media/base/cdm_factory.h"
#include "media/base/key_systems.h"
#include "third_party/blink/public/platform/web_content_decryption_module.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace media {
class CdmContextRef;
class CdmFactory;
struct CdmConfig;
}  // namespace media

namespace blink {

class CdmSessionAdapter;
class WebSecurityOrigin;

using WebCdmCreatedCB =
    base::OnceCallback<void(std::unique_ptr<WebContentDecryptionModule> cdm,
                            media::CreateCdmStatus status)>;

class PLATFORM_EXPORT WebContentDecryptionModuleImpl
    : public WebContentDecryptionModule {
 public:
  static void Create(media::CdmFactory* cdm_factory,
                     media::KeySystems* key_systems,
                     const WebSecurityOrigin& security_origin,
                     const media::CdmConfig& cdm_config,
                     WebCdmCreatedCB web_cdm_created_cb);

  // Takes reference to |adapter|.
  WebContentDecryptionModuleImpl(base::PassKey<CdmSessionAdapter>,
                                 scoped_refptr<CdmSessionAdapter> adapter,
                                 media::KeySystems* key_systems);
  WebContentDecryptionModuleImpl(const WebContentDecryptionModuleImpl&) =
      delete;
  WebContentDecryptionModuleImpl& operator=(
      const WebContentDecryptionModuleImpl&) = delete;
  ~WebContentDecryptionModuleImpl() override;

  // WebContentDecryptionModule implementation.
  std::unique_ptr<WebContentDecryptionModuleSession> CreateSession(
      WebEncryptedMediaSessionType session_type) override;
  void SetServerCertificate(const uint8_t* server_certificate,
                            size_t server_certificate_length,
                            WebContentDecryptionModuleResult result) override;
  void GetStatusForPolicy(const WebString& min_hdcp_version_string,
                          WebContentDecryptionModuleResult result) override;

  std::unique_ptr<media::CdmContextRef> GetCdmContextRef();
  media::CdmConfig GetCdmConfig() const;

 private:
  scoped_refptr<CdmSessionAdapter> adapter_;
  // Non-owned.
  raw_ptr<media::KeySystems> key_systems_;
};

// Allow typecasting from blink type as this is the only implementation.
PLATFORM_EXPORT
inline WebContentDecryptionModuleImpl* ToWebContentDecryptionModuleImpl(
    WebContentDecryptionModule* cdm) {
  return static_cast<WebContentDecryptionModuleImpl*>(cdm);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_WEB_CONTENT_DECRYPTION_MODULE_IMPL_H_
