// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_DECODER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_DECODER_H_

#include <stdint.h>
#include <memory>

#include "base/containers/flat_map.h"
#include "base/time/time.h"
#include "media/base/media_types.h"
#include "media/base/status.h"
#include "media/base/video_decoder.h"
#include "media/base/video_decoder_config.h"
#include "media/media_buildflags.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_video_frame_output_callback.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_webcodecs_error_callback.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/webcodecs/decoder_template.h"
#include "third_party/blink/renderer/modules/webcodecs/video_decoder_helper.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace libgav1 {
class BufferPool;
}

namespace media {

class VideoFrame;
class DecoderBuffer;
class MediaLog;

}  // namespace media

namespace blink {

class EncodedVideoChunk;
class ExceptionState;
class VideoDecoderConfig;
class VideoDecoderInit;
class VideoDecoderSupport;
class VideoFrame;
class V8VideoFrameOutputCallback;

class MODULES_EXPORT VideoDecoderTraits {
 public:
  using InitType = VideoDecoderInit;
  using OutputType = VideoFrame;
  using MediaOutputType = media::VideoFrame;
  using MediaDecoderType = media::VideoDecoder;
  using OutputCallbackType = V8VideoFrameOutputCallback;
  using ConfigType = VideoDecoderConfig;
  using MediaConfigType = media::VideoDecoderConfig;
  using InputType = EncodedVideoChunk;

  static constexpr bool kNeedsGpuFactories = true;

  static std::unique_ptr<MediaDecoderType> CreateDecoder(
      ExecutionContext& execution_context,
      media::GpuVideoAcceleratorFactories* gpu_factories,
      media::MediaLog* media_log);
  static void InitializeDecoder(MediaDecoderType& decoder,
                                bool low_delay,
                                const MediaConfigType& media_config,
                                MediaDecoderType::InitCB init_cb,
                                MediaDecoderType::OutputCB output_cb);
  static int GetMaxDecodeRequests(const MediaDecoderType& decoder);
  static void UpdateDecoderLog(const MediaDecoderType& decoder,
                               const MediaConfigType& media_config,
                               media::MediaLog* media_log);
  static const char* GetName();
};

class MODULES_EXPORT VideoDecoder : public DecoderTemplate<VideoDecoderTraits> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static VideoDecoder* Create(ScriptState*,
                              const VideoDecoderInit*,
                              ExceptionState&);

  static ScriptPromise<VideoDecoderSupport>
  isConfigSupported(ScriptState*, const VideoDecoderConfig*, ExceptionState&);

  static HardwarePreference GetHardwareAccelerationPreference(
      const ConfigType& config);

  // Returns parsed VideoType if the configuration is valid.
  static std::optional<media::VideoType> IsValidVideoDecoderConfig(
      const VideoDecoderConfig& config,
      String* js_error_message);

  // For use by MediaSource
  static std::optional<media::VideoDecoderConfig> MakeMediaVideoDecoderConfig(
      const ConfigType& config,
      String* js_error_message,
      bool* needs_converter_out = nullptr);

  VideoDecoder(ScriptState*, const VideoDecoderInit*, ExceptionState&);
  ~VideoDecoder() override;

  // EventTarget interface
  const AtomicString& InterfaceName() const override;

 protected:
  bool IsValidConfig(const ConfigType& config,
                     String* js_error_message) override;
  std::optional<media::VideoDecoderConfig> MakeMediaConfig(
      const ConfigType& config,
      String* js_error_message) override;
  media::DecoderStatus::Or<scoped_refptr<media::DecoderBuffer>> MakeInput(
      const InputType& input,
      bool verify_key_frame) override;
  media::DecoderStatus::Or<OutputType*> MakeOutput(
      scoped_refptr<MediaOutputType>,
      ExecutionContext*) override;
  void OnActiveConfigChanged(const MediaConfigType& config) override;

 private:
  struct DecoderSpecificData {
    // Bitstream converter to annex B for AVC/HEVC.
    std::unique_ptr<VideoDecoderHelper> decoder_helper;

    // Buffer pool for use with libgav1::ObuParser.
    std::unique_ptr<libgav1::BufferPool> av1_buffer_pool;
  };

  // DecoderTemplate implementation.
  HardwarePreference GetHardwarePreference(const ConfigType& config) override;
  bool GetLowDelayPreference(const ConfigType& config) override;
  void SetHardwarePreference(HardwarePreference preference) override;
  // For use by ::MakeMediaConfig
  static std::optional<media::VideoDecoderConfig>
  MakeMediaVideoDecoderConfigInternal(
      const ConfigType& config,
      DecoderSpecificData& decoder_specific_data,
      String* js_error_message,
      bool* needs_converter_out = nullptr);

  DecoderSpecificData decoder_specific_data_;

  // Note: This may not be the active codec, it is just intended to be the last
  // codec that was passed to a configure() call.
  media::VideoCodec pending_codec_ = media::VideoCodec::kUnknown;

  media::VideoTransformation active_transform_ = media::kNoTransformation;

  // Per-chunk metadata to be applied to outputs, linked by timestamp.
  struct ChunkMetadata {
    base::TimeDelta duration;
  };
  base::flat_map<base::TimeDelta, ChunkMetadata> chunk_metadata_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_DECODER_H_
