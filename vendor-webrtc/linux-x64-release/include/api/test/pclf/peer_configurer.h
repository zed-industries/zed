/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef API_TEST_PCLF_PEER_CONFIGURER_H_
#define API_TEST_PCLF_PEER_CONFIGURER_H_

#include <cstdint>
#include <memory>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "absl/base/macros.h"
#include "absl/strings/string_view.h"
#include "api/async_dns_resolver.h"
#include "api/audio/audio_mixer.h"
#include "api/audio/audio_processing.h"
#include "api/audio_codecs/audio_decoder_factory.h"
#include "api/audio_codecs/audio_encoder_factory.h"
#include "api/fec_controller.h"
#include "api/field_trials_view.h"
#include "api/ice_transport_interface.h"
#include "api/neteq/neteq_factory.h"
#include "api/peer_connection_interface.h"
#include "api/rtc_event_log/rtc_event_log_factory_interface.h"
#include "api/scoped_refptr.h"
#include "api/test/frame_generator_interface.h"
#include "api/test/pclf/media_configuration.h"
#include "api/test/pclf/media_quality_test_params.h"
#include "api/test/peer_network_dependencies.h"
#include "api/transport/bitrate_settings.h"
#include "api/transport/network_control.h"
#include "api/video_codecs/video_decoder_factory.h"
#include "api/video_codecs/video_encoder_factory.h"
#include "rtc_base/rtc_certificate_generator.h"
#include "rtc_base/ssl_certificate.h"

namespace webrtc {
namespace webrtc_pc_e2e {

// This class is used to fully configure one peer inside a call.
class PeerConfigurer {
 public:
  using VideoSource =
      std::variant<std::unique_ptr<test::FrameGeneratorInterface>,
                   CapturingDeviceIndex>;

  explicit PeerConfigurer(PeerNetworkDependencies& network);

  // Sets peer name that will be used to report metrics related to this peer.
  // If not set, some default name will be assigned. All names have to be
  // unique.
  PeerConfigurer* SetName(absl::string_view name);

  // The parameters of the following 7 methods will be passed to the
  // PeerConnectionFactoryInterface implementation that will be created for
  // this peer.
  PeerConfigurer* SetEventLogFactory(
      std::unique_ptr<RtcEventLogFactoryInterface> event_log_factory);
  PeerConfigurer* SetFecControllerFactory(
      std::unique_ptr<FecControllerFactoryInterface> fec_controller_factory);
  PeerConfigurer* SetNetworkControllerFactory(
      std::unique_ptr<NetworkControllerFactoryInterface>
          network_controller_factory);
  PeerConfigurer* SetVideoEncoderFactory(
      std::unique_ptr<VideoEncoderFactory> video_encoder_factory);
  PeerConfigurer* SetVideoDecoderFactory(
      std::unique_ptr<VideoDecoderFactory> video_decoder_factory);
  PeerConfigurer* SetAudioEncoderFactory(
      scoped_refptr<webrtc::AudioEncoderFactory> audio_encoder_factory);
  PeerConfigurer* SetAudioDecoderFactory(
      scoped_refptr<webrtc::AudioDecoderFactory> audio_decoder_factory);
  // Set a custom NetEqFactory to be used in the call.
  PeerConfigurer* SetNetEqFactory(std::unique_ptr<NetEqFactory> neteq_factory);
  PeerConfigurer* SetAudioProcessing(
      std::unique_ptr<AudioProcessingBuilderInterface> audio_processing);
  ABSL_DEPRECATE_AND_INLINE()
  PeerConfigurer* SetAudioProcessing(
      scoped_refptr<webrtc::AudioProcessing> audio_processing) {
    return SetAudioProcessing(
        CustomAudioProcessing(std::move(audio_processing)));
  }
  PeerConfigurer* SetAudioMixer(scoped_refptr<webrtc::AudioMixer> audio_mixer);

  // Forces the Peerconnection to use the network thread as the worker thread.
  // Ie, worker thread and the network thread is the same thread.
  PeerConfigurer* SetUseNetworkThreadAsWorkerThread();

  // The parameters of the following 4 methods will be passed to the
  // PeerConnectionInterface implementation that will be created for this
  // peer.
  PeerConfigurer* SetAsyncDnsResolverFactory(
      std::unique_ptr<webrtc::AsyncDnsResolverFactoryInterface>
          async_dns_resolver_factory);
  PeerConfigurer* SetRTCCertificateGenerator(
      std::unique_ptr<RTCCertificateGeneratorInterface> cert_generator);
  PeerConfigurer* SetSSLCertificateVerifier(
      std::unique_ptr<SSLCertificateVerifier> tls_cert_verifier);
  PeerConfigurer* SetIceTransportFactory(
      std::unique_ptr<IceTransportFactory> factory);
  // Flags to set on `webrtc::PortAllocator`. These flags will be added
  // to the webrtc::kDefaultPortAllocatorFlags with
  // webrtc::PORTALLOCATOR_DISABLE_TCP disabled. For possible values check
  // p2p/base/port_allocator.h.
  PeerConfigurer* SetPortAllocatorExtraFlags(uint32_t extra_flags);
  // Flags to set on `webrtc::PortAllocator`. These flags will override
  // the default ones that are presented on the port allocator.
  //
  // For possible values check p2p/base/port_allocator.h.
  //
  // IMPORTANT: if you use WebRTC Network Emulation
  // (api/test/network_emulation_manager.h) and set this field, remember to set
  // webrtc::PORTALLOCATOR_DISABLE_TCP to 0.
  PeerConfigurer* SetPortAllocatorFlags(uint32_t flags);

