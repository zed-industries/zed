/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_PEER_CONNECTION_INTERNAL_H_
#define PC_PEER_CONNECTION_INTERNAL_H_

#include <map>
#include <memory>
#include <optional>
#include <set>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/audio/audio_device.h"
#include "api/candidate.h"
#include "api/crypto/crypto_options.h"
#include "api/data_channel_interface.h"
#include "api/field_trials_view.h"
#include "api/jsep.h"
#include "api/media_stream_interface.h"
#include "api/media_types.h"
#include "api/peer_connection_interface.h"
#include "api/rtc_error.h"
#include "api/rtp_transceiver_interface.h"
#include "api/scoped_refptr.h"
#include "api/sctp_transport_interface.h"
#include "call/call.h"
#include "call/payload_type_picker.h"
#include "p2p/base/port.h"
#include "p2p/base/port_allocator.h"
#include "pc/data_channel_utils.h"
#include "pc/jsep_transport_controller.h"
#include "pc/peer_connection_message_handler.h"
#include "pc/rtp_transceiver.h"
#include "pc/rtp_transmission_manager.h"
#include "pc/session_description.h"
#include "pc/transport_stats.h"
#include "pc/usage_pattern.h"
#include "rtc_base/rtc_certificate.h"
#include "rtc_base/ssl_certificate.h"
#include "rtc_base/ssl_stream_adapter.h"
#include "rtc_base/thread.h"

namespace webrtc {

class DataChannelController;
class LegacyStatsCollector;

// This interface defines the functions that are needed for
// SdpOfferAnswerHandler to access PeerConnection internal state.
class PeerConnectionSdpMethods {
 public:
  virtual ~PeerConnectionSdpMethods() = default;

  // The SDP session ID as defined by RFC 3264.
  virtual std::string session_id() const = 0;

  // Returns true if the ICE restart flag above was set, and no ICE restart has
  // occurred yet for this transport (by applying a local description with
  // changed ufrag/password). If the transport has been deleted as a result of
  // bundling, returns false.
  virtual bool NeedsIceRestart(const std::string& content_name) const = 0;

  virtual std::optional<std::string> sctp_mid() const = 0;

  // Functions below this comment are known to only be accessed
  // from SdpOfferAnswerHandler.
  // Return a pointer to the active configuration.
  virtual const PeerConnectionInterface::RTCConfiguration* configuration()
      const = 0;

  // Report the UMA metric BundleUsage for the given remote description.
  virtual void ReportSdpBundleUsage(
      const SessionDescriptionInterface& remote_description) = 0;

  virtual PeerConnectionMessageHandler* message_handler() = 0;
  virtual RtpTransmissionManager* rtp_manager() = 0;
  virtual const RtpTransmissionManager* rtp_manager() const = 0;
  virtual bool dtls_enabled() const = 0;
  virtual const PeerConnectionFactoryInterface::Options* options() const = 0;

  // Returns the CryptoOptions for this PeerConnection. This will always
  // return the RTCConfiguration.crypto_options if set and will only default
  // back to the PeerConnectionFactory settings if nothing was set.
  virtual CryptoOptions GetCryptoOptions() = 0;
  virtual JsepTransportController* transport_controller_s() = 0;
  virtual JsepTransportController* transport_controller_n() = 0;
  virtual DataChannelController* data_channel_controller() = 0;
  virtual PortAllocator* port_allocator() = 0;
  virtual LegacyStatsCollector* legacy_stats() = 0;
  // Returns the observer. Will crash on CHECK if the observer is removed.
  virtual PeerConnectionObserver* Observer() const = 0;
  virtual std::optional<SSLRole> GetSctpSslRole_n() = 0;
  virtual PeerConnectionInterface::IceConnectionState
  ice_connection_state_internal() = 0;
  virtual void SetIceConnectionState(
      PeerConnectionInterface::IceConnectionState new_state) = 0;
  virtual void NoteUsageEvent(UsageEvent event) = 0;
  virtual bool IsClosed() const = 0;
  // Returns true if the PeerConnection is configured to use Unified Plan
  // semantics for creating offers/answers and setting local/remote
  // descriptions. If this is true the RtpTransceiver API will also be available
  // to the user. If this is false, Plan B semantics are assumed.
  // TODO(bugs.webrtc.org/8530): Flip the default to be Unified Plan once
  // sufficient time has passed.
  virtual bool IsUnifiedPlan() const = 0;
  virtual bool ValidateBundleSettings(
      const SessionDescription* desc,
      const std::map<std::string, const ContentGroup*>&
          bundle_groups_by_mid) = 0;

