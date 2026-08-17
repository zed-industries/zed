// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EXECUTION_CONTEXT_WINDOW_AGENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EXECUTION_CONTEXT_WINDOW_AGENT_H_

#include "third_party/blink/renderer/core/execution_context/agent.h"
#include "third_party/blink/renderer/platform/scheduler/public/agent_group_scheduler.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

// This corresponds to similar-origin window agent, that is shared by a group
// of Documents that are mutually reachable and have the same-site origins.
// https://html.spec.whatwg.org/C#similar-origin-window-agent
//
// The instance holds per-agent data in addition to the base Agent, that is also
// shared by associated Documents.
class WindowAgent final : public Agent, public AgentGroupScheduler::Agent {
 public:
  // Normally you don't want to call this constructor; instead, use
  // WindowAgentFactory::GetAgentForOrigin() so you can get the agent shared
  // on the same-site frames.
  //
  // This constructor creates a unique agent that won't be shared with any
  // other frames. Use this constructor only if:
  //   - An appropriate instance of WindowAgentFactory is not available
  //     (this should only happen in tests).
  explicit WindowAgent(AgentGroupScheduler& agent_group_scheduler);

  // Normally you don't want to call this constructor; instead, use
  // WindowAgentFactory::GetAgentForOrigin() so you can get the agent shared
  // on the same-site frames. (Same as the constructor above.)
  //
  // This constructor calls WindowAgent::WindowAgent(isolate), but also stores
  // the state of origin agent clustering.
  WindowAgent(AgentGroupScheduler& agent_group_scheduler,
              bool is_origin_agent_cluster,
              bool origin_agent_cluster_left_as_default);

  ~WindowAgent() override;

  // Agent overrides.
  void Trace(Visitor*) const override;
  bool IsWindowAgent() const override;

  // AgentGroupScheduler::Agent overrides.
  void PerformMicrotaskCheckpoint() override;

  AgentGroupScheduler& GetAgentGroupScheduler();

 private:
  // Note clients may attach per-agent data via Supplementable.
  // MutationObservers are attached this way.
  // TODO(keishi): Move per-agent data here with the correct granularity.
  // E.g. CustomElementReactionStack should move here.
  Member<AgentGroupScheduler> agent_group_scheduler_;
};

template <>
struct DowncastTraits<WindowAgent> {
  static bool AllowFrom(const Agent& agent) { return agent.IsWindowAgent(); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EXECUTION_CONTEXT_WINDOW_AGENT_H_
