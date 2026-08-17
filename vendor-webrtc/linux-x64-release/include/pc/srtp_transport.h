/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_SRTP_TRANSPORT_H_
#define PC_SRTP_TRANSPORT_H_

#include <stddef.h>

#include <cstdint>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "api/field_trials_view.h"
#include "call/rtp_demuxer.h"
#include "p2p/base/packet_transport_internal.h"
#include "pc/rtp_transport.h"
#include "pc/srtp_session.h"
#include "rtc_base/async_packet_socket.h"
#include "rtc_base/buffer.h"
#include "rtc_base/copy_on_write_buffer.h"
#include "rtc_base/network/received_packet.h"
#include "rtc_base/network_route.h"

namespace webrtc {

// This subclass of the RtpTransport is used for SRTP which is reponsible for
// protecting/unprotecting the packets. It provides interfaces to set the crypto
// parameters for the SrtpSession underneath.
class SrtpTransport : public RtpTransport {
 public:
  SrtpTransport(bool rtcp_mux_enabled, const FieldTrialsView& field_trials);

  virtual ~SrtpTransport() = default;

  bool SendRtpPacket(CopyOnWriteBuffer* packet,
                     const AsyncSocketPacketOptions& options,
                     int flags) override;

  bool SendRtcpPacket(CopyOnWriteBuffer* packet,
                      const AsyncSocketPacketOptions& options,
                      int flags) override;

  // The transport becomes active if the send_session_ and recv_session_ are
  // created.
  bool IsSrtpActive() const override;

  bool IsWritable(bool rtcp) const override;

  // Create new send/recv sessions and set the negotiated crypto keys for RTP
  // packet encryption. The keys can either come from SDES negotiation or DTLS
  // handshake.
  bool SetRtpParams(int send_crypto_suite,
                    const ZeroOnFreeBuffer<uint8_t>& send_key,
                    const std::vector<int>& send_extension_ids,
                    int recv_crypto_suite,
                    const ZeroOnFreeBuffer<uint8_t>& recv_key,
                    const std::vector<int>& recv_extension_ids);

  // Create new send/recv sessions and set the negotiated crypto keys for RTCP
  // packet encryption. The keys can either come from SDES negotiation or DTLS
  // handshake.
  bool SetRtcpParams(int send_crypto_suite,
                     const ZeroOnFreeBuffer<uint8_t>& send_key,
                     const std::vector<int>& send_extension_ids,
                     int recv_crypto_suite,
                     const ZeroOnFreeBuffer<uint8_t>& recv_key,
                     const std::vector<int>& recv_extension_ids);

  void ResetParams();

  // If external auth is enabled, SRTP will write a dummy auth tag that then
  // later must get replaced before the packet is sent out. Only supported for
  // non-GCM crypto suites and can be checked through "IsExternalAuthActive"
  // if it is actually used. This method is only valid before the RTP params
  // have been set.
  void EnableExternalAuth();
  bool IsExternalAuthEnabled() const;

  // A SrtpTransport supports external creation of the auth tag if a non-GCM
  // cipher is used. This method is only valid after the RTP params have
  // been set.
  bool IsExternalAuthActive() const;

  // Returns srtp overhead for rtp packets.
  bool GetSrtpOverhead(int* srtp_overhead) const;

  // Returns rtp auth params from srtp context.
  bool GetRtpAuthParams(uint8_t** key, int* key_len, int* tag_len);

  // Cache RTP Absoulute SendTime extension header ID. This is only used when
  // external authentication is enabled.
  void CacheRtpAbsSendTimeHeaderExtension(int rtp_abs_sendtime_extn_id) {
    rtp_abs_sendtime_extn_id_ = rtp_abs_sendtime_extn_id;
  }

  // In addition to unregistering the sink, the SRTP transport
  // disassociates all SSRCs of the sink from libSRTP.
  bool UnregisterRtpDemuxerSink(RtpPacketSinkInterface* sink) override;

 protected:
  // If the writable state changed, fire the SignalWritableState.
  void MaybeUpdateWritableState();

 private:
  void ConnectToRtpTransport();
  void CreateSrtpSessions();

  void OnRtpPacketReceived(const ReceivedIpPacket& packet) override;
  void OnRtcpPacketReceived(const ReceivedIpPacket& packet) override;
  void OnNetworkRouteChanged(
      std::optional<NetworkRoute> network_route) override;

  // Override the RtpTransport::OnWritableState.
  void OnWritableState(PacketTransportInternal* packet_transport) override;

  bool ProtectRtp(CopyOnWriteBuffer& buffer);
  // Overloaded version, outputs packet index.
  bool ProtectRtp(CopyOnWriteBuffer& buffer, int64_t* index);
  bool ProtectRtcp(CopyOnWriteBuffer& buffer);

  // Decrypts/verifies an invidiual RTP/RTCP packet.
  // If an HMAC is used, this will decrease the packet size.
  bool UnprotectRtp(CopyOnWriteBuffer& buffer);
  bool UnprotectRtcp(CopyOnWriteBuffer& buffer);

  const std::string content_name_;

  std::unique_ptr<SrtpSession> send_session_;
  std::unique_ptr<SrtpSession> recv_session_;
  std::unique_ptr<SrtpSession> send_rtcp_session_;
  std::unique_ptr<SrtpSession> recv_rtcp_session_;

  std::optional<int> send_crypto_suite_;
  std::optional<int> recv_crypto_suite_;
  ZeroOnFreeBuffer<uint8_t> send_key_;
  ZeroOnFreeBuffer<uint8_t> recv_key_;

  bool writable_ = false;

  bool external_auth_enabled_ = false;

  int rtp_abs_sendtime_extn_id_ = -1;

  int decryption_failure_count_ = 0;

  const FieldTrialsView& field_trials_;
};

}  // namespace webrtc

#endif  // PC_SRTP_TRANSPORT_H_
