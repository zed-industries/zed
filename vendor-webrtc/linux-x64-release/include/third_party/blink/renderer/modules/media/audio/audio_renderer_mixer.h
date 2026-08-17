// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_AUDIO_AUDIO_RENDERER_MIXER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_AUDIO_AUDIO_RENDERER_MIXER_H_

#include <stdint.h>

#include <memory>

#include "base/containers/flat_map.h"
#include "base/containers/flat_set.h"
#include "base/memory/raw_ptr.h"
#include "base/synchronization/lock.h"
#include "base/thread_annotations.h"
#include "base/time/time.h"
#include "media/base/audio_converter.h"
#include "media/base/audio_renderer_sink.h"
#include "media/base/loopback_audio_converter.h"
#include "third_party/blink/public/platform/web_common.h"

namespace blink {
class AudioRendererMixerInput;

// Mixes a set of AudioConverter::InputCallbacks into a single output stream
// which is funneled into a single shared AudioRendererSink; saving a bundle
// on renderer side resources.
class BLINK_MODULES_EXPORT AudioRendererMixer
    : public media::AudioRendererSink::RenderCallback {
 public:
  AudioRendererMixer(const media::AudioParameters& output_params,
                     scoped_refptr<media::AudioRendererSink> sink);

  AudioRendererMixer(const AudioRendererMixer&) = delete;
  AudioRendererMixer& operator=(const AudioRendererMixer&) = delete;

  ~AudioRendererMixer() override;

  // Add or remove a mixer input from mixing; called by AudioRendererMixerInput.
  void AddMixerInput(const media::AudioParameters& input_params,
                     media::AudioConverter::InputCallback* input);
  void RemoveMixerInput(const media::AudioParameters& input_params,
                        media::AudioConverter::InputCallback* input);

  // Since errors may occur even when no inputs are playing, an error callback
  // must be registered separately from adding a mixer input.
  void AddErrorCallback(AudioRendererMixerInput* input);
  void RemoveErrorCallback(AudioRendererMixerInput* input);

  // Returns true if called on rendering thread, otherwise false.
  bool CurrentThreadIsRenderingThread();

  void SetPauseDelayForTesting(base::TimeDelta delay);
  const media::AudioParameters& get_output_params_for_testing() const {
    return output_params_;
  }

  // Return true if this mixer has ever received an error from its sink.
  bool HasSinkError();

 private:
  // AudioRendererSink::RenderCallback implementation.
  int Render(base::TimeDelta delay,
             base::TimeTicks delay_timestamp,
             const media::AudioGlitchInfo& glitch_info,
             media::AudioBus* audio_bus) override;
  void OnRenderError() override;

  bool can_passthrough(int sample_rate) const {
    return sample_rate == output_params_.sample_rate();
  }

  // Output parameters for this mixer.
  const media::AudioParameters output_params_;

  // Output sink for this mixer.
  const scoped_refptr<media::AudioRendererSink> audio_sink_;

  // ---------------[ All variables below protected by `lock_` ]---------------
  base::Lock lock_;

  // List of error callbacks used by this mixer.
  base::flat_set<raw_ptr<AudioRendererMixerInput, CtnExperimental>>
      error_callbacks_ GUARDED_BY(lock_);

  // Maps input sample rate to the dedicated converter.
  using AudioConvertersMap =
      base::flat_map<int, std::unique_ptr<media::LoopbackAudioConverter>>;

  // Each of these converters mixes inputs with a given sample rate and
  // resamples them to the output sample rate. Inputs not requiring resampling
  // go directly to `aggregate_converter_`.
  AudioConvertersMap converters_ GUARDED_BY(lock_);

  // Aggregate converter which mixes all the outputs from `converters_` as well
  // as mixer inputs that are in the output sample rate.
  media::AudioConverter aggregate_converter_ GUARDED_BY(lock_);

  // Handles physical stream pause when no inputs are playing.  For latency
  // reasons we don't want to immediately pause the physical stream.
  base::TimeDelta pause_delay_ GUARDED_BY(lock_);
  base::TimeTicks last_play_time_ GUARDED_BY(lock_);
  bool playing_ GUARDED_BY(lock_);

  // Set if the mixer receives an error from the sink. Indicates that this
  // mixer and sink should no longer be reused.
  bool sink_error_ GUARDED_BY(lock_) = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_AUDIO_AUDIO_RENDERER_MIXER_H_
