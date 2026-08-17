// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_AUDIO_AUDIO_RENDERER_MIXER_POOL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_AUDIO_AUDIO_RENDERER_MIXER_POOL_H_

#include <string>

#include "media/base/audio_latency.h"
#include "media/base/output_device_info.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/platform/web_common.h"

namespace media {
class AudioParameters;
class AudioRendererSink;
}  // namespace media

namespace blink {
class AudioRendererMixer;

// Provides AudioRendererMixer instances for shared usage.
// Thread safe.
class BLINK_MODULES_EXPORT AudioRendererMixerPool {
 public:
  AudioRendererMixerPool() = default;

  AudioRendererMixerPool(const AudioRendererMixerPool&) = delete;
  AudioRendererMixerPool& operator=(const AudioRendererMixerPool&) = delete;

  virtual ~AudioRendererMixerPool() = default;

  // Obtains a pointer to mixer instance based on AudioParameters. The pointer
  // is guaranteed to be valid (at least) until it's rereleased by a call to
  // ReturnMixer().
  //
  // Ownership of `sink` must be passed to GetMixer(), it will be stopped and
  // discard if an existing mixer can be reused. Clients must have called
  // GetOutputDeviceInfoAsync() on `sink` to get `sink_info`, and it must have
  // a device_status() == OUTPUT_DEVICE_STATUS_OK.
  //
  // `source_frame_token`, `main_frame_token` are used to determine when mixers
  // can be shared among multiple AudioRenderMixerInput instances.
  virtual AudioRendererMixer* GetMixer(
      const LocalFrameToken& source_frame_token,
      const FrameToken& main_frame_token,
      const media::AudioParameters& input_params,
      media::AudioLatency::Type latency,
      const media::OutputDeviceInfo& sink_info,
      scoped_refptr<media::AudioRendererSink> sink) = 0;

  // Returns mixer back to the pool, must be called when the mixer is not needed
  // any more to avoid memory leakage.
  virtual void ReturnMixer(AudioRendererMixer* mixer) = 0;

  // Returns an AudioRendererSink for use with GetMixer(). Inputs must call this
  // to get a sink to use with a subsequent GetMixer()
  virtual scoped_refptr<media::AudioRendererSink> GetSink(
      const LocalFrameToken& source_frame_token,
      const FrameToken& main_frame_token,
      std::string_view device_id) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_AUDIO_AUDIO_RENDERER_MIXER_POOL_H_