  // Internal implementation for AddTransceiver family of methods. If
  // `fire_callback` is set, fires OnRenegotiationNeeded callback if successful.
  virtual RTCErrorOr<scoped_refptr<RtpTransceiverInterface>> AddTransceiver(
      webrtc::MediaType media_type,
      scoped_refptr<MediaStreamTrackInterface> track,
      const RtpTransceiverInit& init,
      bool fire_callback = true) = 0;
  // Asynchronously calls SctpTransport::Start() on the network thread for
  // `sctp_mid()` if set. Called as part of setting the local description.
  virtual RTCError StartSctpTransport(const SctpOptions& options) = 0;
  [[deprecated("Call with SctpOptions")]]
  virtual void StartSctpTransport(int local_port,
                                  int remote_port,
                                  int max_message_size) {
    StartSctpTransport({.local_port = local_port,
                        .remote_port = remote_port,
                        .max_message_size = max_message_size});
  }

  // Asynchronously adds a remote candidate on the network thread.
  virtual void AddRemoteCandidate(absl::string_view mid,
                                  const Candidate& candidate) = 0;

  virtual Call* call_ptr() = 0;
  // Returns true if SRTP (either using DTLS-SRTP or SDES) is required by
  // this session.
  virtual bool SrtpRequired() const = 0;
  // Initializes the data channel transport for the peerconnection instance.
  // This will have the effect that `sctp_mid()` and `sctp_transport_name()`
  // will return a set value (even though it might be an empty string) and the
  // dc transport will be initialized on the network thread.
  virtual bool CreateDataChannelTransport(absl::string_view mid) = 0;
  // Tears down the data channel transport state and clears the `sctp_mid()` and
  // `sctp_transport_name()` properties.
  virtual void DestroyDataChannelTransport(RTCError error) = 0;
  virtual const FieldTrialsView& trials() const = 0;

  virtual void ClearStatsCache() = 0;
  // Keeps track of assigned payload types and comes up with reasonable
  // suggestions when new PTs need to be assigned.
  virtual PayloadTypePicker& payload_type_picker() = 0;
};

// Functions defined in this class are called by other objects,
// but not by SdpOfferAnswerHandler.
class PeerConnectionInternal : public PeerConnectionInterface,
                               public PeerConnectionSdpMethods {
 public:
  virtual Thread* network_thread() const = 0;
  virtual Thread* worker_thread() const = 0;

  // Returns true if we were the initial offerer.
  virtual bool initial_offerer() const = 0;

  virtual std::vector<
      scoped_refptr<RtpTransceiverProxyWithInternal<RtpTransceiver>>>
  GetTransceiversInternal() const = 0;

  // Call on the network thread to fetch stats for all the data channels.
  // TODO(tommi): Make pure virtual after downstream updates.
  virtual std::vector<DataChannelStats> GetDataChannelStats() const {
    return {};
  }

  virtual std::optional<std::string> sctp_transport_name() const = 0;

  virtual CandidateStatsList GetPooledCandidateStats() const = 0;

  // Returns a map from transport name to transport stats for all given
  // transport names.
  // Must be called on the network thread.
  virtual std::map<std::string, TransportStats> GetTransportStatsByNames(
      const std::set<std::string>& transport_names) = 0;

  virtual Call::Stats GetCallStats() = 0;

  virtual std::optional<AudioDeviceModule::Stats> GetAudioDeviceStats() = 0;

  virtual bool GetLocalCertificate(
      const std::string& transport_name,
      scoped_refptr<RTCCertificate>* certificate) = 0;
  virtual std::unique_ptr<SSLCertChain> GetRemoteSSLCertChain(
      const std::string& transport_name) = 0;

  // Returns true if there was an ICE restart initiated by the remote offer.
  virtual bool IceRestartPending(const std::string& content_name) const = 0;

  // Get SSL role for an arbitrary m= section (handles bundling correctly).
  virtual bool GetSslRole(const std::string& content_name, SSLRole* role) = 0;
  // Functions needed by DataChannelController
  virtual void NoteDataAddedEvent() {}
  // Handler for sctp data channel state changes.
  // The `channel_id` is the same unique identifier as used in
  // `DataChannelStats::internal_id and
  // `RTCDataChannelStats::data_channel_identifier`.
  virtual void OnSctpDataChannelStateChanged(
      int channel_id,
      DataChannelInterface::DataState state) {}
};

}  // namespace webrtc

#endif  // PC_PEER_CONNECTION_INTERNAL_H_
