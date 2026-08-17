// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_NEW_SESSION_CDM_RESULT_PROMISE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_NEW_SESSION_CDM_RESULT_PROMISE_H_

#include <stdint.h>

#include <string>

#include "base/functional/callback.h"
#include "base/time/time.h"
#include "media/base/cdm_promise.h"
#include "third_party/blink/public/platform/web_content_decryption_module_result.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

enum class SessionInitStatus {
  // Unable to determine the status.
  UNKNOWN_STATUS,

  // New session has been initialized.
  NEW_SESSION,

  // CDM could not find the requested session.
  SESSION_NOT_FOUND,

  // CDM already has a non-closed session that matches the provided
  // parameters.
  SESSION_ALREADY_EXISTS
};

using SessionInitializedCB =
    base::OnceCallback<void(const std::string& session_id,
                            SessionInitStatus* status)>;

// Special class for resolving a new session promise. Resolving a new session
// promise returns the session ID (as a string), but the blink promise needs
// to get passed a SessionStatus. This class converts the session id to a
// SessionStatus by calling |new_session_created_cb|. The value returned by
// |new_session_created_cb| must be in |expected_statuses| for the promise to
// be resolved. If it's not in the list, the promise will be rejected.
class PLATFORM_EXPORT NewSessionCdmResultPromise
    : public media::CdmPromiseTemplate<std::string> {
 public:
  NewSessionCdmResultPromise(
      const WebContentDecryptionModuleResult& result,
      const std::string& key_system_uma_prefix,
      const std::string& uma_name,
      SessionInitializedCB new_session_created_cb,
      const std::vector<SessionInitStatus>& expected_statuses);
  NewSessionCdmResultPromise(const NewSessionCdmResultPromise&) = delete;
  NewSessionCdmResultPromise& operator=(const NewSessionCdmResultPromise&) =
      delete;
  ~NewSessionCdmResultPromise() override;

  // media::CdmPromiseTemplate<T> implementation.
  void resolve(const std::string& session_id) override;
  void reject(CdmPromise::Exception exception_code,
              uint32_t system_code,
              const std::string& error_message) override;

 private:
  WebContentDecryptionModuleResult web_cdm_result_;

  // UMA prefix and name to report result and time to.
  std::string key_system_uma_prefix_;
  std::string uma_name_;

  // Called on resolve() to convert the session ID into a SessionInitStatus to
  // be reported to blink. Returned status must be in |expected_statuses_| or
  // else the promise will be rejected.
  SessionInitializedCB new_session_created_cb_;
  std::vector<SessionInitStatus> expected_statuses_;

  // Time when |this| is created.
  base::TimeTicks creation_time_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_NEW_SESSION_CDM_RESULT_PROMISE_H_
