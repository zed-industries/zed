/*
 *  Copyright 2016 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_OBJC_NATIVE_SRC_AUDIO_AUDIO_SESSION_OBSERVER_H_
#define SDK_OBJC_NATIVE_SRC_AUDIO_AUDIO_SESSION_OBSERVER_H_

#include "rtc_base/thread.h"

namespace webrtc {

// Observer interface for listening to AVAudioSession events.
class AudioSessionObserver {
 public:
  // Called when audio session interruption begins.
  virtual void OnInterruptionBegin() = 0;

  // Called when audio session interruption ends.
  virtual void OnInterruptionEnd(bool should_resume) = 0;

  // Called when audio route changes.
  virtual void OnValidRouteChange() = 0;

  // Called when the ability to play or record changes.
  virtual void OnCanPlayOrRecordChange(bool can_play_or_record) = 0;

  virtual void OnChangedOutputVolume() = 0;

 protected:
  virtual ~AudioSessionObserver() {}
};

}  // namespace webrtc

#endif  //  SDK_OBJC_NATIVE_SRC_AUDIO_AUDIO_SESSION_OBSERVER_H_
