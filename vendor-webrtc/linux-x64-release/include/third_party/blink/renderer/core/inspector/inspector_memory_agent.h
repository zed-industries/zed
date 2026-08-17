/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_MEMORY_AGENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_MEMORY_AGENT_H_

#include "base/sampling_heap_profiler/poisson_allocation_sampler.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/inspector/inspector_base_agent.h"
#include "third_party/blink/renderer/core/inspector/protocol/memory.h"

namespace blink {

class InspectedFrames;

class CORE_EXPORT InspectorMemoryAgent final
    : public InspectorBaseAgent<protocol::Memory::Metainfo> {
 public:
  explicit InspectorMemoryAgent(InspectedFrames*);
  InspectorMemoryAgent(const InspectorMemoryAgent&) = delete;
  InspectorMemoryAgent& operator=(const InspectorMemoryAgent&) = delete;
  ~InspectorMemoryAgent() override;
  void Trace(Visitor*) const override;

  void Restore() override;

  protocol::Response getDOMCounters(int* documents,
                                    int* nodes,
                                    int* js_event_listeners) override;

  protocol::Response forciblyPurgeJavaScriptMemory() override;

  // Memory protocol domain:
  protocol::Response startSampling(
      std::optional<int> in_samplingInterval,
      std::optional<bool> in_suppressRandomness) override;
  protocol::Response stopSampling() override;
  protocol::Response getSamplingProfile(
      std::unique_ptr<protocol::Memory::SamplingProfile>*) override;
  protocol::Response getAllTimeSamplingProfile(
      std::unique_ptr<protocol::Memory::SamplingProfile>*) override;

 private:
  Vector<String> Symbolize(const std::vector<const void*>& addresses);
  std::unique_ptr<protocol::Memory::SamplingProfile> GetSamplingProfileById(
      uint32_t id);

  Member<InspectedFrames> frames_;
  uint32_t profile_id_ = 0;
  HashMap<const void*, String> symbols_cache_;

  InspectorAgentState::Integer sampling_profile_interval_;

  // Allows web tests to suppress randomness while testing the sampling.
  // The sampler will behave deterministically while an instance of this exists.
  std::unique_ptr<
      base::PoissonAllocationSampler::ScopedSuppressRandomnessForTesting>
      randomness_suppressor_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_MEMORY_AGENT_H_
