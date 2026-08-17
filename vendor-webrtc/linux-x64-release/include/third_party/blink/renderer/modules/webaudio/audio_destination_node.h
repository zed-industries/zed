/*
 * Copyright (C) 2010, Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1.  Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH
 * DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_DESTINATION_NODE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_DESTINATION_NODE_H_

#include "media/base/output_device_info.h"
#include "third_party/blink/renderer/modules/webaudio/audio_destination_handler.h"
#include "third_party/blink/renderer/modules/webaudio/audio_node.h"

namespace blink {

class AudioDestinationHandler;
class BaseAudioContext;

// AudioDestinationNode (ADN) is a base class of two different types of nodes:
//   1. DefaultDestinationNode for AudioContext (real time)
//   2. OfflineDestinationNode for OfflineAudioContext (non-real time)
// They have different rendering mechanisms, so the AudioDestinationHandler
// (ADH), which is a counterpart of the destination node, encapsulates a
// different rendering backend.
class MODULES_EXPORT AudioDestinationNode : public AudioNode {
  DEFINE_WRAPPERTYPEINFO();

 public:
  uint32_t maxChannelCount() const;

  // Returns its own handler object instead of a generic one from
  // AudioNode::Handler().
  AudioDestinationHandler& GetAudioDestinationHandler() const;

  // InspectorHelperMixin: Note that this node belongs to BaseAudioContext,
  // so these methods are invoked by the parent context.
  void ReportDidCreate() final;
  void ReportWillBeDestroyed() final;

 protected:
  explicit AudioDestinationNode(BaseAudioContext&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_DESTINATION_NODE_H_
