/*
 *  Copyright 2020 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef CALL_ADAPTATION_VIDEO_STREAM_INPUT_STATE_PROVIDER_H_
#define CALL_ADAPTATION_VIDEO_STREAM_INPUT_STATE_PROVIDER_H_

#include "call/adaptation/encoder_settings.h"
#include "call/adaptation/video_stream_input_state.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread_annotations.h"
#include "video/video_stream_encoder_observer.h"

namespace webrtc {

class VideoStreamInputStateProvider {
 public:
  VideoStreamInputStateProvider(
      VideoStreamEncoderObserver* frame_rate_provider);
  virtual ~VideoStreamInputStateProvider();

  void OnHasInputChanged(bool has_input);
  void OnFrameSizeObserved(int frame_size_pixels);
  void OnEncoderSettingsChanged(EncoderSettings encoder_settings);

  virtual VideoStreamInputState InputState();

 private:
  Mutex mutex_;
  VideoStreamEncoderObserver* const frame_rate_provider_;
  VideoStreamInputState input_state_ RTC_GUARDED_BY(mutex_);
};

}  // namespace webrtc

#endif  // CALL_ADAPTATION_VIDEO_STREAM_INPUT_STATE_PROVIDER_H_
