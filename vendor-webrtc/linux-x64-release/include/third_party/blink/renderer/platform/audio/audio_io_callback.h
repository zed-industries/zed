/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 * 3.  Neither the name of Apple Computer, Inc. ("Apple") nor the names of
 *     its contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_AUDIO_IO_CALLBACK_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_AUDIO_IO_CALLBACK_H_

#include "base/time/time.h"
#include "third_party/blink/renderer/platform/audio/audio_callback_metric_reporter.h"

namespace media {
struct AudioGlitchInfo;
}

namespace blink {

class AudioBus;

struct AudioIOPosition {
  // Audio stream position in seconds.
  double position;
  // System timestamp in seconds corresponding to the contained |position|
  // value.
  double timestamp;
  // The audio hardware output latency reported by the infrastructure.
  double hardware_output_latency;
};

// Abstract base-class for isochronous audio I/O client.
class AudioIOCallback {
 public:
  virtual ~AudioIOCallback() = default;

  // Called periodically to get the next render quantum of audio into
  // |destination_bus|.
  virtual void Render(AudioBus* destination_bus,
                      uint32_t frames_to_process,
                      const AudioIOPosition& output_position,
                      const AudioCallbackMetric& metric,
                      base::TimeDelta playout_delay,
                      const media::AudioGlitchInfo& glitch_info) = 0;

  // Called when an error occurs in the underlying audio stack.
  // (e.g. bad hardware parameters, or an error while rendering)
  virtual void OnRenderError() = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_AUDIO_IO_CALLBACK_H_