  // Add new video stream to the call that will be sent from this peer.
  // Default implementation of video frames generator will be used.
  PeerConfigurer* AddVideoConfig(VideoConfig config);
  // Add new video stream to the call that will be sent from this peer with
  // provided own implementation of video frames generator.
  PeerConfigurer* AddVideoConfig(
      VideoConfig config,
      std::unique_ptr<test::FrameGeneratorInterface> generator);
  // Add new video stream to the call that will be sent from this peer.
  // Capturing device with specified index will be used to get input video.
  PeerConfigurer* AddVideoConfig(VideoConfig config,
                                 CapturingDeviceIndex capturing_device_index);
  // Sets video subscription for the peer. By default subscription will
  // include all streams with `VideoSubscription::kSameAsSendStream`
  // resolution. To this behavior use this method.
  PeerConfigurer* SetVideoSubscription(VideoSubscription subscription);
  // Sets the list of video codecs used by the peer during the test. These
  // codecs will be negotiated in SDP during offer/answer exchange. The order
  // of these codecs during negotiation will be the same as in `video_codecs`.
  // Codecs have to be available in codecs list provided by peer connection to
  // be negotiated. If some of specified codecs won't be found, the test will
  // crash.
  PeerConfigurer* SetVideoCodecs(std::vector<VideoCodecConfig> video_codecs);
  // Sets a list of RTP header extensions which will be enforced on all video
  // streams added to this peer.
  PeerConfigurer* SetExtraVideoRtpHeaderExtensions(
      std::vector<std::string> extensions);
  // Sets the audio stream for the call from this peer. If this method won't
  // be invoked, this peer will send no audio.
  PeerConfigurer* SetAudioConfig(AudioConfig config);
  // Sets a list of RTP header extensions which will be enforced on all audio
  // streams added to this peer.
  PeerConfigurer* SetExtraAudioRtpHeaderExtensions(
      std::vector<std::string> extensions);

  // Set if ULP FEC should be used or not. False by default.
  PeerConfigurer* SetUseUlpFEC(bool value);
  // Set if Flex FEC should be used or not. False by default.
  // Client also must enable `enable_flex_fec_support` in the `RunParams` to
  // be able to use this feature.
  PeerConfigurer* SetUseFlexFEC(bool value);
  // Specifies how much video encoder target bitrate should be different than
  // target bitrate, provided by WebRTC stack. Must be greater than 0. Can be
  // used to emulate overshooting of video encoders. This multiplier will
  // be applied for all video encoder on both sides for all layers. Bitrate
  // estimated by WebRTC stack will be multiplied by this multiplier and then
  // provided into VideoEncoder::SetRates(...). 1.0 by default.
  PeerConfigurer* SetVideoEncoderBitrateMultiplier(double multiplier);

  // If is set, an RTCEventLog will be saved in that location and it will be
  // available for further analysis.
  PeerConfigurer* SetRtcEventLogPath(absl::string_view path);
  // If is set, an AEC dump will be saved in that location and it will be
  // available for further analysis.
  PeerConfigurer* SetAecDumpPath(absl::string_view path);
  PeerConfigurer* SetPCFOptions(
      PeerConnectionFactoryInterface::Options options);
  PeerConfigurer* SetRTCConfiguration(
      PeerConnectionInterface::RTCConfiguration configuration);
  PeerConfigurer* SetRTCOfferAnswerOptions(
      PeerConnectionInterface::RTCOfferAnswerOptions options);
  // Set bitrate parameters on PeerConnection. This constraints will be
  // applied to all summed RTP streams for this peer.
  PeerConfigurer* SetBitrateSettings(BitrateSettings bitrate_settings);
  // Set field trials used for this PeerConnection.
  PeerConfigurer* SetFieldTrials(std::unique_ptr<FieldTrialsView> field_trials);

  // Returns InjectableComponents and transfer ownership to the caller.
  // Can be called once.
  std::unique_ptr<InjectableComponents> ReleaseComponents();

  // Returns Params and transfer ownership to the caller.
  // Can be called once.
  std::unique_ptr<Params> ReleaseParams();

  // Returns ConfigurableParams and transfer ownership to the caller.
  // Can be called once.
  std::unique_ptr<ConfigurableParams> ReleaseConfigurableParams();

  // Returns video sources and transfer frame generators ownership to the
  // caller. Can be called once.
  std::vector<VideoSource> ReleaseVideoSources();

  InjectableComponents* components() { return components_.get(); }
  Params* params() { return params_.get(); }
  ConfigurableParams* configurable_params() {
    return configurable_params_.get();
  }
  const Params& params() const { return *params_; }
  const ConfigurableParams& configurable_params() const {
    return *configurable_params_;
  }
  std::vector<VideoSource>* video_sources() { return &video_sources_; }

 private:
  std::unique_ptr<InjectableComponents> components_;
  std::unique_ptr<Params> params_;
  std::unique_ptr<ConfigurableParams> configurable_params_;
  std::vector<VideoSource> video_sources_;
};

}  // namespace webrtc_pc_e2e
}  // namespace webrtc

#endif  // API_TEST_PCLF_PEER_CONFIGURER_H_
