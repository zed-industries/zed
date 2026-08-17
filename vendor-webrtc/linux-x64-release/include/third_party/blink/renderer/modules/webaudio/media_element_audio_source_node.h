/*
 * Copyright (C) 2011, Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_MEDIA_ELEMENT_AUDIO_SOURCE_NODE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_MEDIA_ELEMENT_AUDIO_SOURCE_NODE_H_

#include "base/memory/scoped_refptr.h"
#include "base/notreached.h"
#include "base/task/single_thread_task_runner.h"
#include "base/thread_annotations.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/modules/webaudio/audio_node.h"
#include "third_party/blink/renderer/modules/webaudio/media_element_audio_source_handler.h"
#include "third_party/blink/renderer/platform/audio/audio_source_provider_client.h"
#include "third_party/blink/renderer/platform/audio/media_multi_channel_resampler.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"

namespace blink {

class AudioContext;
class HTMLMediaElement;
class MediaElementAudioSourceOptions;

class MediaElementAudioSourceNode final
    : public AudioNode,
      public AudioSourceProviderClient,
      public ActiveScriptWrappable<MediaElementAudioSourceNode> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static MediaElementAudioSourceNode* Create(AudioContext&,
                                             HTMLMediaElement&,
                                             ExceptionState&);
  static MediaElementAudioSourceNode*
  Create(AudioContext*, const MediaElementAudioSourceOptions*, ExceptionState&);

  MediaElementAudioSourceNode(AudioContext&, HTMLMediaElement&);

  MediaElementAudioSourceHandler& GetMediaElementAudioSourceHandler() const;

  HTMLMediaElement* mediaElement() const;

  void ConnectToDestinationReady() override;

  // AudioSourceProviderClient functions:
  void SetFormat(uint32_t number_of_channels, float sample_rate) override;
  void lock() override EXCLUSIVE_LOCK_FUNCTION(
      GetMediaElementAudioSourceHandler().GetProcessLock());
  void unlock() override
      UNLOCK_FUNCTION(GetMediaElementAudioSourceHandler().GetProcessLock());
  void OnCurrentSrcChanged(const KURL& current_src) override {}

  // InspectorHelperMixin
  void ReportDidCreate() final;
  void ReportWillBeDestroyed() final;

  // GC
  bool HasPendingActivity() const final;
  void Trace(Visitor*) const override;

 private:
  Member<HTMLMediaElement> media_element_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_MEDIA_ELEMENT_AUDIO_SOURCE_NODE_H_
