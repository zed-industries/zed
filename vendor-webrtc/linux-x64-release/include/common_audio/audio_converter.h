/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef COMMON_AUDIO_AUDIO_CONVERTER_H_
#define COMMON_AUDIO_AUDIO_CONVERTER_H_

#include <stddef.h>

#include <memory>

namespace webrtc {

// Format conversion (remixing and resampling) for audio. Only simple remixing
// conversions are supported: downmix to mono (i.e. `dst_channels` == 1) or
// upmix from mono (i.e. |src_channels == 1|).
//
// The source and destination chunks have the same duration in time; specifying
// the number of frames is equivalent to specifying the sample rates.
class AudioConverter {
 public:
  // Returns a new AudioConverter, which will use the supplied format for its
  // lifetime. Caller is responsible for the memory.
  static std::unique_ptr<AudioConverter> Create(size_t src_channels,
                                                size_t src_frames,
                                                size_t dst_channels,
                                                size_t dst_frames);
  virtual ~AudioConverter() {}

  AudioConverter(const AudioConverter&) = delete;
  AudioConverter& operator=(const AudioConverter&) = delete;

  // Convert `src`, containing `src_size` samples, to `dst`, having a sample
  // capacity of `dst_capacity`. Both point to a series of buffers containing
  // the samples for each channel. The sizes must correspond to the format
  // passed to Create().
  virtual void Convert(const float* const* src,
                       size_t src_size,
                       float* const* dst,
                       size_t dst_capacity) = 0;

  size_t src_channels() const { return src_channels_; }
  size_t src_frames() const { return src_frames_; }
  size_t dst_channels() const { return dst_channels_; }
  size_t dst_frames() const { return dst_frames_; }

 protected:
  AudioConverter();
  AudioConverter(size_t src_channels,
                 size_t src_frames,
                 size_t dst_channels,
                 size_t dst_frames);

  // Helper to RTC_CHECK that inputs are correctly sized.
  void CheckSizes(size_t src_size, size_t dst_capacity) const;

 private:
  const size_t src_channels_;
  const size_t src_frames_;
  const size_t dst_channels_;
  const size_t dst_frames_;
};

}  // namespace webrtc

#endif  // COMMON_AUDIO_AUDIO_CONVERTER_H_
