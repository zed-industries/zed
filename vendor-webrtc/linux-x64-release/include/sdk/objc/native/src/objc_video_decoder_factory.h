/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_OBJC_NATIVE_SRC_OBJC_VIDEO_DECODER_FACTORY_H_
#define SDK_OBJC_NATIVE_SRC_OBJC_VIDEO_DECODER_FACTORY_H_

#include "api/environment/environment.h"
#include "api/video_codecs/video_decoder_factory.h"
#include "media/base/codec.h"
#import "sdk/objc/base/RTCMacros.h"

@protocol RTC_OBJC_TYPE
(RTCVideoDecoderFactory);

namespace webrtc {

class ObjCVideoDecoderFactory : public VideoDecoderFactory {
 public:
  explicit ObjCVideoDecoderFactory(id<RTC_OBJC_TYPE(RTCVideoDecoderFactory)>);
  ~ObjCVideoDecoderFactory() override;

  id<RTC_OBJC_TYPE(RTCVideoDecoderFactory)> wrapped_decoder_factory() const;

  std::vector<SdpVideoFormat> GetSupportedFormats() const override;
  std::unique_ptr<VideoDecoder> Create(const Environment& env,
                                       const SdpVideoFormat& format) override;

 private:
  id<RTC_OBJC_TYPE(RTCVideoDecoderFactory)> decoder_factory_;
};

}  // namespace webrtc

#endif  // SDK_OBJC_NATIVE_SRC_OBJC_VIDEO_DECODER_FACTORY_H_
