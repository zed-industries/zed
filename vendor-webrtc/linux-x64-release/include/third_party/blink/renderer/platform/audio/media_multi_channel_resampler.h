// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_MEDIA_MULTI_CHANNEL_RESAMPLER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_MEDIA_MULTI_CHANNEL_RESAMPLER_H_

#include <memory>
#include "base/functional/callback.h"
#include "media/base/multi_channel_resampler.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"

namespace media {
class AudioBus;
}  // namespace media

namespace blink {

class AudioBus;

// This is a simple wrapper around the MultiChannelResampler provided by the
// media layer.
class PLATFORM_EXPORT MediaMultiChannelResampler final {
  USING_FAST_MALLOC(MediaMultiChannelResampler);

  // Callback type for providing more data into the resampler.  Expects AudioBus
  // to be completely filled with data upon return; zero padded if not enough
  // frames are available to satisfy the request.  |frame_delay| is the number
  // of output frames already processed and can be used to estimate delay.
  typedef WTF::CrossThreadRepeatingFunction<void(int frame_delay,
                                                 blink::AudioBus*)>
      ReadCB;

 public:
  // Constructs a MultiChannelResampler with the specified |read_cb|, which is
  // used to acquire audio data for resampling.  |io_sample_rate_ratio| is the
  // ratio of input / output sample rates.  |request_frames| is the size in
  // frames of the AudioBus to be filled by |read_cb|.
  MediaMultiChannelResampler(int channels,
                             double io_sample_rate_ratio,
                             uint32_t request_frames,
                             ReadCB read_cb);

  MediaMultiChannelResampler(const MediaMultiChannelResampler&) = delete;
  MediaMultiChannelResampler& operator=(const MediaMultiChannelResampler&) =
      delete;

  // Resamples |frames| of data from |read_cb_| into a blink::AudioBus, this
  // requires creating a wrapper for the media::AudioBus on each call and so
  // resampling directly into a media::AudioBus using ResampleInternal() is
  // preferred if possible.
  void Resample(int frames, blink::AudioBus* resampler_input_bus);

  // Resamples |frames| of data from |read_cb_| into a media::AudioBus by
  // directly calling Resample() on the underlying
  // media::MultiChannelResampler.
  void ResampleInternal(int frames, media::AudioBus* resampler_input_bus);

 private:
  // Wrapper method used to provide input to the media::MultiChannelResampler
  // with a media::AudioBus rather than a blink::AudioBus.
  void ProvideResamplerInput(int resampler_frame_delay,
                             media::AudioBus* resampler_output_bus);

  // The resampler being wrapped by this class.
  std::unique_ptr<media::MultiChannelResampler> resampler_;

  // An intermediary storage (wrapper) for buffers from Web Audio.
  const std::unique_ptr<media::AudioBus> resampler_input_bus_wrapper_;

  // An intermediary storage (wrapper) for providing the resampler's output.
  const scoped_refptr<AudioBus> resampler_output_bus_wrapper_;

  // The callback using a blink::AudioBus that will be called by
  // ProvideResamplerInput().
  ReadCB read_cb_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_MEDIA_MULTI_CHANNEL_RESAMPLER_H_
