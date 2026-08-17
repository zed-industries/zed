/*
 * Copyright (C) 2011 Ericsson AB. All rights reserved.
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer
 *    in the documentation and/or other materials provided with the
 *    distribution.
 * 3. Neither the name of Ericsson nor the names of its contributors
 *    may be used to endorse or promote products derived from this
 *    software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIASTREAM_MEDIA_STREAM_COMPONENT_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIASTREAM_MEDIA_STREAM_COMPONENT_IMPL_H_

#include <memory>
#include <vector>

#include "base/memory/raw_ptr.h"
#include "base/synchronization/lock.h"
#include "third_party/blink/public/platform/modules/mediastream/web_media_stream_track.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_component.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_source.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_track_platform.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class WebLocalFrame;

class PLATFORM_EXPORT MediaStreamComponentImpl final
    : public GarbageCollected<MediaStreamComponentImpl>,
      public MediaStreamComponent {
  USING_PRE_FINALIZER(MediaStreamComponentImpl, Dispose);

 private:
  static int GenerateUniqueId();

 public:
  MediaStreamComponentImpl(const String& id,
                           MediaStreamSource*,
                           std::unique_ptr<MediaStreamTrackPlatform>);
  MediaStreamComponentImpl(MediaStreamSource*,
                           std::unique_ptr<MediaStreamTrackPlatform>);

  MediaStreamComponentImpl* Clone() const override;

  // |m_trackData| may hold pointers to GC objects indirectly, and it may touch
  // eagerly finalized objects in destruction.
  // So this class runs pre-finalizer to finalize |m_trackData| promptly.
  void Dispose();

  MediaStreamSource* Source() const override { return source_.Get(); }

  String Id() const override { return id_; }
  int UniqueId() const override { return unique_id_; }
  MediaStreamSource::StreamType GetSourceType() const override {
    return Source()->GetType();
  }
  const String& GetSourceName() const override { return Source()->GetName(); }
  MediaStreamSource::ReadyState GetReadyState() const override {
    return Source()->GetReadyState();
  }
  bool Remote() const override { return Source()->Remote(); }
  bool Enabled() const override { return enabled_; }
  void SetEnabled(bool enabled) override;
  WebMediaStreamTrack::ContentHintType ContentHint() override {
    return content_hint_;
  }
  void SetContentHint(WebMediaStreamTrack::ContentHintType) override;

  MediaStreamTrackPlatform* GetPlatformTrack() const override {
    return platform_track_.get();
  }

  void GetSettings(MediaStreamTrackPlatform::Settings&) override;
  MediaStreamTrackPlatform::CaptureHandle GetCaptureHandle() override;

  WebLocalFrame* CreationFrame() override {
    return creation_frame_getter_ ? creation_frame_getter_.Run() : nullptr;
  }
  void SetCreationFrameGetter(base::RepeatingCallback<WebLocalFrame*()>
                                  creation_frame_getter) override {
    creation_frame_getter_ = std::move(creation_frame_getter);
  }

  void AddSourceObserver(MediaStreamSource::Observer* observer) override;
  void AddSink(WebMediaStreamAudioSink* sink) override;
  void AddSink(WebMediaStreamSink* sink,
               const VideoCaptureDeliverFrameCB& callback,
               MediaStreamVideoSink::IsSecure is_secure,
               MediaStreamVideoSink::UsesAlpha uses_alpha) override;

  String ToString() const override;

  void Trace(Visitor*) const override;

 private:
  Member<MediaStreamSource> source_;

  const String id_;
  const int unique_id_;
  bool enabled_ = true;
  WebMediaStreamTrack::ContentHintType content_hint_ =
      WebMediaStreamTrack::ContentHintType::kNone;
  std::unique_ptr<MediaStreamTrackPlatform> platform_track_;
  // Getter returning the frame where the referenced platform track was created,
  // if it's still alive.
  // Stored as a callback in which code in modules/ can bind a weak pointer to
  // the GCed core class LocalFrame, as this platform/ class can't depend on
  // core/.
  base::RepeatingCallback<WebLocalFrame*()> creation_frame_getter_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIASTREAM_MEDIA_STREAM_COMPONENT_IMPL_H_
