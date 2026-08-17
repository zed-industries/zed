/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef COMMON_AUDIO_CHANNEL_BUFFER_H_
#define COMMON_AUDIO_CHANNEL_BUFFER_H_

#include <string.h>

#include <memory>
#include <vector>

#include "api/array_view.h"
#include "common_audio/include/audio_util.h"
#include "rtc_base/checks.h"
#include "rtc_base/gtest_prod_util.h"

namespace webrtc {

// TODO: b/335805780 - Remove this method. Instead, use Deinterleave() from
// audio_util.h which requires size checked buffer views.
template <typename T>
void Deinterleave(const T* interleaved,
                  size_t samples_per_channel,
                  size_t num_channels,
                  T* const* deinterleaved) {
  for (size_t i = 0; i < num_channels; ++i) {
    T* channel = deinterleaved[i];
    size_t interleaved_idx = i;
    for (size_t j = 0; j < samples_per_channel; ++j) {
      channel[j] = interleaved[interleaved_idx];
      interleaved_idx += num_channels;
    }
  }
}

// `Interleave()` variant for cases where the deinterleaved channels aren't
// represented by a `DeinterleavedView`.
// TODO: b/335805780 - Remove this method. Instead, use Deinterleave() from
// audio_util.h which requires size checked buffer views.
template <typename T>
void Interleave(const T* const* deinterleaved,
                size_t samples_per_channel,
                size_t num_channels,
                InterleavedView<T>& interleaved) {
  RTC_DCHECK_EQ(NumChannels(interleaved), num_channels);
  RTC_DCHECK_EQ(SamplesPerChannel(interleaved), samples_per_channel);
  for (size_t i = 0; i < num_channels; ++i) {
    const T* channel = deinterleaved[i];
    size_t interleaved_idx = i;
    for (size_t j = 0; j < samples_per_channel; ++j) {
      interleaved[interleaved_idx] = channel[j];
      interleaved_idx += num_channels;
    }
  }
}

// Helper to encapsulate a contiguous data buffer, full or split into frequency
// bands, with access to a pointer arrays of the deinterleaved channels and
// bands. The buffer is zero initialized at creation.
//
// The buffer structure is showed below for a 2 channel and 2 bands case:
//
// `data_`:
// { [ --- b1ch1 --- ] [ --- b2ch1 --- ] [ --- b1ch2 --- ] [ --- b2ch2 --- ] }
//
// The pointer arrays for the same example are as follows:
//
// `channels_`:
// { [ b1ch1* ] [ b1ch2* ] [ b2ch1* ] [ b2ch2* ] }
//
// `bands_`:
// { [ b1ch1* ] [ b2ch1* ] [ b1ch2* ] [ b2ch2* ] }
template <typename T>
class ChannelBuffer {
 public:
  ChannelBuffer(size_t num_frames, size_t num_channels, size_t num_bands = 1)
      : data_(new T[num_frames * num_channels]()),
        channels_(new T*[num_channels * num_bands]),
        bands_(new T*[num_channels * num_bands]),
        num_frames_(num_frames),
        num_frames_per_band_(num_frames / num_bands),
        num_allocated_channels_(num_channels),
        num_channels_(num_channels),
        num_bands_(num_bands),
        bands_view_(num_allocated_channels_,
                    std::vector<ArrayView<T>>(num_bands_)),
        channels_view_(num_bands_,
                       std::vector<ArrayView<T>>(num_allocated_channels_)) {
    // Temporarily cast away const_ness to allow populating the array views.
    auto* bands_view =
        const_cast<std::vector<std::vector<ArrayView<T>>>*>(&bands_view_);
    auto* channels_view =
        const_cast<std::vector<std::vector<ArrayView<T>>>*>(&channels_view_);

    for (size_t ch = 0; ch < num_allocated_channels_; ++ch) {
      for (size_t band = 0; band < num_bands_; ++band) {
        (*channels_view)[band][ch] =
            ArrayView<T>(&data_[ch * num_frames_ + band * num_frames_per_band_],
                         num_frames_per_band_);
        (*bands_view)[ch][band] = channels_view_[band][ch];
        channels_[band * num_allocated_channels_ + ch] =
            channels_view_[band][ch].data();
        bands_[ch * num_bands_ + band] =
            channels_[band * num_allocated_channels_ + ch];
      }
    }
  }

  // Returns a pointer array to the channels.
  // If band is explicitly specificed, the channels for a specific band are
  // returned and the usage becomes: channels(band)[channel][sample].
  // Where:
  // 0 <= band < `num_bands_`
  // 0 <= channel < `num_allocated_channels_`
  // 0 <= sample < `num_frames_per_band_`

