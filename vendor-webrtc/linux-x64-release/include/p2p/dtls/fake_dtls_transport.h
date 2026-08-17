/*
 *  Copyright 2017 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef P2P_DTLS_FAKE_DTLS_TRANSPORT_H_
#define P2P_DTLS_FAKE_DTLS_TRANSPORT_H_

#include <cstdint>
#include <cstring>
#include <memory>
#include <optional>
#include <string>
#include <utility>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/dtls_transport_interface.h"
#include "api/rtc_error.h"
#include "api/scoped_refptr.h"
#include "p2p/base/ice_transport_internal.h"
#include "p2p/base/packet_transport_internal.h"
#include "p2p/dtls/dtls_transport_internal.h"
#include "p2p/test/fake_ice_transport.h"
#include "rtc_base/async_packet_socket.h"
#include "rtc_base/buffer.h"
#include "rtc_base/checks.h"
#include "rtc_base/fake_ssl_identity.h"
#include "rtc_base/logging.h"
#include "rtc_base/network/received_packet.h"
#include "rtc_base/network_route.h"
#include "rtc_base/rtc_certificate.h"
#include "rtc_base/socket.h"
#include "rtc_base/ssl_certificate.h"
#include "rtc_base/ssl_fingerprint.h"
#include "rtc_base/ssl_stream_adapter.h"
#include "rtc_base/thread.h"

namespace webrtc {

// Fake DTLS transport which is implemented by wrapping a fake ICE transport.
// Doesn't interact directly with fake ICE transport for anything other than
// sending packets.
class FakeDtlsTransport : public DtlsTransportInternal {
 public:
  explicit FakeDtlsTransport(FakeIceTransport* ice_transport)
      : ice_transport_(ice_transport),
        transport_name_(ice_transport->transport_name()),
        component_(ice_transport->component()),
        dtls_fingerprint_("", nullptr) {
    RTC_DCHECK(ice_transport_);
    ice_transport_->RegisterReceivedPacketCallback(
        this, [&](PacketTransportInternal* transport,
                  const ReceivedIpPacket& packet) {
          OnIceTransportReadPacket(transport, packet);
        });
    ice_transport_->SignalNetworkRouteChanged.connect(
        this, &FakeDtlsTransport::OnNetworkRouteChanged);
  }

  explicit FakeDtlsTransport(std::unique_ptr<FakeIceTransport> ice)
      : owned_ice_transport_(std::move(ice)),
        transport_name_(owned_ice_transport_->transport_name()),
        component_(owned_ice_transport_->component()),
        dtls_fingerprint_("", ArrayView<const uint8_t>()) {
    ice_transport_ = owned_ice_transport_.get();
    ice_transport_->RegisterReceivedPacketCallback(
        this, [&](PacketTransportInternal* transport,
                  const ReceivedIpPacket& packet) {
          OnIceTransportReadPacket(transport, packet);
        });
    ice_transport_->SignalNetworkRouteChanged.connect(
        this, &FakeDtlsTransport::OnNetworkRouteChanged);
  }

  // If this constructor is called, a new fake ICE transport will be created,
  // and this FakeDtlsTransport will take the ownership.
  FakeDtlsTransport(const std::string& name, int component)
      : FakeDtlsTransport(std::make_unique<FakeIceTransport>(name, component)) {
  }
  FakeDtlsTransport(const std::string& name,
                    int component,
                    Thread* network_thread)
      : FakeDtlsTransport(std::make_unique<FakeIceTransport>(name,
                                                             component,
                                                             network_thread)) {}

  ~FakeDtlsTransport() override {
    if (dest_ && dest_->dest_ == this) {
      dest_->dest_ = nullptr;
    }
    ice_transport_->DeregisterReceivedPacketCallback(this);
  }

  // Get inner fake ICE transport.
  FakeIceTransport* fake_ice_transport() { return ice_transport_; }

  // If async, will send packets by "Post"-ing to message queue instead of
  // synchronously "Send"-ing.
  void SetAsync(bool async) { ice_transport_->SetAsync(async); }
  void SetAsyncDelay(int delay_ms) { ice_transport_->SetAsyncDelay(delay_ms); }

  // SetWritable, SetReceiving and SetDestination are the main methods that can
  // be used for testing, to simulate connectivity or lack thereof.
  void SetWritable(bool writable) {
    ice_transport_->SetWritable(writable);
    set_writable(writable);
  }
  void SetReceiving(bool receiving) {
    ice_transport_->SetReceiving(receiving);
    set_receiving(receiving);
  }
  void SetDtlsState(DtlsTransportState state) {
    dtls_state_ = state;
    SendDtlsState(this, dtls_state_);
  }

  // Simulates the two DTLS transports connecting to each other.
  // If `asymmetric` is true this method only affects this FakeDtlsTransport.
  // If false, it affects `dest` as well.
  void SetDestination(FakeDtlsTransport* dest, bool asymmetric = false) {
    if (dest == dest_) {
      return;
    }
    RTC_DCHECK(!dest || !dest_)
        << "Changing fake destination from one to another is not supported.";
    if (dest && !dest_) {
      // This simulates the DTLS handshake.
      dest_ = dest;
      if (local_cert_ && dest_->local_cert_) {
        do_dtls_ = true;
        RTC_LOG(LS_INFO) << "FakeDtlsTransport is doing DTLS";
      } else {
        do_dtls_ = false;
        RTC_LOG(LS_INFO) << "FakeDtlsTransport is not doing DTLS";
      }
      SetWritable(true);
      if (!asymmetric) {
        dest->SetDestination(this, true);
      }
      // If the `dtls_role_` is unset, set it to SSL_CLIENT by default.
      if (!dtls_role_) {
        dtls_role_ = std::move(webrtc::SSL_CLIENT);
      }
      SetDtlsState(DtlsTransportState::kConnected);
      ice_transport_->SetDestination(
          static_cast<FakeIceTransport*>(dest->ice_transport()), asymmetric);
    } else {
      // Simulates loss of connectivity, by asymmetrically forgetting dest_.
      dest_ = nullptr;
      SetWritable(false);
      ice_transport_->SetDestination(nullptr, asymmetric);
    }
  }

  // Fake DtlsTransportInternal implementation.
  DtlsTransportState dtls_state() const override { return dtls_state_; }
  const std::string& transport_name() const override { return transport_name_; }
  int component() const override { return component_; }
  const SSLFingerprint& dtls_fingerprint() const { return dtls_fingerprint_; }
  RTCError SetRemoteParameters(absl::string_view alg,
                               const uint8_t* digest,
                               size_t digest_len,
                               std::optional<SSLRole> role) {
    if (role) {
      SetDtlsRole(*role);
    }
    SetRemoteFingerprint(alg, digest, digest_len);
    return RTCError::OK();
  }
  bool SetRemoteFingerprint(absl::string_view alg,
                            const uint8_t* digest,
                            size_t digest_len) {
    dtls_fingerprint_ = SSLFingerprint(alg, MakeArrayView(digest, digest_len));
    return true;
  }
  bool SetDtlsRole(SSLRole role) override {
    dtls_role_ = std::move(role);
    return true;
  }
  bool GetDtlsRole(SSLRole* role) const override {
    if (!dtls_role_) {
      return false;
    }
    *role = *dtls_role_;
    return true;
  }
  bool SetLocalCertificate(
      const scoped_refptr<RTCCertificate>& certificate) override {
    do_dtls_ = true;
    local_cert_ = certificate;
    return true;
  }
  void SetRemoteSSLCertificate(FakeSSLCertificate* cert) {
    remote_cert_ = cert;
  }
  bool IsDtlsActive() const override { return do_dtls_; }
  bool GetSslVersionBytes(int* version) const override {
    if (!do_dtls_) {
      return false;
    }
    *version = 0x0102;
    return true;
  }
  bool GetSrtpCryptoSuite(int* crypto_suite) const override {
    if (!do_dtls_) {
      return false;
    }
    *crypto_suite = crypto_suite_;
    return true;
  }
  void SetSrtpCryptoSuite(int crypto_suite) { crypto_suite_ = crypto_suite; }

  bool GetSslCipherSuite(int* cipher_suite) const override {
    if (ssl_cipher_suite_) {
      *cipher_suite = *ssl_cipher_suite_;
      return true;
    }
    return false;
  }
  void SetSslCipherSuite(std::optional<int> cipher_suite) {
    ssl_cipher_suite_ = cipher_suite;
  }

  std::optional<absl::string_view> GetTlsCipherSuiteName() const override {
    return "FakeTlsCipherSuite";
  }
  uint16_t GetSslPeerSignatureAlgorithm() const override { return 0; }
  scoped_refptr<RTCCertificate> GetLocalCertificate() const override {
    return local_cert_;
  }
  std::unique_ptr<SSLCertChain> GetRemoteSSLCertChain() const override {
    if (!remote_cert_) {
      return nullptr;
    }
    return std::make_unique<SSLCertChain>(remote_cert_->Clone());
  }
  bool ExportSrtpKeyingMaterial(
      ZeroOnFreeBuffer<uint8_t>& keying_material) override {
    if (do_dtls_) {
      std::memset(keying_material.data(), 0xff, keying_material.size());
    }
    return do_dtls_;
  }
  void set_ssl_max_protocol_version(SSLProtocolVersion version) {
    ssl_max_version_ = version;
  }
  SSLProtocolVersion ssl_max_protocol_version() const {
    return ssl_max_version_;
  }

  IceTransportInternal* ice_transport() override { return ice_transport_; }

  // PacketTransportInternal implementation, which passes through to fake ICE
  // transport for sending actual packets.
  bool writable() const override { return writable_; }
  bool receiving() const override { return receiving_; }
  int SendPacket(const char* data,
                 size_t len,
                 const AsyncSocketPacketOptions& options,
                 int flags) override {
    // We expect only SRTP packets to be sent through this interface.
    if (flags != PF_SRTP_BYPASS && flags != 0) {
      return -1;
    }
    return ice_transport_->SendPacket(data, len, options, flags);
  }
  int SetOption(Socket::Option opt, int value) override {
    return ice_transport_->SetOption(opt, value);
  }
  bool GetOption(Socket::Option opt, int* value) override {
    return ice_transport_->GetOption(opt, value);
  }
  int GetError() override { return ice_transport_->GetError(); }

  std::optional<NetworkRoute> network_route() const override {
    return ice_transport_->network_route();
  }

 private:
  void OnIceTransportReadPacket(PacketTransportInternal* /* ice_ */,
                                const ReceivedIpPacket& packet) {
    NotifyPacketReceived(packet);
  }

  void set_receiving(bool receiving) {
    if (receiving_ == receiving) {
      return;
    }
    receiving_ = receiving;
    SignalReceivingState(this);
  }

  void set_writable(bool writable) {
    if (writable_ == writable) {
      return;
    }
    writable_ = writable;
    if (writable_) {
      SignalReadyToSend(this);
    }
    SignalWritableState(this);
  }

  void OnNetworkRouteChanged(std::optional<NetworkRoute> network_route) {
    SignalNetworkRouteChanged(network_route);
  }

  FakeIceTransport* ice_transport_;
  std::unique_ptr<FakeIceTransport> owned_ice_transport_;
  std::string transport_name_;
  int component_;
  FakeDtlsTransport* dest_ = nullptr;
  scoped_refptr<RTCCertificate> local_cert_;
  FakeSSLCertificate* remote_cert_ = nullptr;
  bool do_dtls_ = false;
  SSLProtocolVersion ssl_max_version_ = webrtc::SSL_PROTOCOL_DTLS_12;
  SSLFingerprint dtls_fingerprint_;
  std::optional<SSLRole> dtls_role_;
  int crypto_suite_ = webrtc::kSrtpAes128CmSha1_80;
  std::optional<int> ssl_cipher_suite_;

  DtlsTransportState dtls_state_ = DtlsTransportState::kNew;

  bool receiving_ = false;
  bool writable_ = false;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::FakeDtlsTransport;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // P2P_DTLS_FAKE_DTLS_TRANSPORT_H_
