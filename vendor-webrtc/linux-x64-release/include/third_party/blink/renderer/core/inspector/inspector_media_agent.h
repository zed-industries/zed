// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_MEDIA_AGENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_MEDIA_AGENT_H_

#include "third_party/blink/public/web/web_media_inspector.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/inspector/inspected_frames.h"
#include "third_party/blink/renderer/core/inspector/inspector_base_agent.h"
#include "third_party/blink/renderer/core/inspector/protocol/media.h"

namespace blink {

class InspectedFrames;
class WorkerGlobalScope;
class ExecutionContext;

class CORE_EXPORT InspectorMediaAgent final
    : public InspectorBaseAgent<protocol::Media::Metainfo> {
 public:
  explicit InspectorMediaAgent(InspectedFrames*, WorkerGlobalScope*);
  InspectorMediaAgent(const InspectorMediaAgent&) = delete;
  InspectorMediaAgent& operator=(const InspectorMediaAgent&) = delete;
  ~InspectorMediaAgent() override;

  ExecutionContext* GetTargetExecutionContext() const;

  // BaseAgent methods.
  void Restore() override;

  // Protocol receive messages.
  protocol::Response enable() override;
  protocol::Response disable() override;

  // Protocol send messages.
  void PlayerErrorsRaised(const WebString&,
                          const Vector<InspectorPlayerError>&);
  void PlayerEventsAdded(const WebString&, const Vector<InspectorPlayerEvent>&);
  void PlayerMessagesLogged(const WebString&,
                            const Vector<InspectorPlayerMessage>&);
  void PlayerPropertiesChanged(const WebString&,
                               const Vector<InspectorPlayerProperty>&);
  void PlayersCreated(const Vector<WebString>&);

  // blink-gc methods.
  void Trace(Visitor*) const override;

 private:
  void RegisterAgent();

  // This is null while inspecting workers.
  Member<InspectedFrames> inspected_frames_;
  // This is null while inspecting frames.
  Member<WorkerGlobalScope> worker_global_scope_;

  InspectorAgentState::Boolean enabled_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_MEDIA_AGENT_H_
