/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_OBJC_NATIVE_SRC_OBJC_VIDEO_ENCODER_FACTORY_H_
#define SDK_OBJC_NATIVE_SRC_OBJC_VIDEO_ENCODER_FACTORY_H_

#import <Foundation/Foundation.h>

#include <optional>
#include <string>

#include "api/environment/environment.h"
#include "api/video_codecs/sdp_video_format.h"
#include "api/video_codecs/video_encoder_factory.h"
#import "sdk/objc/base/RTCMacros.h"

@protocol RTC_OBJC_TYPE
(RTCVideoEncoderFactory);

namespace webrtc {

class ObjCVideoEncoderFactory : public VideoEncoderFactory {
 public:
  explicit ObjCVideoEncoderFactory(id<RTC_OBJC_TYPE(RTCVideoEncoderFactory)>);
  ~ObjCVideoEncoderFactory() override;

  id<RTC_OBJC_TYPE(RTCVideoEncoderFactory)> wrapped_encoder_factory() const;

  std::vector<SdpVideoFormat> GetSupportedFormats() const override;
  std::vector<SdpVideoFormat> GetImplementations() const override;
  CodecSupport QueryCodecSupport(
      const SdpVideoFormat& format,
      std::optional<std::string> scalability_mode) const override;
  std::unique_ptr<VideoEncoder> Create(const Environment& env,
                                       const SdpVideoFormat& format) override;
  std::unique_ptr<EncoderSelectorInterface> GetEncoderSelector() const override;

 private:
  id<RTC_OBJC_TYPE(RTCVideoEncoderFactory)> encoder_factory_;
};

}  // namespace webrtc

#endif  // SDK_OBJC_NATIVE_SRC_OBJC_VIDEO_ENCODER_FACTORY_H_
