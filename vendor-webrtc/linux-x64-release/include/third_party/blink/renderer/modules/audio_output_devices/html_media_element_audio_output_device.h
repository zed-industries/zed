// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_AUDIO_OUTPUT_DEVICES_HTML_MEDIA_ELEMENT_AUDIO_OUTPUT_DEVICE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_AUDIO_OUTPUT_DEVICES_HTML_MEDIA_ELEMENT_AUDIO_OUTPUT_DEVICE_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/html/media/audio_output_device_controller.h"
#include "third_party/blink/renderer/core/html/media/html_media_element.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class HTMLMediaElement;
class ScriptState;

class MODULES_EXPORT HTMLMediaElementAudioOutputDevice final
    : public GarbageCollected<HTMLMediaElementAudioOutputDevice>,
      public AudioOutputDeviceController {
 public:
  HTMLMediaElementAudioOutputDevice(HTMLMediaElement&);

  static HTMLMediaElementAudioOutputDevice& From(HTMLMediaElement&);

  static String sinkId(HTMLMediaElement&);
  static ScriptPromise<IDLUndefined> setSinkId(ScriptState*,
                                               HTMLMediaElement&,
                                               const String& sink_id);
  void setSinkId(const String&);

  // AudioOutputDeviceController implementation.
  void SetSinkId(const String&) override;

  void Trace(Visitor*) const override;

 private:
  String sink_id_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_AUDIO_OUTPUT_DEVICES_HTML_MEDIA_ELEMENT_AUDIO_OUTPUT_DEVICE_H_
