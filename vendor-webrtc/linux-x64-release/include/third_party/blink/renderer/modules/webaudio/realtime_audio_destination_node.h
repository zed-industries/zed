/*
 * Copyright (C) 2011, Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_REALTIME_AUDIO_DESTINATION_NODE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_REALTIME_AUDIO_DESTINATION_NODE_H_

#include <atomic>
#include <memory>

#include "base/memory/weak_ptr.h"
#include "media/base/output_device_info.h"
#include "third_party/blink/public/platform/web_audio_latency_hint.h"
#include "third_party/blink/public/platform/web_audio_sink_descriptor.h"
#include "third_party/blink/renderer/modules/webaudio/audio_destination_node.h"
#include "third_party/blink/renderer/modules/webaudio/realtime_audio_destination_handler.h"
#include "third_party/blink/renderer/platform/audio/audio_callback_metric_reporter.h"
#include "third_party/blink/renderer/platform/audio/audio_destination.h"
#include "third_party/blink/renderer/platform/audio/audio_io_callback.h"

namespace blink {

class AudioContext;
class ExceptionState;
class WebAudioLatencyHint;
class WebAudioSinkDescriptor;

class MODULES_EXPORT RealtimeAudioDestinationNode final
    : public AudioDestinationNode {
 public:
  static RealtimeAudioDestinationNode* Create(
      AudioContext*,
      const WebAudioSinkDescriptor&,
      const WebAudioLatencyHint&,
      std::optional<float> sample_rate,
      bool update_echo_cancellation_on_first_start);

  explicit RealtimeAudioDestinationNode(
      AudioContext&,
      const WebAudioSinkDescriptor&,
      const WebAudioLatencyHint&,
      std::optional<float> sample_rate,
      bool update_echo_cancellation_on_first_start);

  // Returns its own handler object instead of a generic one from
  // AudioNode::Handler().
  RealtimeAudioDestinationHandler& GetOwnHandler() const;

  // See `RealtimeAudioDestinationHandler.SetSinkDescriptor` for details.
  void SetSinkDescriptor(const WebAudioSinkDescriptor&,
                         media::OutputDeviceStatusCB);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_REALTIME_AUDIO_DESTINATION_NODE_H_
