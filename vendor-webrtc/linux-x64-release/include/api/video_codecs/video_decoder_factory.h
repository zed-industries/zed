/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_CODECS_VIDEO_DECODER_FACTORY_H_
#define API_VIDEO_CODECS_VIDEO_DECODER_FACTORY_H_

#include <memory>
#include <vector>

#include "api/environment/environment.h"
#include "api/video_codecs/sdp_video_format.h"
#include "api/video_codecs/video_decoder.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// A factory that creates VideoDecoders.
// NOTE: This class is still under development and may change without notice.
class RTC_EXPORT VideoDecoderFactory {
 public:
  struct CodecSupport {
    bool is_supported = false;
    bool is_power_efficient = false;
  };

  virtual ~VideoDecoderFactory() = default;

  // Returns a list of supported video formats in order of preference, to use
  // for signaling etc.
  virtual std::vector<SdpVideoFormat> GetSupportedFormats() const = 0;

  // Query whether the specifed format is supported or not and if it will be
  // power efficient, which is currently interpreted as if there is support for
  // hardware acceleration.
  // The parameter `reference_scaling` is used to query support for prediction
  // across spatial layers. An example where support for reference scaling is
  // needed is if the video stream is produced with a scalability mode that has
  // a dependency between the spatial layers. See
  // https://w3c.github.io/webrtc-svc/#scalabilitymodes* for a specification of
  // different scalabilty modes. NOTE: QueryCodecSupport is currently an
  // experimental feature that is subject to change without notice.
  virtual CodecSupport QueryCodecSupport(const SdpVideoFormat& format,
                                         bool reference_scaling) const;

  // Creates a VideoDecoder for the specified `format`.
  virtual std::unique_ptr<VideoDecoder> Create(
      const Environment& env,
      const SdpVideoFormat& format) = 0;
};

}  // namespace webrtc

#endif  // API_VIDEO_CODECS_VIDEO_DECODER_FACTORY_H_
