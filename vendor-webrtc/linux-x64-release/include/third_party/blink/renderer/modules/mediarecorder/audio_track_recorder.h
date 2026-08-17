// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIARECORDER_AUDIO_TRACK_RECORDER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIARECORDER_AUDIO_TRACK_RECORDER_H_

#include <memory>
#include <optional>

#include "base/memory/scoped_refptr.h"
#include "base/task/sequenced_task_runner.h"
#include "base/task/single_thread_task_runner.h"
#include "base/task/task_traits.h"
#include "base/task/thread_pool.h"
#include "base/threading/thread_checker.h"
#include "media/base/audio_encoder.h"
#include "media/base/decoder_buffer.h"
#include "third_party/blink/public/platform/modules/mediastream/web_media_stream_audio_sink.h"
#include "third_party/blink/renderer/modules/mediarecorder/track_recorder.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/heap/weak_cell.h"
#include "third_party/blink/renderer/platform/wtf/sequence_bound.h"

namespace media {
class AudioBus;
class AudioParameters;
}  // namespace media

namespace blink {

class AudioTrackEncoder;
class MediaStreamComponent;

// AudioTrackRecorder is a MediaStreamAudioSink that encodes the audio buses
// received from a Stream Audio Track. The class is constructed on a
// single thread (the main Render thread) but can receive MediaStreamAudioSink-
// related calls on a different "live audio" thread (referred to internally as
// the "capture thread"). It owns an internal thread to use for encoding, on
// which lives an AudioTrackEncoder with its own threading subtleties, see the
// implementation file.
class MODULES_EXPORT AudioTrackRecorder
    : public TrackRecorder<WebMediaStreamAudioSink> {
 public:
  enum class CodecId {
    // Do not change the order of codecs. Add new ones right before kLast.
    kOpus,
    kPcm,  // 32-bit little-endian float.
    kAac,
    kLast
  };

  enum class BitrateMode { kConstant, kVariable };

  // Callback interface for AudioTrackRecorders. The methods here need to all be
  // called on the main thread.
  class CallbackInterface : public GarbageCollectedMixin {
   public:
    // Called to indicate there is encoded audio data available.
    virtual void OnEncodedAudio(
        const media::AudioParameters& params,
        scoped_refptr<media::DecoderBuffer> encoded_data,
        std::optional<media::AudioEncoder::CodecDescription> codec_description,
        base::TimeTicks capture_time) = 0;

    // Called when an error occurs during encoding. Once it is called, there
    // is no more calling of `OnEncodedAudio()`.
    virtual void OnAudioEncodingError(media::EncoderStatus error_status) = 0;

    // Called when a track's ready state changes.
    virtual void OnSourceReadyStateChanged() = 0;
  };

  using OnEncodedAudioCB = base::RepeatingCallback<void(
      const media::AudioParameters& params,
      scoped_refptr<media::DecoderBuffer> encoded_data,
      std::optional<media::AudioEncoder::CodecDescription> codec_description,
      base::TimeTicks capture_time)>;

  using OnEncodedAudioErrorCB = media::EncoderStatus::Callback;

  static CodecId GetPreferredCodecId(MediaTrackContainerType container_type);

  AudioTrackRecorder(
      scoped_refptr<base::SingleThreadTaskRunner> main_thread_task_runner,
      CodecId codec,
      MediaStreamComponent* track,
      WeakCell<CallbackInterface>* callback_interface,
      uint32_t bits_per_second,
      BitrateMode bitrate_mode,
      scoped_refptr<base::SequencedTaskRunner> encoder_task_runner =
          base::ThreadPool::CreateSequencedTaskRunner(
              {base::TaskPriority::USER_VISIBLE}));

  AudioTrackRecorder(const AudioTrackRecorder&) = delete;
  AudioTrackRecorder& operator=(const AudioTrackRecorder&) = delete;

  ~AudioTrackRecorder() override;

  // Implement MediaStreamAudioSink.
  void OnSetFormat(const media::AudioParameters& params) override;
  void OnData(const media::AudioBus& audio_bus,
              base::TimeTicks capture_time) override;

  void Pause();
  void Resume();

  WeakCell<CallbackInterface>* callback_interface_for_testing() {
    return callback_interface_;
  }

 private:
  // Creates an audio encoder from |codec|. Returns nullptr if the codec is
  // invalid.
  static std::unique_ptr<AudioTrackEncoder> CreateAudioEncoder(
      CodecId codec,
      scoped_refptr<base::SequencedTaskRunner> encoder_task_runner,
      OnEncodedAudioCB on_encoded_audio_cb,
      OnEncodedAudioErrorCB on_encoded_audio_error_cb,
      uint32_t bits_per_second,
      BitrateMode bitrate_mode);

  void ConnectToTrack();
  void DisconnectFromTrack();

  void Prefinalize();

  // We need to hold on to the Blink track to remove ourselves on destruction.
  Persistent<MediaStreamComponent> track_;

  // Sequence used for the encoder, backed by the thread pool.
  const scoped_refptr<base::SequencedTaskRunner> encoder_task_runner_;

  // Thin wrapper around the chosen encoder.
  WTF::SequenceBound<std::unique_ptr<AudioTrackEncoder>> encoder_;

  // Number of frames per chunked buffer passed to the encoder.
  int frames_per_chunk_ = 0;

  // Integer used for checking OnSetFormat/OnData against races. We can use
  // neither ThreadChecker (due to SilentSinkSuspender), nor SequenceChecker.
  // See crbug.com/1377367.
#if DCHECK_IS_ON()
  std::atomic<int> race_checker_{0};
#endif
  Persistent<WeakCell<CallbackInterface>> callback_interface_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIARECORDER_AUDIO_TRACK_RECORDER_H_
