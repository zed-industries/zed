// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_NODE_WIRING_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_NODE_WIRING_H_

#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class AudioNodeInput;
class AudioNodeOutput;
class AudioParamHandler;

// Utilities for connecting AudioHandlers to one another, via AudioNodeInput and
// AudioNodeOutput. Gathered into one place that can see the internals of both,
// to avoid both needing to call one another back and forth.
//
// An AudioNodeOutput can also be connected to an AudioParamHandler.
//
// All functions require the graph lock.
class MODULES_EXPORT AudioNodeWiring {
  STATIC_ONLY(AudioNodeWiring);

 public:
  // Make or break a connection from an output of one audio node to an input of
  // another.
  static void Connect(AudioNodeOutput&, AudioNodeInput&);
  static void Connect(AudioNodeOutput&, AudioParamHandler&);
  static void Disconnect(AudioNodeOutput&, AudioNodeInput&);
  static void Disconnect(AudioNodeOutput&, AudioParamHandler&);

  // Disable the connection from an output to an input, setting it aside in
  // a separate list of disabled connections. Enabling does the reverse.
  // Should be called only from AudioNodeOutput, when its state has changed.
  static void Disable(AudioNodeOutput&, AudioNodeInput&);
  static void Enable(AudioNodeOutput&, AudioNodeInput&);

  // Queries whether a connection exists, disabled or not.
  static bool IsConnected(AudioNodeOutput&, AudioNodeInput&);
  static bool IsConnected(AudioNodeOutput&, AudioParamHandler&);

  // Called before complete destruction to remove any remaining connections.
  static void WillBeDestroyed(AudioNodeInput&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_NODE_WIRING_H_
