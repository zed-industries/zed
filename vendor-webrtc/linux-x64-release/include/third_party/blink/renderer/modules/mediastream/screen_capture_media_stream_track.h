// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_SCREEN_CAPTURE_MEDIA_STREAM_TRACK_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_SCREEN_CAPTURE_MEDIA_STREAM_TRACK_H_

#include "build/build_config.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/modules/mediastream/media_stream_track_impl.h"
#include "third_party/blink/renderer/modules/mediastream/user_media_request.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_descriptor.h"

namespace blink {

class ScreenDetailed;
class ScreenDetails;

// TODO(crbug.com/1334583): Implement or disable transferbility of screen
// capture tracks.
class MODULES_EXPORT ScreenCaptureMediaStreamTrack final
    : public MediaStreamTrackImpl {
  DEFINE_WRAPPERTYPEINFO();

 public:
  ScreenCaptureMediaStreamTrack(ExecutionContext* context,
                                MediaStreamComponent* component,
                                ScreenDetails* screen_details,
                                ScreenDetailed* screen_detailed);
  ~ScreenCaptureMediaStreamTrack() override = default;

  ScreenDetailed* screenDetailed(ScriptState* script_state,
                                 ExceptionState& exception_state);

  void Trace(Visitor*) const override;

 private:
  // |screen_details_| needs to be stored here as it is the controlling object
  // of |screen_detailed_|, i.e. it updates the |screen_detailed_| object with
  // new screen metadata if the setup changes.
  const Member<ScreenDetails> screen_details_;
  const Member<ScreenDetailed> screen_detailed_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_SCREEN_CAPTURE_MEDIA_STREAM_TRACK_H_
