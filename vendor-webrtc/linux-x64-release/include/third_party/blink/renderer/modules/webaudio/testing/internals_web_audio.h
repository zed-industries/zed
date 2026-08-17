// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_TESTING_INTERNALS_WEB_AUDIO_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_TESTING_INTERNALS_WEB_AUDIO_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Internals;
class AudioContext;

class InternalsWebAudio {
  STATIC_ONLY(InternalsWebAudio);

 public:
  static unsigned audioHandlerCount(Internals&);
  static unsigned audioWorkletProcessorCount(Internals&);
  static void emulateDeviceFailureOnAudioContext(Internals&, AudioContext*);
  static bool isSilenceDetectionActive(Internals&, AudioContext*);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_TESTING_INTERNALS_WEB_AUDIO_H_
