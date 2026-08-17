/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_OBJC_NATIVE_SRC_OBJC_FRAME_BUFFER_H_
#define SDK_OBJC_NATIVE_SRC_OBJC_FRAME_BUFFER_H_

#import <CoreVideo/CoreVideo.h>

#include "common_video/include/video_frame_buffer.h"
#import "sdk/objc/base/RTCMacros.h"

@protocol RTC_OBJC_TYPE
(RTCVideoFrameBuffer);

namespace webrtc {

class ObjCFrameBuffer : public VideoFrameBuffer {
 public:
  explicit ObjCFrameBuffer(id<RTC_OBJC_TYPE(RTCVideoFrameBuffer)>);
  ~ObjCFrameBuffer() override;

  Type type() const override;

  int width() const override;
  int height() const override;

  webrtc::scoped_refptr<I420BufferInterface> ToI420() override;
  webrtc::scoped_refptr<VideoFrameBuffer> CropAndScale(
      int offset_x,
      int offset_y,
      int crop_width,
      int crop_height,
      int scaled_width,
      int scaled_height) override;

  id<RTC_OBJC_TYPE(RTCVideoFrameBuffer)> wrapped_frame_buffer() const;

 private:
  id<RTC_OBJC_TYPE(RTCVideoFrameBuffer)> frame_buffer_;
  int width_;
  int height_;
};

id<RTC_OBJC_TYPE(RTCVideoFrameBuffer)> ToObjCVideoFrameBuffer(
    const webrtc::scoped_refptr<VideoFrameBuffer>& buffer);

}  // namespace webrtc

#endif  // SDK_OBJC_NATIVE_SRC_OBJC_FRAME_BUFFER_H_
