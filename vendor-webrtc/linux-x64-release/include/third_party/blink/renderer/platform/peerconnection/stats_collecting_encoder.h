// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_STATS_COLLECTING_ENCODER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_STATS_COLLECTING_ENCODER_H_

#include <algorithm>
#include <vector>

#include "base/functional/callback.h"
#include "base/memory/raw_ptr.h"
#include "base/thread_annotations.h"
#include "base/time/time.h"
#include "media/base/video_codecs.h"
#include "third_party/blink/renderer/platform/peerconnection/linear_histogram.h"
#include "third_party/blink/renderer/platform/peerconnection/stats_collector.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"
#include "third_party/webrtc/api/video_codecs/sdp_video_format.h"
#include "third_party/webrtc/api/video_codecs/video_encoder.h"

namespace blink {
// This class acts as a wrapper around the WebRTC video encoder that is used in
// Chrome.
//
// Its purpose is to collect encode performance statistics for the current video
// stream. The performance statistics is pushed to a local database through a
// callback and is later used to determine if a specific video configuration is
// considered to be smooth or not, see
// https://w3c.github.io/media-capabilities/. Smooth will be an optimistic
// prediction and data collection therefore only takes place if there's a single
// encoder active.
//
// It's assumed that Configure(), Encode(), and RegisterEncodeCompleteCallback()
// are called on the encode sequence. Encoded() may be called on either the
// encode sequecene or the gpu sequence depending on if the underlying encoder
// is a HW or SW encoder. However, the calls to Encoded() on these sequences are
// mutual exclusive. Release() may be called on any sequence as long as the
// encoding sequence has stopped.
class PLATFORM_EXPORT StatsCollectingEncoder
    : private StatsCollector,
      public webrtc::VideoEncoder,
      private webrtc::EncodedImageCallback {
 public:
  // Creates a StatsCollectingEncoder object for the specified `format`.
  // `encoder` specifies the underlying encoder that is wrapped and all calls to
  // the methods of the webrtc::VideoEncoder interface are forwarded to
  // `encoder`. The provided `stats_callback` will be called periodically to
  // push the performance data that has been collected. The lifetime of
  // `stats_callback` must outlive the lifetime of the StatsCollectingEncoder.
  explicit StatsCollectingEncoder(const webrtc::SdpVideoFormat& format,
                                  std::unique_ptr<webrtc::VideoEncoder> encoder,
                                  StoreProcessingStatsCB stats_callback);

  ~StatsCollectingEncoder() override;

  // Implementation of webrtc::VideoEncoder.
  void SetFecControllerOverride(
      webrtc::FecControllerOverride* fec_controller_override) override;
  int InitEncode(const webrtc::VideoCodec* codec_settings,
                 const webrtc::VideoEncoder::Settings& settings) override;
  int32_t RegisterEncodeCompleteCallback(
      webrtc::EncodedImageCallback* callback) override;
  int32_t Release() override;
  int32_t Encode(
      const webrtc::VideoFrame& frame,
      const std::vector<webrtc::VideoFrameType>* frame_types) override;
  void SetRates(const RateControlParameters& parameters) override;
  void OnPacketLossRateUpdate(float packet_loss_rate) override;
  void OnRttUpdate(int64_t rtt_ms) override;
  void OnLossNotification(const LossNotification& loss_notification) override;
  EncoderInfo GetEncoderInfo() const override;

 private:
  struct EncodeStartInfo {
    uint32_t rtp_timestamp;
    base::TimeTicks encode_start;
  };

  // Implementation of webrtc::EncodedImageCallback.
  Result OnEncodedImage(
      const webrtc::EncodedImage& encoded_image,
      const webrtc::CodecSpecificInfo* codec_specific_info) override;
  void OnDroppedFrame(DropReason reason) override;

  // Lock for variables that are accessed in both Encode() and Encoded(). This
  // is needed because Encode() and Encoded() may be called simultaneously on
  // different sequences if a HW encoder is used.
  base::Lock lock_;

  const std::unique_ptr<webrtc::VideoEncoder> encoder_;
  raw_ptr<webrtc::EncodedImageCallback> encoded_callback_{nullptr};
  // We only care about the highest layer...
  // - In simulcast, the stream index refers to the simulcast index.
  // - In SVC, the stream index refers to the spatial index.
  // The mixed simulcast-SVC case is not considered because it is not supported
  // by WebRTC.
  size_t highest_observed_stream_index_ = 0;

  bool first_frame_encoded_{false};
  base::TimeTicks last_check_for_simultaneous_encoders_;

  WTF::Deque<EncodeStartInfo> encode_start_info_ GUARDED_BY(lock_);

  SEQUENCE_CHECKER(encoding_sequence_checker_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_STATS_COLLECTING_ENCODER_H_
