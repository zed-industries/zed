/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef AUDIO_REMIX_RESAMPLE_H_
#define AUDIO_REMIX_RESAMPLE_H_

#include "api/audio/audio_frame.h"
#include "api/audio/audio_view.h"
#include "common_audio/resampler/include/push_resampler.h"

namespace webrtc {
namespace voe {

// Note: The RemixAndResample methods assume 10ms buffer sizes.

// Upmix or downmix and resample the audio to `dst_frame`. Expects `dst_frame`
// to have its sample rate and channels members set to the desired values.
// Updates the `samples_per_channel_` member accordingly.
//
// This version has an AudioFrame `src_frame` as input and sets the output
// `timestamp_`, `elapsed_time_ms_` and `ntp_time_ms_` members equals to the
// input ones.
void RemixAndResample(const AudioFrame& src_frame,
                      PushResampler<int16_t>* resampler,
                      AudioFrame* dst_frame);

// TODO(tommi): The `sample_rate_hz` argument can probably be removed since it's
// always related to `src_data.samples_per_frame()'.
void RemixAndResample(InterleavedView<const int16_t> src_data,
                      int sample_rate_hz,
                      PushResampler<int16_t>* resampler,
                      AudioFrame* dst_frame);

}  // namespace voe
}  // namespace webrtc

#endif  // AUDIO_REMIX_RESAMPLE_H_
