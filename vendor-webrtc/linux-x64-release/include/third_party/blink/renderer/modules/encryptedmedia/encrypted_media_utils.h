// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ENCRYPTEDMEDIA_ENCRYPTED_MEDIA_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ENCRYPTEDMEDIA_ENCRYPTED_MEDIA_UTILS_H_

#include "third_party/blink/public/platform/web_encrypted_media_key_information.h"
#include "third_party/blink/public/platform/web_encrypted_media_types.h"
#include "third_party/blink/public/platform/web_media_key_system_configuration.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_media_key_status.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_media_keys_requirement.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace media {
enum class EmeInitDataType;
}

namespace blink {

// Deprecated: This was used on some older UKMs. For new UKMs please use
// media::GetKeySystemIntForUKM() instead. Reported to UKM. Existing values must
// not change and new values must be added at the end of the list.
enum KeySystemForUkmLegacy {
  kClearKey = 0,
  kWidevine = 1,
};

// Enum for EME MediaKeySystemAccess, MediaKeys and MediaKeySession APIs.
// Reported to UKM. Existing values should NEVER be changed.
enum class EmeApiType {
  // Value 0 is reserved to detect errors.
  kCreateMediaKeys = 1,
  kSetServerCertificate = 2,
  kGetStatusForPolicy = 3,
  kGenerateRequest = 4,
  kLoad = 5,
  kUpdate = 6,
  kClose = 7,
  kRemove = 8,
};

// Config associated with a MediaKeys and its sessions.
struct MediaKeysConfig {
  String key_system;
  bool use_hardware_secure_codecs = false;
};

constexpr const char* kEncryptedMediaPermissionsPolicyConsoleWarning =
    "Encrypted Media access has been blocked because of a Feature Policy "
    "applied to the current document. See https://goo.gl/EuHzyv for more "
    "details.";

class ExecutionContext;
class LocalDOMWindow;
class V8MediaKeyStatus;
class WebEncryptedMediaClient;

class EncryptedMediaUtils {
  STATIC_ONLY(EncryptedMediaUtils);

 public:
  static media::EmeInitDataType ConvertToInitDataType(
      const String& init_data_type);
  static String ConvertFromInitDataType(media::EmeInitDataType init_data_type);

  static WebEncryptedMediaSessionType ConvertToSessionType(
      const String& session_type);
  static String ConvertFromSessionType(WebEncryptedMediaSessionType);

  static V8MediaKeyStatus ConvertKeyStatusToEnum(
      const WebEncryptedMediaKeyInformation::KeyStatus);

  static WebMediaKeySystemConfiguration::Requirement
      ConvertToMediaKeysRequirement(V8MediaKeysRequirement::Enum);
  static V8MediaKeysRequirement::Enum ConvertMediaKeysRequirementToEnum(
      WebMediaKeySystemConfiguration::Requirement);

  static WebEncryptedMediaClient* GetEncryptedMediaClientFromLocalDOMWindow(
      LocalDOMWindow*);

  static void ReportUsage(EmeApiType api_type,
                          ExecutionContext* execution_context,
                          const String& key_system,
                          bool use_hardware_secure_codecs,
                          bool is_persistent_session);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ENCRYPTEDMEDIA_ENCRYPTED_MEDIA_UTILS_H_
