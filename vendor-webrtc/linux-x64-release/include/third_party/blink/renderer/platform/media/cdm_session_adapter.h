// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_CDM_SESSION_ADAPTER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_CDM_SESSION_ADAPTER_H_

#include <stdint.h>

#include <map>
#include <memory>
#include <string>
#include <unordered_map>
#include <vector>

#include "base/memory/ref_counted.h"
#include "base/memory/weak_ptr.h"
#include "media/base/cdm_config.h"
#include "media/base/cdm_factory.h"
#include "media/base/content_decryption_module.h"
#include "media/base/key_systems.h"
#include "third_party/blink/public/platform/web_content_decryption_module_session.h"
#include "third_party/blink/renderer/platform/media/web_content_decryption_module_impl.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace base {
class Time;
class TimeTicks;
}  // namespace base

namespace media {
class CdmContextRef;
class CdmFactory;
struct CdmConfig;
}  // namespace media

namespace blink {
class WebContentDecryptionModuleSessionImpl;

// Owns the CDM instance and makes calls from session objects to the CDM.
// Forwards the session ID-based callbacks of the media::ContentDecryptionModule
// interface to the appropriate session object. Callers should hold references
// to this class as long as they need the CDM instance.
class PLATFORM_EXPORT CdmSessionAdapter
    : public base::RefCounted<CdmSessionAdapter> {
 public:
  explicit CdmSessionAdapter(media::KeySystems* key_systems);
  CdmSessionAdapter(const CdmSessionAdapter&) = delete;
  CdmSessionAdapter& operator=(const CdmSessionAdapter&) = delete;

  // Creates the CDM for |cdm_config| using |cdm_factory| and returns the result
  // via |result|.
  void CreateCdm(media::CdmFactory* cdm_factory,
                 const media::CdmConfig& cdm_config,
                 WebCdmCreatedCB web_cdm_created_cb);

  // Provides a server certificate to be used to encrypt messages to the
  // license server.
  void SetServerCertificate(const std::vector<uint8_t>& certificate,
                            std::unique_ptr<media::SimpleCdmPromise> promise);

  // Gets the key status for a hypothetical key with |min_hdcp_version|
  // requirement.
  void GetStatusForPolicy(media::HdcpVersion min_hdcp_version,
                          std::unique_ptr<media::KeyStatusCdmPromise> promise);

  // Creates a new session and adds it to the internal map. RemoveSession()
  // must be called when destroying it, if RegisterSession() was called.
  std::unique_ptr<WebContentDecryptionModuleSessionImpl> CreateSession(
      WebEncryptedMediaSessionType session_type);

  // Adds a session to the internal map. Called once the session is successfully
  // initialized. Returns true if the session was registered, false if there is
  // already an existing session with the same |session_id|.
  bool RegisterSession(
      const std::string& session_id,
      base::WeakPtr<WebContentDecryptionModuleSessionImpl> session);

  // Removes a session from the internal map.
  void UnregisterSession(const std::string& session_id);

  // Initializes a session with the |init_data_type|, |init_data| and
  // |session_type| provided.
  void InitializeNewSession(
      media::EmeInitDataType init_data_type,
      const std::vector<uint8_t>& init_data,
      media::CdmSessionType session_type,
      std::unique_ptr<media::NewSessionCdmPromise> promise);

  // Loads the session specified by |session_id|.
  void LoadSession(media::CdmSessionType session_type,
                   const std::string& session_id,
                   std::unique_ptr<media::NewSessionCdmPromise> promise);

  // Updates the session specified by |session_id| with |response|.
  void UpdateSession(const std::string& session_id,
                     const std::vector<uint8_t>& response,
                     std::unique_ptr<media::SimpleCdmPromise> promise);

  // Closes the session specified by |session_id|.
  void CloseSession(const std::string& session_id,
                    std::unique_ptr<media::SimpleCdmPromise> promise);

  // Removes stored session data associated with the session specified by
  // |session_id|.
  void RemoveSession(const std::string& session_id,
                     std::unique_ptr<media::SimpleCdmPromise> promise);

  // Returns a CdmContextRef which provides access to CdmContext and by holding
  // the CdmContextRef, makes sure the CdmContext is kept alive.
  std::unique_ptr<media::CdmContextRef> GetCdmContextRef();

  // Returns the key system name.
  const std::string& GetKeySystem() const;

  // Returns a prefix to use for UMAs.
  const std::string& GetKeySystemUMAPrefix() const;

  // Returns the CdmConfig used in creation of CDM.
  const media::CdmConfig& GetCdmConfig() const;

 private:
  friend class base::RefCounted<CdmSessionAdapter>;

  // Session ID to WebContentDecryptionModuleSessionImpl mapping.
  typedef std::unordered_map<
      std::string,
      base::WeakPtr<WebContentDecryptionModuleSessionImpl>>
      SessionMap;

  ~CdmSessionAdapter();

  // Callback for CreateCdm().
  void OnCdmCreated(const media::CdmConfig& cdm_config,
                    base::TimeTicks start_time,
                    const scoped_refptr<media::ContentDecryptionModule>& cdm,
                    media::CreateCdmStatus status);

  // Callbacks for firing session events.
  void OnSessionMessage(const std::string& session_id,
                        media::CdmMessageType message_type,
                        const std::vector<uint8_t>& message);
  void OnSessionKeysChange(const std::string& session_id,
                           bool has_additional_usable_key,
                           media::CdmKeysInfo keys_info);
  void OnSessionExpirationUpdate(const std::string& session_id,
                                 base::Time new_expiry_time);
  void OnSessionClosed(const std::string& session_id,
                       media::CdmSessionClosedReason reason);

  // Helper function of the callbacks.
  WebContentDecryptionModuleSessionImpl* GetSession(
      const std::string& session_id);

  // Non-owned
  raw_ptr<media::KeySystems> key_systems_;

  scoped_refptr<media::ContentDecryptionModule> cdm_;

  SessionMap sessions_;

  std::string key_system_uma_prefix_;

  // media::CdmConfig used in creation of cdm_.
  media::CdmConfig cdm_config_;

  // A unique ID to trace CdmSessionAdapter::CreateCdm() call and the matching
  // OnCdmCreated() call.
  uint32_t trace_id_;

  WebCdmCreatedCB web_cdm_created_cb_;

  // NOTE: Weak pointers must be invalidated before all other member variables.
  base::WeakPtrFactory<CdmSessionAdapter> weak_ptr_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_CDM_SESSION_ADAPTER_H_
