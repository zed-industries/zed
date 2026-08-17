/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AUDIO_BUFFER_H_
#define MODULES_AUDIO_PROCESSING_AUDIO_BUFFER_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <vector>

#include "api/audio/audio_processing.h"
#include "api/audio/audio_view.h"
#include "common_audio/channel_buffer.h"
#include "common_audio/include/audio_util.h"

namespace webrtc {

class PushSincResampler;
class SplittingFilter;

enum Band { kBand0To8kHz = 0, kBand8To16kHz = 1, kBand16To24kHz = 2 };

// Stores any audio data in a way that allows the audio processing module to
// operate on it in a controlled manner.
class AudioBuffer {
 public:
  static const int kSplitBandSize = 160;
  // TODO(tommi): Remove this (`AudioBuffer::kMaxSampleRate`) constant.
  static const int kMaxSampleRate = webrtc::kMaxSampleRateHz;
  AudioBuffer(size_t input_rate,
              size_t input_num_channels,
              size_t buffer_rate,
              size_t buffer_num_channels,
              size_t output_rate,
              size_t output_num_channels);

  virtual ~AudioBuffer();

  AudioBuffer(const AudioBuffer&) = delete;
  AudioBuffer& operator=(const AudioBuffer&) = delete;

  // Specify that downmixing should be done by selecting a single channel.
  void set_downmixing_to_specific_channel(size_t channel);

  // Specify that downmixing should be done by averaging all channels,.
  void set_downmixing_by_averaging();

  // Set the number of channels in the buffer. The specified number of channels
  // cannot be larger than the specified buffer_num_channels. The number is also
  // reset at each call to CopyFrom or InterleaveFrom.
  void set_num_channels(size_t num_channels);

  // Returns a DeinterleavedView<> over the channel data.
  DeinterleavedView<float> view() {
    return DeinterleavedView<float>(
        num_channels_ && buffer_num_frames_ ? channels()[0] : nullptr,
        buffer_num_frames_, num_channels_);
  }

  size_t num_channels() const { return num_channels_; }
  size_t num_frames() const { return buffer_num_frames_; }
  size_t num_frames_per_band() const { return num_split_frames_; }
  size_t num_bands() const { return num_bands_; }

  // Returns pointer arrays to the full-band channels.
  // Usage:
  // channels()[channel][sample].
  // Where:
  // 0 <= channel < `buffer_num_channels_`
  // 0 <= sample < `buffer_num_frames_`
  float* const* channels() { return data_->channels(); }
  const float* const* channels_const() const { return data_->channels(); }

  // Returns pointer arrays to the bands for a specific channel.
  // Usage:
  // split_bands(channel)[band][sample].
  // Where:
  // 0 <= channel < `buffer_num_channels_`
  // 0 <= band < `num_bands_`
  // 0 <= sample < `num_split_frames_`
  const float* const* split_bands_const(size_t channel) const {
    return split_data_.get() ? split_data_->bands(channel)
                             : data_->bands(channel);
  }
  float* const* split_bands(size_t channel) {
    return split_data_.get() ? split_data_->bands(channel)
                             : data_->bands(channel);
  }

  // Returns a pointer array to the channels for a specific band.
  // Usage:
  // split_channels(band)[channel][sample].
  // Where:
  // 0 <= band < `num_bands_`
  // 0 <= channel < `buffer_num_channels_`
  // 0 <= sample < `num_split_frames_`
  const float* const* split_channels_const(Band band) const {
    if (split_data_.get()) {
      return split_data_->channels(band);
    } else {
      return band == kBand0To8kHz ? data_->channels() : nullptr;
    }
  }

  // Copies data into the buffer.
  void CopyFrom(const int16_t* const interleaved_data,
                const StreamConfig& stream_config);
  void CopyFrom(const float* const* stacked_data,
                const StreamConfig& stream_config);

  // Copies data from the buffer.
  void CopyTo(const StreamConfig& stream_config,
              int16_t* const interleaved_data);
  void CopyTo(const StreamConfig& stream_config, float* const* stacked_data);
  void CopyTo(AudioBuffer* buffer) const;

  // Splits the buffer data into frequency bands.
  void SplitIntoFrequencyBands();

  // Recombines the frequency bands into a full-band signal.
  void MergeFrequencyBands();

  // Copies the split bands data into the integer two-dimensional array.
  void ExportSplitChannelData(size_t channel,
                              int16_t* const* split_band_data) const;

  // Copies the data in the integer two-dimensional array into the split_bands
  // data.
  void ImportSplitChannelData(size_t channel,
                              const int16_t* const* split_band_data);

  static const size_t kMaxSplitFrameLength = 160;
  static const size_t kMaxNumBands = 3;

  // Deprecated methods, will be removed soon.
  float* const* channels_f() { return channels(); }
  const float* const* channels_const_f() const { return channels_const(); }
  const float* const* split_bands_const_f(size_t channel) const {
    return split_bands_const(channel);
  }
  float* const* split_bands_f(size_t channel) { return split_bands(channel); }
  const float* const* split_channels_const_f(Band band) const {
    return split_channels_const(band);
  }

 private:
  FRIEND_TEST_ALL_PREFIXES(AudioBufferTest,
                           SetNumChannelsSetsChannelBuffersNumChannels);
  void RestoreNumChannels();

  const size_t input_num_frames_;
  const size_t input_num_channels_;
  const size_t buffer_num_frames_;
  const size_t buffer_num_channels_;
  const size_t output_num_frames_;
  const size_t output_num_channels_;

  size_t num_channels_;
  size_t num_bands_;
  size_t num_split_frames_;

  std::unique_ptr<ChannelBuffer<float>> data_;
  std::unique_ptr<ChannelBuffer<float>> split_data_;
  std::unique_ptr<SplittingFilter> splitting_filter_;
  std::vector<std::unique_ptr<PushSincResampler>> input_resamplers_;
  std::vector<std::unique_ptr<PushSincResampler>> output_resamplers_;
  bool downmix_by_averaging_ = true;
  size_t channel_for_downmixing_ = 0;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AUDIO_BUFFER_H_
