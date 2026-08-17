// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_ENCODER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_ENCODER_H_

#include <memory>
#include <optional>

#include "base/containers/flat_map.h"
#include "base/time/time.h"
#include "media/base/video_codecs.h"
#include "media/base/video_color_space.h"
#include "media/base/video_encoder.h"
#include "media/base/video_frame_pool.h"
#include "media/video/video_encoder_info.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_encoded_video_chunk_output_callback.h"
#include "third_party/blink/renderer/modules/webcodecs/encoder_base.h"
#include "third_party/blink/renderer/modules/webcodecs/hardware_preference.h"
#include "third_party/blink/renderer/modules/webcodecs/video_frame.h"
#include "ui/gfx/color_space.h"

namespace media {
class GpuVideoAcceleratorFactories;
class VideoEncoderMetricsProvider;
class VideoEncoder;
struct VideoEncoderOutput;
}  // namespace media

namespace blink {

class VideoEncoderBuffer;
class VideoEncoderConfig;
class VideoEncoderInit;
class VideoEncoderEncodeOptions;
class VideoEncoderSupport;
class WebGraphicsContext3DVideoFramePool;
class BackgroundReadback;

class MODULES_EXPORT VideoEncoderTraits {
 public:
  struct ParsedConfig final : public GarbageCollected<ParsedConfig> {
    media::VideoCodec codec;
    media::VideoCodecProfile profile;
    uint8_t level;

    HardwarePreference hw_pref;

    media::VideoEncoder::Options options;
    String codec_string;
    std::optional<gfx::Size> display_size;

    std::optional<String> not_supported_error_message;

    String ToString();
    void Trace(Visitor*) const {}
  };

  using Init = VideoEncoderInit;
  using Config = VideoEncoderConfig;
  using InternalConfig = ParsedConfig;
  using Input = VideoFrame;
  using EncodeOptions = VideoEncoderEncodeOptions;
  using OutputChunk = EncodedVideoChunk;
  using OutputCallback = V8EncodedVideoChunkOutputCallback;
  using MediaEncoder = media::VideoEncoder;

  // Can't be a virtual method, because it's used from base ctor.
  static const char* GetName();
};

class MODULES_EXPORT VideoEncoder : public EncoderBase<VideoEncoderTraits> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static VideoEncoder* Create(ScriptState*,
                              const VideoEncoderInit*,
                              ExceptionState&);
  VideoEncoder(ScriptState*, const VideoEncoderInit*, ExceptionState&);
  ~VideoEncoder() override;

  static ScriptPromise<VideoEncoderSupport>
  isConfigSupported(ScriptState*, const VideoEncoderConfig*, ExceptionState&);

  HeapVector<Member<VideoEncoderBuffer>> getAllFrameBuffers(ScriptState*,
                                                            ExceptionState&);

  // EventTarget interface
  const AtomicString& InterfaceName() const override;

  // ScriptWrappable override.
  bool HasPendingActivity() const override;

  // GarbageCollected override.
  void Trace(Visitor*) const override;

  // If `is_error_message_from_software_codec` is true, `error_message` will be
  // updated to include `status.message()` if non-empty.
  void ReportError(const char* error_message,
                   const media::EncoderStatus& status,
                   bool is_error_message_from_software_codec);

  std::unique_ptr<media::VideoEncoderMetricsProvider> encoder_metrics_provider_
      GUARDED_BY_CONTEXT(sequence_checker_);

 protected:
  using Base = EncoderBase<VideoEncoderTraits>;
  using ParsedConfig = VideoEncoderTraits::ParsedConfig;

  void OnMediaEncoderInfoChanged(const media::VideoEncoderInfo& encoder_info);
  void CallOutputCallback(
      ParsedConfig* active_config,
      uint32_t reset_count,
      media::VideoEncoderOutput output,
      std::optional<media::VideoEncoder::CodecDescription> codec_desc);
  bool ReadyToProcessNextRequest() override;
  void ProcessEncode(Request* request) override;
  void ProcessConfigure(Request* request) override;
  void ProcessReconfigure(Request* request) override;
  void ResetInternal(DOMException* ex) override;
  void OnNewEncode(VideoFrame* input, ExceptionState& exception_state) override;

