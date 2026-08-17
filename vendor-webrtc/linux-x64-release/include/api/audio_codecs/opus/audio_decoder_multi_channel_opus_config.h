/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_AUDIO_CODECS_OPUS_AUDIO_DECODER_MULTI_CHANNEL_OPUS_CONFIG_H_
#define API_AUDIO_CODECS_OPUS_AUDIO_DECODER_MULTI_CHANNEL_OPUS_CONFIG_H_

#include <cstddef>
#include <vector>

#include "api/audio_codecs/audio_decoder.h"

namespace webrtc {
struct AudioDecoderMultiChannelOpusConfig {
  // The number of channels that the decoder will output.
  int num_channels;

  // Number of mono or stereo encoded Opus streams.
  int num_streams;

  // Number of channel pairs coupled together, see RFC 7845 section
  // 5.1.1. Has to be less than the number of streams.
  int coupled_streams;

  // Channel mapping table, defines the mapping from encoded streams to output
  // channels. See RFC 7845 section 5.1.1.
  std::vector<unsigned char> channel_mapping;

  bool IsOk() const {
    if (num_channels < 1 || num_channels > AudioDecoder::kMaxNumberOfChannels ||
        num_streams < 0 || coupled_streams < 0) {
      return false;
    }
    if (num_streams < coupled_streams) {
      return false;
    }
    if (channel_mapping.size() != static_cast<size_t>(num_channels)) {
      return false;
    }

    // Every mono stream codes one channel, every coupled stream codes two. This
    // is the total coded channel count:
    const int max_coded_channel = num_streams + coupled_streams;
    for (const auto& x : channel_mapping) {
      // Coded channels >= max_coded_channel don't exist. Except for 255, which
      // tells Opus to put silence in output channel x.
      if (x >= max_coded_channel && x != 255) {
        return false;
      }
    }

    if (num_channels > 255 || max_coded_channel >= 255) {
      return false;
    }
    return true;
  }
};

}  // namespace webrtc

#endif  //  API_AUDIO_CODECS_OPUS_AUDIO_DECODER_MULTI_CHANNEL_OPUS_CONFIG_H_
