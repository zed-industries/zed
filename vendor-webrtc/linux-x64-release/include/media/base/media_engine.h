/*
 *  Copyright (c) 2004 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MEDIA_BASE_MEDIA_ENGINE_H_
#define MEDIA_BASE_MEDIA_ENGINE_H_

#include <cstdint>
#include <memory>
#include <optional>
#include <vector>

#include "api/array_view.h"
#include "api/audio/audio_device.h"
#include "api/audio_codecs/audio_codec_pair_id.h"
#include "api/audio_codecs/audio_decoder_factory.h"
#include "api/audio_codecs/audio_encoder_factory.h"
#include "api/audio_options.h"
#include "api/crypto/crypto_options.h"
#include "api/field_trials_view.h"
#include "api/rtc_error.h"
#include "api/rtp_parameters.h"
#include "api/scoped_refptr.h"
#include "api/video/video_bitrate_allocator_factory.h"
#include "call/audio_state.h"
#include "media/base/codec.h"
#include "media/base/media_channel.h"
#include "media/base/media_config.h"
#include "media/base/stream_params.h"
#include "rtc_base/system/file_wrapper.h"

namespace webrtc {

class AudioMixer;
class Call;

// Checks that the scalability_mode value of each encoding is supported by at
// least one video codec of the list. If the list is empty, no check is done.
RTCError CheckScalabilityModeValues(const RtpParameters& new_parameters,
                                    ArrayView<Codec> send_codecs,
                                    std::optional<Codec> send_codec);

// Checks the parameters have valid and supported values, and checks parameters
// with CheckScalabilityModeValues().
RTCError CheckRtpParametersValues(const RtpParameters& new_parameters,
                                  ArrayView<Codec> send_codecs,
                                  std::optional<Codec> send_codec,
                                  const FieldTrialsView& field_trials);

// Checks that the immutable values have not changed in new_parameters and
// checks all parameters with CheckRtpParametersValues().
RTCError CheckRtpParametersInvalidModificationAndValues(
    const RtpParameters& old_parameters,
    const RtpParameters& new_parameters,
    ArrayView<Codec> send_codecs,
    std::optional<Codec> send_codec,
    const FieldTrialsView& field_trials);

// Checks that the immutable values have not changed in new_parameters and
// checks parameters (except SVC) with CheckRtpParametersValues(). It should
// usually be paired with a call to CheckScalabilityModeValues().
RTCError CheckRtpParametersInvalidModificationAndValues(
    const RtpParameters& old_parameters,
    const RtpParameters& new_parameters,
    const FieldTrialsView& field_trials);

class RtpHeaderExtensionQueryInterface {
 public:
  virtual ~RtpHeaderExtensionQueryInterface() = default;

  // Returns a vector of RtpHeaderExtensionCapability, whose direction is
  // kStopped if the extension is stopped (not used) by default.
  virtual std::vector<RtpHeaderExtensionCapability> GetRtpHeaderExtensions()
      const = 0;
};

class VoiceEngineInterface : public RtpHeaderExtensionQueryInterface {
 public:
  VoiceEngineInterface() = default;
  virtual ~VoiceEngineInterface() = default;

  VoiceEngineInterface(const VoiceEngineInterface&) = delete;
  VoiceEngineInterface& operator=(const VoiceEngineInterface&) = delete;

  // Initialization
  // Starts the engine.
  virtual void Init() = 0;

  // TODO(solenberg): Remove once VoE API refactoring is done.
  virtual scoped_refptr<AudioState> GetAudioState() const = 0;

  virtual std::unique_ptr<VoiceMediaSendChannelInterface> CreateSendChannel(
      Call* /* call */,
      const MediaConfig& /* config */,
      const AudioOptions& /* options */,
      const CryptoOptions& /* crypto_options */,
      AudioCodecPairId /* codec_pair_id */) = 0;

  virtual std::unique_ptr<VoiceMediaReceiveChannelInterface>
  CreateReceiveChannel(Call* /* call */,
                       const MediaConfig& /* config */,
                       const AudioOptions& /* options */,
                       const CryptoOptions& /* crypto_options */,
                       AudioCodecPairId /* codec_pair_id */) = 0;

  // Legacy: Retrieve list of supported codecs.
  // + protection codecs, and assigns PT numbers that may have to be
  // reassigned.
  // This function is being moved to CodecVendor
  // TODO: https://issues.webrtc.org/360058654 - remove when all users updated.
  [[deprecated]] inline const std::vector<Codec>& send_codecs() const {
    return LegacySendCodecs();
  }
  [[deprecated]] inline const std::vector<Codec>& recv_codecs() const {
    return LegacyRecvCodecs();
  }
  virtual const std::vector<Codec>& LegacySendCodecs() const = 0;
  virtual const std::vector<Codec>& LegacyRecvCodecs() const = 0;

  virtual AudioEncoderFactory* encoder_factory() const = 0;
  virtual AudioDecoderFactory* decoder_factory() const = 0;

  // Starts AEC dump using existing file, a maximum file size in bytes can be
  // specified. Logging is stopped just before the size limit is exceeded.
  // If max_size_bytes is set to a value <= 0, no limit will be used.
  virtual bool StartAecDump(FileWrapper file, int64_t max_size_bytes) = 0;

  // Stops recording AEC dump.
  virtual void StopAecDump() = 0;

  virtual std::optional<AudioDeviceModule::Stats> GetAudioDeviceStats() = 0;
};

