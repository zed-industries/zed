// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_INSPECTOR_WEB_AUDIO_AGENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_INSPECTOR_WEB_AUDIO_AGENT_H_

#include <memory>

#include "third_party/blink/renderer/core/inspector/inspector_base_agent.h"
#include "third_party/blink/renderer/core/inspector/protocol/web_audio.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

class AudioListener;
class AudioNode;
class AudioParam;
class BaseAudioContext;
class Page;

using protocol::WebAudio::ContextRealtimeData;

class MODULES_EXPORT InspectorWebAudioAgent final
    : public InspectorBaseAgent<protocol::WebAudio::Metainfo> {
 public:
  explicit InspectorWebAudioAgent(Page*);

  InspectorWebAudioAgent(const InspectorWebAudioAgent&) = delete;
  InspectorWebAudioAgent& operator=(const InspectorWebAudioAgent&) = delete;

  ~InspectorWebAudioAgent() override;

  // Base Agent methods.
  void Restore() override;

  // Protocol method implementations (agent -> front_end/web_audio)
  protocol::Response enable() override;
  protocol::Response disable() override;
  protocol::Response getRealtimeData(
      const protocol::WebAudio::GraphObjectId&,
      std::unique_ptr<ContextRealtimeData>*) override;

  // API for InspectorInstrumentation (modules/webaudio -> agent)
  void DidCreateBaseAudioContext(BaseAudioContext*);
  void WillDestroyBaseAudioContext(BaseAudioContext*);
  void DidChangeBaseAudioContext(BaseAudioContext*);
  void DidCreateAudioListener(AudioListener*);
  void WillDestroyAudioListener(AudioListener*);
  void DidCreateAudioNode(AudioNode*);
  void WillDestroyAudioNode(AudioNode*);
  void DidCreateAudioParam(AudioParam*);
  void WillDestroyAudioParam(AudioParam*);
  void DidConnectNodes(AudioNode* source_node,
                       AudioNode* destination_node,
                       int32_t source_output_index = 0,
                       int32_t destination_input_index = 0);
  void DidDisconnectNodes(AudioNode* source_node,
                          AudioNode* destination_node = nullptr,
                          int32_t source_output_index = 0,
                          int32_t destination_input_index = 0);
  void DidConnectNodeParam(AudioNode* source_node,
                           AudioParam* destination_param,
                           int32_t source_output_index = 0);
  void DidDisconnectNodeParam(AudioNode* source_node,
                              AudioParam* destination_param,
                              int32_t source_output_index = 0);

  void Trace(Visitor*) const override;

 private:
  std::unique_ptr<protocol::WebAudio::BaseAudioContext>
      BuildProtocolContext(BaseAudioContext*);

  Member<Page> page_;
  InspectorAgentState::Boolean enabled_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_INSPECTOR_WEB_AUDIO_AGENT_H_