  // If band is not explicitly specified, the full-band channels (or lower band
  // channels) are returned and the usage becomes: channels()[channel][sample].
  // Where:
  // 0 <= channel < `num_allocated_channels_`
  // 0 <= sample < `num_frames_`
  const T* const* channels(size_t band = 0) const {
    RTC_DCHECK_LT(band, num_bands_);
    return &channels_[band * num_allocated_channels_];
  }
  T* const* channels(size_t band = 0) {
    const ChannelBuffer<T>* t = this;
    return const_cast<T* const*>(t->channels(band));
  }
  ArrayView<const ArrayView<T>> channels_view(size_t band = 0) {
    return channels_view_[band];
  }
  ArrayView<const ArrayView<T>> channels_view(size_t band = 0) const {
    return channels_view_[band];
  }

  // Returns a pointer array to the bands for a specific channel.
  // Usage:
  // bands(channel)[band][sample].
  // Where:
  // 0 <= channel < `num_channels_`
  // 0 <= band < `num_bands_`
  // 0 <= sample < `num_frames_per_band_`
  const T* const* bands(size_t channel) const {
    RTC_DCHECK_LT(channel, num_channels_);
    RTC_DCHECK_GE(channel, 0);
    return &bands_[channel * num_bands_];
  }
  T* const* bands(size_t channel) {
    const ChannelBuffer<T>* t = this;
    return const_cast<T* const*>(t->bands(channel));
  }

  ArrayView<const ArrayView<T>> bands_view(size_t channel) {
    return bands_view_[channel];
  }
  ArrayView<const ArrayView<T>> bands_view(size_t channel) const {
    return bands_view_[channel];
  }

  // Sets the `slice` pointers to the `start_frame` position for each channel.
  // Returns `slice` for convenience.
  const T* const* Slice(T** slice, size_t start_frame) const {
    RTC_DCHECK_LT(start_frame, num_frames_);
    for (size_t i = 0; i < num_channels_; ++i)
      slice[i] = &channels_[i][start_frame];
    return slice;
  }
  T** Slice(T** slice, size_t start_frame) {
    const ChannelBuffer<T>* t = this;
    return const_cast<T**>(t->Slice(slice, start_frame));
  }

  size_t num_frames() const { return num_frames_; }
  size_t num_frames_per_band() const { return num_frames_per_band_; }
  size_t num_channels() const { return num_channels_; }
  size_t num_bands() const { return num_bands_; }
  size_t size() const { return num_frames_ * num_allocated_channels_; }

  void set_num_channels(size_t num_channels) {
    RTC_DCHECK_LE(num_channels, num_allocated_channels_);
    num_channels_ = num_channels;
  }

  void SetDataForTesting(const T* data, size_t size) {
    RTC_CHECK_EQ(size, this->size());
    memcpy(data_.get(), data, size * sizeof(*data));
  }

 private:
  std::unique_ptr<T[]> data_;
  std::unique_ptr<T*[]> channels_;
  std::unique_ptr<T*[]> bands_;
  const size_t num_frames_;
  const size_t num_frames_per_band_;
  // Number of channels the internal buffer holds.
  const size_t num_allocated_channels_;
  // Number of channels the user sees.
  size_t num_channels_;
  const size_t num_bands_;
  const std::vector<std::vector<ArrayView<T>>> bands_view_;
  const std::vector<std::vector<ArrayView<T>>> channels_view_;
};

// One int16_t and one float ChannelBuffer that are kept in sync. The sync is
// broken when someone requests write access to either ChannelBuffer, and
// reestablished when someone requests the outdated ChannelBuffer. It is
// therefore safe to use the return value of ibuf_const() and fbuf_const()
// until the next call to ibuf() or fbuf(), and the return value of ibuf() and
// fbuf() until the next call to any of the other functions.
class IFChannelBuffer {
 public:
  IFChannelBuffer(size_t num_frames, size_t num_channels, size_t num_bands = 1);
  ~IFChannelBuffer();

  ChannelBuffer<int16_t>* ibuf();
  ChannelBuffer<float>* fbuf();
  const ChannelBuffer<int16_t>* ibuf_const() const;
  const ChannelBuffer<float>* fbuf_const() const;

  size_t num_frames() const { return ibuf_.num_frames(); }
  size_t num_frames_per_band() const { return ibuf_.num_frames_per_band(); }
  size_t num_channels() const {
    return ivalid_ ? ibuf_.num_channels() : fbuf_.num_channels();
  }
  void set_num_channels(size_t num_channels) {
    ibuf_.set_num_channels(num_channels);
    fbuf_.set_num_channels(num_channels);
  }
  size_t num_bands() const { return ibuf_.num_bands(); }

 private:
  void RefreshF() const;
  void RefreshI() const;

  mutable bool ivalid_;
  mutable ChannelBuffer<int16_t> ibuf_;
  mutable bool fvalid_;
  mutable ChannelBuffer<float> fbuf_;
};

}  // namespace webrtc

#endif  // COMMON_AUDIO_CHANNEL_BUFFER_H_
