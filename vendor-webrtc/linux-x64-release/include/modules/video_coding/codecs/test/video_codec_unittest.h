/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_CODECS_TEST_VIDEO_CODEC_UNITTEST_H_
#define MODULES_VIDEO_CODING_CODECS_TEST_VIDEO_CODEC_UNITTEST_H_

#include <cstddef>
#include <cstdint>
#include <memory>
#include <optional>
#include <vector>

#include "api/environment/environment.h"
#include "api/environment/environment_factory.h"
#include "api/test/frame_generator_interface.h"
#include "api/video/encoded_image.h"
#include "api/video/video_frame.h"
#include "api/video_codecs/video_codec.h"
#include "api/video_codecs/video_decoder.h"
#include "api/video_codecs/video_encoder.h"
#include "modules/video_coding/include/video_codec_interface.h"
#include "rtc_base/checks.h"
#include "rtc_base/event.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread_annotations.h"
#include "test/gtest.h"

namespace webrtc {

class VideoCodecUnitTest : public ::testing::Test {
 public:
  VideoCodecUnitTest()
      : env_(CreateEnvironment()),
        encode_complete_callback_(this),
        decode_complete_callback_(this),
        wait_for_encoded_frames_threshold_(1),
        last_input_frame_timestamp_(0) {}

 protected:
  class FakeEncodeCompleteCallback : public webrtc::EncodedImageCallback {
   public:
    explicit FakeEncodeCompleteCallback(VideoCodecUnitTest* test)
        : test_(test) {}

    Result OnEncodedImage(const EncodedImage& frame,
                          const CodecSpecificInfo* codec_specific_info);

   private:
    VideoCodecUnitTest* const test_;
  };

  class FakeDecodeCompleteCallback : public webrtc::DecodedImageCallback {
   public:
    explicit FakeDecodeCompleteCallback(VideoCodecUnitTest* test)
        : test_(test) {}

    int32_t Decoded(VideoFrame& /* frame */) override {
      RTC_DCHECK_NOTREACHED();
      return -1;
    }
    int32_t Decoded(VideoFrame& /* frame */,
                    int64_t /* decode_time_ms */) override {
      RTC_DCHECK_NOTREACHED();
      return -1;
    }
    void Decoded(VideoFrame& frame,
                 std::optional<int32_t> decode_time_ms,
                 std::optional<uint8_t> qp) override;

   private:
    VideoCodecUnitTest* const test_;
  };

  virtual std::unique_ptr<VideoEncoder> CreateEncoder() = 0;
  virtual std::unique_ptr<VideoDecoder> CreateDecoder() = 0;

  void SetUp() override;

  virtual void ModifyCodecSettings(VideoCodec* codec_settings);

  VideoFrame NextInputFrame();

  // Helper method for waiting a single encoded frame.
  bool WaitForEncodedFrame(EncodedImage* frame,
                           CodecSpecificInfo* codec_specific_info);

  // Helper methods for waiting for multiple encoded frames. Caller must
  // define how many frames are to be waited for via `num_frames` before calling
  // Encode(). Then, they can expect to retrive them via WaitForEncodedFrames().
  void SetWaitForEncodedFramesThreshold(size_t num_frames);
  bool WaitForEncodedFrames(
      std::vector<EncodedImage>* frames,
      std::vector<CodecSpecificInfo>* codec_specific_info);

  // Helper method for waiting a single decoded frame.
  bool WaitForDecodedFrame(std::unique_ptr<VideoFrame>* frame,
                           std::optional<uint8_t>* qp);

  size_t GetNumEncodedFrames();

  const Environment env_;
  VideoCodec codec_settings_;

  std::unique_ptr<VideoEncoder> encoder_;
  std::unique_ptr<VideoDecoder> decoder_;
  std::unique_ptr<test::FrameGeneratorInterface> input_frame_generator_;

 private:
  FakeEncodeCompleteCallback encode_complete_callback_;
  FakeDecodeCompleteCallback decode_complete_callback_;

  Event encoded_frame_event_;
  Mutex encoded_frame_section_;
  size_t wait_for_encoded_frames_threshold_;
  std::vector<EncodedImage> encoded_frames_
      RTC_GUARDED_BY(encoded_frame_section_);
  std::vector<CodecSpecificInfo> codec_specific_infos_
      RTC_GUARDED_BY(encoded_frame_section_);

  Event decoded_frame_event_;
  Mutex decoded_frame_section_;
  std::optional<VideoFrame> decoded_frame_
      RTC_GUARDED_BY(decoded_frame_section_);
  std::optional<uint8_t> decoded_qp_ RTC_GUARDED_BY(decoded_frame_section_);

  uint32_t last_input_frame_timestamp_;
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_CODECS_TEST_VIDEO_CODEC_UNITTEST_H_
