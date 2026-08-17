// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_TESTING_INTERNALS_MEDIA_STREAM_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_TESTING_INTERNALS_MEDIA_STREAM_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Internals;
class MediaDeviceInfo;
class MediaStreamTrack;
class MediaTrackConstraints;
class ScriptState;

class InternalsMediaStream {
  STATIC_ONLY(InternalsMediaStream);

 public:
  static ScriptPromise<IDLUndefined> addFakeDevice(
      ScriptState*,
      Internals&,
      const MediaDeviceInfo*,
      const MediaTrackConstraints* capabilities,
      const MediaStreamTrack* data_source);

  // Trigger a fake device capture configuration change on video track source.
  static void fakeCaptureConfigurationChanged(Internals&,
                                              MediaStreamTrack* track);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_TESTING_INTERNALS_MEDIA_STREAM_H_
