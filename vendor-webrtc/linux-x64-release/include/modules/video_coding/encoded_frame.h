/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_ENCODED_FRAME_H_
#define MODULES_VIDEO_CODING_ENCODED_FRAME_H_

#include <cstdint>

#include "api/video/encoded_image.h"
#include "api/video/video_codec_type.h"
#include "modules/rtp_rtcp/source/rtp_video_header.h"
#include "modules/video_coding/include/video_codec_interface.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

class RTC_EXPORT VCMEncodedFrame : public EncodedImage {
 public:
  VCMEncodedFrame();
  VCMEncodedFrame(const VCMEncodedFrame&);

  ~VCMEncodedFrame();
  /**
   *   Set render time in milliseconds
   */
  void SetRenderTime(const int64_t renderTimeMs) {
    _renderTimeMs = renderTimeMs;
  }

  /**
   *   Get the encoded image
   */
  const webrtc::EncodedImage& EncodedImage() const {
    return static_cast<const webrtc::EncodedImage&>(*this);
  }

  using EncodedImage::ColorSpace;
  using EncodedImage::data;
  using EncodedImage::GetEncodedData;
  using EncodedImage::NtpTimeMs;
  using EncodedImage::PacketInfos;
  using EncodedImage::RtpTimestamp;
  using EncodedImage::set_size;
  using EncodedImage::SetColorSpace;
  using EncodedImage::SetEncodedData;
  using EncodedImage::SetPacketInfos;
  using EncodedImage::SetRtpTimestamp;
  using EncodedImage::SetSpatialIndex;
  using EncodedImage::SetSpatialLayerFrameSize;
  using EncodedImage::size;
  using EncodedImage::SpatialIndex;
  using EncodedImage::SpatialLayerFrameSize;

  /**
   *   Get render time in milliseconds
   */
  int64_t RenderTimeMs() const { return _renderTimeMs; }
  /**
   *   True if there's a frame missing before this frame
   */
  bool MissingFrame() const { return _missingFrame; }
  /**
   *   Payload type of the encoded payload
   */
  uint8_t PayloadType() const { return _payloadType; }
  /**
   *   Get codec specific info.
   *   The returned pointer is only valid as long as the VCMEncodedFrame
   *   is valid. Also, VCMEncodedFrame owns the pointer and will delete
   *   the object.
   */
  const CodecSpecificInfo* CodecSpecific() const { return &_codecSpecificInfo; }
  void SetCodecSpecific(const CodecSpecificInfo* codec_specific) {
    _codecSpecificInfo = *codec_specific;
  }

 protected:
  void Reset();

  void CopyCodecSpecific(const RTPVideoHeader* header);

  int64_t _renderTimeMs;
  uint8_t _payloadType;
  bool _missingFrame;
  CodecSpecificInfo _codecSpecificInfo;
  webrtc::VideoCodecType _codec;
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_ENCODED_FRAME_H_
