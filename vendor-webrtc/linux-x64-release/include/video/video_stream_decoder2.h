/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_VIDEO_STREAM_DECODER2_H_
#define VIDEO_VIDEO_STREAM_DECODER2_H_

#include <cstdint>
#include <list>
#include <map>
#include <memory>
#include <vector>

#include "api/scoped_refptr.h"
#include "api/video/video_frame.h"
#include "api/video/video_sink_interface.h"
#include "api/video_codecs/video_decoder.h"
#include "modules/remote_bitrate_estimator/include/remote_bitrate_estimator.h"
#include "modules/video_coding/include/video_coding_defines.h"
#include "rtc_base/platform_thread.h"

namespace webrtc {

class VideoReceiver2;

namespace internal {

class ReceiveStatisticsProxy;

class VideoStreamDecoder : public VCMReceiveCallback {
 public:
  VideoStreamDecoder(VideoReceiver2* video_receiver,
                     ReceiveStatisticsProxy* receive_statistics_proxy,
                     VideoSinkInterface<VideoFrame>* incoming_video_stream);
  ~VideoStreamDecoder() override;

  // Implements VCMReceiveCallback.
  int32_t OnFrameToRender(const FrameToRender& arguments) override;
  void OnDroppedFrames(uint32_t frames_dropped) override;
  void OnIncomingPayloadType(int payload_type) override;
  void OnDecoderInfoChanged(
      const VideoDecoder::DecoderInfo& decoder_info) override;

 private:
  VideoReceiver2* const video_receiver_;
  ReceiveStatisticsProxy* const receive_stats_callback_;
  VideoSinkInterface<VideoFrame>* const incoming_video_stream_;
};

}  // namespace internal
}  // namespace webrtc

#endif  // VIDEO_VIDEO_STREAM_DECODER2_H_
