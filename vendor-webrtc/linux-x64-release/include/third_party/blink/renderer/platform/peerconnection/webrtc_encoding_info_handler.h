// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_WEBRTC_ENCODING_INFO_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_WEBRTC_ENCODING_INFO_HANDLER_H_

#include <memory>
#include <optional>

#include "base/functional/callback_forward.h"
#include "third_party/blink/renderer/platform/peerconnection/audio_codec_factory.h"
#include "third_party/blink/renderer/platform/peerconnection/video_codec_factory.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class PLATFORM_EXPORT WebrtcEncodingInfoHandler {
 public:
  static WebrtcEncodingInfoHandler* Instance();

  WebrtcEncodingInfoHandler();
  // Constructor for unittest to inject video and audio encoder factory
  // instances.
  WebrtcEncodingInfoHandler(
      std::unique_ptr<webrtc::VideoEncoderFactory> video_encoder_factory,
      webrtc::scoped_refptr<webrtc::AudioEncoderFactory> audio_encoder_factory);
  // Not copyable or movable.
  WebrtcEncodingInfoHandler(const WebrtcEncodingInfoHandler&) = delete;
  WebrtcEncodingInfoHandler& operator=(const WebrtcEncodingInfoHandler&) =
      delete;
  ~WebrtcEncodingInfoHandler();

  // Queries the capabilities of the given encoding configuration and passes
  // the result via callbacks.
  // It implements WICG Media Capabilities encodingInfo() call for webrtc
  // encoding.
  // https://wicg.github.io/media-capabilities/#media-capabilities-interface
  using OnMediaCapabilitiesEncodingInfoCallback =
      base::OnceCallback<void(bool, bool)>;
  void EncodingInfo(
      const std::optional<webrtc::SdpAudioFormat> sdp_audio_format,
      const std::optional<webrtc::SdpVideoFormat> sdp_video_format,
      const String video_scalability_mode,
      OnMediaCapabilitiesEncodingInfoCallback callback) const;

 private:
  std::unique_ptr<webrtc::VideoEncoderFactory> video_encoder_factory_;
  webrtc::scoped_refptr<webrtc::AudioEncoderFactory> audio_encoder_factory_;
  // List of supported audio codecs.
  HashSet<String> supported_audio_codecs_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_WEBRTC_ENCODING_INFO_HANDLER_H_
