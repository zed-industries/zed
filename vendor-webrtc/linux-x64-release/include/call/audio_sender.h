/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef CALL_AUDIO_SENDER_H_
#define CALL_AUDIO_SENDER_H_

#include <memory>

#include "api/audio/audio_frame.h"

namespace webrtc {

class AudioSender {
 public:
  // Encode and send audio.
  virtual void SendAudioData(std::unique_ptr<AudioFrame> audio_frame) = 0;

  virtual ~AudioSender() = default;
};

}  // namespace webrtc

#endif  // CALL_AUDIO_SENDER_H_
