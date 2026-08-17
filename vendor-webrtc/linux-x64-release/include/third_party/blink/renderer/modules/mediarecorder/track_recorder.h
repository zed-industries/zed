// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIARECORDER_TRACK_RECORDER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIARECORDER_TRACK_RECORDER_H_

#include "third_party/blink/public/platform/modules/mediastream/web_media_stream_sink.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

// This class exists to give MediaRecorder support for behavior related to
// all tracks ending.
// It's a template because VideoTrackRecorder and AudioTrackRecorder inherit
// from different base classes.
// Note: this class supports instantiation with classes that inherit from
// WebMediaStreamSink.
template <class MediaStreamSink>
class TrackRecorder : public MediaStreamSink {
 public:
  explicit TrackRecorder(base::OnceClosure track_ended_cb);
  ~TrackRecorder() override = default;

  // WebMediaStreamSink
  void OnReadyStateChanged(WebMediaStreamSource::ReadyState state) override;

 private:
  base::OnceClosure track_ended_cb_;
};

template <class MediaStreamSink>
TrackRecorder<MediaStreamSink>::TrackRecorder(base::OnceClosure track_ended_cb)
    : track_ended_cb_(std::move(track_ended_cb)) {}

template <class MediaStreamSink>
void TrackRecorder<MediaStreamSink>::OnReadyStateChanged(
    WebMediaStreamSource::ReadyState state) {
  if (state == WebMediaStreamSource::kReadyStateEnded)
    std::move(track_ended_cb_).Run();
}

// It is muxer container type for the video and audio types.
enum class MediaTrackContainerType {
  kNone,
  kVideoMp4,
  kVideoWebM,
  kVidoMatroska,
  kAudioMp4,
  kAudioWebM,
};

MODULES_EXPORT MediaTrackContainerType
GetMediaContainerTypeFromString(const WTF::String& type);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIARECORDER_TRACK_RECORDER_H_
