/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_CONTENT_DECRYPTION_MODULE_SESSION_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_CONTENT_DECRYPTION_MODULE_SESSION_H_

#include <vector>

#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_content_decryption_module_exception.h"
#include "third_party/blink/public/platform/web_content_decryption_module_result.h"
#include "third_party/blink/public/platform/web_encrypted_media_types.h"

namespace media {
enum class CdmMessageType;
enum class CdmSessionClosedReason;
enum class EmeInitDataType;
}

namespace blink {

class WebEncryptedMediaKeyInformation;
class WebString;

class BLINK_PLATFORM_EXPORT WebContentDecryptionModuleSession {
 public:
  class BLINK_PLATFORM_EXPORT Client {
   public:
    virtual void OnSessionMessage(media::CdmMessageType,
                                  const unsigned char* message,
                                  size_t message_length) = 0;
    virtual void OnSessionClosed(media::CdmSessionClosedReason reason) = 0;

    // Called when the expiration time for the session changes.
    // |updated_expiry_time_in_ms| is specified as the number of milliseconds
    // since 01 January, 1970 UTC.
    virtual void OnSessionExpirationUpdate(
        double updated_expiry_time_in_ms) = 0;

    // Called when the set of keys for this session changes or existing keys
    // change state. |has_additional_usable_key| is set if a key is newly
    // usable (e.g. new key available, previously expired key has been
    // renewed, etc.) and the browser should attempt to resume playback
    // if necessary.
    virtual void OnSessionKeysChange(
        const std::vector<WebEncryptedMediaKeyInformation>&,
        bool has_additional_usable_key) = 0;

   protected:
    virtual ~Client();
  };

  virtual ~WebContentDecryptionModuleSession();

  virtual void SetClientInterface(Client*) = 0;
  virtual WebString SessionId() const = 0;

  virtual void InitializeNewSession(media::EmeInitDataType,
                                    const unsigned char* init_data,
                                    size_t init_data_length,
                                    WebContentDecryptionModuleResult) = 0;
  virtual void Load(const WebString& session_id,
                    WebContentDecryptionModuleResult) = 0;
  virtual void Update(const unsigned char* response,
                      size_t response_length,
                      WebContentDecryptionModuleResult) = 0;
  virtual void Close(WebContentDecryptionModuleResult) = 0;
  virtual void Remove(WebContentDecryptionModuleResult) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_CONTENT_DECRYPTION_MODULE_SESSION_H_
