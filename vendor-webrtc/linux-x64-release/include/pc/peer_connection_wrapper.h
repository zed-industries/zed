/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_PEER_CONNECTION_WRAPPER_H_
#define PC_PEER_CONNECTION_WRAPPER_H_

#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "api/data_channel_interface.h"
#include "api/function_view.h"
#include "api/jsep.h"
#include "api/media_stream_interface.h"
#include "api/media_types.h"
#include "api/peer_connection_interface.h"
#include "api/rtc_error.h"
#include "api/rtp_parameters.h"
#include "api/rtp_sender_interface.h"
#include "api/rtp_transceiver_interface.h"
#include "api/scoped_refptr.h"
#include "api/stats/rtc_stats_report.h"
#include "pc/peer_connection.h"
#include "pc/test/mock_peer_connection_observers.h"

namespace webrtc {

// Class that wraps a PeerConnection so that it is easier to use in unit tests.
// Namely, gives a synchronous API for the event-callback-based API of
// PeerConnection and provides an observer object that stores information from
// PeerConnectionObserver callbacks.
//
// This is intended to be subclassed if additional information needs to be
// stored with the PeerConnection (e.g., fake PeerConnection parameters so that
// tests can be written against those interactions). The base
// PeerConnectionWrapper should only have helper methods that are broadly
// useful. More specific helper methods should be created in the test-specific
// subclass.
//
// The wrapper is intended to be constructed by specialized factory methods on
// a test fixture class then used as a local variable in each test case.
class PeerConnectionWrapper {
 public:
  // Constructs a PeerConnectionWrapper from the given PeerConnection.
  // The given PeerConnectionFactory should be the factory that created the
  // PeerConnection and the MockPeerConnectionObserver should be the observer
  // that is watching the PeerConnection.
  PeerConnectionWrapper(
      scoped_refptr<PeerConnectionFactoryInterface> pc_factory,
      scoped_refptr<PeerConnectionInterface> pc,
      std::unique_ptr<MockPeerConnectionObserver> observer);
  virtual ~PeerConnectionWrapper();

  PeerConnectionFactoryInterface* pc_factory();
  PeerConnectionInterface* pc();
  MockPeerConnectionObserver* observer();

  PeerConnection* GetInternalPeerConnection();

  // Calls the underlying PeerConnection's CreateOffer method and returns the
  // resulting SessionDescription once it is available. If the method call
  // failed, null is returned.
  std::unique_ptr<SessionDescriptionInterface> CreateOffer(
      const PeerConnectionInterface::RTCOfferAnswerOptions& options,
      std::string* error_out = nullptr);
  // Calls CreateOffer with default options.
  std::unique_ptr<SessionDescriptionInterface> CreateOffer();
  // Calls CreateOffer and sets a copy of the offer as the local description.
  std::unique_ptr<SessionDescriptionInterface> CreateOfferAndSetAsLocal(
      const PeerConnectionInterface::RTCOfferAnswerOptions& options);
  // Calls CreateOfferAndSetAsLocal with default options.
  std::unique_ptr<SessionDescriptionInterface> CreateOfferAndSetAsLocal();

  // Calls the underlying PeerConnection's CreateAnswer method and returns the
  // resulting SessionDescription once it is available. If the method call
  // failed, null is returned.
  std::unique_ptr<SessionDescriptionInterface> CreateAnswer(
      const PeerConnectionInterface::RTCOfferAnswerOptions& options,
      std::string* error_out = nullptr);
  // Calls CreateAnswer with the default options.
  std::unique_ptr<SessionDescriptionInterface> CreateAnswer();
  // Calls CreateAnswer and sets a copy of the offer as the local description.
  std::unique_ptr<SessionDescriptionInterface> CreateAnswerAndSetAsLocal(
      const PeerConnectionInterface::RTCOfferAnswerOptions& options);
  // Calls CreateAnswerAndSetAsLocal with default options.
  std::unique_ptr<SessionDescriptionInterface> CreateAnswerAndSetAsLocal();
  std::unique_ptr<SessionDescriptionInterface> CreateRollback();

  // Calls the underlying PeerConnection's SetLocalDescription method with the
  // given session description and waits for the success/failure response.
  // Returns true if the description was successfully set.
  bool SetLocalDescription(std::unique_ptr<SessionDescriptionInterface> desc,
                           std::string* error_out = nullptr);
  bool SetLocalDescription(std::unique_ptr<SessionDescriptionInterface> desc,
                           RTCError* error_out);
  // Calls the underlying PeerConnection's SetRemoteDescription method with the
  // given session description and waits for the success/failure response.
  // Returns true if the description was successfully set.
  bool SetRemoteDescription(std::unique_ptr<SessionDescriptionInterface> desc,
                            std::string* error_out = nullptr);
  bool SetRemoteDescription(std::unique_ptr<SessionDescriptionInterface> desc,
                            RTCError* error_out);