class VideoEngineInterface : public RtpHeaderExtensionQueryInterface {
 public:
  VideoEngineInterface() = default;
  virtual ~VideoEngineInterface() = default;

  VideoEngineInterface(const VideoEngineInterface&) = delete;
  VideoEngineInterface& operator=(const VideoEngineInterface&) = delete;

  virtual std::unique_ptr<VideoMediaSendChannelInterface> CreateSendChannel(
      Call* /* call */,
      const MediaConfig& /* config */,
      const VideoOptions& /* options */,
      const CryptoOptions& /* crypto_options */,
      VideoBitrateAllocatorFactory*
      /* video_bitrate_allocator_factory */) = 0;

  virtual std::unique_ptr<VideoMediaReceiveChannelInterface>
  CreateReceiveChannel(Call* /* call */,
                       const MediaConfig& /* config */,
                       const VideoOptions& /* options */,
                       const CryptoOptions& /* crypto_options */) = 0;

  // Legacy: Retrieve list of supported codecs.
  // + protection codecs, and assigns PT numbers that may have to be
  // reassigned.
  // This functionality is being moved to the CodecVendor class.
  // TODO: https://issues.webrtc.org/360058654 - deprecate and remove.
  [[deprecated]] inline std::vector<Codec> send_codecs() const {
    return LegacySendCodecs();
  }
  [[deprecated]] inline std::vector<Codec> recv_codecs() const {
    return LegacyRecvCodecs();
  }
  virtual std::vector<Codec> LegacySendCodecs() const = 0;
  virtual std::vector<Codec> LegacyRecvCodecs() const = 0;
  // As above, but if include_rtx is false, don't include RTX codecs.
  [[deprecated]] inline std::vector<Codec> send_codecs(bool include_rtx) const {
    return LegacySendCodecs(include_rtx);
  }
  virtual std::vector<Codec> LegacySendCodecs(bool include_rtx) const = 0;
  virtual std::vector<Codec> LegacyRecvCodecs(bool include_rtx) const = 0;
  [[deprecated]] inline std::vector<Codec> recv_codecs(bool include_rtx) const {
    return LegacyRecvCodecs(include_rtx);
  }
};

// MediaEngineInterface is an abstraction of a media engine which can be
// subclassed to support different media componentry backends.
// It supports voice and video operations in the same class to facilitate
// proper synchronization between both media types.
class MediaEngineInterface {
 public:
  virtual ~MediaEngineInterface() {}

  // Initialization. Needs to be called on the worker thread.
  virtual bool Init() = 0;

  virtual VoiceEngineInterface& voice() = 0;
  virtual VideoEngineInterface& video() = 0;
  virtual const VoiceEngineInterface& voice() const = 0;
  virtual const VideoEngineInterface& video() const = 0;
};

// CompositeMediaEngine constructs a MediaEngine from separate
// voice and video engine classes.
// Optionally owns a FieldTrialsView trials map.
class CompositeMediaEngine : public MediaEngineInterface {
 public:
  CompositeMediaEngine(std::unique_ptr<FieldTrialsView> trials,
                       std::unique_ptr<VoiceEngineInterface> audio_engine,
                       std::unique_ptr<VideoEngineInterface> video_engine);
  CompositeMediaEngine(std::unique_ptr<VoiceEngineInterface> audio_engine,
                       std::unique_ptr<VideoEngineInterface> video_engine);
  ~CompositeMediaEngine() override;

  // Always succeeds.
  bool Init() override;

  VoiceEngineInterface& voice() override;
  VideoEngineInterface& video() override;
  const VoiceEngineInterface& voice() const override;
  const VideoEngineInterface& video() const override;

 private:
  const std::unique_ptr<FieldTrialsView> trials_;
  const std::unique_ptr<VoiceEngineInterface> voice_engine_;
  const std::unique_ptr<VideoEngineInterface> video_engine_;
};

RtpParameters CreateRtpParametersWithOneEncoding();
RtpParameters CreateRtpParametersWithEncodings(StreamParams sp);

// Returns a vector of RTP extensions as visible from RtpSender/Receiver
// GetCapabilities(). The returned vector only shows what will definitely be
// offered by default, i.e. the list of extensions returned from
// GetRtpHeaderExtensions() that are not kStopped.
std::vector<RtpExtension> GetDefaultEnabledRtpHeaderExtensions(
    const RtpHeaderExtensionQueryInterface& query_interface);

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::CheckRtpParametersInvalidModificationAndValues;
using ::webrtc::CheckRtpParametersValues;
using ::webrtc::CheckScalabilityModeValues;
using ::webrtc::CompositeMediaEngine;
using ::webrtc::CreateRtpParametersWithEncodings;
using ::webrtc::CreateRtpParametersWithOneEncoding;
using ::webrtc::GetDefaultEnabledRtpHeaderExtensions;
using ::webrtc::MediaEngineInterface;
using ::webrtc::RtpHeaderExtensionQueryInterface;
using ::webrtc::VideoEngineInterface;
using ::webrtc::VoiceEngineInterface;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // MEDIA_BASE_MEDIA_ENGINE_H_