  void OnEncodeDone(Request* request, media::EncoderStatus status);
  media::VideoEncoder::EncodeOptions CreateEncodeOptions(Request* request);
  // This will execute shortly after the async readback completes.
  void OnReadbackDone(Request* request,
                      scoped_refptr<media::VideoFrame> txt_frame,
                      media::VideoEncoder::EncoderStatusCB done_callback,
                      scoped_refptr<media::VideoFrame> result_frame);
  static media::EncoderStatus::Or<std::unique_ptr<media::VideoEncoder>>
  CreateSoftwareVideoEncoder(VideoEncoder* self,
                             bool fallback,
                             media::VideoCodec codec);

  ParsedConfig* OnNewConfigure(const VideoEncoderConfig*,
                               ExceptionState&) override;
  bool VerifyCodecSupport(ParsedConfig*, String* js_error_message) override;

  // Virtual for UTs.
  // Returns the VideoEncoder.
  virtual media::EncoderStatus::Or<std::unique_ptr<media::VideoEncoder>>
  CreateMediaVideoEncoder(const ParsedConfig& config,
                          media::GpuVideoAcceleratorFactories* gpu_factories,
                          bool& is_platform_encoder);
  virtual std::unique_ptr<media::VideoEncoderMetricsProvider>
  CreateVideoEncoderMetricsProvider() const;

  void ContinueConfigureWithGpuFactories(
      Request* request,
      media::GpuVideoAcceleratorFactories* gpu_factories);
  media::EncoderStatus::Or<std::unique_ptr<media::VideoEncoder>>
  CreateAcceleratedVideoEncoder(
      media::VideoCodecProfile profile,
      const media::VideoEncoder::Options& options,
      media::GpuVideoAcceleratorFactories* gpu_factories,
      HardwarePreference hw_pref);
  bool CanReconfigure(ParsedConfig& original_config,
                      ParsedConfig& new_config) override;

  using ReadbackDoneCallback =
      base::OnceCallback<void(scoped_refptr<media::VideoFrame>)>;
  bool StartReadback(scoped_refptr<media::VideoFrame> frame,
                     ReadbackDoneCallback result_cb);

  std::unique_ptr<WebGraphicsContext3DVideoFramePool> accelerated_frame_pool_;
  Member<BackgroundReadback> background_readback_;

  // True if an error occurs during frame pool usage.
  bool disable_accelerated_frame_pool_ = false;

  // The number of encoding requests currently handled by |media_encoder_|
  // Should not exceed |max_active_encodes_|.
  int active_encodes_ = 0;

  // The current upper limit on |active_encodes_|.
  int max_active_encodes_;

  // True if a running video encoder is hardware accelerated.
  bool is_platform_encoder_ = false;

  // Per-frame metadata to be applied to outputs, linked by timestamp.
  struct FrameMetadata {
    base::TimeDelta duration;
    media::VideoTransformation transformation;
  };
  base::flat_map<base::TimeDelta, FrameMetadata> frame_metadata_;

  // Buffers returned by getAllFrameBuffers()
  HeapVector<Member<VideoEncoderBuffer>> frame_reference_buffers_;

  // The color space corresponding to the last emitted output. Used to update
  // emitted VideoDecoderConfig when necessary.
  gfx::ColorSpace last_output_color_space_;

  // The transformation corresponding to the last input received by
  // ProcessEncode(). Used to request a key frame.
  std::optional<media::VideoTransformation> first_input_transformation_;

  // Latest VideoEncoderInfo reported by encoder
  media::VideoEncoderInfo encoder_info_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_ENCODER_H_
