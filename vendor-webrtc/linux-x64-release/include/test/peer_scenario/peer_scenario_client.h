/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_PEER_SCENARIO_PEER_SCENARIO_CLIENT_H_
#define TEST_PEER_SCENARIO_PEER_SCENARIO_CLIENT_H_

#include <functional>
#include <list>
#include <map>
#include <memory>
#include <string>
#include <vector>

#include "absl/memory/memory.h"
#include "api/peer_connection_interface.h"
#include "api/test/network_emulation_manager.h"
#include "api/test/time_controller.h"
#include "pc/test/frame_generator_capturer_video_track_source.h"
#include "test/create_frame_generator_capturer.h"
#include "test/logging/log_writer.h"

namespace webrtc {
namespace test {

// Wrapper for a PeerConnection for use in PeerScenario tests. It's intended to
// be a minimal wrapper for a peer connection that's simple to use in testing.
// In particular the constructor hides a lot of the required setup for a peer
// connection.
class PeerScenarioClient {
 public:
  struct CallbackHandlers {
    std::vector<std::function<void(PeerConnectionInterface::SignalingState)>>
        on_signaling_change;
    std::vector<
        std::function<void(webrtc::scoped_refptr<DataChannelInterface>)>>
        on_data_channel;
    std::vector<std::function<void()>> on_renegotiation_needed;
    std::vector<
        std::function<void(PeerConnectionInterface::IceConnectionState)>>
        on_standardized_ice_connection_change;
    std::vector<
        std::function<void(PeerConnectionInterface::PeerConnectionState)>>
        on_connection_change;
    std::vector<std::function<void(PeerConnectionInterface::IceGatheringState)>>
        on_ice_gathering_change;
    std::vector<std::function<void(const IceCandidateInterface*)>>
        on_ice_candidate;
    std::vector<std::function<void(const std::string&,
                                   int,
                                   const std::string&,
                                   int,
                                   const std::string&)>>
        on_ice_candidate_error;
    std::vector<std::function<void(const std::vector<webrtc::Candidate>&)>>
        on_ice_candidates_removed;
    std::vector<std::function<void(
        webrtc::scoped_refptr<RtpReceiverInterface>,
        const std::vector<webrtc::scoped_refptr<MediaStreamInterface>>&)>>
        on_add_track;
    std::vector<
        std::function<void(webrtc::scoped_refptr<RtpTransceiverInterface>)>>
        on_track;
    std::vector<
        std::function<void(webrtc::scoped_refptr<RtpReceiverInterface>)>>
        on_remove_track;
  };
  struct Config {
    // WebRTC only support one audio device that is setup up on construction, so
    // we provide the audio generator configuration here rather than on creation
    // of the tracks. This is unlike video, where multiple capture sources can
    // be used at the same time.
    struct AudioSource {
      int sample_rate = 48000;
      int channels = 1;
      struct PulsedNoise {
        double amplitude = 0.1;
      };
      std::optional<PulsedNoise> pulsed_noise = PulsedNoise();
    } audio;
    struct Video {
      bool use_fake_codecs = false;
    } video;
    // The created endpoints can be accessed using the map key as `index` in
    // PeerScenarioClient::endpoint(index).
    std::map<int, EmulatedEndpointConfig> endpoints = {
        {0, EmulatedEndpointConfig()}};
    CallbackHandlers handlers;
    PeerConnectionInterface::RTCConfiguration rtc_config;
    bool disable_encryption = false;
    Config() { rtc_config.sdp_semantics = SdpSemantics::kUnifiedPlan; }
  };

  struct VideoSendTrackConfig {
    FrameGeneratorCapturerConfig generator;
    bool screencast = false;
  };

  struct AudioSendTrack {
    scoped_refptr<AudioTrackInterface> track;
    scoped_refptr<RtpSenderInterface> sender;
  };

  struct VideoSendTrack {
    // Raw pointer to the capturer owned by `source`.
    FrameGeneratorCapturer* capturer;
    scoped_refptr<FrameGeneratorCapturerVideoTrackSource> source;
    scoped_refptr<VideoTrackInterface> track;
    scoped_refptr<RtpSenderInterface> sender;
  };

  PeerScenarioClient(
      NetworkEmulationManager* net,
      Thread* signaling_thread,
      std::unique_ptr<LogWriterFactoryInterface> log_writer_factory,
      Config config);

  PeerConnectionFactoryInterface* factory() { return pc_factory_.get(); }
  PeerConnectionInterface* pc() {
    RTC_DCHECK_RUN_ON(signaling_thread_);
    return peer_connection_.get();
  }
  Thread* thread() { return signaling_thread_; }
  Clock* clock() { return Clock::GetRealTimeClock(); }

  // Returns the endpoint created from the EmulatedEndpointConfig with the same
  // index in PeerScenarioClient::config.
  EmulatedEndpoint* endpoint(int index = 0);

  AudioSendTrack CreateAudio(std::string track_id, AudioOptions options);
  VideoSendTrack CreateVideo(std::string track_id, VideoSendTrackConfig config);

  void AddVideoReceiveSink(std::string track_id,
                           VideoSinkInterface<VideoFrame>* video_sink);

  CallbackHandlers* handlers() { return &handlers_; }

  // The `munge_offer` function can be used to munge the SDP, i.e. modify a
  // local description afer creating it but before setting it. Note that this is
  // legacy behavior. It's added here only to be able to have test coverage for
  // scenarios even if they are not spec compliant.
  void CreateAndSetSdp(
      std::function<void(SessionDescriptionInterface*)> munge_offer,
      std::function<void(std::string)> offer_handler);
  void SetSdpOfferAndGetAnswer(std::string remote_offer,
                               std::function<void()> remote_description_set,
                               std::function<void(std::string)> answer_handler);
  void SetSdpAnswer(
      std::string remote_answer,
      std::function<void(const SessionDescriptionInterface& answer)>
          done_handler);

  // Adds the given ice candidate when the peer connection is ready.
  void AddIceCandidate(std::unique_ptr<IceCandidateInterface> candidate);

 private:
  const std::map<int, EmulatedEndpoint*> endpoints_;
  TaskQueueFactory* const task_queue_factory_;
  Thread* const signaling_thread_;
  const std::unique_ptr<LogWriterFactoryInterface> log_writer_factory_;
  const std::unique_ptr<Thread> worker_thread_;
  CallbackHandlers handlers_ RTC_GUARDED_BY(signaling_thread_);
  const std::unique_ptr<PeerConnectionObserver> observer_;
  std::map<std::string, std::vector<VideoSinkInterface<VideoFrame>*>>
      track_id_to_video_sinks_ RTC_GUARDED_BY(signaling_thread_);
  std::list<std::unique_ptr<IceCandidateInterface>> pending_ice_candidates_
      RTC_GUARDED_BY(signaling_thread_);

  scoped_refptr<PeerConnectionFactoryInterface> pc_factory_;
  scoped_refptr<PeerConnectionInterface> peer_connection_
      RTC_GUARDED_BY(signaling_thread_);
};

}  // namespace test
}  // namespace webrtc

#endif  // TEST_PEER_SCENARIO_PEER_SCENARIO_CLIENT_H_
