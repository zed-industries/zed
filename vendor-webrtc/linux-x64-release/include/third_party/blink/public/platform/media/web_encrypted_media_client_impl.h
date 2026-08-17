// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MEDIA_WEB_ENCRYPTED_MEDIA_CLIENT_IMPL_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MEDIA_WEB_ENCRYPTED_MEDIA_CLIENT_IMPL_H_

#include <memory>
#include <string>
#include <unordered_map>

#include "base/functional/callback.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/weak_ptr.h"
#include "third_party/blink/public/platform/media/key_system_config_selector.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_encrypted_media_client.h"

namespace media {
class CdmFactory;
class KeySystems;
class MediaPermission;
struct CdmConfig;
}  // namespace media

namespace blink {
class WebContentDecryptionModuleResult;
class WebSecurityOrigin;
struct WebMediaKeySystemConfiguration;

class BLINK_PLATFORM_EXPORT WebEncryptedMediaClientImpl
    : public WebEncryptedMediaClient {
 public:
  WebEncryptedMediaClientImpl(
      media::KeySystems* key_systems,
      media::CdmFactory* cdm_factory,
      media::MediaPermission* media_permission,
      std::unique_ptr<KeySystemConfigSelector::WebLocalFrameDelegate>
          web_frame_delegate);
  ~WebEncryptedMediaClientImpl() override;

  // WebEncryptedMediaClient implementation.
  void RequestMediaKeySystemAccess(WebEncryptedMediaRequest request) override;

  // Create the CDM for |security_origin| and |cdm_config|. The caller owns
  // the created cdm (passed back using |result|).
  void CreateCdm(const WebSecurityOrigin& security_origin,
                 const media::CdmConfig& cdm_config,
                 std::unique_ptr<WebContentDecryptionModuleResult> result);

 private:
  // Report usage of key system to UMA. There are 2 different counts logged:
  // 1. The key system is requested.
  // 2. The requested key system and options are supported.
  // Each stat is only reported once per renderer frame per key system.
  class Reporter;

  // Callback for media::KeySystems initialization.
  void OnKeySystemsUpdated();

  // Helper function to call `KeySystemConfigSelector::SelectConfig()`.
  void SelectConfig(WebEncryptedMediaRequest request);

  // Callback for `KeySystemConfigSelector::SelectConfig()`.
  // `accumulated_configuration` and `cdm_config` are non-null iff `status` is
  // `kSupported`. `cdm_config->key_system` is the same as the requested key
  // system in most cases unless a sub key system is queried and the base key
  // system should be used.
  void OnConfigSelected(
      WebEncryptedMediaRequest request,
      KeySystemConfigSelector::Status status,
      WebMediaKeySystemConfiguration* accumulated_configuration,
      media::CdmConfig* cdm_config);

  // Gets the Reporter for |key_system|. If it doesn't already exist,
  // create one.
  Reporter* GetReporter(const WebString& key_system);

  // Reporter singletons.
  std::unordered_map<std::string, std::unique_ptr<Reporter>> reporters_;

  const raw_ptr<media::KeySystems> key_systems_;
  const raw_ptr<media::CdmFactory> cdm_factory_;
  KeySystemConfigSelector key_system_config_selector_;

  // Pending requests while waiting for KeySystems initialization.
  std::vector<WebEncryptedMediaRequest> pending_requests_;

  base::WeakPtrFactory<WebEncryptedMediaClientImpl> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MEDIA_WEB_ENCRYPTED_MEDIA_CLIENT_IMPL_H_
