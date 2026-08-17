/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_EXPAND_H_
#define MODULES_AUDIO_CODING_NETEQ_EXPAND_H_

#include <memory>

#include "modules/audio_coding/neteq/audio_vector.h"

namespace webrtc {

// Forward declarations.
class AudioMultiVector;
class BackgroundNoise;
class RandomVector;
class StatisticsCalculator;
class SyncBuffer;

// This class handles extrapolation of audio data from the sync_buffer to
// produce packet-loss concealment.
// TODO(hlundin): Refactor this class to divide the long methods into shorter
// ones.
class Expand {
 public:
  Expand(BackgroundNoise* background_noise,
         SyncBuffer* sync_buffer,
         RandomVector* random_vector,
         StatisticsCalculator* statistics,
         int fs,
         size_t num_channels);

  virtual ~Expand();

  Expand(const Expand&) = delete;
  Expand& operator=(const Expand&) = delete;

  // Resets the object.
  virtual void Reset();

  // The main method to produce concealment data. The data is appended to the
  // end of `output`.
  virtual int Process(AudioMultiVector* output);

  // Prepare the object to do extra expansion during normal operation following
  // a period of expands.
  virtual void SetParametersForNormalAfterExpand();

  // Prepare the object to do extra expansion during merge operation following
  // a period of expands.
  virtual void SetParametersForMergeAfterExpand();

  // Returns the mute factor for `channel`.
  int16_t MuteFactor(size_t channel) const {
    RTC_DCHECK_LT(channel, num_channels_);
    return channel_parameters_[channel].mute_factor;
  }

  // Returns true if expansion has been faded down to zero amplitude (for all
  // channels); false otherwise.
  bool Muted() const;

  // Accessors and mutators.
  virtual size_t overlap_length() const;
  size_t max_lag() const { return max_lag_; }

 protected:
  static const int kMaxConsecutiveExpands = 200;
  void GenerateRandomVector(int16_t seed_increment,
                            size_t length,
                            int16_t* random_vector);

  // Initializes member variables at the beginning of an expand period.
  void InitializeForAnExpandPeriod();

  bool TooManyExpands();

  // Analyzes the signal history in `sync_buffer_`, and set up all parameters
  // necessary to produce concealment data.
  void AnalyzeSignal(int16_t* random_vector);

  RandomVector* const random_vector_;
  SyncBuffer* const sync_buffer_;
  bool first_expand_;
  const int fs_hz_;
  const size_t num_channels_;
  int consecutive_expands_;

 private:
  static const size_t kUnvoicedLpcOrder = 6;
  static const size_t kNumCorrelationCandidates = 3;
  static const size_t kDistortionLength = 20;
  static const size_t kLpcAnalysisLength = 160;
  static const size_t kMaxSampleRate = 48000;
  static const int kNumLags = 3;

  struct ChannelParameters {
    ChannelParameters();
    int16_t mute_factor;
    int16_t ar_filter[kUnvoicedLpcOrder + 1];
    int16_t ar_filter_state[kUnvoicedLpcOrder];
    int16_t ar_gain;
    int16_t ar_gain_scale;
    int16_t voice_mix_factor;         /* Q14 */
    int16_t current_voice_mix_factor; /* Q14 */
    AudioVector expand_vector0;
    AudioVector expand_vector1;
    bool onset;
    int mute_slope; /* Q20 */
  };

  // Calculate the auto-correlation of `input`, with length `input_length`
  // samples. The correlation is calculated from a downsampled version of
  // `input`, and is written to `output`.
  void Correlation(const int16_t* input,
                   size_t input_length,
                   int16_t* output) const;

  void UpdateLagIndex();

  BackgroundNoise* const background_noise_;
  StatisticsCalculator* const statistics_;
  const size_t overlap_length_;
  size_t max_lag_;
  size_t expand_lags_[kNumLags];
  int lag_index_direction_;
  int current_lag_index_;
  bool stop_muting_;
  size_t expand_duration_samples_;
  std::unique_ptr<ChannelParameters[]> channel_parameters_;
};

struct ExpandFactory {
  ExpandFactory() {}
  virtual ~ExpandFactory() {}

  virtual Expand* Create(BackgroundNoise* background_noise,
                         SyncBuffer* sync_buffer,
                         RandomVector* random_vector,
                         StatisticsCalculator* statistics,
                         int fs,
                         size_t num_channels) const;
};

}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_EXPAND_H_
