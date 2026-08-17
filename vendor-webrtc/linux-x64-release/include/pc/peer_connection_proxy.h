/*
 *  Copyright 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_PEER_CONNECTION_PROXY_H_
#define PC_PEER_CONNECTION_PROXY_H_

#include <cstdint>
#include <functional>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "api/adaptation/resource.h"
#include "api/candidate.h"
#include "api/data_channel_event_observer_interface.h"
#include "api/data_channel_interface.h"
#include "api/dtls_transport_interface.h"
#include "api/jsep.h"
#include "api/media_stream_interface.h"
#include "api/media_types.h"
#include "api/peer_connection_interface.h"
#include "api/rtc_error.h"
#include "api/rtc_event_log_output.h"
#include "api/rtp_parameters.h"
#include "api/rtp_receiver_interface.h"
#include "api/rtp_sender_interface.h"
#include "api/rtp_transceiver_interface.h"
#include "api/scoped_refptr.h"
#include "api/sctp_transport_interface.h"
#include "api/set_local_description_observer_interface.h"
#include "api/set_remote_description_observer_interface.h"
#include "api/stats/rtc_stats_collector_callback.h"
#include "api/transport/bandwidth_estimation_settings.h"
#include "api/transport/bitrate_settings.h"
#include "api/transport/network_control.h"
#include "pc/proxy.h"
#include "rtc_base/thread.h"

namespace webrtc {

// PeerConnection proxy objects will be constructed with two thread pointers,
// signaling and network. The proxy macros don't have 'network' specific macros
// and support for a secondary thread is provided via 'SECONDARY' macros.
// TODO(deadbeef): Move this to .cc file. What threads methods are called on is
// an implementation detail.
BEGIN_PROXY_MAP(PeerConnection)
PROXY_PRIMARY_THREAD_DESTRUCTOR()
PROXY_METHOD0(scoped_refptr<StreamCollectionInterface>, local_streams)
PROXY_METHOD0(scoped_refptr<StreamCollectionInterface>, remote_streams)
PROXY_METHOD1(bool, AddStream, MediaStreamInterface*)
PROXY_METHOD1(void, RemoveStream, MediaStreamInterface*)
PROXY_METHOD2(RTCErrorOr<scoped_refptr<RtpSenderInterface>>,
              AddTrack,
              scoped_refptr<MediaStreamTrackInterface>,
              const std::vector<std::string>&)
PROXY_METHOD3(RTCErrorOr<scoped_refptr<RtpSenderInterface>>,
              AddTrack,
              scoped_refptr<MediaStreamTrackInterface>,
              const std::vector<std::string>&,
              const std::vector<RtpEncodingParameters>&)
PROXY_METHOD1(RTCError, RemoveTrackOrError, scoped_refptr<RtpSenderInterface>)
PROXY_METHOD1(RTCErrorOr<scoped_refptr<RtpTransceiverInterface>>,
              AddTransceiver,
              scoped_refptr<MediaStreamTrackInterface>)
PROXY_METHOD2(RTCErrorOr<scoped_refptr<RtpTransceiverInterface>>,
              AddTransceiver,
              scoped_refptr<MediaStreamTrackInterface>,
              const RtpTransceiverInit&)
PROXY_METHOD1(RTCErrorOr<scoped_refptr<RtpTransceiverInterface>>,
              AddTransceiver,
              webrtc::MediaType)
PROXY_METHOD2(RTCErrorOr<scoped_refptr<RtpTransceiverInterface>>,
              AddTransceiver,
              webrtc::MediaType,
              const RtpTransceiverInit&)
PROXY_METHOD2(scoped_refptr<RtpSenderInterface>,
              CreateSender,
              const std::string&,
              const std::string&)
PROXY_CONSTMETHOD0(std::vector<scoped_refptr<RtpSenderInterface>>, GetSenders)
PROXY_CONSTMETHOD0(std::vector<scoped_refptr<RtpReceiverInterface>>,
                   GetReceivers)
PROXY_CONSTMETHOD0(std::vector<scoped_refptr<RtpTransceiverInterface>>,
                   GetTransceivers)
PROXY_METHOD3(bool,
              GetStats,
              StatsObserver*,
              MediaStreamTrackInterface*,
              StatsOutputLevel)
PROXY_METHOD1(void, GetStats, RTCStatsCollectorCallback*)
PROXY_METHOD2(void,
              GetStats,
              scoped_refptr<RtpSenderInterface>,
              scoped_refptr<RTCStatsCollectorCallback>)
PROXY_METHOD2(void,
              GetStats,
              scoped_refptr<RtpReceiverInterface>,
              scoped_refptr<RTCStatsCollectorCallback>)
PROXY_METHOD0(void, ClearStatsCache)
PROXY_METHOD2(RTCErrorOr<scoped_refptr<DataChannelInterface>>,
              CreateDataChannelOrError,
              const std::string&,
              const DataChannelInit*)
PROXY_CONSTMETHOD0(const SessionDescriptionInterface*, local_description)
PROXY_CONSTMETHOD0(const SessionDescriptionInterface*, remote_description)
PROXY_CONSTMETHOD0(const SessionDescriptionInterface*,
                   current_local_description)
PROXY_CONSTMETHOD0(const SessionDescriptionInterface*,
                   current_remote_description)
PROXY_CONSTMETHOD0(const SessionDescriptionInterface*,
                   pending_local_description)
PROXY_CONSTMETHOD0(const SessionDescriptionInterface*,
                   pending_remote_description)
PROXY_METHOD0(void, RestartIce)
PROXY_METHOD2(void,
              CreateOffer,
              CreateSessionDescriptionObserver*,
              const RTCOfferAnswerOptions&)
PROXY_METHOD2(void,
              CreateAnswer,
              CreateSessionDescriptionObserver*,
              const RTCOfferAnswerOptions&)
PROXY_METHOD2(void,
              SetLocalDescription,
              std::unique_ptr<SessionDescriptionInterface>,
              scoped_refptr<SetLocalDescriptionObserverInterface>)
PROXY_METHOD1(void,
              SetLocalDescription,
              scoped_refptr<SetLocalDescriptionObserverInterface>)
PROXY_METHOD2(void,
              SetLocalDescription,
              SetSessionDescriptionObserver*,
              SessionDescriptionInterface*)
PROXY_METHOD1(void, SetLocalDescription, SetSessionDescriptionObserver*)
PROXY_METHOD2(void,
              SetRemoteDescription,
              std::unique_ptr<SessionDescriptionInterface>,
              scoped_refptr<SetRemoteDescriptionObserverInterface>)
PROXY_METHOD2(void,
              SetRemoteDescription,
              SetSessionDescriptionObserver*,
              SessionDescriptionInterface*)
PROXY_METHOD1(bool, ShouldFireNegotiationNeededEvent, uint32_t)
PROXY_METHOD0(PeerConnectionInterface::RTCConfiguration, GetConfiguration)
PROXY_METHOD1(RTCError,
              SetConfiguration,
              const PeerConnectionInterface::RTCConfiguration&)
PROXY_METHOD1(bool, AddIceCandidate, const IceCandidateInterface*)
PROXY_METHOD2(void,
              AddIceCandidate,
              std::unique_ptr<IceCandidateInterface>,
              std::function<void(RTCError)>)
PROXY_METHOD1(bool, RemoveIceCandidates, const std::vector<Candidate>&)
PROXY_METHOD1(RTCError, SetBitrate, const BitrateSettings&)
PROXY_METHOD1(void,
              ReconfigureBandwidthEstimation,
              const BandwidthEstimationSettings&)
PROXY_METHOD1(void, SetAudioPlayout, bool)
PROXY_METHOD1(void, SetAudioRecording, bool)
// This method will be invoked on the network thread. See
// PeerConnectionFactory::CreatePeerConnectionOrError for more details.
PROXY_SECONDARY_METHOD1(scoped_refptr<DtlsTransportInterface>,
                        LookupDtlsTransportByMid,
                        const std::string&)
// This method will be invoked on the network thread. See
// PeerConnectionFactory::CreatePeerConnectionOrError for more details.
PROXY_SECONDARY_CONSTMETHOD0(scoped_refptr<SctpTransportInterface>,
                             GetSctpTransport)
PROXY_METHOD0(SignalingState, signaling_state)
PROXY_METHOD0(IceConnectionState, ice_connection_state)
PROXY_METHOD0(IceConnectionState, standardized_ice_connection_state)
PROXY_METHOD0(PeerConnectionState, peer_connection_state)
PROXY_METHOD0(IceGatheringState, ice_gathering_state)
PROXY_METHOD0(std::optional<bool>, can_trickle_ice_candidates)
PROXY_METHOD1(void, AddAdaptationResource, scoped_refptr<Resource>)
PROXY_METHOD2(bool,
              StartRtcEventLog,
              std::unique_ptr<RtcEventLogOutput>,
              int64_t)
PROXY_METHOD1(bool, StartRtcEventLog, std::unique_ptr<RtcEventLogOutput>)
PROXY_METHOD0(void, StopRtcEventLog)
PROXY_METHOD1(void,
              SetDataChannelEventObserver,
              std::unique_ptr<DataChannelEventObserverInterface>)
PROXY_METHOD0(void, Close)
PROXY_METHOD0(NetworkControllerInterface*, GetNetworkController)
BYPASS_PROXY_CONSTMETHOD0(Thread*, signaling_thread)
END_PROXY_MAP(PeerConnection)

}  // namespace webrtc

#endif  // PC_PEER_CONNECTION_PROXY_H_
