// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MEDIA_KEY_SYSTEM_CONFIG_SELECTOR_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MEDIA_KEY_SYSTEM_CONFIG_SELECTOR_H_

#include <memory>
#include <string>
#include <vector>

#include "base/functional/callback_forward.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/weak_ptr.h"
#include "media/base/eme_constants.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_content_settings_client.h"
#include "third_party/blink/public/platform/web_media_key_system_media_capability.h"

namespace media {
class KeySystems;
class MediaPermission;
struct CdmConfig;
}  // namespace media

namespace blink {
class WebString;
struct WebMediaKeySystemConfiguration;

class BLINK_PLATFORM_EXPORT KeySystemConfigSelector {
 public:
  // This is to facilitate testing, it abstracts the calls we are making into
  // WebLocalFrame so we can override them for testing without implementing the
  // entire interface for WebLocalFrame.
  class BLINK_PLATFORM_EXPORT WebLocalFrameDelegate {
   public:
    virtual ~WebLocalFrameDelegate() = default;

    // Delegate to `WebLocalFrame`.
    virtual bool IsCrossOriginToOutermostMainFrame() = 0;

    // Delegate to `WebContentSettingsClient` within `WebLocalFrame`.
    virtual bool AllowStorageAccessSync(
        WebContentSettingsClient::StorageType storage_type) = 0;
  };

  KeySystemConfigSelector(
      media::KeySystems* key_systems,
      media::MediaPermission* media_permission,
      std::unique_ptr<WebLocalFrameDelegate> web_frame_delegate);
  KeySystemConfigSelector(const KeySystemConfigSelector&) = delete;
  KeySystemConfigSelector& operator=(const KeySystemConfigSelector&) = delete;

  ~KeySystemConfigSelector();

  // The unsupported statuses will be mapped to different rejection messages.
  // The statuses should not leak sensitive information, e.g. incognito mode or
  // user settings. See https://crbug.com/760720
  enum class Status {
    kSupported,
    kUnsupportedKeySystem,
    kUnsupportedConfigs,
  };

  // Callback for the result of `SelectConfig()`. The returned configs must be
  // non-null iff `status` is `kSupported`.
  using SelectConfigCB = base::OnceCallback<
      void(Status status, WebMediaKeySystemConfiguration*, media::CdmConfig*)>;

  void SelectConfig(const WebString& key_system,
                    const std::vector<WebMediaKeySystemConfiguration>&
                        candidate_configurations,
                    SelectConfigCB cb);

  using IsSupportedMediaTypeCB =
      base::RepeatingCallback<bool(const std::string& container_mime_type,
                                   const std::string& codecs,
                                   bool use_aes_decryptor)>;

  void SetIsSupportedMediaTypeCBForTesting(IsSupportedMediaTypeCB cb) {
    is_supported_media_type_cb_ = std::move(cb);
  }

 private:
  struct SelectionRequest;
  class ConfigState;

  enum ConfigurationSupport {
    CONFIGURATION_NOT_SUPPORTED,
    CONFIGURATION_REQUIRES_PERMISSION,
    CONFIGURATION_SUPPORTED,
  };

  void SelectConfigInternal(std::unique_ptr<SelectionRequest> request);

  void OnPermissionResult(std::unique_ptr<SelectionRequest> request,
                          bool is_permission_granted);

#if BUILDFLAG(IS_WIN)
  void OnHardwareSecureDecryptionAllowedResult(
      std::unique_ptr<SelectionRequest> request,
      bool is_hardware_secure_decryption_allowed);
#endif  // BUILDFLAG(IS_WIN)

  ConfigurationSupport GetSupportedConfiguration(
      const std::string& key_system,
      const WebMediaKeySystemConfiguration& candidate,
      ConfigState* config_state,
      WebMediaKeySystemConfiguration* accumulated_configuration);

  bool GetSupportedCapabilities(
      const std::string& key_system,
      media::EmeMediaType media_type,
      const std::vector<WebMediaKeySystemMediaCapability>&
          requested_media_capabilities,
      ConfigState* config_state,
      std::vector<WebMediaKeySystemMediaCapability>*
          supported_media_capabilities);

  bool IsSupportedContentType(const std::string& key_system,
                              media::EmeMediaType media_type,
                              const std::string& container_mime_type,
                              const std::string& codecs,
                              ConfigState* config_state);

  media::EmeConfig::Rule GetEncryptionSchemeConfigRule(
      const std::string& key_system,
      const WebMediaKeySystemMediaCapability::EncryptionScheme
          encryption_scheme);

  const raw_ptr<media::KeySystems> key_systems_;

  // This object is unowned but its pointer is always valid. It has the same
  // lifetime as RenderFrameImpl, and |this| also has the same lifetime
  // as RenderFrameImpl. RenderFrameImpl owns content::MediaFactory which owns
  // WebEncryptedMediaClientImpl which owns |this|.
  raw_ptr<media::MediaPermission> media_permission_;

  std::unique_ptr<WebLocalFrameDelegate> web_frame_delegate_;

  // A callback used to check whether a media type is supported. Only set in
  // tests. If null the implementation will check the support using MimeUtil.
  IsSupportedMediaTypeCB is_supported_media_type_cb_;

  base::WeakPtrFactory<KeySystemConfigSelector> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MEDIA_KEY_SYSTEM_CONFIG_SELECTOR_H_
