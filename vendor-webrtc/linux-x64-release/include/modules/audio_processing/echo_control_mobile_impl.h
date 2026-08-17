/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_ECHO_CONTROL_MOBILE_IMPL_H_
#define MODULES_AUDIO_PROCESSING_ECHO_CONTROL_MOBILE_IMPL_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <vector>

#include "api/array_view.h"

namespace webrtc {

class AudioBuffer;

// The acoustic echo control for mobile (AECM) component is a low complexity
// robust option intended for use on mobile devices.
class EchoControlMobileImpl {
 public:
  EchoControlMobileImpl();

  ~EchoControlMobileImpl();

  // Recommended settings for particular audio routes. In general, the louder
  // the echo is expected to be, the higher this value should be set. The
  // preferred setting may vary from device to device.
  enum RoutingMode {
    kQuietEarpieceOrHeadset,
    kEarpiece,
    kLoudEarpiece,
    kSpeakerphone,
    kLoudSpeakerphone
  };

  // Sets echo control appropriate for the audio routing `mode` on the device.
  // It can and should be updated during a call if the audio routing changes.
  int set_routing_mode(RoutingMode mode);
  RoutingMode routing_mode() const;

  // Comfort noise replaces suppressed background noise to maintain a
  // consistent signal level.
  int enable_comfort_noise(bool enable);
  bool is_comfort_noise_enabled() const;

  void ProcessRenderAudio(ArrayView<const int16_t> packed_render_audio);
  int ProcessCaptureAudio(AudioBuffer* audio, int stream_delay_ms);

  void Initialize(int sample_rate_hz,
                  size_t num_reverse_channels,
                  size_t num_output_channels);

  static void PackRenderAudioBuffer(const AudioBuffer* audio,
                                    size_t num_output_channels,
                                    size_t num_channels,
                                    std::vector<int16_t>* packed_buffer);

  static size_t NumCancellersRequired(size_t num_output_channels,
                                      size_t num_reverse_channels);

 private:
  class Canceller;
  struct StreamProperties;

  int Configure();

  RoutingMode routing_mode_;
  bool comfort_noise_enabled_;

  std::vector<std::unique_ptr<Canceller>> cancellers_;
  std::unique_ptr<StreamProperties> stream_properties_;
  std::vector<std::array<int16_t, 160>> low_pass_reference_;
  bool reference_copied_ = false;
};
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_ECHO_CONTROL_MOBILE_IMPL_H_
