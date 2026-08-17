/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MEDIA_BASE_AUDIO_SOURCE_H_
#define MEDIA_BASE_AUDIO_SOURCE_H_

#include <cstddef>
#include <cstdint>
#include <optional>

namespace webrtc {

// Abstract interface for providing the audio data.
// TODO(deadbeef): Rename this to AudioSourceInterface, and rename
// webrtc::AudioSourceInterface to AudioTrackSourceInterface.
class AudioSource {
 public:
  class Sink {
   public:
    // Callback to receive data from the AudioSource.
    virtual void OnData(
        const void* audio_data,
        int bits_per_sample,
        int sample_rate,
        size_t number_of_channels,
        size_t number_of_frames,
        std::optional<int64_t> absolute_capture_timestamp_ms) = 0;

    // Called when the AudioSource is going away.
    virtual void OnClose() = 0;

    // Returns the number of channels encoded by the sink. This can be less than
    // the number_of_channels if down-mixing occur. A value of -1 means an
    // unknown number.
    virtual int NumPreferredChannels() const = 0;

   protected:
    virtual ~Sink() {}
  };

  // Sets a sink to the AudioSource. There can be only one sink connected
  // to the source at a time.
  virtual void SetSink(Sink* sink) = 0;

 protected:
  virtual ~AudioSource() {}
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::AudioSource;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // MEDIA_BASE_AUDIO_SOURCE_H_
