// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_WEBRTC_DECODING_INFO_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_WEBRTC_DECODING_INFO_HANDLER_H_

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

class PLATFORM_EXPORT WebrtcDecodingInfoHandler {
 public:
  static WebrtcDecodingInfoHandler* Instance();

  WebrtcDecodingInfoHandler();
  // Constructor for unittest to inject video and audio decoder factory
  // instances.
  WebrtcDecodingInfoHandler(
      std::unique_ptr<webrtc::VideoDecoderFactory> video_decoder_factory,
      webrtc::scoped_refptr<webrtc::AudioDecoderFactory> audio_decoder_factory);
  // Not copyable or movable.
  WebrtcDecodingInfoHandler(const WebrtcDecodingInfoHandler&) = delete;
  WebrtcDecodingInfoHandler& operator=(const WebrtcDecodingInfoHandler&) =
      delete;
  ~WebrtcDecodingInfoHandler();

  // Queries the capabilities of the given decoding configuration and passes
  // the result via callbacks.
  // It implements WICG Media Capabilities decodingInfo() call for webrtc
  // encoding.
  // https://wicg.github.io/media-capabilities/#media-capabilities-interface
  using OnMediaCapabilitiesDecodingInfoCallback =
      base::OnceCallback<void(bool, bool)>;
  void DecodingInfo(
      const std::optional<webrtc::SdpAudioFormat> sdp_audio_format,
      const std::optional<webrtc::SdpVideoFormat> sdp_video_format,
      const bool video_spatial_scalability,
      OnMediaCapabilitiesDecodingInfoCallback callback) const;

 private:
  std::unique_ptr<webrtc::VideoDecoderFactory> video_decoder_factory_;
  webrtc::scoped_refptr<webrtc::AudioDecoderFactory> audio_decoder_factory_;
  // List of supported audio codecs.
  HashSet<String> supported_audio_codecs_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_WEBRTC_DECODING_INFO_HANDLER_H_
