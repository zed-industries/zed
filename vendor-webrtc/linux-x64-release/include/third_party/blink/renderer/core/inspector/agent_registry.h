// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_AGENT_REGISTRY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_AGENT_REGISTRY_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

// A set of agents. Support modification while iterating by means of
// Copy-On-Write.
template <class AgentType>
class CORE_EXPORT AgentRegistry {
  DISALLOW_NEW();

 public:
  // Add an agent to this list. An agent should not be added to the same
  // list more than once.
  void AddAgent(AgentType* agent) {
    if (HasAgent(agent))
      return;
    if (!RequiresCopy()) {
      agents_.push_back(agent);
      return;
    }
    HeapVector<Member<AgentType>> new_agents = agents_;
    new_agents.push_back(agent);
    agents_.swap(new_agents);
    iteration_counter_ = 0;
  }

  // Removes the given agent from this list. Does nothing if this agent is
  // not in this list.
  void RemoveAgent(AgentType* agent) {
    if (!HasAgent(agent))
      return;
    wtf_size_t position = agents_.Find(agent);
    if (!RequiresCopy()) {
      agents_.EraseAt(position);
      return;
    }
    HeapVector<Member<AgentType>> new_agents = agents_;
    new_agents.EraseAt(position);
    agents_.swap(new_agents);
    iteration_counter_ = 0;
  }

  // Returns true if the list is being iterated over and requires copy for
  // modification.
  bool RequiresCopy() const { return iteration_counter_ != 0; }

  bool IsEmpty() const { return agents_.empty(); }

  wtf_size_t size() const { return agents_.size(); }

  bool HasAgent(AgentType* agent) const {
    return agents_.Find(agent) != kNotFound;
  }

  // Safely iterate over the registered agents.
  //
  // Sample usage:
  //     ForEachAgent([](AgentType* agent) {
  //       agent->SomeMethod();
  //     });
  template <typename ForEachCallable>
  void ForEachAgent(const ForEachCallable& callable) const {
    iteration_counter_++;
    for (const Member<AgentType>& agent : agents_) {
      callable(agent);
    }
    if (iteration_counter_ > 0)
      iteration_counter_--;
  }

  void Trace(Visitor* visitor) const { visitor->Trace(agents_); }

 private:
  // Number of iteration over original vector is recorded.
  mutable size_t iteration_counter_ = 0;
  HeapVector<Member<AgentType>> agents_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_AGENT_REGISTRY_H_
