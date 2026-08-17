/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_SRTP_SESSION_H_
#define PC_SRTP_SESSION_H_

#include <stddef.h>
#include <stdint.h>

#include <vector>

#include "api/field_trials_view.h"
#include "api/sequence_checker.h"
#include "rtc_base/buffer.h"
#include "rtc_base/copy_on_write_buffer.h"

// Forward declaration to avoid pulling in libsrtp headers here
struct srtp_event_data_t;
struct srtp_ctx_t_;  // Trailing _ is required.

namespace webrtc {

// Prohibits webrtc from initializing libsrtp. This can be used if libsrtp is
// initialized by another library or explicitly. Note that this must be called
// before creating an SRTP session with WebRTC.
void ProhibitLibsrtpInitialization();

// Class that wraps a libSRTP session.
class SrtpSession {
 public:
  SrtpSession();
  explicit SrtpSession(const FieldTrialsView& field_trials);
  ~SrtpSession();

  SrtpSession(const SrtpSession&) = delete;
  SrtpSession& operator=(const SrtpSession&) = delete;

  // Configures the session for sending data using the specified
  // crypto suite and key. Receiving must be done by a separate session.
  bool SetSend(int crypto_suite,
               const ZeroOnFreeBuffer<uint8_t>& key,
               const std::vector<int>& extension_ids);
  bool UpdateSend(int crypto_suite,
                  const ZeroOnFreeBuffer<uint8_t>& key,
                  const std::vector<int>& extension_ids);

  // Configures the session for receiving data using the specified
  // crypto suite and key. Sending must be done by a separate session.
  bool SetReceive(int crypto_suite,
                  const ZeroOnFreeBuffer<uint8_t>& key,
                  const std::vector<int>& extension_ids);
  bool UpdateReceive(int crypto_suite,
                     const ZeroOnFreeBuffer<uint8_t>& key,
                     const std::vector<int>& extension_ids);

  // Encrypts/signs an individual RTP/RTCP packet, in-place.
  // If an HMAC is used, this will increase the packet size.
  [[deprecated("Pass CopyOnWriteBuffer")]] bool ProtectRtp(void* data,
                                                           int in_len,
                                                           int max_len,
                                                           int* out_len);
  bool ProtectRtp(CopyOnWriteBuffer& buffer);
  // Overloaded version, outputs packet index.
  [[deprecated("Pass CopyOnWriteBuffer")]] bool ProtectRtp(void* data,
                                                           int in_len,
                                                           int max_len,
                                                           int* out_len,
                                                           int64_t* index);
  bool ProtectRtp(CopyOnWriteBuffer& buffer, int64_t* index);

  [[deprecated("Pass CopyOnWriteBuffer")]] bool ProtectRtcp(void* data,
                                                            int in_len,
                                                            int max_len,
                                                            int* out_len);
  bool ProtectRtcp(CopyOnWriteBuffer& buffer);
  // Decrypts/verifies an invidiual RTP/RTCP packet.
  // If an HMAC is used, this will decrease the packet size.
  [[deprecated("Pass CopyOnWriteBuffer")]] bool UnprotectRtp(void* data,
                                                             int in_len,
                                                             int* out_len);
  bool UnprotectRtp(CopyOnWriteBuffer& buffer);
  [[deprecated("Pass CopyOnWriteBuffer")]] bool UnprotectRtcp(void* data,
                                                              int in_len,
                                                              int* out_len);
  bool UnprotectRtcp(CopyOnWriteBuffer& buffer);

  // Helper method to get authentication params.
  bool GetRtpAuthParams(uint8_t** key, int* key_len, int* tag_len);

  int GetSrtpOverhead() const;

  // If external auth is enabled, SRTP will write a dummy auth tag that then
  // later must get replaced before the packet is sent out. Only supported for
  // non-GCM cipher suites and can be checked through "IsExternalAuthActive"
  // if it is actually used. This method is only valid before the RTP params
  // have been set.
  void EnableExternalAuth();
  bool IsExternalAuthEnabled() const;

  // A SRTP session supports external creation of the auth tag if a non-GCM
  // cipher is used. This method is only valid after the RTP params have
  // been set.
  bool IsExternalAuthActive() const;

  // Removes a SSRC from the underlying libSRTP session.
  // Note: this should only be done for SSRCs that are received.
  // Removing SSRCs that were sent and then reusing them leads to
  // cryptographic weaknesses described in
  // https://www.rfc-editor.org/rfc/rfc3711#section-8
  // https://www.rfc-editor.org/rfc/rfc7714#section-8.4
  bool RemoveSsrcFromSession(uint32_t ssrc);

 private:
  bool DoSetKey(int type,
                int crypto_suite,
                const ZeroOnFreeBuffer<uint8_t>& key,
                const std::vector<int>& extension_ids);
  bool SetKey(int type,
              int crypto_suite,
              const ZeroOnFreeBuffer<uint8_t>& key,
              const std::vector<int>& extension_ids);
  bool UpdateKey(int type,
                 int crypto_suite,
                 const ZeroOnFreeBuffer<uint8_t>& key,
                 const std::vector<int>& extension_ids);
  // Returns send stream current packet index from srtp db.
  bool GetSendStreamPacketIndex(CopyOnWriteBuffer& buffer, int64_t* index);

  // Writes unencrypted packets in text2pcap format to the log file
  // for debugging.
  void DumpPacket(const CopyOnWriteBuffer& buffer, bool outbound);
  [[deprecated("Pass CopyOnWriteBuffer")]] void DumpPacket(const void* buf,
                                                           int len,
                                                           bool outbound);

  void HandleEvent(const srtp_event_data_t* ev);
  static void HandleEventThunk(srtp_event_data_t* ev);

  SequenceChecker thread_checker_;
  srtp_ctx_t_* session_ = nullptr;

  // Overhead of the SRTP auth tag for RTP and RTCP in bytes.
  // Depends on the cipher suite used and is usually the same with the exception
  // of the kCsAesCm128HmacSha1_32 cipher suite. The additional four bytes
  // required for RTCP protection are not included.
  int rtp_auth_tag_len_ = 0;
  int rtcp_auth_tag_len_ = 0;

  bool inited_ = false;
  int last_send_seq_num_ = -1;
  bool external_auth_active_ = false;
  bool external_auth_enabled_ = false;
  int decryption_failure_count_ = 0;
  bool dump_plain_rtp_ = false;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::ProhibitLibsrtpInitialization;
using ::webrtc::SrtpSession;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // PC_SRTP_SESSION_H_
