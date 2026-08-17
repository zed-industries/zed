/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_INCLUDE_AUDIO_FRAME_PROXIES_H_
#define MODULES_AUDIO_PROCESSING_INCLUDE_AUDIO_FRAME_PROXIES_H_

namespace webrtc {

class AudioFrame;
class AudioProcessing;

// Processes a 10 ms `frame` of the primary audio stream using the provided
// AudioProcessing object. On the client-side, this is the near-end (or
// captured) audio. The `sample_rate_hz_`, `num_channels_`, and
// `samples_per_channel_` members of `frame` must be valid. If changed from the
// previous call to this function, it will trigger an initialization of the
// provided AudioProcessing object.
// The function returns any error codes passed from the AudioProcessing
// ProcessStream method.
int ProcessAudioFrame(AudioProcessing* ap, AudioFrame* frame);

// Processes a 10 ms `frame` of the reverse direction audio stream using the
// provided AudioProcessing object. The frame may be modified. On the
// client-side, this is the far-end (or to be rendered) audio. The
// `sample_rate_hz_`, `num_channels_`, and `samples_per_channel_` members of
// `frame` must be valid. If changed from the previous call to this function, it
// will trigger an initialization of the provided AudioProcessing object.
// The function returns any error codes passed from the AudioProcessing
// ProcessReverseStream method.
int ProcessReverseAudioFrame(AudioProcessing* ap, AudioFrame* frame);

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_INCLUDE_AUDIO_FRAME_PROXIES_H_
