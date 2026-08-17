// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_AUDIO_FRAME_STATS_ACCUMULATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_AUDIO_FRAME_STATS_ACCUMULATOR_H_

#include "base/numerics/clamped_math.h"
#include "base/time/time.h"
#include "media/base/audio_glitch_info.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

class PLATFORM_EXPORT AudioFrameStatsAccumulator final {
 public:
  AudioFrameStatsAccumulator() = default;
  AudioFrameStatsAccumulator(const AudioFrameStatsAccumulator&) = delete;
  AudioFrameStatsAccumulator& operator=(const AudioFrameStatsAccumulator&) =
      delete;
  ~AudioFrameStatsAccumulator() = default;

  // Updates the stats with information from a new buffer.
  void Update(size_t observed_frames,
              int sample_rate,
              base::TimeDelta latency,
              const media::AudioGlitchInfo& glitch_info);

  // Absorbs stats from an object that contains stats from a more recent
  // interval. This merges the average_latency(), min_latency and
  // max_latency() information into this object, and resets it on the `from`
  // object. `from`'s latency information interval should start where
  // `this`'s latency information interval ends. The frame counters, frame
  // durations, and current latency are simply copied from `from`.
  void Absorb(AudioFrameStatsAccumulator& from);

  uint64_t observed_frames() const { return observed_frames_; }

  base::TimeDelta observed_frames_duration() const {
    return observed_frames_duration_;
  }

  uint64_t glitch_frames() const { return glitch_frames_; }

  base::TimeDelta glitch_frames_duration() const {
    return glitch_frames_duration_;
  }

  size_t glitch_event_count() const { return glitch_event_count_; }

  base::TimeDelta latency() const { return last_latency_; }

  base::TimeDelta average_latency() const {
    return interval_frames_ > 0
               ? interval_frames_latency_sum_ / interval_frames_
               : last_latency_;
  }

  base::TimeDelta min_latency() const { return interval_minimum_latency_; }

  base::TimeDelta max_latency() const { return interval_maximum_latency_; }

 private:
  // Counters for observed frames, glitched frames and glitch events. These
  // only increment.
  base::ClampedNumeric<uint64_t> observed_frames_ = 0u;
  base::TimeDelta observed_frames_duration_;
  base::ClampedNumeric<uint64_t> glitch_frames_ = 0u;
  base::TimeDelta glitch_frames_duration_;
  size_t glitch_event_count_ = 0u;

  // Latency of the last audio buffer.
  base::TimeDelta last_latency_;

  // Latency information about an interval. It is accumulated on calls to
  // Update() and Absorb(), and reset when the object is used as an input for
  // a call to Absorb() on another AudioFrameStatsAccumulator object.
  base::ClampedNumeric<uint64_t> interval_frames_ = 0u;
  base::TimeDelta interval_frames_latency_sum_;
  base::TimeDelta interval_minimum_latency_;
  base::TimeDelta interval_maximum_latency_;

  // Helper function to merge new latency extremes into the existing latency
  // extremes.
  void MergeLatencyExtremes(base::TimeDelta new_minumum,
                            base::TimeDelta new_maximum);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_AUDIO_FRAME_STATS_ACCUMULATOR_H_
