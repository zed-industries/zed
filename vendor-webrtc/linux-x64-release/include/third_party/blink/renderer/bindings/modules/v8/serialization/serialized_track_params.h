// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_MODULES_V8_SERIALIZATION_SERIALIZED_TRACK_PARAMS_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_MODULES_V8_SERIALIZATION_SERIALIZED_TRACK_PARAMS_H_

#include "third_party/blink/renderer/modules/mediastream/media_stream_track.h"

namespace blink {

// Enum values must remain contiguous and starting at zero.
enum class SerializedContentHintType : uint32_t {
  kNone = 0,
  kAudioSpeech = 1,
  kAudioMusic = 2,
  kVideoMotion = 3,
  kVideoDetail = 4,
  kVideoText = 5,
  kLast = kVideoText,
};

// Enum values must remain contiguous and starting at zero.
enum class SerializedReadyState : uint32_t {
  kReadyStateLive = 0,
  kReadyStateMuted = 1,
  kReadyStateEnded = 2,
  kLast = kReadyStateEnded
};

// Enum values must remain contiguous and starting at zero.
enum class SerializedTrackImplSubtype : uint32_t {
  kTrackImplSubtypeBase = 0,            // MediaStreamTrack
  kTrackImplSubtypeCanvasCapture = 1,   // CanvasCaptureMediaStreamTrack
  kTrackImplSubtypeGenerator = 2,       // MediaStreamTrackGenerator
  kTrackImplSubtypeBrowserCapture = 3,  // BrowserCaptureMediaStreamTrack
  kLast = kTrackImplSubtypeBrowserCapture
};

SerializedContentHintType SerializeContentHint(
    WebMediaStreamTrack::ContentHintType type);

SerializedReadyState SerializeReadyState(MediaStreamSource::ReadyState state);

SerializedTrackImplSubtype SerializeTrackImplSubtype(
    ScriptWrappable::TypeDispatcher& dispatcher);

WebMediaStreamTrack::ContentHintType DeserializeContentHint(
    SerializedContentHintType type);

MediaStreamSource::ReadyState DeserializeReadyState(SerializedReadyState state);

const WrapperTypeInfo* DeserializeTrackImplSubtype(
    SerializedTrackImplSubtype type);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_MODULES_V8_SERIALIZATION_SERIALIZED_TRACK_PARAMS_H_
