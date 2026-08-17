// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_DECODER_BROKER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_DECODER_BROKER_H_

#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "base/functional/callback.h"
#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/sequence_checker.h"
#include "media/base/decoder_status.h"
#include "media/base/video_decoder.h"
#include "media/base/video_frame.h"
#include "media/video/gpu_video_accelerator_factories.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/webcodecs/hardware_preference.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"

namespace base {
class SequencedTaskRunner;
}  // namespace base

namespace blink {

// Implementation detail of VideoDecoderBroker. Helps safely perform decoder
// tasks on the media thread.
class MediaVideoTaskWrapper;

// Client interface for MediaVideoTaskWrapper. Implementation detail of
// VideoDecoderBroker, but we need to define it here to implement it below.
//
// This avoids having pass-through callbacks from main->media task sequence,
// which is unsafe because the public callers of broker APIs may be broken if
// their callback is destructed on another thread.
//
// An int "cb_id" is used for those that are traditionally OnceCallbacks to
// lookup the correct public callback.
class CrossThreadVideoDecoderClient {
 public:
  struct DecoderDetails {
    media::VideoDecoderType decoder_id;
    bool is_platform_decoder;
    bool needs_bitstream_conversion;
    int max_decode_requests;
  };

  virtual void OnInitialize(media::DecoderStatus status,
                            std::optional<DecoderDetails> details) = 0;

  virtual void OnDecodeDone(int cb_id, media::DecoderStatus status) = 0;

  virtual void OnDecodeOutput(scoped_refptr<media::VideoFrame> frame,
                              bool can_read_without_stalling) = 0;

  virtual void OnReset(int cb_id) = 0;
};

// This class brokers the connection between WebCodecs and an underlying
// media::VideoDecoder. It abstracts away details of construction and selection
// of the media/ decoder. It also handles thread-hopping as required by
// underlying APIS.
//
// A new underlying decoder is selected anytime Initialize() is called.
// TODO(chcunningham): Elide re-selection if the config has not significantly
// changed.
//
// All API calls and callbacks must occur on the main thread.
class MODULES_EXPORT VideoDecoderBroker : public media::VideoDecoder,
                                          public CrossThreadVideoDecoderClient {
 public:
  // |gpu_factories| may be null when GPU accelerated decoding is not available.
  explicit VideoDecoderBroker(
      ExecutionContext& execution_context,
      media::GpuVideoAcceleratorFactories* gpu_factories,
      media::MediaLog* media_log);
  ~VideoDecoderBroker() override;

  // Disallow copy and assign.
  VideoDecoderBroker(const VideoDecoderBroker&) = delete;
  VideoDecoderBroker& operator=(const VideoDecoderBroker&) = delete;

  // VideoDecoder implementation.
  media::VideoDecoderType GetDecoderType() const override;
  bool IsPlatformDecoder() const override;
  void Initialize(const media::VideoDecoderConfig& config,
                  bool low_delay,
                  media::CdmContext* cdm_context,
                  InitCB init_cb,
                  const OutputCB& output_cb,
                  const media::WaitingCB& waiting_cb) override;
  void Decode(scoped_refptr<media::DecoderBuffer> buffer,
              DecodeCB decode_cb) override;
  void Reset(base::OnceClosure reset_cb) override;
  bool NeedsBitstreamConversion() const override;
  bool CanReadWithoutStalling() const override;
  int GetMaxDecodeRequests() const override;

  void SetHardwarePreference(HardwarePreference hardware_preference);

 private:
  // Creates a new (incremented) callback ID from |last_callback_id_| for
  // mapping in |pending_decode_cb_map_|.
  int CreateCallbackId();

  // MediaVideoTaskWrapper::CrossThreadVideoDecoderClient
  void OnInitialize(media::DecoderStatus status,
                    std::optional<DecoderDetails> details) override;
  void OnDecodeDone(int cb_id, media::DecoderStatus status) override;
  void OnDecodeOutput(scoped_refptr<media::VideoFrame> frame,
                      bool can_read_without_stalling) override;
  void OnReset(int cb_id) override;

  // When media::GpuVideoAcceleratorFactories is provided, its API requires
  // that we use its TaskRunner (the media thread). When not provided, this task
  // runner will still be used to reduce contention on the main thread.
  // TODO(chcunningham): Try to eliminate the Post(). Most of the
  // underlying::VideoDecoders already offload their work, so this just adds
  // overhead.
  scoped_refptr<base::SequencedTaskRunner> media_task_runner_;

  // Owner of state and methods to be used on media_task_runner_;
  std::unique_ptr<MediaVideoTaskWrapper> media_tasks_;

  // Wrapper state for GetDecoderType(), IsPlatformDecoder() and others.
  std::optional<DecoderDetails> decoder_details_;

  // Set to match the underlying decoder's answer at every OnDecodeOutput().
  bool can_read_without_stalling_ = true;

  // Holds the last key for callbacks in the map below. Incremented for each
  // usage via CreateCallbackId().
  uint32_t last_callback_id_ = 0;

  // Maps a callback ID to pending Decode(). See CrossThreadVideoDecoderClient.
  HashMap<int, DecodeCB> pending_decode_cb_map_;

  // Maps a callback ID to pending Reset(). See CrossThreadVideoDecoderClient.
  HashMap<int, base::OnceClosure> pending_reset_cb_map_;

  // Pending InitCB saved from the last call to Initialize();
  InitCB init_cb_;

  // OutputCB saved from last call to Initialize().
  OutputCB output_cb_;

  SEQUENCE_CHECKER(sequence_checker_);

  base::WeakPtrFactory<VideoDecoderBroker> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_DECODER_BROKER_H_
