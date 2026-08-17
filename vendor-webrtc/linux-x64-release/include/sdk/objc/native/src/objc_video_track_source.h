/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_OBJC_CLASSES_VIDEO_OBJC_VIDEO_TRACK_SOURCE_H_
#define SDK_OBJC_CLASSES_VIDEO_OBJC_VIDEO_TRACK_SOURCE_H_

#import "base/RTCVideoCapturer.h"

#include "media/base/adapted_video_track_source.h"
#include "rtc_base/timestamp_aligner.h"
#include "sdk/objc/base/RTCMacros.h"

RTC_FWD_DECL_OBJC_CLASS(RTC_OBJC_TYPE(RTCVideoFrame));

@interface RTC_OBJC_TYPE(RTCObjCVideoSourceAdapter) : NSObject <RTC_OBJC_TYPE (RTCVideoCapturerDelegate)>
@end

namespace webrtc {

class ObjCVideoTrackSource : public webrtc::AdaptedVideoTrackSource {
 public:
  ObjCVideoTrackSource();
  explicit ObjCVideoTrackSource(bool is_screencast);
  explicit ObjCVideoTrackSource(RTC_OBJC_TYPE(RTCObjCVideoSourceAdapter)* adapter);

  bool is_screencast() const override;

  // Indicates that the encoder should denoise video before encoding it.
  // If it is not set, the default configuration is used which is different
  // depending on video codec.
  std::optional<bool> needs_denoising() const override;

  SourceState state() const override;

  bool remote() const override;

  void OnCapturedFrame(RTC_OBJC_TYPE(RTCVideoFrame) * frame);

  // Called by RTCVideoSource.
  void OnOutputFormatRequest(int width, int height, int fps);

 private:
  webrtc::VideoBroadcaster broadcaster_;
  webrtc::TimestampAligner timestamp_aligner_;

  RTC_OBJC_TYPE(RTCObjCVideoSourceAdapter)* adapter_;
  bool is_screencast_;
};

}  // namespace webrtc

#endif  // SDK_OBJC_CLASSES_VIDEO_OBJC_VIDEO_TRACK_SOURCE_H_
