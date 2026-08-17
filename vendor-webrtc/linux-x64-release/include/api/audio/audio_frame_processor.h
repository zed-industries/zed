/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_AUDIO_AUDIO_FRAME_PROCESSOR_H_
#define API_AUDIO_AUDIO_FRAME_PROCESSOR_H_

#include <functional>
#include <memory>

namespace webrtc {

class AudioFrame;

// If passed into PeerConnectionFactory, will be used for additional
// processing of captured audio frames, performed before encoding.
// Implementations must be thread-safe.
class AudioFrameProcessor {
 public:
  using OnAudioFrameCallback = std::function<void(std::unique_ptr<AudioFrame>)>;
  virtual ~AudioFrameProcessor() = default;

  // Processes the frame received from WebRTC, is called by WebRTC off the
  // realtime audio capturing path. AudioFrameProcessor must reply with
  // processed frames by calling `sink_callback` if it was provided in SetSink()
  // call. `sink_callback` can be called in the context of Process().
  virtual void Process(std::unique_ptr<AudioFrame> frame) = 0;

  // Atomically replaces the current sink with the new one. Before the
  // first call to this function, or if the provided `sink_callback` is nullptr,
  // processed frames are simply discarded.
  virtual void SetSink(OnAudioFrameCallback sink_callback) = 0;
};

}  // namespace webrtc

#endif  // API_AUDIO_AUDIO_FRAME_PROCESSOR_H_
