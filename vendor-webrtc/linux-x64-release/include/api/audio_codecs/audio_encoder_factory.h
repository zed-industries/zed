/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_AUDIO_CODECS_AUDIO_ENCODER_FACTORY_H_
#define API_AUDIO_CODECS_AUDIO_ENCODER_FACTORY_H_

#include <memory>
#include <optional>
#include <vector>

#include "absl/base/nullability.h"
#include "api/audio_codecs/audio_codec_pair_id.h"
#include "api/audio_codecs/audio_encoder.h"
#include "api/audio_codecs/audio_format.h"
#include "api/environment/environment.h"
#include "api/ref_count.h"

namespace webrtc {

// A factory that creates AudioEncoders.
class AudioEncoderFactory : public RefCountInterface {
 public:
  struct Options {
    // The encoder will tags its payloads with the specified payload type.
    // TODO(ossu): Try to avoid audio encoders having to know their payload
    // type.
    int payload_type = -1;

    // Links encoders and decoders that talk to the same remote entity: if
    // a AudioEncoderFactory::Create() and a AudioDecoderFactory::Create() call
    // receive non-null IDs that compare equal, the factory implementations may
    // assume that the encoder and decoder form a pair. (The intended use case
    // for this is to set up communication between the AudioEncoder and
    // AudioDecoder instances, which is needed for some codecs with built-in
    // bandwidth adaptation.)
    //
    // Note: Implementations need to be robust against combinations other than
    // one encoder, one decoder getting the same ID; such encoders must still
    // work.
    std::optional<AudioCodecPairId> codec_pair_id;
  };

  // Returns a prioritized list of audio codecs, to use for signaling etc.
  virtual std::vector<AudioCodecSpec> GetSupportedEncoders() = 0;

  // Returns information about how this format would be encoded, provided it's
  // supported. More format and format variations may be supported than those
  // returned by GetSupportedEncoders().
  virtual std::optional<AudioCodecInfo> QueryAudioEncoder(
      const SdpAudioFormat& format) = 0;

  // Creates an AudioEncoder for the specified format.
  // Returns null if the format isn't supported.
  virtual absl_nullable std::unique_ptr<AudioEncoder> Create(
      const Environment& env,
      const SdpAudioFormat& format,
      Options options) = 0;
};

}  // namespace webrtc

#endif  // API_AUDIO_CODECS_AUDIO_ENCODER_FACTORY_H_
