/*
 * Copyright (C) 2012, Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1.  Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH
 * DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_MEDIA_STREAM_AUDIO_SOURCE_NODE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_MEDIA_STREAM_AUDIO_SOURCE_NODE_H_

#include <memory>

#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/modules/mediastream/media_stream.h"
#include "third_party/blink/renderer/modules/webaudio/audio_node.h"
#include "third_party/blink/renderer/platform/audio/audio_source_provider_client.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class AudioContext;
class MediaStreamAudioSourceOptions;
class MediaStreamAudioSourceHandler;

class MediaStreamAudioSourceNode final
    : public AudioNode,
      public AudioSourceProviderClient,
      public ActiveScriptWrappable<MediaStreamAudioSourceNode> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static MediaStreamAudioSourceNode* Create(AudioContext&,
                                            MediaStream&,
                                            ExceptionState&);
  static MediaStreamAudioSourceNode* Create(
      AudioContext*, const MediaStreamAudioSourceOptions*, ExceptionState&);

  MediaStreamAudioSourceNode(AudioContext&,
                             MediaStream&,
                             MediaStreamTrack*,
                             std::unique_ptr<AudioSourceProvider>);

  // V8 binding
  MediaStream* getMediaStream() const { return media_stream_.Get(); }

  // AudioSourceProviderClient
  void SetFormat(uint32_t number_of_channels, float sample_rate) override;
  void lock() override {}
  void unlock() override {}
  void OnCurrentSrcChanged(const KURL& current_src) override {}

  // InspectorHelperMixin
  void ReportDidCreate() final;
  void ReportWillBeDestroyed() final;

  // GC
  bool HasPendingActivity() const final;
  void Trace(Visitor*) const override;

 private:
  MediaStreamAudioSourceHandler& GetMediaStreamAudioSourceHandler() const;

  // https://chromium.googlesource.com/chromium/src/+/refs/heads/main/docs/media/capture/README.md#logs
  void SendLogMessage(const char* const function_name, const String& message);

  Member<MediaStreamTrack> audio_track_;
  Member<MediaStream> media_stream_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_MEDIA_STREAM_AUDIO_SOURCE_NODE_H_
