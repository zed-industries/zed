// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_WORKLET_PROCESSOR_ERROR_STATE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_WORKLET_PROCESSOR_ERROR_STATE_H_

namespace blink {

// A list of state regarding the error in AudioWorkletProcessor object.
enum class AudioWorkletProcessorErrorState : unsigned {
  // The constructor or the process method in the processor has not thrown any
  // exception.
  kNoError = 0,

  // An exception thrown from the construction failure.
  kConstructionError = 1,

  // An exception thrown from the process method.
  kProcessError = 2,

  // An exception thrown if the process method is undefined.
  kProcessMethodUndefinedError = 3,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_WORKLET_PROCESSOR_ERROR_STATE_H_
