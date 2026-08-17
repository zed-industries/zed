/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_ANDROID_SRC_JNI_VIDEO_ENCODER_WRAPPER_H_
#define SDK_ANDROID_SRC_JNI_VIDEO_ENCODER_WRAPPER_H_

#include <jni.h>

#include <deque>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "api/video_codecs/video_encoder.h"
#include "common_video/h264/h264_bitstream_parser.h"
#ifdef RTC_ENABLE_H265
#include "common_video/h265/h265_bitstream_parser.h"
#endif
#include "modules/video_coding/codecs/vp9/include/vp9_globals.h"
#include "modules/video_coding/svc/scalable_video_controller_no_layering.h"
#include "rtc_base/synchronization/mutex.h"
#include "sdk/android/src/jni/jni_helpers.h"

namespace webrtc {
namespace jni {

// Wraps a Java encoder and delegates all calls to it.
class VideoEncoderWrapper : public VideoEncoder {
 public:
  VideoEncoderWrapper(JNIEnv* jni, const JavaRef<jobject>& j_encoder);
  ~VideoEncoderWrapper() override;

  int32_t InitEncode(const VideoCodec* codec_settings,
                     const Settings& settings) override;

  int32_t RegisterEncodeCompleteCallback(
      EncodedImageCallback* callback) override;

  int32_t Release() override;

  int32_t Encode(const VideoFrame& frame,
                 const std::vector<VideoFrameType>* frame_types) override;

  void SetRates(const RateControlParameters& rc_parameters) override;

  EncoderInfo GetEncoderInfo() const override;

  // Should only be called by JNI.
  void OnEncodedFrame(JNIEnv* jni, const JavaRef<jobject>& j_encoded_image);

 private:
  struct FrameExtraInfo {
    int64_t capture_time_ns;  // Used as an identifier of the frame.

    uint32_t timestamp_rtp;
  };

  int32_t InitEncodeInternal(JNIEnv* jni);

  // Takes Java VideoCodecStatus, handles it and returns WEBRTC_VIDEO_CODEC_*
  // status code.
  int32_t HandleReturnCode(JNIEnv* jni,
                           const JavaRef<jobject>& j_value,
                           const char* method_name);

  int ParseQp(ArrayView<const uint8_t> buffer);

  CodecSpecificInfo ParseCodecSpecificInfo(const EncodedImage& frame);

  ScopedJavaLocalRef<jobject> ToJavaBitrateAllocation(
      JNIEnv* jni,
      const VideoBitrateAllocation& allocation);

  ScopedJavaLocalRef<jobject> ToJavaRateControlParameters(
      JNIEnv* jni,
      const VideoEncoder::RateControlParameters& rc_parameters);

  void UpdateEncoderInfo(JNIEnv* jni);

  ScalingSettings GetScalingSettingsInternal(JNIEnv* jni) const;
  std::vector<ResolutionBitrateLimits> GetResolutionBitrateLimits(
      JNIEnv* jni) const;

  VideoEncoder::EncoderInfo GetEncoderInfoInternal(JNIEnv* jni) const;

  const ScopedJavaGlobalRef<jobject> encoder_;
  const ScopedJavaGlobalRef<jclass> int_array_class_;

  // Modified both on the encoder thread and the callback thread.
  Mutex frame_extra_infos_lock_;
  std::deque<FrameExtraInfo> frame_extra_infos_
      RTC_GUARDED_BY(frame_extra_infos_lock_);
  EncodedImageCallback* callback_;
  bool initialized_;
  int num_resets_;
  std::optional<VideoEncoder::Capabilities> capabilities_;
  int number_of_cores_;
  VideoCodec codec_settings_;
  EncoderInfo encoder_info_;
  H264BitstreamParser h264_bitstream_parser_;
#ifdef RTC_ENABLE_H265
  H265BitstreamParser h265_bitstream_parser_;
#endif

  // Fills frame dependencies in codec-agnostic format.
  ScalableVideoControllerNoLayering svc_controller_;
  // VP9 variables to populate codec specific structure.
  GofInfoVP9 gof_;  // Contains each frame's temporal information for
                    // non-flexible VP9 mode.
  size_t gof_idx_;
};

/* If the j_encoder is a wrapped native encoder, unwrap it. If it is not,
 * wrap it in a VideoEncoderWrapper.
 */
std::unique_ptr<VideoEncoder> JavaToNativeVideoEncoder(
    JNIEnv* jni,
    const JavaRef<jobject>& j_encoder,
    jlong webrtcEnvRef);

bool IsHardwareVideoEncoder(JNIEnv* jni, const JavaRef<jobject>& j_encoder);

std::vector<VideoEncoder::ResolutionBitrateLimits>
JavaToNativeResolutionBitrateLimits(
    JNIEnv* jni,
    const JavaRef<jobjectArray>& j_bitrate_limits_array);

}  // namespace jni
}  // namespace webrtc

#endif  // SDK_ANDROID_SRC_JNI_VIDEO_ENCODER_WRAPPER_H_
