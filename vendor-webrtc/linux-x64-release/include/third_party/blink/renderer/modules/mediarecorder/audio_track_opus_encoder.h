// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIARECORDER_AUDIO_TRACK_OPUS_ENCODER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIARECORDER_AUDIO_TRACK_OPUS_ENCODER_H_

#include <memory>

#include "base/containers/heap_array.h"
#include "base/memory/raw_ptr.h"
#include "media/base/audio_bus.h"
#include "media/base/audio_converter.h"
#include "media/base/audio_fifo.h"
#include "media/base/audio_parameters.h"
#include "third_party/blink/renderer/modules/mediarecorder/audio_track_encoder.h"
#include "third_party/opus/src/include/opus.h"

namespace blink {

// Class encapsulating Opus-related encoding details. It contains an
// AudioConverter to adapt incoming data to the format Opus likes to have.
class AudioTrackOpusEncoder : public AudioTrackEncoder,
                              public media::AudioConverter::InputCallback {
 public:
  AudioTrackOpusEncoder(OnEncodedAudioCB on_encoded_audio_cb,
                        OnEncodedAudioErrorCB on_encoded_audio_error_cb,
                        uint32_t bits_per_second,
                        bool vbr_enabled = true);
  ~AudioTrackOpusEncoder() override;

  AudioTrackOpusEncoder(const AudioTrackOpusEncoder&) = delete;
  AudioTrackOpusEncoder& operator=(const AudioTrackOpusEncoder&) = delete;

  void OnSetFormat(const media::AudioParameters& params) override;
  void EncodeAudio(std::unique_ptr<media::AudioBus> input_bus,
                   base::TimeTicks capture_time) override;

 private:
  bool is_initialized() const { return !!opus_encoder_; }

  void DestroyExistingOpusEncoder();

  // media::AudioConverted::InputCallback implementation.
  double ProvideInput(media::AudioBus* audio_bus,
                      uint32_t frames_delayed,
                      const media::AudioGlitchInfo& glitch_info) override;

  void NotifyError(media::EncoderStatus error);

  // Target bitrate for Opus. If 0, Opus provide automatic bitrate is used.
  const uint32_t bits_per_second_;

  // Opus operates in VBR or constrained VBR modes even when a fixed bitrate
  // is specified, unless 'hard' CBR is explicitly enabled by disabling VBR
  // mode with this flag.
  const bool vbr_enabled_;

  // Output parameters after audio conversion. This differs from the input
  // parameters only in sample_rate() and frames_per_buffer(): output should be
  // 48ksamples/s and 2880, respectively.
  media::AudioParameters converted_params_;

  // Sample rate adapter from the input audio to what OpusEncoder desires.
  std::unique_ptr<media::AudioConverter> converter_;

  // Buffer for holding the original input audio before it goes to the
  // converter.
  std::unique_ptr<media::AudioFifo> fifo_;

  // Buffer for passing AudioBus data from the converter to the encoder.
  std::unique_ptr<float[]> buffer_;

  raw_ptr<OpusEncoder, DanglingUntriaged> opus_encoder_;

  base::HeapArray<uint8_t> packet_buffer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIARECORDER_AUDIO_TRACK_OPUS_ENCODER_H_
