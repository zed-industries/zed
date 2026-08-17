/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_CODECS_VIDEO_ENCODER_FACTORY_H_
#define API_VIDEO_CODECS_VIDEO_ENCODER_FACTORY_H_

#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "api/environment/environment.h"
#include "api/units/data_rate.h"
#include "api/video/render_resolution.h"
#include "api/video_codecs/sdp_video_format.h"
#include "api/video_codecs/video_encoder.h"

namespace webrtc {

// A factory that creates VideoEncoders.
// NOTE: This class is still under development and may change without notice.
class VideoEncoderFactory {
 public:
  struct CodecSupport {
    bool is_supported = false;
    bool is_power_efficient = false;
  };

  // An injectable class that is continuously updated with encoding conditions
  // and selects the best encoder given those conditions. An implementation is
  // typically stateful to avoid toggling between different encoders, which is
  // costly due to recreation of objects, a new codec will always start with a
  // key-frame.
  class EncoderSelectorInterface {
   public:
    virtual ~EncoderSelectorInterface() {}

    // Informs the encoder selector about which encoder that is currently being
    // used.
    virtual void OnCurrentEncoder(const SdpVideoFormat& format) = 0;

    // Called every time the available bitrate is updated. Should return a
    // non-empty if an encoder switch should be performed.
    virtual std::optional<SdpVideoFormat> OnAvailableBitrate(
        const DataRate& rate) = 0;

    // Called every time the encoder input resolution change. Should return a
    // non-empty if an encoder switch should be performed.
    virtual std::optional<SdpVideoFormat> OnResolutionChange(
        const RenderResolution& /* resolution */) {
      return std::nullopt;
    }

    // Called if the currently used encoder reports itself as broken. Should
    // return a non-empty if an encoder switch should be performed.
    virtual std::optional<SdpVideoFormat> OnEncoderBroken() = 0;
  };

  // Returns a list of supported video formats in order of preference, to use
  // for signaling etc.
  virtual std::vector<SdpVideoFormat> GetSupportedFormats() const = 0;

  // Returns a list of supported video formats in order of preference, that can
  // also be tagged with additional information to allow the VideoEncoderFactory
  // to separate between different implementations when CreateVideoEncoder is
  // called.
  virtual std::vector<SdpVideoFormat> GetImplementations() const {
    return GetSupportedFormats();
  }

  // Query whether the specifed format is supported or not and if it will be
  // power efficient, which is currently interpreted as if there is support for
  // hardware acceleration.
  // See https://w3c.github.io/webrtc-svc/#scalabilitymodes* for a specification
  // of valid values for `scalability_mode`.
  // NOTE: QueryCodecSupport is currently an experimental feature that is
  // subject to change without notice.
  virtual CodecSupport QueryCodecSupport(
      const SdpVideoFormat& format,
      std::optional<std::string> scalability_mode) const {
    CodecSupport codec_support;
    codec_support.is_supported = format.IsCodecInList(GetSupportedFormats());
    return codec_support;
  }

  // Creates a VideoEncoder for the specified format.
  virtual std::unique_ptr<VideoEncoder> Create(
      const Environment& env,
      const SdpVideoFormat& format) = 0;

  // This method creates a EncoderSelector to use for a VideoSendStream.
  // (and hence should probably been called CreateEncoderSelector()).
  //
  // Note: This method is unsuitable if encoding several streams that
  // are using same VideoEncoderFactory (either by several streams in one
  // PeerConnection or streams with different PeerConnection but same
  // PeerConnectionFactory). This is due to the fact that the method is not
  // given any stream identifier, nor is the EncoderSelectorInterface given any
  // stream identifiers, i.e one does not know which stream is being encoded
  // with help of the selector.
  //
  // In such scenario, the `RtpSenderInterface::SetEncoderSelector` is
  // recommended.
  //
  // TODO(bugs.webrtc.org:14122): Deprecate and remove in favor of
  // `RtpSenderInterface::SetEncoderSelector`.
  virtual std::unique_ptr<EncoderSelectorInterface> GetEncoderSelector() const {
    return nullptr;
  }

  virtual ~VideoEncoderFactory() {}
};

}  // namespace webrtc

#endif  // API_VIDEO_CODECS_VIDEO_ENCODER_FACTORY_H_
