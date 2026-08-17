// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_MEDIA_STREAM_AUDIO_SOURCE_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_MEDIA_STREAM_AUDIO_SOURCE_HANDLER_H_

#include <memory>

#include "base/memory/scoped_refptr.h"
#include "base/synchronization/lock.h"
#include "third_party/blink/renderer/modules/webaudio/audio_node.h"
#include "third_party/blink/renderer/platform/audio/audio_source_provider.h"

namespace blink {

class AudioNode;

class MediaStreamAudioSourceHandler final : public AudioHandler {
 public:
  static scoped_refptr<MediaStreamAudioSourceHandler> Create(
      AudioNode&,
      std::unique_ptr<AudioSourceProvider>);
  ~MediaStreamAudioSourceHandler() override;

  // AudioHandler
  void Process(uint32_t frames_to_process) override;

  void SetFormat(uint32_t number_of_channels, float sample_rate);

  double TailTime() const override { return 0; }
  double LatencyTime() const override { return 0; }
  bool RequiresTailProcessing() const final { return false; }

 private:
  MediaStreamAudioSourceHandler(AudioNode&,
                                std::unique_ptr<AudioSourceProvider>);

  // AudioHandler: MediaStreamAudioSourceNode never propagates silence.
  bool PropagatesSilence() const override { return false; }

  // https://chromium.googlesource.com/chromium/src/+/refs/heads/main/docs/media/capture/README.md#logs
  void SendLogMessage(const char* const function_name, const String& message);

  std::unique_ptr<AudioSourceProvider> audio_source_provider_;

  base::Lock process_lock_;
  unsigned source_number_of_channels_ GUARDED_BY(process_lock_) = 0;

  // Used to trigger one single textlog indicating that processing started as
  // intended. Set to true once in the first call to the Process callback.
  bool is_processing_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_MEDIA_STREAM_AUDIO_SOURCE_HANDLER_H_
