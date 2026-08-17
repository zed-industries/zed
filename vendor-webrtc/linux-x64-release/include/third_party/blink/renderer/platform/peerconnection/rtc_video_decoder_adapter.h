// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_VIDEO_DECODER_ADAPTER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_VIDEO_DECODER_ADAPTER_H_

#include <atomic>
#include <memory>
#include <vector>

#include "base/feature_list.h"
#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/sequence_checker.h"
#include "build/build_config.h"
#include "media/base/decoder.h"
#include "media/base/video_decoder_config.h"
#include "third_party/blink/renderer/platform/peerconnection/resolution_monitor.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/webrtc/api/video_codecs/sdp_video_format.h"
#include "third_party/webrtc/api/video_codecs/video_decoder.h"
#include "third_party/webrtc/modules/video_coding/include/video_codec_interface.h"
#include "ui/gfx/geometry/size.h"

namespace base {
class SequencedTaskRunner;
}  // namespace base

namespace media {
class DecoderBuffer;
class GpuVideoAcceleratorFactories;
}  // namespace media

namespace blink {

// This class decodes video for WebRTC using a media::VideoDecoder. In
// particular, MojoVideoDecoder are used to provide access to hardware decoding
// in the GPU process.
//
// This is created and destroyed on a WebRTC DecodingThread and decoding happens
// also on the thread.
//
// The webrtc::VideoDecoder functions delegates |impl_| functions through
// posting the task to |media_task_runner_|.
class PLATFORM_EXPORT RTCVideoDecoderAdapter : public webrtc::VideoDecoder {
 public:
  // Minimum resolution that we'll consider "not low resolution" for the purpose
  // of falling back to software.
#if BUILDFLAG(IS_CHROMEOS)
  // Effectively opt-out CrOS, since it may cause tests to fail (b/179724180).
  static constexpr gfx::Size kMinResolution{2, 2};
#else
  static constexpr gfx::Size kMinResolution{320, 240};
#endif

  // Maximum number of decoder instances we'll allow before fallback to software
  // if the resolution is too low.  We'll allow more than this for high
  // resolution streams, but they'll fall back if they adapt below the limit.
  static constexpr int32_t kMaxDecoderInstances = 8;

  // Creates and initializes an RTCVideoDecoderAdapter. Returns nullptr if
  // |format| cannot be supported.
  // Called on the worker thread.
  static std::unique_ptr<RTCVideoDecoderAdapter> Create(
      media::GpuVideoAcceleratorFactories* gpu_factories,
      const webrtc::SdpVideoFormat& format,
      std::unique_ptr<ResolutionMonitor> resolution_monitor = nullptr);

  RTCVideoDecoderAdapter(const RTCVideoDecoderAdapter&) = delete;
  RTCVideoDecoderAdapter& operator=(const RTCVideoDecoderAdapter&) = delete;

  // Called on |media_task_runner_|.
  ~RTCVideoDecoderAdapter() override;

  // webrtc::VideoDecoder implementation.
  // Called on the DecodingThread.
  bool Configure(const Settings& _settings) override;
  int32_t RegisterDecodeCompleteCallback(
      webrtc::DecodedImageCallback* callback) override;
  int32_t Decode(const webrtc::EncodedImage& input_image,
                 bool missing_frames,
                 int64_t render_time_ms) override;
  // Called on the worker thread and on the DecodingThread.
  int32_t Release() override;
  // Called on the worker thread and on the DecodingThread.
  DecoderInfo GetDecoderInfo() const override { return decoder_info_; }

  // Gets / adjusts the current decoder count.
  // They are must be executed on media thread.
  static int GetCurrentDecoderCountForTesting();
  static void IncrementCurrentDecoderCountForTesting();
  static void DecrementCurrentDecoderCountForTesting();

  static std::atomic<int> g_num_decoders_;

 private:
  class Impl;

  enum class DecodeResult {
    kOk,
    kErrorRequestKeyFrame,
  };
  enum class Status : uint8_t {
    kOk = 0,            // Status other than kNeedKeyFrame and kError.
    kNeedKeyFrame = 1,  // A decoder needs a key frame.
    kError = 2,         // A decoder will never be able to decode frames.
  };

  // Called on the worker thread.
  RTCVideoDecoderAdapter(media::GpuVideoAcceleratorFactories* gpu_factories,
                         const media::VideoDecoderConfig& config,
                         std::unique_ptr<ResolutionMonitor> resolution_monitor);

  bool InitializeSync(const media::VideoDecoderConfig& config);
  std::optional<DecodeResult> DecodeInternal(
      const webrtc::EncodedImage& input_image,
      bool missing_frames,
      int64_t render_time_ms);
  bool ShouldReinitializeForSettingColorSpace(
      const webrtc::EncodedImage& input_image) const;
  bool ReinitializeSync(const media::VideoDecoderConfig& config);
  void ChangeStatus(Status new_status);
  bool CheckResolutionAndNumInstances(const media::DecoderBuffer& buffer);
  // Construction parameters.
  const scoped_refptr<base::SequencedTaskRunner> media_task_runner_;
  std::unique_ptr<Impl> impl_ GUARDED_BY_CONTEXT(decoding_sequence_checker_);

  // Construction parameters.
  media::VideoDecoderConfig config_;

  const std::unique_ptr<ResolutionMonitor> resolution_monitor_
      GUARDED_BY_CONTEXT(decoding_sequence_checker_);

  // Decoding thread members.
  // Has anything been sent to Decode() yet?
  Status status_ GUARDED_BY_CONTEXT(decoding_sequence_checker_){
      Status::kNeedKeyFrame};

  // DecoderInfo is constant after InitializeSync() is complete.
  DecoderInfo decoder_info_;

  bool have_started_decoding_ GUARDED_BY_CONTEXT(decoding_sequence_checker_){
      false};

  media::VideoDecoderType decoder_type_ GUARDED_BY_CONTEXT(
      decoding_sequence_checker_){media::VideoDecoderType::kUnknown};

  // Thread management.
  SEQUENCE_CHECKER(decoding_sequence_checker_);

  // This weak pointer is bound to |media_task_runner_|.
  base::WeakPtr<Impl> weak_impl_;

  // These are bound to |decoding_sequence_checker_|.
  base::WeakPtr<RTCVideoDecoderAdapter> weak_this_;
  base::WeakPtrFactory<RTCVideoDecoderAdapter> weak_this_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_VIDEO_DECODER_ADAPTER_H_
