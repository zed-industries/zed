// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIACAPTUREFROMELEMENT_HTML_AUDIO_ELEMENT_CAPTURER_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIACAPTUREFROMELEMENT_HTML_AUDIO_ELEMENT_CAPTURER_SOURCE_H_

#include "base/functional/callback.h"
#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/threading/thread_checker.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_audio_source.h"

namespace blink {
class WebMediaPlayer;
class WebAudioSourceProviderImpl;
}  // namespace blink

namespace media {
class AudioBus;
}  // namespace media

namespace blink {

// This class is a blink::MediaStreamAudioSource that registers to the
// constructor- passed weak WebAudioSourceProviderImpl to receive a copy of the
// audio data intended for rendering. This copied data is received on
// OnAudioBus() and sent to all the registered Tracks.
class MODULES_EXPORT HtmlAudioElementCapturerSource final
    : public blink::MediaStreamAudioSource {
 public:
  static HtmlAudioElementCapturerSource* CreateFromWebMediaPlayerImpl(
      blink::WebMediaPlayer* player,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);

  HtmlAudioElementCapturerSource(
      scoped_refptr<blink::WebAudioSourceProviderImpl> audio_source,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);

  HtmlAudioElementCapturerSource(const HtmlAudioElementCapturerSource&) =
      delete;
  HtmlAudioElementCapturerSource& operator=(
      const HtmlAudioElementCapturerSource&) = delete;

  ~HtmlAudioElementCapturerSource() override;

 private:
  // blink::MediaStreamAudioSource implementation.
  bool EnsureSourceIsStarted() final;
  void EnsureSourceIsStopped() final;
  void SetAudioCallback();

  // To act as an WebAudioSourceProviderImpl::CopyAudioCB.
  void OnAudioBus(std::unique_ptr<media::AudioBus> audio_bus,
                  uint32_t frames_delayed,
                  int sample_rate);

  scoped_refptr<blink::WebAudioSourceProviderImpl> audio_source_;

  bool is_started_;
  int last_sample_rate_;
  int last_num_channels_;
  int last_bus_frames_;

  THREAD_CHECKER(thread_checker_);

  base::WeakPtrFactory<HtmlAudioElementCapturerSource> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIACAPTUREFROMELEMENT_HTML_AUDIO_ELEMENT_CAPTURER_SOURCE_H_
