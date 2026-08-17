// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_AUDIO_DECODER_BROKER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_AUDIO_DECODER_BROKER_H_

#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "base/functional/callback.h"
#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/sequence_checker.h"
#include "media/base/audio_buffer.h"
#include "media/base/audio_decoder.h"
#include "media/base/decoder_status.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"

namespace base {
class SequencedTaskRunner;
}  // namespace base

namespace media {
class MediaLog;
}

namespace blink {

// Implementation detail of AudioDecoderBroker. Helps safely perform decoder
// tasks on the media thread.
class MediaAudioTaskWrapper;

// Client interface for MediaAudioTaskWrapper. Implementation detail of
// AudioDecoderBroker, but we need to define it here to implement it below.
class CrossThreadAudioDecoderClient {
 public:
  struct DecoderDetails {
    media::AudioDecoderType decoder_type;
    bool is_platform_decoder;
    bool needs_bitstream_conversion;
  };

  virtual void OnInitialize(media::DecoderStatus status,
                            std::optional<DecoderDetails> details) = 0;

  virtual void OnDecodeDone(int cb_id, media::DecoderStatus status) = 0;

  virtual void OnDecodeOutput(scoped_refptr<media::AudioBuffer> buffer) = 0;

  virtual void OnReset(int cb_id) = 0;
};

// This class brokers the connection between WebCodecs and an underlying
// media::AudioDecoder. It abstracts away details of construction and selection
// of the media/ decoder. It also handles thread-hopping as required by
// underlying APIS.
//
// A new underlying decoder is selected anytime Initialize() is called.
// TODO(chcunningham): Elide re-selection if the config has not significantly
// changed.
//
// All API calls and callbacks must occur on the main thread.
class MODULES_EXPORT AudioDecoderBroker : public media::AudioDecoder,
                                          public CrossThreadAudioDecoderClient {
 public:
  explicit AudioDecoderBroker(media::MediaLog* media_log,
                              ExecutionContext& execution_context);
  ~AudioDecoderBroker() override;

  // Disallow copy and assign.
  AudioDecoderBroker(const AudioDecoderBroker&) = delete;
  AudioDecoderBroker& operator=(const AudioDecoderBroker&) = delete;

  // AudioDecoder implementation.
  media::AudioDecoderType GetDecoderType() const override;
  bool IsPlatformDecoder() const override;
  void Initialize(const media::AudioDecoderConfig& config,
                  media::CdmContext* cdm_context,
                  InitCB init_cb,
                  const OutputCB& output_cb,
                  const media::WaitingCB& waiting_cb) override;
  void Decode(scoped_refptr<media::DecoderBuffer> buffer,
              DecodeCB decode_cb) override;
  void Reset(base::OnceClosure reset_cb) override;
  // Should always be false. See https://crbug.com/1119947
  bool NeedsBitstreamConversion() const override;

 private:
  // Creates a new (incremented) callback ID from |last_callback_id_| for
  // mapping in |pending_decode_cb_map_|.
  int CreateCallbackId();

  // MediaAudioTaskWrapper::CrossThreadAudioDecoderClient
  void OnInitialize(media::DecoderStatus status,
                    std::optional<DecoderDetails> details) override;
  void OnDecodeDone(int cb_id, media::DecoderStatus status) override;
  void OnDecodeOutput(scoped_refptr<media::AudioBuffer> buffer) override;
  void OnReset(int cb_id) override;

  // Holds the last key for callbacks in the map below. Incremented for each
  // usage via CreateCallbackId().
  uint32_t last_callback_id_ = 0;

  // Maps a callback ID to pending Decode(). See CrossThreadVideoDecoderClient.
  HashMap<int, DecodeCB> pending_decode_cb_map_;

  // Maps a callback ID to pending Reset(). See CrossThreadVideoDecoderClient.
  HashMap<int, base::OnceClosure> pending_reset_cb_map_;

  // Task runner for running codec work (traditionally the media thread).
  scoped_refptr<base::SequencedTaskRunner> media_task_runner_;

  // Owner of state and methods to be used on media_task_runner_;
  std::unique_ptr<MediaAudioTaskWrapper> media_tasks_;

  // Wrapper state for DecoderType(), IsPlatformDecoder() and others.
  std::optional<DecoderDetails> decoder_details_;

  // Pending InitCB saved from the last call to Initialize();
  InitCB init_cb_;

  // OutputCB saved from last call to Initialize().
  OutputCB output_cb_;

  SEQUENCE_CHECKER(sequence_checker_);

  base::WeakPtrFactory<AudioDecoderBroker> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_AUDIO_DECODER_BROKER_H_
