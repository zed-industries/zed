/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_INCLUDE_AUDIO_FRAME_VIEW_H_
#define MODULES_AUDIO_PROCESSING_INCLUDE_AUDIO_FRAME_VIEW_H_

#include "api/audio/audio_view.h"

namespace webrtc {

// Class to pass audio data in T** format, where T is a numeric type.
template <class T>
class AudioFrameView {
 public:
  // `num_channels` and `channel_size` describe the T**
  // `audio_samples`. `audio_samples` is assumed to point to a
  // two-dimensional |num_channels * channel_size| array of floats.
  //
  // Note: The implementation now only requires the first channel pointer.
  // The previous implementation retained a pointer to externally owned array
  // of channel pointers, but since the channel size and count are provided
  // and the array is assumed to be a single two-dimensional array, the other
  // channel pointers can be calculated based on that (which is what the class
  // now uses `DeinterleavedView<>` internally for).
  AudioFrameView(T* const* audio_samples, int num_channels, int channel_size)
      : view_(num_channels && channel_size ? audio_samples[0] : nullptr,
              channel_size,
              num_channels) {
    RTC_DCHECK_GE(view_.num_channels(), 0);
    RTC_DCHECK_GE(view_.samples_per_channel(), 0);
  }

  // Implicit cast to allow converting AudioFrameView<float> to
  // AudioFrameView<const float>.
  template <class U>
  AudioFrameView(AudioFrameView<U> other) : view_(other.view()) {}

  // Allow constructing AudioFrameView from a DeinterleavedView.
  template <class U>
  explicit AudioFrameView(DeinterleavedView<U> view) : view_(view) {}

  AudioFrameView() = delete;

  int num_channels() const { return view_.num_channels(); }
  int samples_per_channel() const { return view_.samples_per_channel(); }
  MonoView<T> channel(int idx) { return view_[idx]; }
  MonoView<const T> channel(int idx) const { return view_[idx]; }
  MonoView<T> operator[](int idx) { return view_[idx]; }
  MonoView<const T> operator[](int idx) const { return view_[idx]; }

  DeinterleavedView<T> view() { return view_; }
  DeinterleavedView<const T> view() const { return view_; }

 private:
  DeinterleavedView<T> view_;
};
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_INCLUDE_AUDIO_FRAME_VIEW_H_