  // Does a round of offer/answer with the local PeerConnectionWrapper
  // generating the offer and the given PeerConnectionWrapper generating the
  // answer.
  // Equivalent to:
  // 1. this->CreateOffer(offer_options)
  // 2. this->SetLocalDescription(offer)
  // 3. answerer->SetRemoteDescription(offer)
  // 4. answerer->CreateAnswer(answer_options)
  // 5. answerer->SetLocalDescription(answer)
  // 6. this->SetRemoteDescription(answer)
  // Returns true if all steps succeed, false otherwise.
  // Suggested usage:
  //   ASSERT_TRUE(caller->ExchangeOfferAnswerWith(callee.get()));
  bool ExchangeOfferAnswerWith(PeerConnectionWrapper* answerer);
  bool ExchangeOfferAnswerWith(
      PeerConnectionWrapper* answerer,
      const PeerConnectionInterface::RTCOfferAnswerOptions& offer_options,
      const PeerConnectionInterface::RTCOfferAnswerOptions& answer_options);

  // The following are wrappers for the underlying PeerConnection's
  // AddTransceiver method. They return the result of calling AddTransceiver
  // with the given arguments, DCHECKing if there is an error.
  scoped_refptr<RtpTransceiverInterface> AddTransceiver(
      webrtc::MediaType media_type);
  scoped_refptr<RtpTransceiverInterface> AddTransceiver(
      webrtc::MediaType media_type,
      const RtpTransceiverInit& init);
  scoped_refptr<RtpTransceiverInterface> AddTransceiver(
      scoped_refptr<MediaStreamTrackInterface> track);
  scoped_refptr<RtpTransceiverInterface> AddTransceiver(
      scoped_refptr<MediaStreamTrackInterface> track,
      const RtpTransceiverInit& init);

  // Returns a new dummy audio track with the given label.
  scoped_refptr<AudioTrackInterface> CreateAudioTrack(const std::string& label);

  // Returns a new dummy video track with the given label.
  scoped_refptr<VideoTrackInterface> CreateVideoTrack(const std::string& label);

  // Wrapper for the underlying PeerConnection's AddTrack method. DCHECKs if
  // AddTrack fails.
  scoped_refptr<RtpSenderInterface> AddTrack(
      scoped_refptr<MediaStreamTrackInterface> track,
      const std::vector<std::string>& stream_ids = {});

  scoped_refptr<RtpSenderInterface> AddTrack(
      scoped_refptr<MediaStreamTrackInterface> track,
      const std::vector<std::string>& stream_ids,
      const std::vector<RtpEncodingParameters>& init_send_encodings);

  // Calls the underlying PeerConnection's AddTrack method with an audio media
  // stream track not bound to any source.
  scoped_refptr<RtpSenderInterface> AddAudioTrack(
      const std::string& track_label,
      const std::vector<std::string>& stream_ids = {});

  // Calls the underlying PeerConnection's AddTrack method with a video media
  // stream track fed by a FakeVideoTrackSource.
  scoped_refptr<RtpSenderInterface> AddVideoTrack(
      const std::string& track_label,
      const std::vector<std::string>& stream_ids = {});

  // Calls the underlying PeerConnection's CreateDataChannel method with default
  // initialization parameters.
  scoped_refptr<DataChannelInterface> CreateDataChannel(
      const std::string& label,
      const std::optional<DataChannelInit>& config = std::nullopt);

  // Returns the signaling state of the underlying PeerConnection.
  PeerConnectionInterface::SignalingState signaling_state();

  // Returns true if ICE has finished gathering candidates.
  bool IsIceGatheringDone();

  // Returns true if ICE has established a connection.
  bool IsIceConnected();

  // Calls GetStats() on the underlying PeerConnection and returns the resulting
  // report. If GetStats() fails, this method returns null and fails the test.
  scoped_refptr<const RTCStatsReport> GetStats();

 private:
  std::unique_ptr<SessionDescriptionInterface> CreateSdp(
      FunctionView<void(CreateSessionDescriptionObserver*)> fn,
      std::string* error_out);
  bool SetSdp(FunctionView<void(SetSessionDescriptionObserver*)> fn,
              std::string* error_out);

  scoped_refptr<PeerConnectionFactoryInterface> pc_factory_;
  std::unique_ptr<MockPeerConnectionObserver> observer_;
  scoped_refptr<PeerConnectionInterface> pc_;
};

}  // namespace webrtc

#endif  // PC_PEER_CONNECTION_WRAPPER_H_
