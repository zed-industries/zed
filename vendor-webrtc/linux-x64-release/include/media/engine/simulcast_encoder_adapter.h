/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 *
 */

#ifndef MEDIA_ENGINE_SIMULCAST_ENCODER_ADAPTER_H_
#define MEDIA_ENGINE_SIMULCAST_ENCODER_ADAPTER_H_

#include <atomic>
#include <cstddef>
#include <cstdint>
#include <list>
#include <memory>
#include <optional>
#include <vector>

#include "absl/base/nullability.h"
#include "api/environment/environment.h"
#include "api/fec_controller_override.h"
#include "api/sequence_checker.h"
#include "api/units/timestamp.h"
#include "api/video/encoded_image.h"
#include "api/video/video_frame.h"
#include "api/video/video_frame_type.h"
#include "api/video_codecs/sdp_video_format.h"
#include "api/video_codecs/video_codec.h"
#include "api/video_codecs/video_encoder.h"
#include "api/video_codecs/video_encoder_factory.h"
#include "common_video/framerate_controller.h"
#include "modules/video_coding/include/video_codec_interface.h"
#include "rtc_base/experiments/encoder_info_settings.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// SimulcastEncoderAdapter implements simulcast support by creating multiple
// webrtc::VideoEncoder instances with the given VideoEncoderFactory.
// The object is created and destroyed on the worker thread, but all public
// interfaces should be called from the encoder task queue.
class RTC_EXPORT SimulcastEncoderAdapter : public VideoEncoder {
 public:
  // `primary_factory` produces the first-choice encoders to use.
  // `fallback_factory`, if non-null, is used to create fallback encoder that
  // will be used if InitEncode() fails for the primary encoder.
  SimulcastEncoderAdapter(const Environment& env,
                          VideoEncoderFactory* absl_nonnull primary_factory,
                          VideoEncoderFactory* absl_nullable fallback_factory,
                          const SdpVideoFormat& format);

  ~SimulcastEncoderAdapter() override;

  // Implements VideoEncoder.
  void SetFecControllerOverride(
      FecControllerOverride* fec_controller_override) override;
  int Release() override;
  int InitEncode(const VideoCodec* codec_settings,
                 const VideoEncoder::Settings& settings) override;
  int Encode(const VideoFrame& input_image,
             const std::vector<VideoFrameType>* frame_types) override;
  int RegisterEncodeCompleteCallback(EncodedImageCallback* callback) override;
  void SetRates(const RateControlParameters& parameters) override;
  void OnPacketLossRateUpdate(float packet_loss_rate) override;
  void OnRttUpdate(int64_t rtt_ms) override;
  void OnLossNotification(const LossNotification& loss_notification) override;

  EncoderInfo GetEncoderInfo() const override;

 protected:
  void DestroyStoredEncoders();

 private:
  class EncoderContext {
   public:
    EncoderContext(std::unique_ptr<VideoEncoder> encoder,
                   bool prefer_temporal_support,
                   VideoEncoder::EncoderInfo primary_info,
                   VideoEncoder::EncoderInfo fallback_info);
    EncoderContext& operator=(EncoderContext&&) = delete;

    VideoEncoder& encoder() { return *encoder_; }
    bool prefer_temporal_support() { return prefer_temporal_support_; }
    void Release();

    const VideoEncoder::EncoderInfo& PrimaryInfo() { return primary_info_; }

    const VideoEncoder::EncoderInfo& FallbackInfo() { return fallback_info_; }

   private:
    std::unique_ptr<VideoEncoder> encoder_;
    bool prefer_temporal_support_;
    const VideoEncoder::EncoderInfo primary_info_;
    const VideoEncoder::EncoderInfo fallback_info_;
  };

