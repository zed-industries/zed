// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIASTREAM_TRANSFERRED_MEDIA_STREAM_COMPONENT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIASTREAM_TRANSFERRED_MEDIA_STREAM_COMPONENT_H_

#include "base/memory/raw_ptr.h"
#include "third_party/blink/public/platform/modules/mediastream/web_media_stream_track.h"
#include "third_party/blink/renderer/platform/audio/audio_source_provider.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_component.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_track_platform.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class MediaStreamSource;
class WebLocalFrame;

class PLATFORM_EXPORT TransferredMediaStreamComponent final
    : public GarbageCollected<TransferredMediaStreamComponent>,
      public MediaStreamComponent {
 public:
  // For carrying deserialized data to the TransferredMediaStreamComponent
  // constructor.
  struct TransferredValues {
    String id;
  };

  explicit TransferredMediaStreamComponent(const TransferredValues& data);

  TransferredMediaStreamComponent(const TransferredMediaStreamComponent&) =
      delete;
  TransferredMediaStreamComponent& operator=(
      const TransferredMediaStreamComponent&) = delete;

  void SetImplementation(MediaStreamComponent* component);

  MediaStreamComponent* Clone() const override;

  MediaStreamSource* Source() const override;

  String Id() const override;
  int UniqueId() const override;
  MediaStreamSource::StreamType GetSourceType() const override;
  const String& GetSourceName() const override;
  MediaStreamSource::ReadyState GetReadyState() const override;
  bool Remote() const override;
  bool Enabled() const override;
  void SetEnabled(bool enabled) override;
  WebMediaStreamTrack::ContentHintType ContentHint() override;
  void SetContentHint(WebMediaStreamTrack::ContentHintType) override;

  MediaStreamTrackPlatform* GetPlatformTrack() const override;

  void GetSettings(MediaStreamTrackPlatform::Settings&) override;
  MediaStreamTrackPlatform::CaptureHandle GetCaptureHandle() override;

  WebLocalFrame* CreationFrame() override;
  void SetCreationFrameGetter(
      base::RepeatingCallback<WebLocalFrame*()>) override;

  void AddSourceObserver(MediaStreamSource::Observer* observer) override;
  void AddSink(WebMediaStreamAudioSink* sink) override;
  void AddSink(WebMediaStreamSink* sink,
               const VideoCaptureDeliverFrameCB& callback,
               MediaStreamVideoSink::IsSecure is_secure,
               MediaStreamVideoSink::UsesAlpha uses_alpha) override;

  String ToString() const override;

  void Trace(Visitor*) const override;

 private:
  struct AddSinkArgs {
    raw_ptr<WebMediaStreamSink> sink;
    VideoCaptureDeliverFrameCB callback;
    MediaStreamVideoSink::IsSecure is_secure;
    MediaStreamVideoSink::UsesAlpha uses_alpha;
  };

  Member<MediaStreamComponent> component_;
  TransferredValues data_;

  HeapVector<Member<MediaStreamSource::Observer>> observers_;
  Vector<AddSinkArgs> add_video_sink_calls_;
  Vector<WebMediaStreamAudioSink*> add_audio_sink_calls_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIASTREAM_TRANSFERRED_MEDIA_STREAM_COMPONENT_H_