  class StreamContext : public EncodedImageCallback {
   public:
    StreamContext(SimulcastEncoderAdapter* parent,
                  std::unique_ptr<EncoderContext> encoder_context,
                  std::unique_ptr<FramerateController> framerate_controller,
                  int stream_idx,
                  uint16_t width,
                  uint16_t height,
                  bool send_stream);
    StreamContext(StreamContext&& rhs);
    StreamContext& operator=(StreamContext&&) = delete;
    ~StreamContext() override;

    Result OnEncodedImage(
        const EncodedImage& encoded_image,
        const CodecSpecificInfo* codec_specific_info) override;
    void OnDroppedFrame(DropReason reason) override;

    VideoEncoder& encoder() { return encoder_context_->encoder(); }
    const VideoEncoder& encoder() const { return encoder_context_->encoder(); }
    int stream_idx() const { return stream_idx_; }
    uint16_t width() const { return width_; }
    uint16_t height() const { return height_; }
    bool is_keyframe_needed() const {
      return !is_paused_ && is_keyframe_needed_;
    }
    void set_is_keyframe_needed() { is_keyframe_needed_ = true; }
    bool is_paused() const { return is_paused_; }
    void set_is_paused(bool is_paused) { is_paused_ = is_paused; }
    std::optional<double> target_fps() const {
      return framerate_controller_ == nullptr
                 ? std::nullopt
                 : std::optional<double>(
                       framerate_controller_->GetMaxFramerate());
    }

    std::unique_ptr<EncoderContext> ReleaseEncoderContext() &&;
    void OnKeyframe(Timestamp timestamp);
    bool ShouldDropFrame(Timestamp timestamp);

   private:
    SimulcastEncoderAdapter* const parent_;
    std::unique_ptr<EncoderContext> encoder_context_;
    std::unique_ptr<FramerateController> framerate_controller_;
    const int stream_idx_;
    const uint16_t width_;
    const uint16_t height_;
    bool is_keyframe_needed_;
    bool is_paused_;
  };

  bool Initialized() const;

  // This method creates encoder. May reuse previously created encoders from
  // `cached_encoder_contexts_`. It's const because it's used from
  // const GetEncoderInfo().
  std::unique_ptr<EncoderContext> FetchOrCreateEncoderContext(
      bool is_lowest_quality_stream) const;

  webrtc::VideoCodec MakeStreamCodec(const webrtc::VideoCodec& codec,
                                     int stream_idx,
                                     uint32_t start_bitrate_kbps,
                                     bool is_lowest_quality_stream,
                                     bool is_highest_quality_stream);

  EncodedImageCallback::Result OnEncodedImage(
      size_t stream_idx,
      const EncodedImage& encoded_image,
      const CodecSpecificInfo* codec_specific_info);

  void OnDroppedFrame(size_t stream_idx);

  void OverrideFromFieldTrial(VideoEncoder::EncoderInfo* info) const;

  const Environment env_;
  std::atomic<int> inited_;
  VideoEncoderFactory* const primary_encoder_factory_;
  VideoEncoderFactory* const fallback_encoder_factory_;
  const SdpVideoFormat video_format_;
  VideoCodec codec_;
  int total_streams_count_;
  bool bypass_mode_;
  std::vector<StreamContext> stream_contexts_;
  EncodedImageCallback* encoded_complete_callback_;

  // Used for checking the single-threaded access of the encoder interface.
  RTC_NO_UNIQUE_ADDRESS SequenceChecker encoder_queue_;

  // Store previously created and released encoders , so they don't have to be
  // recreated. Remaining encoders are destroyed by the destructor.
  // Marked as `mutable` becuase we may need to temporarily create encoder in
  // GetEncoderInfo(), which is const.
  mutable std::list<std::unique_ptr<EncoderContext>> cached_encoder_contexts_;

  const bool boost_base_layer_quality_;
  const bool prefer_temporal_support_on_base_layer_;
  const bool per_layer_pli_;

  const SimulcastEncoderAdapterEncoderInfoSettings encoder_info_override_;
};

}  // namespace webrtc

#endif  // MEDIA_ENGINE_SIMULCAST_ENCODER_ADAPTER_H_
